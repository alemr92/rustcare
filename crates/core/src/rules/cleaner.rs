use crate::config::UserPreferences;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanerConfig {
    pub groups: Vec<CleanerRuleGroup>,
}

impl CleanerConfig {
    /// Filtra los objetivos de limpieza basándose en las preferencias granulares del usuario (Estilo ASC Pro)
    pub fn apply_preferences(&mut self, prefs: &UserPreferences) {
        for group in &mut self.groups {
            group.targets.retain(|t| {
                // Excluir cualquier objetivo que coincida con una ruta en la lista blanca
                if prefs.excluded_paths.iter().any(|ex| !ex.trim().is_empty() && t.path.contains(ex)) {
                    return false;
                }

                match t.id.as_str() {
                    "win_temp" => prefs.junk_windows_temp && prefs.junk_clean_system_temp,
                    "user_temp" => prefs.junk_windows_temp && prefs.junk_clean_user_temp,
                    "win_minidump" | "win_crashdumps" => prefs.junk_memory_dumps,
                    "win_logs" => prefs.junk_windows_logs,
                    "win_update_cache" => prefs.junk_clean_windows_update,
                    "chrome_cache" => prefs.chrome_cache,
                    "chrome_cookies" => prefs.chrome_cookies,
                    "chrome_history" => prefs.chrome_history,
                    "edge_cache" => prefs.edge_cache,
                    "edge_cookies" => prefs.edge_cookies,
                    "edge_history" => prefs.edge_history,
                    "firefox_cache" => prefs.firefox_cache,
                    "firefox_cookies" => prefs.firefox_cookies,
                    "firefox_history" => prefs.firefox_history,
                    "brave_cache" => prefs.brave_cache,
                    "brave_cookies" => prefs.brave_cookies,
                    "brave_history" => prefs.brave_history,
                    _ => true,
                }
            });
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanerRuleGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub targets: Vec<CleanerTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanerTarget {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(default = "default_true")]
    pub recursive: bool,
    #[serde(default = "default_patterns")]
    pub patterns: Vec<String>,
    pub description: String,
    #[serde(default)]
    pub requires_admin: bool,
    #[serde(default = "default_safe")]
    pub safety_level: String,
    #[serde(default)]
    pub why_it_exists: String,
    #[serde(default)]
    pub safe_to_delete_reason: String,
}

fn default_safe() -> String {
    "safe".to_string()
}

fn default_true() -> bool {
    true
}

fn default_patterns() -> Vec<String> {
    vec!["*".to_string()]
}
