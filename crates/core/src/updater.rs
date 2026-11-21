//! Gestor de Actualizaciones Multiplataforma (WinGet, Flatpak, Homebrew)
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
    /// Comprueba si el gestor de paquetes está disponible en el entorno
    pub fn is_winget_available() -> bool {
        #[cfg(windows)]
        {
            Command::new("winget")
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("flatpak")
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("brew")
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            false
        }
    }

    /// Lista todas las aplicaciones con actualizaciones disponibles
    pub fn check_updates() -> anyhow::Result<Vec<WingetPackageUpdate>> {
        #[cfg(windows)]
        {
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

                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 4 {
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

        #[cfg(target_os = "linux")]
        {
            let mut updates = Vec::new();
            if let Ok(output) = Command::new("flatpak").args(["remote-ls", "--updates"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if !parts.is_empty() {
                        let name = parts[0].to_string();
                        updates.push(WingetPackageUpdate {
                            name: name.clone(),
                            id: name,
                            installed_version: "Current".to_string(),
                            available_version: "Newer".to_string(),
                        });
                    }
                }
            }
            Ok(updates)
        }

        #[cfg(target_os = "macos")]
        {
            let mut updates = Vec::new();
            if let Ok(output) = Command::new("brew").args(["outdated", "--json=v2"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&stdout) {
                    if let Some(formulae) = parsed.get("formulae").and_then(|f| f.as_array()) {
                        for item in formulae {
                            if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                                updates.push(WingetPackageUpdate {
                                    name: name.to_string(),
                                    id: name.to_string(),
                                    installed_version: "Outdated".to_string(),
                                    available_version: "Latest".to_string(),
                                });
                            }
                        }
                    }
                }
            }
            Ok(updates)
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            Ok(Vec::new())
        }
    }

    /// Alias para actualizar paquete
    pub fn update_package(package_id: &str) -> anyhow::Result<bool> {
        Self::upgrade_package(package_id)
    }

    /// Ejecuta la actualización de un paquete dado
    pub fn upgrade_package(package_id: &str) -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let status = Command::new("winget")
                .args(["upgrade", "--id", package_id, "--accept-package-agreements", "--accept-source-agreements"])
                .status()?;
            Ok(status.success())
        }

        #[cfg(target_os = "linux")]
        {
            let status = Command::new("flatpak")
                .args(["update", "-y", package_id])
                .status()?;
            Ok(status.success())
        }

        #[cfg(target_os = "macos")]
        {
            let status = Command::new("brew")
                .args(["upgrade", package_id])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            let _ = package_id;
            Ok(false)
        }
    }
}
