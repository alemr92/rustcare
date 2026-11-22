//! Gestor de Configuración y Preferencias del Usuario para RustCare
//! Estilo y opciones basados exactamente en Advanced SystemCare (IObit) PRO
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPreferences {
    // ----------------------------------------------------
    // Care & Monitor General
    // ----------------------------------------------------
    pub care_clean_junk: bool,
    pub care_clean_browsers: bool,
    pub care_privacy_telemetry: bool,
    pub care_ram_boost: bool,
    pub care_malware_shield: bool,
    pub auto_refresh_monitor_seconds: u64,

    // ----------------------------------------------------
    // 1. Ajustes Generales (Screen 1)
    // ----------------------------------------------------
    pub minimize_to_tray_on_close: bool,
    pub autostart_tray_on_boot: bool,
    pub autostart_service_on_boot: bool,
    pub skip_uac_warning: bool,
    pub enable_perf_monitor: bool,

    // Menú Contextual del Explorador de Windows (Screen 1)
    pub ctx_recover_file: bool,
    pub ctx_destroy_file: bool,
    pub ctx_fix_shortcut: bool,
    pub ctx_sweep_privacy: bool,
    pub ctx_repair_disk: bool,
    pub ctx_clean_disk: bool,
    pub ctx_defrag_disk: bool,

    // ----------------------------------------------------
    // 2. Limpiador de Registro (Screen 2)
    // ----------------------------------------------------
    pub reg_activex: bool,
    pub reg_integrity: bool,
    pub reg_app_paths: bool,
    pub reg_fonts: bool,
    pub reg_help_files: bool,
    pub reg_history_lists: bool,
    pub reg_shared_files: bool,
    pub reg_shortcuts: bool,
    pub reg_software: bool,
    pub reg_sounds: bool,
    pub reg_start_menu: bool,
    pub reg_startup_items: bool,
    pub reg_mui_cache: bool,
    pub reg_deep_clean: bool,
    pub registry_auto_backup: bool,
    pub registry_clean_uninstalled: bool,
    pub registry_clean_file_extensions: bool,

    // ----------------------------------------------------
    // 3. Limpieza de Privacidad (Screen 3)
    // ----------------------------------------------------
    pub privacy_clean_user_account: bool,
    pub privacy_dod_wipe: bool,
    pub privacy_keep_history_duration: String, // "1 semana", "2 semanas", "1 mes", "Todo"

    // Edge
    pub edge_cache: bool,
    pub edge_cookies: bool,
    pub edge_history: bool,
    pub edge_downloads: bool,
    pub edge_session: bool,
    pub edge_autofill_form: bool,
    pub edge_autofill_passwords: bool,
    pub edge_metrics: bool,
    pub edge_bookmarks_backup: bool,
    pub edge_compact_db: bool,
    pub edge_site_characteristics: bool,

    // Chrome
    pub chrome_cache: bool,
    pub chrome_cookies: bool,
    pub chrome_history: bool,
    pub chrome_downloads: bool,
    pub chrome_session: bool,
    pub chrome_autofill_form: bool,
    pub chrome_autofill_passwords: bool,
    pub chrome_metrics: bool,
    pub chrome_bookmarks_backup: bool,
    pub chrome_compact_db: bool,

    // Firefox
    pub firefox_cache: bool,
    pub firefox_cookies: bool,
    pub firefox_history: bool,
    pub firefox_downloads: bool,
    pub firefox_session: bool,
    pub firefox_autofill_form: bool,
    pub firefox_autofill_passwords: bool,
    pub firefox_compact_db: bool,

    // Brave
    pub brave_cache: bool,
    pub brave_cookies: bool,
    pub brave_history: bool,
    pub brave_downloads: bool,
    pub brave_session: bool,
    pub brave_compact_db: bool,

    // Compatibilidad retroactiva
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

    // ----------------------------------------------------
    // 4. Limpiador de Archivos Basura (Screen 4)
    // ----------------------------------------------------
    pub junk_recycle_bin: bool,
    pub junk_windows_temp: bool,
    pub junk_ie_edge_temp: bool,
    pub junk_directx_cache: bool,
    pub junk_clipboard: bool,
    pub junk_memory_dumps: bool,
    pub junk_chkdsk_fragments: bool,
    pub junk_windows_logs: bool,
    pub junk_error_reporting: bool,
    pub junk_dns_cache: bool,
    pub junk_clean_user_account: bool,
    pub junk_dod_wipe: bool,
    pub junk_keep_temp_duration: String, // "1 semana", "2 semanas", "1 mes"
    pub junk_keep_recycle_duration: String, // "1 mes", "2 meses", "Nunca"

    // Compatibilidad retroactiva junk
    pub junk_clean_user_temp: bool,
    pub junk_clean_system_temp: bool,
    pub junk_clean_recycle_bin: bool,
    pub junk_clean_crash_dumps: bool,
    pub junk_clean_windows_update: bool,
    pub junk_clean_thumbnails: bool,
    pub junk_clean_dev_caches: bool,

    // ----------------------------------------------------
    // 5. Optimización del Sistema [PRO] (Screen 5)
    // ----------------------------------------------------
    pub system_opt_profile: String, // "Básica", "Negocios", "Juegos", "Estación de Trabajo"

    // ----------------------------------------------------
    // 6. Acelerador de Internet [PRO] (Screen 6)
    // ----------------------------------------------------
    pub network_connection_mode: String, // "Básica", "DSL", "Cable/Fibra", "LAN"
    pub network_advanced_opt: bool,

    // ----------------------------------------------------
    // Anti-Spyware & Seguridad Avanzada
    // ----------------------------------------------------
    pub spyware_realtime_shield: bool,
    pub spyware_heuristic_scan: bool,
    pub security_reinforce_rdp: bool,
    pub security_reinforce_smb: bool,
    pub security_reinforce_uac: bool,
    pub vuln_auto_scan_winget: bool,
    pub disk_opt_enable_trim: bool,
    pub disk_opt_defrag_hdd: bool,

    // ----------------------------------------------------
    // Servicio Automático (AutoCare)
    // ----------------------------------------------------
    pub auto_start_with_system: bool,
    pub auto_care_idle_only: bool,
    pub auto_care_battery_pause: bool,
    pub auto_clean_enabled: bool,
    pub auto_clean_threshold_mb: u64,
    pub auto_clean_on_browser_close: bool,
    pub auto_clean_on_shutdown: bool,
    pub auto_update_definitions: bool,
    pub auto_update_app: bool,
    pub auto_ram_boost_enabled: bool,
    pub auto_ram_threshold_percent: f32,

    // Notificaciones
    pub notifications_enabled: bool,
    pub notify_on_updates: bool,
    pub notify_on_drivers: bool,
    pub notify_on_high_junk: bool,

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

            // Screen 1: Ajustes Generales
            minimize_to_tray_on_close: true,
            autostart_tray_on_boot: false,
            autostart_service_on_boot: false,
            skip_uac_warning: true,
            enable_perf_monitor: false,

            ctx_recover_file: true,
            ctx_destroy_file: true,
            ctx_fix_shortcut: true,
            ctx_sweep_privacy: true,
            ctx_repair_disk: true,
            ctx_clean_disk: true,
            ctx_defrag_disk: true,

            // Screen 2: Limpiador de Registro
            reg_activex: true,
            reg_integrity: true,
            reg_app_paths: true,
            reg_fonts: true,
            reg_help_files: true,
            reg_history_lists: true,
            reg_shared_files: true,
            reg_shortcuts: true,
            reg_software: true,
            reg_sounds: true,
            reg_start_menu: true,
            reg_startup_items: true,
            reg_mui_cache: true,
            reg_deep_clean: false,
            registry_auto_backup: true,
            registry_clean_uninstalled: true,
            registry_clean_file_extensions: true,

            // Screen 3: Limpieza de Privacidad
            privacy_clean_user_account: true,
            privacy_dod_wipe: false,
            privacy_keep_history_duration: "1 semana".to_string(),

            edge_cache: true,
            edge_cookies: false,
            edge_history: true,
            edge_downloads: true,
            edge_session: false,
            edge_autofill_form: false,
            edge_autofill_passwords: false,
            edge_metrics: true,
            edge_bookmarks_backup: true,
            edge_compact_db: false,
            edge_site_characteristics: false,

            chrome_cache: true,
            chrome_cookies: false,
            chrome_history: true,
            chrome_downloads: true,
            chrome_session: false,
            chrome_autofill_form: false,
            chrome_autofill_passwords: false,
            chrome_metrics: true,
            chrome_bookmarks_backup: true,
            chrome_compact_db: false,

            firefox_cache: true,
            firefox_cookies: false,
            firefox_history: true,
            firefox_downloads: true,
            firefox_session: false,
            firefox_autofill_form: false,
            firefox_autofill_passwords: false,
            firefox_compact_db: false,

            brave_cache: true,
            brave_cookies: false,
            brave_history: true,
            brave_downloads: true,
            brave_session: false,
            brave_compact_db: false,

            browser_clean_chrome_cache: true,
            browser_clean_chrome_history: true,
            browser_clean_chrome_cookies: false,
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

            // Screen 4: Limpiador de Archivos Basura
            junk_recycle_bin: true,
            junk_windows_temp: true,
            junk_ie_edge_temp: true,
            junk_directx_cache: true,
            junk_clipboard: true,
            junk_memory_dumps: true,
            junk_chkdsk_fragments: true,
            junk_windows_logs: true,
            junk_error_reporting: true,
            junk_dns_cache: true,
            junk_clean_user_account: true,
            junk_dod_wipe: false,
            junk_keep_temp_duration: "1 semana".to_string(),
            junk_keep_recycle_duration: "1 mes".to_string(),

            junk_clean_user_temp: true,
            junk_clean_system_temp: true,
            junk_clean_recycle_bin: true,
            junk_clean_crash_dumps: true,
            junk_clean_windows_update: true,
            junk_clean_thumbnails: true,
            junk_clean_dev_caches: false,

            // Screen 5: Optimización del Sistema [PRO]
            system_opt_profile: "Básica".to_string(),

            // Screen 6: Acelerador de Internet [PRO]
            network_connection_mode: "Básica".to_string(),
            network_advanced_opt: false,

            // Anti-Spyware & Seguridad
            spyware_realtime_shield: true,
            spyware_heuristic_scan: true,
            security_reinforce_rdp: true,
            security_reinforce_smb: true,
            security_reinforce_uac: true,
            vuln_auto_scan_winget: true,
            disk_opt_enable_trim: true,
            disk_opt_defrag_hdd: false,

            // Servicio Automático
            auto_start_with_system: false,
            auto_care_idle_only: true,
            auto_care_battery_pause: true,
            auto_clean_enabled: false,
            auto_clean_threshold_mb: 1024,
            auto_clean_on_browser_close: false,
            auto_clean_on_shutdown: false,
            auto_update_definitions: true,
            auto_update_app: false,
            auto_ram_boost_enabled: true,
            auto_ram_threshold_percent: 80.0,

            notifications_enabled: true,
            notify_on_updates: true,
            notify_on_drivers: true,
            notify_on_high_junk: true,

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
        assert!(prefs.minimize_to_tray_on_close);
        assert!(prefs.skip_uac_warning);
        assert_eq!(prefs.system_opt_profile, "Básica");
        assert_eq!(prefs.network_connection_mode, "Básica");
    }
}
