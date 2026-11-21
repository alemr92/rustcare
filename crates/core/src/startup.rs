//! Gestor de Arranque Reversible para Windows 10/11
use crate::security::is_elevated;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StartupLocation {
    HkcuRun,
    HkcuRunOnce,
    HklmRun,
    StartupFolder,
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
    const DISABLED_KEY: &'static str = "Software\\RustCare\\DisabledStartup";
    const HKCU_RUN: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    const HKCU_RUNONCE: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce";
    const HKLM_RUN: &'static str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";

    /// Obtiene la carpeta de inicio del usuario: `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup`
    pub fn startup_folder() -> Option<PathBuf> {
        std::env::var("APPDATA").ok().map(|appdata| {
            PathBuf::from(appdata)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Startup")
        })
    }

    /// Lista todas las entradas de arranque del usuario y del sistema (lectura segura)
    pub fn list_entries() -> Vec<StartupEntry> {
        let mut entries = Vec::new();
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

        // 3. Entradas deshabilitadas en RustCare Backup
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

        // 4. HKLM Run (solo lectura para usuario estándar, requiere admin para modificar)
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

        // 5. Carpeta de inicio (Startup Folder)
        if let Some(folder) = Self::startup_folder() {
            if folder.exists() {
                if let Ok(read_dir) = fs::read_dir(&folder) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        let is_disabled = file_name.ends_with(".disabled");
                        let clean_name = if is_disabled {
                            file_name.trim_end_matches(".disabled").to_string()
                        } else {
                            file_name
                        };

                        entries.push(StartupEntry {
                            id: format!("folder_{}", clean_name),
                            name: clean_name,
                            command: path.to_string_lossy().to_string(),
                            location: StartupLocation::StartupFolder,
                            is_enabled: !is_disabled,
                            requires_admin: false,
                        });
                    }
                }
            }
        }

        entries
    }

    /// Deshabilita una entrada de arranque de forma 100% reversible (sin borrado destructivo)
    pub fn disable_entry(entry: &StartupEntry) -> anyhow::Result<()> {
        match entry.location {
            StartupLocation::HkcuRun => {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                // 1. Guardar en clave de respaldo
                let (backup_key, _) = hkcu.create_subkey(Self::DISABLED_KEY)?;
                backup_key.set_value(&entry.name, &entry.command)?;
                // 2. Eliminar de la clave activa
                if let Ok(run_key) = hkcu.open_subkey_with_flags(Self::HKCU_RUN, winreg::enums::KEY_WRITE) {
                    let _ = run_key.delete_value(&entry.name);
                }
                Ok(())
            }
            StartupLocation::StartupFolder => {
                let path = PathBuf::from(&entry.command);
                if path.exists() && !entry.command.ends_with(".disabled") {
                    let new_path = path.with_extension(format!("{}.disabled", path.extension().unwrap_or_default().to_string_lossy()));
                    fs::rename(path, new_path)?;
                }
                Ok(())
            }
            _ => anyhow::bail!("Ubicación no soportada o requiere privilegios elevados"),
        }
    }

    /// Habilita una entrada previamente deshabilitada
    pub fn enable_entry(entry: &StartupEntry) -> anyhow::Result<()> {
        match entry.location {
            StartupLocation::HkcuRun => {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                // 1. Escribir en clave activa Run
                let (run_key, _) = hkcu.create_subkey(Self::HKCU_RUN)?;
                run_key.set_value(&entry.name, &entry.command)?;
                // 2. Eliminar de la clave de deshabilitadas
                if let Ok(backup_key) = hkcu.open_subkey_with_flags(Self::DISABLED_KEY, winreg::enums::KEY_WRITE) {
                    let _ = backup_key.delete_value(&entry.name);
                }
                Ok(())
            }
            StartupLocation::StartupFolder => {
                let path = PathBuf::from(&entry.command);
                if path.exists() && entry.command.ends_with(".disabled") {
                    let new_path_str = entry.command.trim_end_matches(".disabled");
                    fs::rename(&path, PathBuf::from(new_path_str))?;
                }
                Ok(())
            }
            _ => anyhow::bail!("Ubicación no soportada"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_startup() {
        let list = StartupEngine::list_entries();
        // Debe ejecutarse sin error
        println!("Entradas de inicio encontradas: {}", list.len());
    }
}
