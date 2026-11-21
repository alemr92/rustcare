//! Gestor de Configuración y Preferencias del Usuario para RustCare
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub care_clean_junk: bool,
    pub care_clean_browsers: bool,
    pub care_privacy_telemetry: bool,
    pub care_ram_boost: bool,
    pub care_malware_shield: bool,
    pub auto_refresh_monitor_seconds: u64,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            care_clean_junk: true,
            care_clean_browsers: true,
            care_privacy_telemetry: true,
            care_ram_boost: true,
            care_malware_shield: true,
            auto_refresh_monitor_seconds: 2,
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    /// Ruta del fichero de configuración: `%LOCALAPPDATA%\RustCare\settings.json`
    pub fn config_path() -> PathBuf {
        let base = std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData"));
        base.join("RustCare").join("settings.json")
    }

    /// Carga las preferencias guardadas o devuelve las predeterminadas si no existe
    pub fn load() -> UserPreferences {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(prefs) = serde_json::from_str(&content) {
                    return prefs;
                }
            }
        }
        UserPreferences::default()
    }

    /// Guarda las preferencias del usuario en el disco de forma segura
    pub fn save(prefs: &UserPreferences) -> anyhow::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(prefs)?;
        fs::write(path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences_defaults() {
        let prefs = UserPreferences::default();
        assert!(prefs.care_clean_junk);
    }
}
