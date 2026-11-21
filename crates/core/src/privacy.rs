//! Motor de Privacidad, Telemetría y Respaldo del Registro para Windows 10/11
use crate::rules::privacy::{PrivacyConfig, PrivacySetting};
use crate::security::is_elevated;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

#[derive(Debug, Clone, Copy)]
enum RegHive {
    CurrentUser,
    LocalMachine,
}

impl RegHive {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "HKCU" | "HKEY_CURRENT_USER" => Some(RegHive::CurrentUser),
            "HKLM" | "HKEY_LOCAL_MACHINE" => Some(RegHive::LocalMachine),
            _ => None,
        }
    }

    fn to_regkey(self) -> RegKey {
        match self {
            RegHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
            RegHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingStatus {
    Optimal,
    NeedsOptimization,
    RequiresAdmin,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyCheckResult {
    pub setting: PrivacySetting,
    pub current_value: Option<u32>,
    pub status: SettingStatus,
}

pub struct PrivacyEngine;

impl PrivacyEngine {
    /// Comprueba el estado actual de todas las configuraciones de privacidad declaradas
    pub fn check_all(config: &PrivacyConfig) -> Vec<PrivacyCheckResult> {
        let user_is_admin = is_elevated();

        config
            .settings
            .iter()
            .map(|setting| {
                let is_hklm = setting.hive.to_uppercase() == "HKLM";
                if is_hklm && !user_is_admin {
                    return PrivacyCheckResult {
                        setting: setting.clone(),
                        current_value: None,
                        status: SettingStatus::RequiresAdmin,
                    };
                }

                let hive = match RegHive::from_str(&setting.hive) {
                    Some(h) => h,
                    None => {
                        return PrivacyCheckResult {
                            setting: setting.clone(),
                            current_value: None,
                            status: SettingStatus::Unknown,
                        }
                    }
                };

                let root = hive.to_regkey();
                let current_val: Option<u32> = root
                    .open_subkey(&setting.path)
                    .and_then(|key| key.get_value(&setting.key))
                    .ok();

                let status = match current_val {
                    Some(v) if v == setting.recommended_value => SettingStatus::Optimal,
                    _ => SettingStatus::NeedsOptimization,
                };

                PrivacyCheckResult {
                    setting: setting.clone(),
                    current_value: current_val,
                    status,
                }
            })
            .collect()
    }

    /// Directorio de copias de seguridad de RustCare: `%LOCALAPPDATA%\RustCare\backups\`
    pub fn backup_dir() -> PathBuf {
        let base = std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData"));
        base.join("RustCare").join("backups")
    }

    /// Exporta un respaldo `.reg` de las claves antes de modificarlas
    pub fn create_backup(settings: &[PrivacySetting]) -> anyhow::Result<PathBuf> {
        let dir = Self::backup_dir();
        fs::create_dir_all(&dir)?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let backup_file = dir.join(format!("privacy_backup_{}.reg", timestamp));
        let mut reg_content = String::from("Windows Registry Editor Version 5.00\r\n\r\n");

        for setting in settings {
            let hive = match RegHive::from_str(&setting.hive) {
                Some(h) => h,
                None => continue,
            };

            let root = hive.to_regkey();
            let current_val: Option<u32> = root
                .open_subkey(&setting.path)
                .and_then(|k| k.get_value(&setting.key))
                .ok();

            let hive_full = if setting.hive.to_uppercase() == "HKLM" {
                "HKEY_LOCAL_MACHINE"
            } else {
                "HKEY_CURRENT_USER"
            };

            reg_content.push_str(&format!("[{}\\{}]\r\n", hive_full, setting.path));
            if let Some(val) = current_val {
                reg_content.push_str(&format!("\"{}\"=dword:{:08x}\r\n\r\n", setting.key, val));
            } else {
                reg_content.push_str(&format!("\"{}\"=-\r\n\r\n", setting.key));
            }
        }

        fs::write(&backup_file, reg_content)?;
        Ok(backup_file)
    }

    /// Aplica las directivas recomendadas creando previamente una copia de respaldo `.reg`
    pub fn apply_recommendations(
        settings: &[PrivacySetting],
    ) -> anyhow::Result<(usize, usize, Option<PathBuf>)> {
        let user_is_admin = is_elevated();
        let backup_path = Self::create_backup(settings).ok();

        let mut applied = 0;
        let mut skipped = 0;

        for setting in settings {
            let is_hklm = setting.hive.to_uppercase() == "HKLM";
            if is_hklm && !user_is_admin {
                skipped += 1;
                continue;
            }

            let hive = match RegHive::from_str(&setting.hive) {
                Some(h) => h,
                None => {
                    skipped += 1;
                    continue;
                }
            };

            let root = hive.to_regkey();
            match root.create_subkey(&setting.path) {
                Ok((key, _)) => {
                    if key.set_value(&setting.key, &setting.recommended_value).is_ok() {
                        applied += 1;
                    } else {
                        skipped += 1;
                    }
                }
                Err(_) => {
                    skipped += 1;
                }
            }
        }

        Ok((applied, skipped, backup_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_check() {
        let config = crate::RulesEngine::load_privacy_rules().unwrap();
        let results = PrivacyEngine::check_all(&config);
        assert!(!results.is_empty());
    }
}
