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

    // Configuración Granular de Limpieza de Navegadores (Estilo ASC Pro)
    pub browser_clean_chrome_cache: bool,
    pub browser_clean_chrome_history: bool,
    pub browser_clean_chrome_cookies: bool,
    pub browser_clean_chrome_downloads: bool,
    pub browser_clean_edge_cache: bool,
    pub browser_clean_edge_history: bool,
    pub browser_clean_edge_cookies: bool,
    pub browser_clean_edge_downloads: bool,
    pub browser_clean_firefox_cache: bool,
    pub browser_clean_firefox_history: bool,
    pub browser_clean_firefox_cookies: bool,
    pub browser_clean_firefox_downloads: bool,
    pub browser_clean_brave_cache: bool,
    pub browser_clean_brave_history: bool,
    pub browser_vacuum_sqlite_auto: bool,

    // Configuración Granular de Archivos Basura del Sistema
    pub junk_clean_user_temp: bool,
    pub junk_clean_system_temp: bool,
    pub junk_clean_recycle_bin: bool,
    pub junk_clean_windows_logs: bool,
    pub junk_clean_crash_dumps: bool,
    pub junk_clean_windows_update: bool,
    pub junk_clean_thumbnails: bool,
    pub junk_clean_dev_caches: bool,

    // Configuración de Limpieza y Reparación del Registro
    pub registry_clean_uninstalled: bool,
    pub registry_clean_file_extensions: bool,
    pub registry_clean_app_paths: bool,
    pub registry_auto_backup: bool,

    // Lista Blanca de Exclusiones
    pub excluded_paths: Vec<String>,
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

            browser_clean_chrome_cache: true,
            browser_clean_chrome_history: true,
            browser_clean_chrome_cookies: false, // Por seguridad de sesión
            browser_clean_chrome_downloads: true,
            browser_clean_edge_cache: true,
            browser_clean_edge_history: true,
            browser_clean_edge_cookies: false,
            browser_clean_edge_downloads: true,
            browser_clean_firefox_cache: true,
            browser_clean_firefox_history: true,
            browser_clean_firefox_cookies: false,
            browser_clean_firefox_downloads: true,
            browser_clean_brave_cache: true,
            browser_clean_brave_history: true,
            browser_vacuum_sqlite_auto: true,

            junk_clean_user_temp: true,
            junk_clean_system_temp: true,
            junk_clean_recycle_bin: true,
            junk_clean_windows_logs: true,
            junk_clean_crash_dumps: true,
            junk_clean_windows_update: true,
            junk_clean_thumbnails: true,
            junk_clean_dev_caches: false,

            registry_clean_uninstalled: true,
            registry_clean_file_extensions: true,
            registry_clean_app_paths: true,
            registry_auto_backup: true,

            excluded_paths: Vec::new(),
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
