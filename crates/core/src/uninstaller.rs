//! Desinstalador Profundo Multiplataforma con Rastreo de Carpetas Residuales
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
#[cfg(windows)]
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
    #[cfg(windows)]
    const UNINSTALL_KEY: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
    #[cfg(windows)]
    const WOW64_UNINSTALL_KEY: &'static str = "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

    #[cfg(windows)]
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

    /// Lista todas las aplicaciones instaladas en el sistema
    pub fn list_installed_apps() -> Vec<InstalledApp> {
        let mut apps = Vec::new();

        #[cfg(windows)]
        {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            apps.extend(Self::scan_uninstall_key(&hkcu, Self::UNINSTALL_KEY, false));

            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            apps.extend(Self::scan_uninstall_key(&hklm, Self::UNINSTALL_KEY, true));
            apps.extend(Self::scan_uninstall_key(&hklm, Self::WOW64_UNINSTALL_KEY, true));
        }

        #[cfg(target_os = "linux")]
        {
            let app_dirs = [
                PathBuf::from("/usr/share/applications"),
                dirs_fallback_local_share(),
            ];

            for dir in &app_dirs {
                if let Ok(read_dir) = fs::read_dir(dir) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                            let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                            apps.push(InstalledApp {
                                id: format!("linux_{}", name),
                                name: name.clone(),
                                version: "Linux Desktop App".to_string(),
                                publisher: "OpenDesktop".to_string(),
                                uninstall_string: format!("rm -f '{}'", path.display()),
                                quiet_uninstall_string: None,
                                install_location: Some(path.to_string_lossy().to_string()),
                            });
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let app_dirs = [
                PathBuf::from("/Applications"),
                dirs_fallback_macos_apps(),
            ];

            for dir in &app_dirs {
                if let Ok(read_dir) = fs::read_dir(dir) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.extension().map(|e| e == "app").unwrap_or(false) {
                            let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                            apps.push(InstalledApp {
                                id: format!("macos_{}", name),
                                name: name.clone(),
                                version: "macOS App Bundle".to_string(),
                                publisher: "Apple / Third-Party".to_string(),
                                uninstall_string: format!("trash '{}'", path.display()),
                                quiet_uninstall_string: None,
                                install_location: Some(path.to_string_lossy().to_string()),
                            });
                        }
                    }
                }
            }
        }

        apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        apps.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
        apps
    }

    /// Rastrear carpetas residuales huérfanas de una aplicación
    pub fn scan_residuals(app_name: &str) -> Vec<ResidualFolder> {
        let mut residuals = Vec::new();
        let target_name = app_name.to_lowercase();
        let candidate_parents = Self::get_candidate_dirs();

        for parent in candidate_parents {
            if let Ok(read_dir) = fs::read_dir(parent) {
                for entry in read_dir.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        let folder_name = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();

                        if folder_name.contains(&target_name) {
                            let size = Self::calculate_dir_size(&path);
                            residuals.push(ResidualFolder {
                                path,
                                size_bytes: size,
                            });
                        }
                    }
                }
            }
        }

        residuals
    }

    fn get_candidate_dirs() -> Vec<PathBuf> {
        let mut list = Vec::new();

        #[cfg(windows)]
        {
            if let Ok(loc) = env::var("LOCALAPPDATA") {
                list.push(PathBuf::from(loc));
            }
            if let Ok(app) = env::var("APPDATA") {
                list.push(PathBuf::from(app));
            }
            if let Ok(prog) = env::var("ProgramData") {
                list.push(PathBuf::from(prog));
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = env::var("HOME") {
                list.push(PathBuf::from(&home).join(".config"));
                list.push(PathBuf::from(&home).join(".local/share"));
                list.push(PathBuf::from(&home).join(".cache"));
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = env::var("HOME") {
                list.push(PathBuf::from(&home).join("Library/Application Support"));
                list.push(PathBuf::from(&home).join("Library/Caches"));
            }
        }

        list
    }

    fn calculate_dir_size(path: &std::path::Path) -> u64 {
        walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.metadata().ok())
            .filter(|m| m.is_file())
            .map(|m| m.len())
            .sum()
    }

    /// Lanza el desinstalador de la aplicación
    pub fn run_uninstaller(app: &InstalledApp) -> anyhow::Result<bool> {
        if app.uninstall_string.trim().is_empty() {
            return Ok(false);
        }

        #[cfg(windows)]
        {
            let status = Command::new("cmd")
                .args(["/C", &app.uninstall_string])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(windows))]
        {
            let status = Command::new("sh")
                .args(["-c", &app.uninstall_string])
                .status()?;
            Ok(status.success())
        }
    }
}

#[cfg(target_os = "linux")]
fn dirs_fallback_local_share() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".local/share/applications"))
        .unwrap_or_else(|_| PathBuf::from("/usr/share/applications"))
}

#[cfg(target_os = "macos")]
fn dirs_fallback_macos_apps() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h).join("Applications"))
        .unwrap_or_else(|_| PathBuf::from("/Applications"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_installed_apps() {
        let apps = UninstallerEngine::list_installed_apps();
        let _ = apps.len();
    }
}
