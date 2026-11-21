//! Desinstalador Profundo con Rastreo Heurístico de Carpetas Residuales
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    pub id: String,
    pub name: String,
    pub version: String,
    pub publisher: String,
    pub uninstall_string: String,
    pub quiet_uninstall_string: Option<String>,
    pub install_location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidualFolder {
    pub path: PathBuf,
    pub size_bytes: u64,
}

pub struct UninstallerEngine;

impl UninstallerEngine {
    const UNINSTALL_KEY: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
    const WOW64_UNINSTALL_KEY: &'static str = "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

    fn scan_uninstall_key(root: &RegKey, path: &str, is_hklm: bool) -> Vec<InstalledApp> {
        let mut apps = Vec::new();
        if let Ok(key) = root.open_subkey(path) {
            for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
                if let Ok(app_key) = key.open_subkey(&subkey_name) {
                    let display_name: String = match app_key.get_value("DisplayName") {
                        Ok(n) => n,
                        Err(_) => continue,
                    };

                    if display_name.trim().is_empty() {
                        continue;
                    }

                    // Ignorar componentes de sistema o actualizaciones de Windows
                    let is_system: u32 = app_key.get_value("SystemComponent").unwrap_or(0);
                    if is_system == 1 {
                        continue;
                    }

                    let version: String = app_key.get_value("DisplayVersion").unwrap_or_default();
                    let publisher: String = app_key.get_value("Publisher").unwrap_or_default();
                    let uninstall_string: String = app_key.get_value("UninstallString").unwrap_or_default();
                    let quiet_uninstall_string: Option<String> = app_key.get_value("QuietUninstallString").ok();
                    let install_location: Option<String> = app_key.get_value("InstallLocation").ok();

                    let prefix = if is_hklm { "hklm" } else { "hkcu" };
                    apps.push(InstalledApp {
                        id: format!("{}_{}", prefix, subkey_name),
                        name: display_name,
                        version,
                        publisher,
                        uninstall_string,
                        quiet_uninstall_string,
                        install_location,
                    });
                }
            }
        }
        apps
    }

    /// Lista todas las aplicaciones instaladas en la sesión del usuario y a nivel de máquina
    pub fn list_installed_apps() -> Vec<InstalledApp> {
        let mut apps = Vec::new();

        // 1. HKCU Uninstall
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        apps.extend(Self::scan_uninstall_key(&hkcu, Self::UNINSTALL_KEY, false));

        // 2. HKLM Uninstall (64-bit y 32-bit nativo)
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        apps.extend(Self::scan_uninstall_key(&hklm, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall", true));
        apps.extend(Self::scan_uninstall_key(&hklm, Self::WOW64_UNINSTALL_KEY, true));

        // Ordenar alfabéticamente por nombre
        apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        apps
    }

    /// Lanza el desinstalador oficial de la aplicación
    pub fn run_uninstaller(app: &InstalledApp) -> anyhow::Result<bool> {
        let cmd = app
            .quiet_uninstall_string
            .as_ref()
            .unwrap_or(&app.uninstall_string);

        if cmd.trim().is_empty() {
            anyhow::bail!("No hay comando de desinstalación disponible");
        }

        // Ejecutar a través de cmd /C para admitir parámetros y rutas con comillas
        let status = Command::new("cmd")
            .args(["/C", cmd])
            .status()?;

        Ok(status.success())
    }

    /// Escaneo posterior para localizar carpetas residuales huérfanas en %AppData%, %LocalAppData% y %ProgramData%
    pub fn scan_residuals(app_name: &str) -> Vec<ResidualFolder> {
        let mut residuals = Vec::new();
        let target_name = app_name.to_lowercase().trim().to_string();
        if target_name.len() < 3 {
            return residuals;
        }

        let search_dirs = [
            env::var("APPDATA").ok(),
            env::var("LOCALAPPDATA").ok(),
            env::var("ProgramData").ok(),
        ];

        for base in search_dirs.into_iter().flatten() {
            let base_path = PathBuf::from(base);
            if let Ok(entries) = fs::read_dir(&base_path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    if let Ok(ft) = entry.file_type() {
                        if ft.is_dir() {
                            let folder_name = entry.file_name().to_string_lossy().to_lowercase();
                            if folder_name.contains(&target_name) {
                                let path = entry.path();
                                let size = crate::disk_analyzer::DiskAnalyzer::calculate_dir_size(&path);
                                residuals.push(ResidualFolder {
                                    path,
                                    size_bytes: size,
                                });
                            }
                        }
                    }
                }
            }
        }

        residuals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_installed_apps() {
        let apps = UninstallerEngine::list_installed_apps();
        println!("Aplicaciones instaladas detectadas: {}", apps.len());
        // En cualquier máquina Windows habrá al menos algunas aplicaciones registradas
        assert!(!apps.is_empty());
    }
}
