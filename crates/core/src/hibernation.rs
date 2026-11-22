//! Gestor del Archivo de Hibernación (hiberfil.sys)
//! Permite desactivar la hibernación para recuperar instantáneamente de 8 a 64 GB de SSD según la memoria RAM.
use serde::{Deserialize, Serialize};
#[cfg_attr(not(windows), allow(unused_imports))]
use std::fs;
#[cfg_attr(not(windows), allow(unused_imports))]
use std::path::Path;
#[cfg_attr(not(windows), allow(unused_imports))]
use std::process::Command;
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HibernationStatus {
    pub is_enabled: bool,
    pub file_size_bytes: u64,
    pub ram_size_bytes: u64,
    pub potential_savings: String,
}

pub struct HibernationManager;

impl HibernationManager {
    /// Obtiene el estado actual de la hibernación y el tamaño potencial a liberar
    pub fn get_status() -> HibernationStatus {
        let mut sys = System::new_all();
        sys.refresh_memory();
        let ram_bytes = sys.total_memory();

        #[cfg(windows)]
        {
            let hiber_path = Path::new("C:\\hiberfil.sys");
            let file_size = if hiber_path.exists() {
                fs::metadata(hiber_path).map(|m| m.len()).unwrap_or(ram_bytes)
            } else {
                0
            };

            let is_enabled = file_size > 0 || hiber_path.exists();

            HibernationStatus {
                is_enabled,
                file_size_bytes: file_size,
                ram_size_bytes: ram_bytes,
                potential_savings: crate::cleaner::CleanerEngine::format_bytes(if file_size > 0 { file_size } else { ram_bytes }),
            }
        }

        #[cfg(not(windows))]
        {
            HibernationStatus {
                is_enabled: false,
                file_size_bytes: 0,
                ram_size_bytes: ram_bytes,
                potential_savings: "0 B".to_string(),
            }
        }
    }

    /// Desactiva la hibernación y elimina inmediatamente C:\hiberfil.sys
    pub fn disable_hibernation() -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let status = Command::new("powercfg")
                .args(["/hibernate", "off"])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(windows))]
        {
            Ok(true)
        }
    }

    /// Activa la hibernación estándar de Windows
    pub fn enable_hibernation() -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let status = Command::new("powercfg")
                .args(["/hibernate", "on"])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(windows))]
        {
            Ok(true)
        }
    }

    /// Configura la hibernación en modo 'reducido' (Fast Startup activo consumiendo solo ~40% de tamaño)
    pub fn set_reduced_hibernation() -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let _ = Command::new("powercfg").args(["/hibernate", "on"]).status();
            let status = Command::new("powercfg")
                .args(["/h", "/type", "reduced"])
                .status()?;
            Ok(status.success())
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
    fn test_hibernation_status_query() {
        let status = HibernationManager::get_status();
        assert!(status.ram_size_bytes > 0);
        assert!(!status.potential_savings.is_empty());
    }
}
