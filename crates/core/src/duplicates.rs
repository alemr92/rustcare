//! Buscador de Archivos Duplicados Multihilo con BLAKE3
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFile {
    pub path: PathBuf,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub file_size: u64,
    pub blake3_hash: String,
    pub files: Vec<PathBuf>,
    pub wasted_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DuplicateScanResult {
    pub groups: Vec<DuplicateGroup>,
    pub total_duplicate_files: usize,
    pub total_wasted_bytes: u64,
}

pub struct DuplicateFinder;

impl DuplicateFinder {
    /// Lee hasta 4KB de un archivo para generar un hash preliminar rápido
    fn compute_prefix_hash(path: &Path) -> Option<[u8; 32]> {
        let mut file = File::open(path).ok()?;
        let mut buffer = [0u8; 4096];
        let bytes_read = file.read(&mut buffer).ok()?;
        if bytes_read == 0 {
            return None;
        }

        let mut hasher = blake3::Hasher::new();
        hasher.update(&buffer[..bytes_read]);
        Some(*hasher.finalize().as_bytes())
    }

    /// Calcula el hash BLAKE3 completo de un archivo mediante streaming
    fn compute_full_hash(path: &Path) -> Option<String> {
        let mut file = File::open(path).ok()?;
        let mut hasher = blake3::Hasher::new();
        let mut buffer = [0u8; 65536]; // 64 KB chunks

        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(_) => return None,
            }
        }

        Some(hasher.finalize().to_hex().to_string())
    }

    /// Escanea recursivamente una ruta en 3 fases:
    /// Fase 1: Agrupar por tamaño exacto (descartando archivos únicos sin leer disco)
    /// Fase 2: Hash preliminar de 4KB para archivos con tamaño duplicado
    /// Fase 3: Hash completo BLAKE3 en paralelo con Rayon
    pub fn scan_directory<P: AsRef<Path>>(root: P) -> DuplicateScanResult {
        // Fase 1: Recolección y agrupación por tamaño
        let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();

        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_type().is_symlink())
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(meta) = entry.metadata() {
                    let size = meta.len();
                    if size > 0 {
                        size_map.entry(size).or_default().push(entry.path().to_path_buf());
                    }
                }
            }
        }

        // Filtrar tamaños que tienen 2 o más archivos
        let candidate_sizes: Vec<(u64, Vec<PathBuf>)> = size_map
            .into_iter()
            .filter(|(_, files)| files.len() > 1)
            .collect();

        // Fase 2: Filtrado por hash preliminar de 4 KB
        let candidate_prefix_groups: Vec<(u64, Vec<PathBuf>)> = candidate_sizes
            .into_par_iter()
            .flat_map(|(size, files)| {
                let mut prefix_map: HashMap<[u8; 32], Vec<PathBuf>> = HashMap::new();
                for f in files {
                    if let Some(h) = Self::compute_prefix_hash(&f) {
                        prefix_map.entry(h).or_default().push(f);
                    }
                }
                prefix_map
                    .into_values()
                    .filter(|f_list| f_list.len() > 1)
                    .map(move |f_list| (size, f_list))
                    .collect::<Vec<_>>()
            })
            .collect();

        // Fase 3: Cálculo de hash BLAKE3 completo
        let duplicate_groups: Vec<DuplicateGroup> = candidate_prefix_groups
            .into_par_iter()
            .flat_map(|(size, files)| {
                let mut full_hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
                for f in files {
                    if let Some(hash) = Self::compute_full_hash(&f) {
                        full_hash_map.entry(hash).or_default().push(f);
                    }
                }

                full_hash_map
                    .into_iter()
                    .filter(|(_, f_list)| f_list.len() > 1)
                    .map(move |(hash, f_list)| {
                        let count = f_list.len();
                        let wasted = size * (count as u64 - 1);
                        DuplicateGroup {
                            file_size: size,
                            blake3_hash: hash,
                            files: f_list,
                            wasted_bytes: wasted,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut total_duplicate_files = 0;
        let mut total_wasted_bytes = 0;
        for g in &duplicate_groups {
            total_duplicate_files += g.files.len() - 1;
            total_wasted_bytes += g.wasted_bytes;
        }

        DuplicateScanResult {
            groups: duplicate_groups,
            total_duplicate_files,
            total_wasted_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_duplicate_finder() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("b.txt");
        let file3 = dir.path().join("c.txt");

        fs::write(&file1, b"RustCare Duplicate Test Content 1234567890").unwrap();
        fs::write(&file2, b"RustCare Duplicate Test Content 1234567890").unwrap();
        fs::write(&file3, b"Different Content").unwrap();

        let result = DuplicateFinder::scan_directory(dir.path());
        assert_eq!(result.groups.len(), 1);
        assert_eq!(result.groups[0].files.len(), 2);
    }
}
