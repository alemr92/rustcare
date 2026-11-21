//! Centro de Rescate (Rescue Center) y Restauración de Copias del Registro
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    pub file_name: String,
    pub file_path: PathBuf,
    pub created_at: String,
    pub size_bytes: u64,
}

pub struct RescueCenter;

impl RescueCenter {
    /// Directorio oficial de backups de RustCare: `%LOCALAPPDATA%\RustCare\backups\`
    pub fn backup_directory() -> PathBuf {
        let base = std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData"));
        base.join("RustCare").join("backups")
    }

    /// Lista todas las copias de seguridad de registro generadas antes de optimizaciones
    pub fn list_backups() -> Vec<BackupEntry> {
        let mut backups = Vec::new();
        let dir = Self::backup_directory();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().map(|ext| ext.to_string_lossy().to_lowercase()) == Some("reg".to_string()) {
                    if let Ok(meta) = entry.metadata() {
                        let created_at = meta
                            .created()
                            .or_else(|_| meta.modified())
                            .ok()
                            .map(|time| {
                                let dt: std::time::Duration = time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                                format!("Timestamp: {}", dt.as_secs())
                            })
                            .unwrap_or_else(|| "Desconocido".to_string());

                        backups.push(BackupEntry {
                            file_name: entry.file_name().to_string_lossy().to_string(),
                            file_path: path,
                            created_at,
                            size_bytes: meta.len(),
                        });
                    }
                }
            }
        }

        backups.sort_by(|a, b| b.file_path.cmp(&a.file_path));
        backups
    }

    /// Restaura una copia de respaldo importando el fichero .reg con el comando nativo de Windows `reg import`
    pub fn restore_backup(backup: &BackupEntry) -> anyhow::Result<bool> {
        let status = Command::new("reg")
            .args(["import", &backup.file_path.to_string_lossy()])
            .status()?;

        Ok(status.success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_backups_directory() {
        let _ = RescueCenter::list_backups();
    }
}
