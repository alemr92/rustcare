//! Buscador y Limpiador de Carpetas Vacías (Empty Folders Finder)
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyFolderItem {
    pub path: PathBuf,
    pub name: String,
}

pub struct EmptyFoldersFinder;

impl EmptyFoldersFinder {
    /// Comprueba si un directorio está vacío o solo contiene archivos basurilla de sistema (Thumbs.db, desktop.ini)
    fn is_effectively_empty(dir: &Path) -> bool {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name != "desktop.ini" && name != "thumbs.db" && name != ".ds_store" {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// Escanea recursivamente de abajo hacia arriba recolectando carpetas vacías
    pub fn scan_directory<P: AsRef<Path>>(root: P) -> Vec<EmptyFolderItem> {
        let mut empty_folders = Vec::new();
        Self::scan_recursive(root.as_ref(), &mut empty_folders);
        empty_folders
    }

    fn scan_recursive(current: &Path, acc: &mut Vec<EmptyFolderItem>) {
        if let Ok(entries) = fs::read_dir(current) {
            let mut subdirs = Vec::new();

            for entry in entries.filter_map(|e| e.ok()) {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_dir() && !ft.is_symlink() {
                        subdirs.push(entry.path());
                    }
                }
            }

            for sub in subdirs {
                Self::scan_recursive(&sub, acc);
            }

            // Comprobar si la carpeta actual (excepto si es el root) está vacía
            if Self::is_effectively_empty(current) {
                acc.push(EmptyFolderItem {
                    path: current.to_path_buf(),
                    name: current.file_name().unwrap_or_default().to_string_lossy().to_string(),
                });
            }
        }
    }

    /// Elimina una carpeta vacía
    pub fn delete_folder(item: &EmptyFolderItem) -> anyhow::Result<()> {
        // Primero eliminar cualquier desktop.ini o Thumbs.db si existiera
        if let Ok(entries) = fs::read_dir(&item.path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let _ = fs::remove_file(entry.path());
            }
        }
        fs::remove_dir(&item.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_empty_folders_finder() {
        let dir = tempdir().unwrap();
        let empty_sub = dir.path().join("empty_folder");
        let non_empty_sub = dir.path().join("full_folder");
        fs::create_dir(&empty_sub).unwrap();
        fs::create_dir(&non_empty_sub).unwrap();
        fs::write(non_empty_sub.join("file.txt"), b"data").unwrap();

        let results = EmptyFoldersFinder::scan_directory(dir.path());
        assert!(results.iter().any(|f| f.path == empty_sub));
        assert!(!results.iter().any(|f| f.path == non_empty_sub));
    }
}
