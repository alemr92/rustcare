//! Gestor de Menú Contextual de Clic Derecho (Context Menu Manager)
use serde::{Deserialize, Serialize};
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
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
    const HKCU_STAR_HANDLERS: &'static str = "Software\\Classes\\*\\shellex\\ContextMenuHandlers";
    const HKCU_DIR_HANDLERS: &'static str = "Software\\Classes\\Directory\\shellex\\ContextMenuHandlers";
    const HKLM_STAR_HANDLERS: &'static str = "SOFTWARE\\Classes\\*\\shellex\\ContextMenuHandlers";

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

        // 1. HKCU Handlers (Modificables por el usuario sin admin)
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        items.extend(Self::scan_handlers(&hkcu, Self::HKCU_STAR_HANDLERS, true));
        items.extend(Self::scan_handlers(&hkcu, Self::HKCU_DIR_HANDLERS, true));

        // 2. HKLM Handlers (Lectura global)
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        items.extend(Self::scan_handlers(&hklm, Self::HKLM_STAR_HANDLERS, false));

        items
    }

    /// Deshabilita un elemento del menú contextual de forma reversible anteponiendo un guion '-'
    pub fn toggle_handler(item: &ContextMenuItem) -> anyhow::Result<()> {
        if !item.is_user_specific {
            anyhow::bail!("Los elementos globales del sistema requieren privilegios elevados");
        }

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let parent_path = if item.key_path.contains("Directory") {
            Self::HKCU_DIR_HANDLERS
        } else {
            Self::HKCU_STAR_HANDLERS
        };

        let parent_key = hkcu.open_subkey_with_flags(parent_path, winreg::enums::KEY_ALL_ACCESS)?;
        let old_name = if item.is_enabled {
            item.name.clone()
        } else {
            format!("-{}", item.name)
        };

        let new_name = if item.is_enabled {
            format!("-{}", item.name)
        } else {
            item.name.clone()
        };

        // Copiar valor a la nueva clave y borrar la anterior
        let (new_key, _) = parent_key.create_subkey(&new_name)?;
        new_key.set_value("", &item.clsid)?;
        let _ = parent_key.delete_subkey(&old_name);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_context_menu_handlers() {
        let handlers = ContextMenuManager::list_handlers();
        println!("Handlers de menú contextual detectados: {}", handlers.len());
    }
}
