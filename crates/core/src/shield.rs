//! Escáner de Seguridad, Malware y Persistencias Multiplataforma ("RustCare Shield")
use serde::{Deserialize, Serialize};
use std::env;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
use sysinfo::{ProcessesToUpdate, System};

#[cfg(windows)]
use winreg::enums::HKEY_LOCAL_MACHINE;
#[cfg(windows)]
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

        let temp_dir = env::var("TEMP").or_else(|_| env::var("TMPDIR")).map(PathBuf::from).ok();
        let local_temp = env::var("LOCALAPPDATA")
            .map(|l| PathBuf::from(l).join("Temp"))
            .ok();

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
                Some(p) => p,
                None => continue,
            };

            // 1. Detección de binarios ejecutándose desde carpetas temporales
            let in_temp = temp_dir.as_ref().map(|t| exe_path.starts_with(t)).unwrap_or(false)
                || local_temp.as_ref().map(|t| exe_path.starts_with(t)).unwrap_or(false)
                || exe_path.starts_with("/tmp")
                || exe_path.starts_with("/dev/shm");

            if in_temp {
                threats.push(ThreatItem {
                    id: format!("proc_temp_{}", pid),
                    title: format!("Proceso sospechoso en carpeta temporal: {}", name),
                    description: format!(
                        "El binario (PID {}) se está ejecutando desde un directorio temporal.",
                        pid
                    ),
                    path: exe_path.to_path_buf(),
                    level: ThreatLevel::High,
                });
            }

            // 2. Detección de binarios impostores de sistema
            if reserved_system_names.contains(&name.as_str()) {
                let is_genuine = exe_path.starts_with("C:\\Windows\\System32")
                    || exe_path.starts_with("C:\\Windows\\SysWOW64");

                if !is_genuine {
                    threats.push(ThreatItem {
                        id: format!("proc_impostor_{}", pid),
                        title: format!("Impostor de binario del sistema detectado: {}", name),
                        description: format!(
                            "Un ejecutable fuera de System32 está utilizando el nombre '{}' (PID {}).",
                            name, pid
                        ),
                        path: exe_path.to_path_buf(),
                        level: ThreatLevel::Critical,
                    });
                }
            }
        }

        (threats, count)
    }

    /// Audita claves de persistencia y secuestro
    pub fn scan_registry_persistences() -> (Vec<ThreatItem>, usize) {
        #[allow(unused_mut)]
        let mut threats = Vec::new();
        #[allow(unused_mut)]
        let mut keys_scanned = 0;

        #[cfg(windows)]
        {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

            // 1. Image File Execution Options (IFEO) - Debugger Hijack
            let ifeo_path = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options";
            if let Ok(ifeo_key) = hklm.open_subkey(ifeo_path) {
                keys_scanned += 1;
                for subkey_name in ifeo_key.enum_keys().filter_map(|k| k.ok()) {
                    keys_scanned += 1;
                    if let Ok(app_key) = ifeo_key.open_subkey(&subkey_name) {
                        if let Ok(debugger) = app_key.get_value::<String, _>("Debugger") {
                            if !debugger.trim().is_empty() {
                                threats.push(ThreatItem {
                                    id: format!("ifeo_{}", subkey_name),
                                    title: format!("Secuestro IFEO (Debugger) detectado en {}", subkey_name),
                                    description: format!(
                                        "El binario {} redirige su ejecución hacia '{}' mediante IFEO.",
                                        subkey_name, debugger
                                    ),
                                    path: PathBuf::from(format!("{}\\{}", ifeo_path, subkey_name)),
                                    level: ThreatLevel::Critical,
                                });
                            }
                        }
                    }
                }
            }

            // 2. AppInit_DLLs
            let winlogon_path = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Windows";
            if let Ok(win_key) = hklm.open_subkey(winlogon_path) {
                keys_scanned += 1;
                if let Ok(appinit) = win_key.get_value::<String, _>("AppInit_DLLs") {
                    if !appinit.trim().is_empty() {
                        threats.push(ThreatItem {
                            id: "appinit_dlls".to_string(),
                            title: "Inyección de AppInit_DLLs activa".to_string(),
                            description: format!(
                                "Librerías registradas para inyección global en todos los procesos: '{}'.",
                                appinit
                            ),
                            path: PathBuf::from(winlogon_path),
                            level: ThreatLevel::High,
                        });
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            keys_scanned += 1;
            if Path::new("/etc/ld.so.preload").exists() {
                threats.push(ThreatItem {
                    id: "ld_so_preload".to_string(),
                    title: "Archivo /etc/ld.so.preload presente".to_string(),
                    description: "Se detectó pre-carga de librerías globales dinámica en el sistema.".to_string(),
                    path: PathBuf::from("/etc/ld.so.preload"),
                    level: ThreatLevel::High,
                });
            }
        }

        (threats, keys_scanned)
    }

    /// Realiza un análisis integral de seguridad
    pub fn scan_all() -> ShieldScanResult {
        let (mut proc_threats, proc_count) = Self::scan_processes();
        let (reg_threats, reg_count) = Self::scan_registry_persistences();

        proc_threats.extend(reg_threats);

        ShieldScanResult {
            threats: proc_threats,
            processes_scanned: proc_count,
            registry_keys_scanned: reg_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_scan() {
        let result = ShieldEngine::scan_all();
        assert!(result.processes_scanned > 0);
    }
}
