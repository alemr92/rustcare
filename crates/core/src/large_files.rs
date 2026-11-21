//! Buscador de Archivos Gigantes (Large Files Finder)
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileCategory {
    Video,
    Archive,
    Installer,
    Document,
    VirtualDisk,
    Other,
}

impl FileCategory {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" => FileCategory::Video,
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "iso" | "img" => FileCategory::Archive,
            "exe" | "msi" | "cab" | "pkg" => FileCategory::Installer,
            "pdf" | "docx" | "xlsx" | "pptx" | "psd" | "ai" => FileCategory::Document,
            "vmdk" | "vdi" | "vhd" | "vhdx" | "qcow2" => FileCategory::VirtualDisk,
            _ => FileCategory::Other,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            FileCategory::Video => "Vídeo",
            FileCategory::Archive => "Comprimido / ISO",
            FileCategory::Installer => "Instalador",
            FileCategory::Document => "Documento",
            FileCategory::VirtualDisk => "Disco Virtual",
            FileCategory::Other => "Otros",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeFileItem {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub category: FileCategory,
}

pub struct LargeFilesFinder;

impl LargeFilesFinder {
    /// Escanea recursivamente buscando archivos mayores o iguales a min_size_bytes
    pub fn scan_directory<P: AsRef<Path>>(root: P, min_size_bytes: u64) -> Vec<LargeFileItem> {
        let mut raw_candidates = Vec::new();

        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_type().is_symlink())
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(meta) = entry.metadata() {
                    let size = meta.len();
                    if size >= min_size_bytes {
                        let path = entry.path().to_path_buf();
                        let name = entry.file_name().to_string_lossy().to_string();
                        let ext = path.extension().unwrap_or_default().to_string_lossy().to_string();
                        raw_candidates.push((name, path, size, ext));
                    }
                }
            }
        }

        let mut items: Vec<LargeFileItem> = raw_candidates
            .into_par_iter()
            .map(|(name, path, size, ext)| {
                let category = FileCategory::from_extension(&ext);
                LargeFileItem {
                    name,
                    path,
                    size_bytes: size,
                    category,
                }
            })
            .collect();

        items.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        items
    }

    /// Elimina un archivo grande seleccionado
    pub fn delete_file(item: &LargeFileItem) -> anyhow::Result<()> {
        fs::remove_file(&item.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_large_files_finder() {
        let dir = tempdir().unwrap();
        let big_file = dir.path().join("big_video.mp4");
        let small_file = dir.path().join("small.txt");

        // Crear archivo de 1 MB
        fs::write(&big_file, vec![0u8; 1024 * 1024]).unwrap();
        fs::write(&small_file, b"small").unwrap();

        let results = LargeFilesFinder::scan_directory(dir.path(), 500 * 1024);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "big_video.mp4");
    }
}
