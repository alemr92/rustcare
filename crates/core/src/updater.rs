//! Wrapper de Actualizaciones mediante WinGet (Windows Package Manager)
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WingetPackageUpdate {
    pub name: String,
    pub id: String,
    pub installed_version: String,
    pub available_version: String,
}

pub struct UpdaterEngine;

impl UpdaterEngine {
    /// Comprueba si WinGet está disponible en el entorno
    pub fn is_winget_available() -> bool {
        Command::new("winget")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Lista todas las aplicaciones con actualizaciones disponibles vía `winget upgrade`
    pub fn check_updates() -> anyhow::Result<Vec<WingetPackageUpdate>> {
        let output = Command::new("winget")
            .args(["upgrade", "--include-unknown", "--accept-source-agreements"])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut updates = Vec::new();
        let mut parsing = false;

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("---") {
                parsing = true;
                continue;
            }

            if parsing {
                if trimmed.is_empty() || trimmed.contains("actualizaciones disponibles") || trimmed.contains("upgrades available") {
                    continue;
                }

                // Las columnas de winget son típicamente: Nombre, Id, Versión, Disponible, Origen
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 4 {
                    // El último suele ser el Origen (winget), el penúltimo la nueva versión, el antepenúltimo la instalada
                    let available_version = parts[parts.len() - 2].to_string();
                    let installed_version = parts[parts.len() - 3].to_string();
                    let id = parts[parts.len() - 4].to_string();
                    let name = parts[..parts.len() - 4].join(" ");

                    updates.push(WingetPackageUpdate {
                        name: if name.is_empty() { id.clone() } else { name },
                        id,
                        installed_version,
                        available_version,
                    });
                }
            }
        }

        Ok(updates)
    }

    /// Lanza la actualización desatendida de un paquete específico
    pub fn update_package(package_id: &str) -> anyhow::Result<bool> {
        let status = Command::new("winget")
            .args([
                "upgrade",
                "--id",
                package_id,
                "-e",
                "--silent",
                "--accept-source-agreements",
                "--accept-package-agreements",
            ])
            .status()?;

        Ok(status.success())
    }
}
