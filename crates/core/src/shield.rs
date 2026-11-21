//! Escáner de Seguridad, Malware y Persistencias ("RustCare Shield")
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use sysinfo::{ProcessesToUpdate, System};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    Warning,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: PathBuf,
    pub level: ThreatLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShieldScanResult {
    pub threats: Vec<ThreatItem>,
    pub processes_scanned: usize,
    pub registry_keys_scanned: usize,
}

pub struct ShieldEngine;

impl ShieldEngine {
    /// Escanea los procesos en memoria buscando anomalías (ejecución desde temporales, nombres impostores de sistema)
    pub fn scan_processes() -> (Vec<ThreatItem>, usize) {
        let mut threats = Vec::new();
        let mut sys = System::new();
        sys.refresh_processes(ProcessesToUpdate::All, true);

        let temp_dir = env::var("TEMP").map(PathBuf::from).ok();
        let local_temp = env::var("LOCALAPPDATA")
            .map(|l| PathBuf::from(l).join("Temp"))
            .ok();

        // Nombres reservados de procesos esenciales de Windows
        let reserved_system_names = [
            "svchost.exe",
            "csrss.exe",
            "lsass.exe",
            "services.exe",
            "smss.exe",
            "winlogon.exe",
        ];

        let mut count = 0;
        for (pid, proc) in sys.processes() {
            count += 1;
            let name = proc.name().to_string_lossy().to_lowercase();
            let exe_path = match proc.exe() {
                Some(p) => p.to_path_buf(),
                None => continue,
            };

            // 1. Proceso ejecutándose desde directorio temporal
            let is_in_temp = (temp_dir.as_ref().map(|t| exe_path.starts_with(t)).unwrap_or(false))
                || (local_temp.as_ref().map(|t| exe_path.starts_with(t)).unwrap_or(false));

            if is_in_temp {
                threats.push(ThreatItem {
                    id: format!("proc_temp_{}", pid),
                    title: format!("Proceso ejecutándose desde carpeta temporal: {}", name),
                    description: format!(
                        "El proceso (PID {}) se está ejecutando en una carpeta temporal: {}",
                        pid,
                        exe_path.display()
                    ),
                    path: exe_path.clone(),
                    level: ThreatLevel::Warning,
                });
            }

            // 2. Proceso impostor (suplantación de binario crítico de Windows fuera de System32)
            if reserved_system_names.contains(&name.as_str()) {
                let path_str = exe_path.to_string_lossy().to_lowercase();
                if !path_str.contains("system32") && !path_str.contains("syswow64") {
                    threats.push(ThreatItem {
                        id: format!("proc_impostor_{}", pid),
                        title: format!("Proceso impostor del sistema detectado: {}", name),
                        description: format!(
                            "Un binario con nombre '{}' se está ejecutando fuera de System32: {}",
                            name,
                            exe_path.display()
                        ),
                        path: exe_path,
                        level: ThreatLevel::Critical,
                    });
                }
            }
        }

        (threats, count)
    }

    /// Escanea persistencias anómalas en el Registro (Image File Execution Options, AppInit_DLLs)
    pub fn scan_registry_persistences() -> (Vec<ThreatItem>, usize) {
        let mut threats = Vec::new();
        let mut count = 0;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // 1. IFEO (Image File Execution Options / Debugger Hijacks)
        const IFEO_KEY: &str = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options";
        if let Ok(key) = hklm.open_subkey(IFEO_KEY) {
            for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
                count += 1;
                if let Ok(sub) = key.open_subkey(&subkey_name) {
                    if let Ok(debugger) = sub.get_value::<String, _>("Debugger") {
                        if !debugger.trim().is_empty() {
                            threats.push(ThreatItem {
                                id: format!("ifeo_{}", subkey_name),
                                title: format!("Secuestro IFEO (Debugger) detectado en {}", subkey_name),
                                description: format!(
                                    "La aplicación {} tiene asignado el depurador/interceptador: {}",
                                    subkey_name, debugger
                                ),
                                path: PathBuf::from(&debugger),
                                level: ThreatLevel::High,
                            });
                        }
                    }
                }
            }
        }

        // 2. AppInit_DLLs
        const WINDOWS_KEY: &str = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Windows";
        if let Ok(key) = hklm.open_subkey(WINDOWS_KEY) {
            count += 1;
            if let Ok(appinit) = key.get_value::<String, _>("AppInit_DLLs") {
                if !appinit.trim().is_empty() {
                    threats.push(ThreatItem {
                        id: "appinit_dlls".to_string(),
                        title: "Inyección global AppInit_DLLs activa".to_string(),
                        description: format!("Se detectaron DLLs de inyección en todos los procesos: {}", appinit),
                        path: PathBuf::from(&appinit),
                        level: ThreatLevel::Critical,
                    });
                }
            }
        }

        (threats, count)
    }

    /// Ejecuta el escaneo de seguridad completo de RustCare Shield
    pub fn scan_all() -> ShieldScanResult {
        let (proc_threats, processes_scanned) = Self::scan_processes();
        let (reg_threats, registry_keys_scanned) = Self::scan_registry_persistences();

        let mut all_threats = proc_threats;
        all_threats.extend(reg_threats);

        ShieldScanResult {
            threats: all_threats,
            processes_scanned,
            registry_keys_scanned,
        }
    }

    /// Calcula la firma BLAKE3 de un archivo para comprobación de integridad
    pub fn calculate_hash<P: AsRef<Path>>(path: P) -> Option<String> {
        let bytes = std::fs::read(path).ok()?;
        let hash = blake3::hash(&bytes);
        Some(hash.to_hex().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_scan() {
        let result = ShieldEngine::scan_all();
        println!("Procesos analizados por Shield: {}", result.processes_scanned);
        assert!(result.processes_scanned > 0);
    }
}
