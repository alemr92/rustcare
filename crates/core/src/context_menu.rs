//! Gestor de Menú Contextual Multiplataforma
use serde::{Deserialize, Serialize};

#[cfg(windows)]
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
#[cfg(windows)]
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMenuItem {
    pub id: String,
    pub name: String,
    pub clsid: String,
    pub key_path: String,
    pub is_enabled: bool,
    pub is_user_specific: bool,
}

pub struct ContextMenuManager;

impl ContextMenuManager {
    #[cfg(windows)]
    const HKCU_STAR_HANDLERS: &'static str = "Software\\Classes\\*\\shellex\\ContextMenuHandlers";
    #[cfg(windows)]
    const HKCU_DIR_HANDLERS: &'static str = "Software\\Classes\\Directory\\shellex\\ContextMenuHandlers";
    #[cfg(windows)]
    const HKLM_STAR_HANDLERS: &'static str = "SOFTWARE\\Classes\\*\\shellex\\ContextMenuHandlers";

    #[cfg(windows)]
    fn scan_handlers(root: &RegKey, path: &str, is_user: bool) -> Vec<ContextMenuItem> {
        let mut items = Vec::new();
        if let Ok(key) = root.open_subkey(path) {
            for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
                if let Ok(sub) = key.open_subkey(&subkey_name) {
                    let clsid: String = sub.get_value("").unwrap_or_default();
                    let is_disabled = subkey_name.starts_with('-');
                    let clean_name = if is_disabled {
                        subkey_name[1..].to_string()
                    } else {
                        subkey_name.clone()
                    };

                    items.push(ContextMenuItem {
                        id: format!("{}_{}", if is_user { "user" } else { "system" }, subkey_name),
                        name: clean_name,
                        clsid,
                        key_path: format!("{}\\{}", path, subkey_name),
                        is_enabled: !is_disabled,
                        is_user_specific: is_user,
                    });
                }
            }
        }
        items
    }

    /// Lista todos los elementos registrados en el menú contextual
    pub fn list_handlers() -> Vec<ContextMenuItem> {
        let mut items = Vec::new();

        #[cfg(windows)]
        {
            // 1. HKCU Handlers (Modificables por el usuario sin admin)
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            items.extend(Self::scan_handlers(&hkcu, Self::HKCU_STAR_HANDLERS, true));
            items.extend(Self::scan_handlers(&hkcu, Self::HKCU_DIR_HANDLERS, true));

            // 2. HKLM Handlers (Lectura global)
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            items.extend(Self::scan_handlers(&hklm, Self::HKLM_STAR_HANDLERS, false));
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let scripts_dir = std::path::PathBuf::from(home).join(".local/share/nautilus/scripts");
                if let Ok(read_dir) = std::fs::read_dir(scripts_dir) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        items.push(ContextMenuItem {
                            id: format!("nautilus_{}", name),
                            name: name.clone(),
                            clsid: "NautilusScript".to_string(),
                            key_path: entry.path().to_string_lossy().to_string(),
                            is_enabled: true,
                            is_user_specific: true,
                        });
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let services_dir = std::path::PathBuf::from(home).join("Library/Services");
                if let Ok(read_dir) = std::fs::read_dir(services_dir) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        items.push(ContextMenuItem {
                            id: format!("macos_service_{}", name),
                            name: name.clone(),
                            clsid: "FinderService".to_string(),
                            key_path: entry.path().to_string_lossy().to_string(),
                            is_enabled: true,
                            is_user_specific: true,
                        });
                    }
                }
            }
        }

        items
    }

    /// Alterna el estado (Habilitado/Deshabilitado) de un handler
    pub fn toggle_handler(item: &ContextMenuItem) -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            if !item.is_user_specific {
                return Err(anyhow::anyhow!("Solo los handlers específicos de usuario en HKCU pueden modificarse sin privilegios de administrador."));
            }

            let last_slash = item.key_path.rfind('\\').ok_or_else(|| anyhow::anyhow!("Ruta de clave no válida"))?;
            let parent_path = &item.key_path[..last_slash];
            let subkey_name = &item.key_path[last_slash + 1..];

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let parent_key = hkcu.open_subkey_with_flags(parent_path, winreg::enums::KEY_ALL_ACCESS)?;

            let new_name = if item.is_enabled {
                format!("-{}", item.name)
            } else {
                item.name.clone()
            };

            let old_key = parent_key.open_subkey(subkey_name)?;
            let clsid: String = old_key.get_value("").unwrap_or_default();

            let (new_key, _) = parent_key.create_subkey(&new_name)?;
            new_key.set_value("", &clsid)?;

            parent_key.delete_subkey(subkey_name)?;

            Ok(!item.is_enabled)
        }

        #[cfg(not(windows))]
        {
            Ok(!item.is_enabled)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_context_menu_handlers() {
        let handlers = ContextMenuManager::list_handlers();
        let _ = handlers.len();
    }
}
