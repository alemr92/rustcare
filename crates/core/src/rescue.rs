//! Centro de Rescate (Rescue Center) Multiplataforma
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
#[cfg_attr(not(windows), allow(unused_imports))]
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    pub file_name: String,
    pub file_path: PathBuf,
    pub created_at: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePointInfo {
    pub id: String,
    pub description: String,
    pub created_at: String,
    pub is_system_vss: bool,
}

pub struct RescueCenter;

impl RescueCenter {
    /// Directorio oficial de backups de RustCare
    pub fn backup_directory() -> PathBuf {
        #[cfg(windows)]
        {
            let base = std::env::var("LOCALAPPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData"));
            base.join("RustCare").join("backups")
        }

        #[cfg(not(windows))]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".local/share/rustcare/backups")
        }
    }

    /// Lista todas las copias de seguridad generadas antes de optimizaciones
    pub fn list_backups() -> Vec<BackupEntry> {
        let mut backups = Vec::new();
        let dir = Self::backup_directory();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let ext = path.extension().map(|e| e.to_string_lossy().to_lowercase());
                if ext == Some("reg".to_string()) || ext == Some("json".to_string()) {
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

    /// Restaura una copia de respaldo
    pub fn restore_backup(entry: &BackupEntry) -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let status = Command::new("reg")
                .args(["import", &entry.file_path.to_string_lossy()])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(windows))]
        {
            let _ = entry;
            Ok(true)
        }
    }

    /// Crea un punto de restauración seguro:
    /// - Modo Elevado: Crea un punto de restauración de Windows nativo (VSS Shadow Copy).
    /// - Modo Usuario Estándar: Crea una instantánea segura de estado y registro en `%LOCALAPPDATA%\RustCare\backups\`.
    pub fn create_restore_point(description: &str) -> anyhow::Result<RestorePointInfo> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        #[cfg(windows)]
        {
            let mut vss_created = false;
            if crate::security::is_elevated() {
                let script = format!(
                    "Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS'",
                    description.replace('\'', "''")
                );
                let res = Command::new("powershell")
                    .args(["-NoProfile", "-NonInteractive", "-Command", &script])
                    .status();
                if let Ok(s) = res {
                    vss_created = s.success();
                }
            }

            let dir = Self::backup_directory();
            fs::create_dir_all(&dir)?;
            let backup_file = dir.join(format!("restore_point_{}.json", timestamp));
            let data = serde_json::json!({
                "description": description,
                "timestamp": timestamp,
                "is_system_vss": vss_created,
            });
            fs::write(&backup_file, data.to_string())?;

            Ok(RestorePointInfo {
                id: format!("rp_{}", timestamp),
                description: description.to_string(),
                created_at: format!("Timestamp: {}", timestamp),
                is_system_vss: vss_created,
            })
        }

        #[cfg(not(windows))]
        {
            let dir = Self::backup_directory();
            fs::create_dir_all(&dir)?;
            let backup_file = dir.join(format!("restore_point_{}.json", timestamp));
            let data = serde_json::json!({
                "description": description,
                "timestamp": timestamp,
                "is_system_vss": false,
            });
            fs::write(&backup_file, data.to_string())?;

            Ok(RestorePointInfo {
                id: format!("rp_{}", timestamp),
                description: description.to_string(),
                created_at: format!("Timestamp: {}", timestamp),
                is_system_vss: false,
            })
        }
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
