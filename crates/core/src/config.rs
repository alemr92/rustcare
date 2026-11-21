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

    // Opciones de Automatización y Notificaciones
    pub auto_start_with_system: bool,
    pub auto_ram_boost_enabled: bool,
    pub auto_ram_threshold_percent: f32,
    pub auto_clean_enabled: bool,
    pub auto_clean_threshold_mb: u64,
    pub notifications_enabled: bool,
    pub notify_on_updates: bool,
    pub notify_on_drivers: bool,
    pub notify_on_high_junk: bool,
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

            auto_start_with_system: false,
            auto_ram_boost_enabled: true,
            auto_ram_threshold_percent: 80.0,
            auto_clean_enabled: false,
            auto_clean_threshold_mb: 1024,
            notifications_enabled: true,
            notify_on_updates: true,
            notify_on_drivers: true,
            notify_on_high_junk: true,
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    /// Ruta del fichero de configuración: `%LOCALAPPDATA%\RustCare\settings.json` o `$HOME/.config/rustcare/settings.json`
    pub fn config_path() -> PathBuf {
        #[cfg(windows)]
        {
            let base = std::env::var("LOCALAPPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData"));
            base.join("RustCare").join("settings.json")
        }

        #[cfg(not(windows))]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".config/rustcare/settings.json")
        }
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
        assert!(prefs.auto_ram_boost_enabled);
        assert_eq!(prefs.auto_ram_threshold_percent, 80.0);
    }
}
