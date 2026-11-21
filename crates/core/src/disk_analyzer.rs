//! Analizador de Espacio en Disco y Treemap Estilo WizTree
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskEntryInfo {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreemapItem {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub extension: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskAnalysisResult {
    pub root_path: PathBuf,
    pub total_size_bytes: u64,
    pub items: Vec<DiskEntryInfo>,
    pub treemap_items: Vec<TreemapItem>,
}

pub struct DiskAnalyzer;

impl DiskAnalyzer {
    /// Calcula el tamaño recursivo de un directorio de forma segura
    pub fn calculate_dir_size<P: AsRef<Path>>(dir: P) -> u64 {
        let mut total = 0u64;
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Ok(meta) = entry.metadata() {
                    if meta.file_type().is_symlink() {
                        continue;
                    }
                    if meta.is_dir() {
                        total += Self::calculate_dir_size(entry.path());
                    } else {
                        total += meta.len();
                    }
                }
            }
        }
        total
    }

    /// Analiza el directorio y recolecta tanto el primer nivel como los elementos más pesados para el Treemap estilo WizTree
    pub fn analyze_directory<P: AsRef<Path>>(root: P) -> DiskAnalysisResult {
        let root_buf = root.as_ref().to_path_buf();
        let mut raw_items = Vec::new();

        if let Ok(entries) = fs::read_dir(&root_buf) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let is_dir = entry.file_type().map(|f| f.is_dir()).unwrap_or(false);
                let name = entry.file_name().to_string_lossy().to_string();
                raw_items.push((name, path, is_dir));
            }
        }

        let mut items: Vec<DiskEntryInfo> = raw_items
            .into_par_iter()
            .map(|(name, path, is_dir)| {
                let size_bytes = if is_dir {
                    Self::calculate_dir_size(&path)
                } else {
                    fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
                };

                DiskEntryInfo {
                    name,
                    path,
                    is_directory: is_dir,
                    size_bytes,
                }
            })
            .collect();

        items.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        let total_size_bytes = items.iter().map(|i| i.size_bytes).sum();

        // Recolectar los archivos más grandes dentro del árbol para alimentar el Treemap visual
        let mut all_files = Vec::new();
        for entry in WalkDir::new(&root_buf)
            .max_depth(4)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_type().is_symlink())
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(meta) = entry.metadata() {
                    let len = meta.len();
                    if len > 1024 * 1024 { // Mayor a 1 MB
                        let p = entry.path().to_path_buf();
                        let ext = p.extension().unwrap_or_default().to_string_lossy().to_lowercase();
                        let name = entry.file_name().to_string_lossy().to_string();
                        all_files.push(TreemapItem {
                            name,
                            path: p,
                            size_bytes: len,
                            extension: ext,
                        });
                    }
                }
            }
        }

        all_files.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        all_files.truncate(60); // Top 60 elementos visuales para renderizar el treemap

        DiskAnalysisResult {
            root_path: root_buf,
            total_size_bytes,
            items,
            treemap_items: all_files,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_disk_analyzer_with_treemap() {
        let dir = tempdir().unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("f1.bin"), vec![0u8; 1000]).unwrap();
        fs::write(dir.path().join("f2.bin"), vec![0u8; 500]).unwrap();

        let res = DiskAnalyzer::analyze_directory(dir.path());
        assert_eq!(res.items.len(), 2);
        assert_eq!(res.total_size_bytes, 1500);
    }
}
