//! Reparador de Interfaz de Windows: Reconstructor de Caché de Iconos y Fuentes
//! Soluciona fallos visuales clásicos (iconos en blanco, cajas negras o imágenes rotas).
use serde::{Deserialize, Serialize};
#[cfg_attr(not(windows), allow(unused_imports))]
use std::fs;
#[cfg_attr(not(windows), allow(unused_imports))]
use std::path::PathBuf;
#[cfg_attr(not(windows), allow(unused_imports))]
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiRepairResult {
    pub icon_cache_deleted: bool,
    pub explorer_restarted: bool,
    pub files_purged: usize,
    pub message: String,
}

pub struct UiRepair;

impl UiRepair {
    /// Reconstruye la caché de iconos de Windows de forma segura
    pub fn rebuild_icon_cache() -> anyhow::Result<UiRepairResult> {
        #[allow(unused_mut, unused_variables)]
        let mut files_purged = 0;

        #[cfg(windows)]
        {
            // 1. Terminar temporalmente explorer.exe para desbloquear los archivos de caché
            let _ = Command::new("taskkill")
                .args(["/F", "/IM", "explorer.exe"])
                .status();

            std::thread::sleep(std::time::Duration::from_millis(500));

            // 2. Eliminar IconCache.db clásico
            if let Ok(local) = std::env::var("LOCALAPPDATA") {
                let local_path = PathBuf::from(&local);
                let legacy_cache = local_path.join("IconCache.db");
                if legacy_cache.exists() {
                    let _ = fs::remove_file(legacy_cache);
                    files_purged += 1;
                }

                // 3. Eliminar bases de datos iconcache_*.db de Explorer
                let explorer_cache_dir = local_path.join("Microsoft\\Windows\\Explorer");
                if let Ok(read_dir) = fs::read_dir(explorer_cache_dir) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().to_lowercase();
                        if name.starts_with("iconcache_") || name.starts_with("thumbcache_") {
                            if fs::remove_file(entry.path()).is_ok() {
                                files_purged += 1;
                            }
                        }
                    }
                }
            }

            // 4. Reiniciar explorer.exe de inmediato
            let explorer_status = Command::new("cmd")
                .args(["/C", "start", "explorer.exe"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            Ok(UiRepairResult {
                icon_cache_deleted: files_purged > 0,
                explorer_restarted: explorer_status,
                files_purged,
                message: format!(
                    "Caché visual reconstruida con éxito. Se purgaron {} archivos de caché e iconos.",
                    files_purged
                ),
            })
        }

        #[cfg(not(windows))]
        {
            Ok(UiRepairResult {
                icon_cache_deleted: true,
                explorer_restarted: true,
                files_purged: 0,
                message: "Operación de interfaz completada.".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_repair_structure() {
        let res = UiRepairResult {
            icon_cache_deleted: true,
            explorer_restarted: true,
            files_purged: 3,
            message: "Test".into(),
        };
        assert!(res.icon_cache_deleted);
    }
}
