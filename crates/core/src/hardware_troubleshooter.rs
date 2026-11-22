//! Detector de Conflictos y Errores de Dispositivos / Hardware
//! Diagnostica dispositivos con fallos (Código 10, Código 43, Código 28) reportados en el Administrador de Dispositivos
use serde::{Deserialize, Serialize};
#[cfg_attr(not(windows), allow(unused_imports))]
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDevice {
    pub instance_id: String,
    pub description: String,
    pub problem_code: String,
    pub status: String,
}

pub struct HardwareTroubleshooter;

impl HardwareTroubleshooter {
    /// Escanea todos los dispositivos PnP de Windows en busca de aquellos con error de inicialización
    pub fn scan_problem_devices() -> Vec<ProblemDevice> {
        let mut problem_devices = Vec::new();

        #[cfg(windows)]
        {
            // Ejecuta pnputil /enum-devices /problem para obtener la lista oficial de dispositivos con error
            let output = Command::new("pnputil")
                .args(["/enum-devices", "/problem"])
                .output();

            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let mut current_id = String::new();
                let mut current_desc = String::new();
                let mut current_code = String::new();
                let mut current_status = String::new();

                for line in stdout.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        if !current_id.is_empty() || !current_code.is_empty() {
                            problem_devices.push(ProblemDevice {
                                instance_id: current_id.clone(),
                                description: if current_desc.is_empty() { "Dispositivo sin nombre".into() } else { current_desc.clone() },
                                problem_code: if current_code.is_empty() { "Desconocido".into() } else { current_code.clone() },
                                status: if current_status.is_empty() { "Error PnP".into() } else { current_status.clone() },
                            });
                            current_id.clear();
                            current_desc.clear();
                            current_code.clear();
                            current_status.clear();
                        }
                        continue;
                    }

                    if trimmed.to_lowercase().contains("id. de instancia") || trimmed.to_lowercase().contains("instance id") {
                        if let Some(val) = trimmed.split(':').nth(1) {
                            current_id = val.trim().to_string();
                        }
                    } else if trimmed.to_lowercase().contains("descripción") || trimmed.to_lowercase().contains("description") {
                        if let Some(val) = trimmed.split(':').nth(1) {
                            current_desc = val.trim().to_string();
                        }
                    } else if trimmed.to_lowercase().contains("código de problema") || trimmed.to_lowercase().contains("problem code") {
                        if let Some(val) = trimmed.split(':').nth(1) {
                            current_code = val.trim().to_string();
                        }
                    } else if trimmed.to_lowercase().contains("estado") || trimmed.to_lowercase().contains("status") {
                        if let Some(val) = trimmed.split(':').nth(1) {
                            current_status = val.trim().to_string();
                        }
                    }
                }

                if !current_id.is_empty() {
                    problem_devices.push(ProblemDevice {
                        instance_id: current_id,
                        description: if current_desc.is_empty() { "Dispositivo sin nombre".into() } else { current_desc },
                        problem_code: if current_code.is_empty() { "Desconocido".into() } else { current_code },
                        status: if current_status.is_empty() { "Error PnP".into() } else { current_status },
                    });
                }
            }
        }

        problem_devices
    }

    /// Intenta reiniciar un dispositivo PnP problemático para recargar su controlador
    pub fn restart_device(instance_id: &str) -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let status = Command::new("pnputil")
                .args(["/restart-device", instance_id])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(windows))]
        {
            let _ = instance_id;
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_problem_devices_runs_without_panic() {
        let devices = HardwareTroubleshooter::scan_problem_devices();
        // Puede estar vacía si el equipo no tiene dispositivos dañados, pero no debe fallar
        let _ = devices.len();
    }
}
