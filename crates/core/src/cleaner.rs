//! Motor de Limpieza Declarativo Seguro para Windows 10/11
use crate::rules::cleaner::{CleanerConfig, CleanerTarget};
use crate::security::is_elevated;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedFile {
    pub path: PathBuf,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetScanResult {
    pub target_id: String,
    pub target_name: String,
    pub group_id: String,
    pub files: Vec<ScannedFile>,
    pub total_bytes: u64,
    pub file_count: usize,
    pub requires_admin: bool,
    pub skipped_unprivileged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FullScanResult {
    pub targets: Vec<TargetScanResult>,
    pub total_bytes: u64,
    pub total_files: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TargetCleanResult {
    pub target_id: String,
    pub bytes_freed: u64,
    pub files_deleted: usize,
    pub failed_files: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FullCleanResult {
    pub target_results: Vec<TargetCleanResult>,
    pub total_bytes_freed: u64,
    pub total_files_deleted: usize,
    pub total_failures: usize,
}

pub struct CleanerEngine;

impl CleanerEngine {
    /// Expande variables de entorno de Windows en una ruta dada (ej: `%TEMP%\foo` -> `C:\Users\...\AppData\Local\Temp\foo`)
    pub fn expand_env_path(path_str: &str) -> Option<PathBuf> {
        let mut result = path_str.to_string();
        let env_vars = [
            "TEMP",
            "TMP",
            "LOCALAPPDATA",
            "APPDATA",
            "USERPROFILE",
            "SystemRoot",
            "SystemDrive",
            "ProgramData",
            "ProgramFiles",
            "ProgramFiles(x86)",
        ];

        for var in &env_vars {
            let token = format!("%{}%", var);
            if result.contains(&token) {
                if let Ok(val) = env::var(var) {
                    result = result.replace(&token, &val);
                }
            }
        }

        // Si todavía contiene '%' no resueltos, intentamos resolverlos genéricamente
        if result.contains('%') {
            for (k, v) in env::vars() {
                let token = format!("%{}%", k);
                if result.contains(&token) {
                    result = result.replace(&token, &v);
                }
            }
        }

        let p = PathBuf::from(result);
        if p.exists() {
            Some(p)
        } else {
            None
        }
    }

    /// Comprueba si un nombre de archivo coincide con los patrones dados (*, *.pf, *.log, etc.)
    fn matches_patterns(file_name: &str, patterns: &[String]) -> bool {
        if patterns.is_empty() || patterns.iter().any(|p| p == "*") {
            return true;
        }

        let lower_name = file_name.to_lowercase();
        for pattern in patterns {
            let lower_pat = pattern.to_lowercase();
            if lower_pat == "*" {
                return true;
            }
            if lower_pat.starts_with("*.") {
                let ext = &lower_pat[1..]; // ej: ".pf"
                if lower_name.ends_with(ext) {
                    return true;
                }
            } else if lower_name == lower_pat {
                return true;
            }
        }
        false
    }

    /// Escanea un objetivo individual en modo Dry-Run (solo lectura).
    /// Evita expresamente seguir puntos de reanálisis (Reparse Points/Symlinks) por seguridad.
    pub fn scan_target(target: &CleanerTarget, group_id: &str) -> TargetScanResult {
        let user_is_admin = is_elevated();
        let target_requires_admin = target.requires_admin
            || target.path.to_uppercase().contains("%SYSTEMROOT%")
            || target.path.to_uppercase().contains("C:\\WINDOWS");

        if target_requires_admin && !user_is_admin {
            return TargetScanResult {
                target_id: target.id.clone(),
                target_name: target.name.clone(),
                group_id: group_id.to_string(),
                files: Vec::new(),
                total_bytes: 0,
                file_count: 0,
                requires_admin: true,
                skipped_unprivileged: true,
            };
        }

        let base_path = match Self::expand_env_path(&target.path) {
            Some(p) => p,
            None => {
                return TargetScanResult {
                    target_id: target.id.clone(),
                    target_name: target.name.clone(),
                    group_id: group_id.to_string(),
                    files: Vec::new(),
                    total_bytes: 0,
                    file_count: 0,
                    requires_admin: target_requires_admin,
                    skipped_unprivileged: false,
                };
            }
        };

        let max_depth = if target.recursive { 10 } else { 1 };
        let mut scanned_files = Vec::new();
        let mut total_bytes = 0u64;

        // Recorrido seguro sin seguir symlinks/junctions
        for entry in WalkDir::new(&base_path)
            .max_depth(max_depth)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                // No seguir reparse points ni junctions
                !e.file_type().is_symlink()
            })
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_string_lossy();
                if Self::matches_patterns(&file_name, &target.patterns) {
                    if let Ok(meta) = entry.metadata() {
                        let size = meta.len();
                        total_bytes += size;
                        scanned_files.push(ScannedFile {
                            path: entry.path().to_path_buf(),
                            size_bytes: size,
                        });
                    }
                }
            }
        }

        let file_count = scanned_files.len();
        TargetScanResult {
            target_id: target.id.clone(),
            target_name: target.name.clone(),
            group_id: group_id.to_string(),
            files: scanned_files,
            total_bytes,
            file_count,
            requires_admin: target_requires_admin,
            skipped_unprivileged: false,
        }
    }

    /// Escaneo completo de todas las reglas configuradas en paralelo con Rayon
    pub fn scan_all(config: &CleanerConfig) -> FullScanResult {
        let all_targets: Vec<(&CleanerTarget, &str)> = config
            .groups
            .iter()
            .flat_map(|g| g.targets.iter().map(move |t| (t, g.id.as_str())))
            .collect();

        let target_results: Vec<TargetScanResult> = all_targets
            .into_par_iter()
            .map(|(t, group_id)| Self::scan_target(t, group_id))
            .collect();

        let mut total_bytes = 0;
        let mut total_files = 0;
        for r in &target_results {
            total_bytes += r.total_bytes;
            total_files += r.file_count;
        }

        FullScanResult {
            targets: target_results,
            total_bytes,
            total_files,
        }
    }

    /// Limpia de forma segura los archivos escaneados para un objetivo.
    /// Si un archivo está bloqueado por el sistema o por otra aplicación, se ignora con seguridad
    /// sin abortar la operación ni causar fallo en cascada.
    pub fn clean_target_files(scan_result: &TargetScanResult) -> TargetCleanResult {
        let mut bytes_freed = 0u64;
        let mut files_deleted = 0usize;
        let mut failed_files = 0usize;
        let mut errors = Vec::new();

        for file in &scan_result.files {
            match fs::remove_file(&file.path) {
                Ok(_) => {
                    bytes_freed += file.size_bytes;
                    files_deleted += 1;
                }
                Err(e) => {
                    failed_files += 1;
                    if errors.len() < 10 {
                        errors.push(format!("{}: {}", file.path.display(), e));
                    }
                }
            }
        }

        TargetCleanResult {
            target_id: scan_result.target_id.clone(),
            bytes_freed,
            files_deleted,
            failed_files,
            errors,
        }
    }

    /// Limpieza completa de los objetivos seleccionados
    pub fn clean_all(scan_result: &FullScanResult) -> FullCleanResult {
        let target_results: Vec<TargetCleanResult> = scan_result
            .targets
            .par_iter()
            .map(Self::clean_target_files)
            .collect();

        let mut total_bytes_freed = 0;
        let mut total_files_deleted = 0;
        let mut total_failures = 0;

        for r in &target_results {
            total_bytes_freed += r.bytes_freed;
            total_files_deleted += r.files_deleted;
            total_failures += r.failed_files;
        }

        FullCleanResult {
            target_results,
            total_bytes_freed,
            total_files_deleted,
            total_failures,
        }
    }

    /// Formatea bytes en cadena legible (B, KB, MB, GB)
    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(CleanerEngine::format_bytes(500), "500 B");
        assert_eq!(CleanerEngine::format_bytes(1024), "1.00 KB");
        assert_eq!(CleanerEngine::format_bytes(1048576), "1.00 MB");
        assert_eq!(CleanerEngine::format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_expand_temp() {
        let p = CleanerEngine::expand_env_path("%TEMP%");
        assert!(p.is_some());
    }
}
