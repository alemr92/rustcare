//! Gestor de Arranque Reversible Multiplataforma (Windows, Linux, macOS)
#[cfg_attr(not(windows), allow(unused_imports))]
use crate::security::is_elevated;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[cfg(windows)]
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
#[cfg(windows)]
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StartupLocation {
    HkcuRun,
    HkcuRunOnce,
    HklmRun,
    StartupFolder,
    AutostartDesktop,
    LaunchAgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupEntry {
    pub id: String,
    pub name: String,
    pub command: String,
    pub location: StartupLocation,
    pub is_enabled: bool,
    pub requires_admin: bool,
}

pub struct StartupEngine;

impl StartupEngine {
    #[cfg(windows)]
    const DISABLED_KEY: &'static str = "Software\\RustCare\\DisabledStartup";
    #[cfg(windows)]
    const HKCU_RUN: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    #[cfg(windows)]
    const HKCU_RUNONCE: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce";
    #[cfg(windows)]
    const HKLM_RUN: &'static str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";

    /// Obtiene la carpeta de inicio del usuario
    pub fn startup_folder() -> Option<PathBuf> {
        #[cfg(windows)]
        {
            std::env::var("APPDATA").ok().map(|appdata| {
                PathBuf::from(appdata)
                    .join("Microsoft")
                    .join("Windows")
                    .join("Start Menu")
                    .join("Programs")
                    .join("Startup")
            })
        }

        #[cfg(target_os = "linux")]
        {
            std::env::var("HOME").ok().map(|home| {
                PathBuf::from(home).join(".config").join("autostart")
            })
        }

        #[cfg(target_os = "macos")]
        {
            std::env::var("HOME").ok().map(|home| {
                PathBuf::from(home).join("Library").join("LaunchAgents")
            })
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            None
        }
    }

    /// Lista todas las entradas de arranque del usuario y del sistema
    pub fn list_entries() -> Vec<StartupEntry> {
        let mut entries = Vec::new();

        #[cfg(windows)]
        {
            let is_admin = is_elevated();

            // 1. HKCU Run
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            if let Ok(key) = hkcu.open_subkey(Self::HKCU_RUN) {
                for val in key.enum_values().filter_map(|v| v.ok()) {
                    entries.push(StartupEntry {
                        id: format!("hkcu_run_{}", val.0),
                        name: val.0.clone(),
                        command: val.1.to_string(),
                        location: StartupLocation::HkcuRun,
                        is_enabled: true,
                        requires_admin: false,
                    });
                }
            }

            // 2. HKCU RunOnce
            if let Ok(key) = hkcu.open_subkey(Self::HKCU_RUNONCE) {
                for val in key.enum_values().filter_map(|v| v.ok()) {
                    entries.push(StartupEntry {
                        id: format!("hkcu_runonce_{}", val.0),
                        name: val.0.clone(),
                        command: val.1.to_string(),
                        location: StartupLocation::HkcuRunOnce,
                        is_enabled: true,
                        requires_admin: false,
                    });
                }
            }

            // 3. HKLM Run
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            if let Ok(key) = hklm.open_subkey(Self::HKLM_RUN) {
                for val in key.enum_values().filter_map(|v| v.ok()) {
                    entries.push(StartupEntry {
                        id: format!("hklm_run_{}", val.0),
                        name: val.0.clone(),
                        command: val.1.to_string(),
                        location: StartupLocation::HklmRun,
                        is_enabled: true,
                        requires_admin: !is_admin,
                    });
                }
            }

            // 4. Carpeta de Inicio
            if let Some(folder) = Self::startup_folder() {
                if let Ok(read_dir) = fs::read_dir(folder) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.is_file() {
                            let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                            entries.push(StartupEntry {
                                id: format!("startup_folder_{}", file_name),
                                name: file_name,
                                command: path.to_string_lossy().to_string(),
                                location: StartupLocation::StartupFolder,
                                is_enabled: true,
                                requires_admin: false,
                            });
                        }
                    }
                }
            }

            // 5. Entradas deshabilitadas en HKCU\Software\RustCare\DisabledStartup
            if let Ok(key) = hkcu.open_subkey(Self::DISABLED_KEY) {
                for val in key.enum_values().filter_map(|v| v.ok()) {
                    entries.push(StartupEntry {
                        id: format!("disabled_{}", val.0),
                        name: val.0.clone(),
                        command: val.1.to_string(),
                        location: StartupLocation::HkcuRun,
                        is_enabled: false,
                        requires_admin: false,
                    });
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(folder) = Self::startup_folder() {
                if let Ok(read_dir) = fs::read_dir(folder) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                            let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                            entries.push(StartupEntry {
                                id: format!("autostart_{}", name),
                                name,
                                command: path.to_string_lossy().to_string(),
                                location: StartupLocation::AutostartDesktop,
                                is_enabled: true,
                                requires_admin: false,
                            });
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(folder) = Self::startup_folder() {
                if let Ok(read_dir) = fs::read_dir(folder) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.extension().map(|e| e == "plist").unwrap_or(false) {
                            let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                            entries.push(StartupEntry {
                                id: format!("launch_agent_{}", name),
                                name,
                                command: path.to_string_lossy().to_string(),
                                location: StartupLocation::LaunchAgent,
                                is_enabled: true,
                                requires_admin: false,
                            });
                        }
                    }
                }
            }
        }

        entries
    }

    /// Deshabilita una entrada de inicio de forma reversible
    pub fn disable_entry(entry: &mut StartupEntry) -> anyhow::Result<()> {
        if !entry.is_enabled {
            return Ok(());
        }

        #[cfg(windows)]
        {
            if let StartupLocation::HkcuRun = entry.location {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                let (disabled_key, _) = hkcu.create_subkey(Self::DISABLED_KEY)?;
                disabled_key.set_value(&entry.name, &entry.command)?;

                if let Ok(run_key) = hkcu.open_subkey_with_flags(Self::HKCU_RUN, winreg::enums::KEY_WRITE) {
                    let _ = run_key.delete_value(&entry.name);
                }
                entry.is_enabled = false;
            }
        }

        #[cfg(not(windows))]
        {
            entry.is_enabled = false;
        }

        Ok(())
    }

    /// Reactiva una entrada deshabilitada previamente
    pub fn enable_entry(entry: &mut StartupEntry) -> anyhow::Result<()> {
        if entry.is_enabled {
            return Ok(());
        }

        #[cfg(windows)]
        {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let (run_key, _) = hkcu.create_subkey(Self::HKCU_RUN)?;
            run_key.set_value(&entry.name, &entry.command)?;

            if let Ok(disabled_key) = hkcu.open_subkey_with_flags(Self::DISABLED_KEY, winreg::enums::KEY_WRITE) {
                let _ = disabled_key.delete_value(&entry.name);
            }
            entry.is_enabled = true;
        }

        #[cfg(not(windows))]
        {
            entry.is_enabled = true;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_startup() {
        let _entries = StartupEngine::list_entries();
    }
}
