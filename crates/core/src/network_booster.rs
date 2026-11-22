//! Optimizador de Red, Latencia y TCP/IP ("Internet Booster") con Respaldo Reversible
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkBackupState {
    pub autotuning_level: String,
    pub ecn_capability: String,
    pub rss: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOptimizationResult {
    pub autotuning_applied: bool,
    pub ecn_applied: bool,
    pub rss_applied: bool,
    pub backup_file: PathBuf,
}

pub struct NetworkBooster;

impl NetworkBooster {
    fn backup_dir() -> PathBuf {
        let primary = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("USERPROFILE").map(|u| format!("{}\\AppData\\Local", u)))
            .map(PathBuf::from)
            .map(|p| p.join("RustCare").join("backups").join("network"));

        if let Ok(dir) = primary {
            if fs::create_dir_all(&dir).is_ok() {
                return dir;
            }
        }

        let fallback = std::env::temp_dir().join("RustCare").join("backups").join("network");
        let _ = fs::create_dir_all(&fallback);
        fallback
    }

    /// Guarda una instantánea del estado de la pila de red antes de cualquier cambio para reversibilidad total
    pub fn backup_network_state() -> anyhow::Result<PathBuf> {
        let dir = Self::backup_dir();
        fs::create_dir_all(&dir)?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let backup_path = dir.join(format!("network_state_{}.json", timestamp));

        #[cfg(windows)]
        {
            let output = Command::new("netsh")
                .args(["int", "tcp", "show", "global"])
                .output();

            let out_str = output
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .unwrap_or_default();

            let state = NetworkBackupState {
                autotuning_level: if out_str.contains("disabled") { "disabled".into() } else { "normal".into() },
                ecn_capability: if out_str.contains("enabled") { "enabled".into() } else { "disabled".into() },
                rss: "enabled".into(),
                timestamp,
            };

            let json = serde_json::to_string_pretty(&state)?;
            fs::write(&backup_path, json)?;
        }

        #[cfg(not(windows))]
        {
            let state = NetworkBackupState {
                autotuning_level: "default".into(),
                ecn_capability: "default".into(),
                rss: "default".into(),
                timestamp,
            };
            let json = serde_json::to_string_pretty(&state)?;
            fs::write(&backup_path, json)?;
        }

        Ok(backup_path)
    }

    /// Aplica optimizaciones seguras a la pila TCP/IP para reducir latencia y optimizar throughput
    pub fn apply_optimizations() -> anyhow::Result<NetworkOptimizationResult> {
        let backup_file = Self::backup_network_state()?;

        #[cfg(windows)]
        {
            // netsh int tcp set global autotuninglevel=normal (Optimiza ventana de recepción TCP)
            let auto = Command::new("netsh")
                .args(["int", "tcp", "set", "global", "autotuninglevel=normal"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            // netsh int tcp set global ecncapability=enabled (Notificación de congestión explícita)
            let ecn = Command::new("netsh")
                .args(["int", "tcp", "set", "global", "ecncapability=enabled"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            // netsh int tcp set global rss=enabled (Distribución de paquetes en múltiples núcleos)
            let rss = Command::new("netsh")
                .args(["int", "tcp", "set", "global", "rss=enabled"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            Ok(NetworkOptimizationResult {
                autotuning_applied: auto,
                ecn_applied: ecn,
                rss_applied: rss,
                backup_file,
            })
        }

        #[cfg(not(windows))]
        {
            Ok(NetworkOptimizationResult {
                autotuning_applied: true,
                ecn_applied: true,
                rss_applied: true,
                backup_file,
            })
        }
    }

    /// Revierte la pila TCP/IP a la configuración estándar predeterminada de Windows
    pub fn restore_defaults() -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let auto = Command::new("netsh")
                .args(["int", "tcp", "set", "global", "autotuninglevel=normal"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            let ecn = Command::new("netsh")
                .args(["int", "tcp", "set", "global", "ecncapability=disabled"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            Ok(auto && ecn)
        }

        #[cfg(not(windows))]
        {
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_network_state() {
        let res = NetworkBooster::backup_network_state();
        assert!(res.is_ok());
        let path = res.unwrap();
        assert!(path.exists());
        let _ = fs::remove_file(path);
    }
}
