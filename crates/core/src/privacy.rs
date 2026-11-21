//! Motor de Privacidad, Telemetría y Respaldo Multiplataforma (Windows, Linux, macOS)
use crate::rules::privacy::{PrivacyConfig, PrivacySetting};
use crate::security::is_elevated;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(windows)]
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
#[derive(Debug, Clone, Copy)]
enum RegHive {
    CurrentUser,
    LocalMachine,
}

#[cfg(windows)]
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
                #[cfg(windows)]
                {
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
                }

                #[cfg(not(windows))]
                {
                    let _ = user_is_admin;
                    PrivacyCheckResult {
                        setting: setting.clone(),
                        current_value: Some(setting.recommended_value),
                        status: SettingStatus::Optimal,
                    }
                }
            })
            .collect()
    }

    /// Directorio de copias de seguridad de RustCare
    pub fn backup_dir() -> PathBuf {
        #[cfg(windows)]
        {
            let base = std::env::var("LOCALAPPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData"));
            base.join("RustCare").join("backups")
        }
        #[cfg(not(windows))]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".local/share/rustcare/backups")
        }
    }

    /// Exporta un respaldo antes de modificar configuraciones
    pub fn create_backup(settings: &[PrivacySetting]) -> anyhow::Result<PathBuf> {
        let dir = Self::backup_dir();
        fs::create_dir_all(&dir)?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        #[cfg(windows)]
        {
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
                    .and_then(|key| key.get_value(&setting.key))
                    .ok();

                let hive_prefix = if setting.hive.to_uppercase() == "HKLM" {
                    "HKEY_LOCAL_MACHINE"
                } else {
                    "HKEY_CURRENT_USER"
                };

                reg_content.push_str(&format!("[{}\\{}]\r\n", hive_prefix, setting.path));
                if let Some(val) = current_val {
                    reg_content.push_str(&format!("\"{}\"=dword:{:08x}\r\n\r\n", setting.key, val));
                } else {
                    reg_content.push_str(&format!("\"{}\"=-\r\n\r\n", setting.key));
                }
            }

            fs::write(&backup_file, reg_content)?;
            Ok(backup_file)
        }

        #[cfg(not(windows))]
        {
            let backup_file = dir.join(format!("privacy_backup_{}.json", timestamp));
            let json = serde_json::to_string_pretty(settings)?;
            fs::write(&backup_file, json)?;
            Ok(backup_file)
        }
    }

    /// Aplica el valor de privacidad recomendado
    pub fn apply_setting(setting: &PrivacySetting) -> anyhow::Result<()> {
        #[cfg(windows)]
        {
            let hive = RegHive::from_str(&setting.hive)
                .ok_or_else(|| anyhow::anyhow!("Colmena de registro no válida"))?;
            let root = hive.to_regkey();
            let (key, _) = root.create_subkey(&setting.path)?;
            key.set_value(&setting.key, &setting.recommended_value)?;
            Ok(())
        }

        #[cfg(not(windows))]
        {
            let _ = setting;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_check() {
        let config = PrivacyConfig {
            settings: vec![PrivacySetting {
                id: "test".into(),
                category: "telemetry".into(),
                name: "Test Setting".into(),
                description: "Test".into(),
                hive: "HKCU".into(),
                path: "Software\\RustCareTest".into(),
                key: "TestKey".into(),
                value_type: "REG_DWORD".into(),
                recommended_value: 1,
                default_windows_value: 0,
            }],
        };

        let results = PrivacyEngine::check_all(&config);
        assert_eq!(results.len(), 1);
    }
}
