//! Optimizador de Planes de Energía de Windows (Power Plan Optimizer)
//! Permite alternar de forma segura entre Equilibrado, Alto Rendimiento y Máximo Rendimiento (Ultimate Performance).
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSchemeInfo {
    pub guid: String,
    pub name: String,
    pub is_active: bool,
}

pub struct PowerPlanOptimizer;

impl PowerPlanOptimizer {
    /// Lista los planes de energía disponibles en el equipo
    pub fn list_schemes() -> Vec<PowerSchemeInfo> {
        let mut schemes = Vec::new();

        #[cfg(windows)]
        {
            let output = Command::new("powercfg").arg("/list").output();
            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    let trimmed = line.trim();
                    if trimmed.to_lowercase().contains("guid") && trimmed.contains(':') {
                        if let Some(after_colon) = trimmed.split(':').nth(1) {
                            let parts: Vec<&str> = after_colon.split_whitespace().collect();
                            if let Some(guid) = parts.first() {
                                if guid.contains('-') && guid.len() >= 32 {
                                    let is_active = trimmed.ends_with('*') || trimmed.contains('*');
                                    let name = if let Some(open) = trimmed.find('(') {
                                        if let Some(close) = trimmed.find(')') {
                                            trimmed[open + 1..close].to_string()
                                        } else {
                                            "Plan de energía".to_string()
                                        }
                                    } else {
                                        "Plan de energía".to_string()
                                    };

                                    schemes.push(PowerSchemeInfo {
                                        guid: guid.to_string(),
                                        name,
                                        is_active,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        #[cfg(not(windows))]
        {
            schemes.push(PowerSchemeInfo {
                guid: "balanced".into(),
                name: "Equilibrado (Nativo)".into(),
                is_active: true,
            });
        }

        schemes
    }

    /// Activa un plan de energía dado por su GUID
    pub fn set_active_scheme(guid: &str) -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let status = Command::new("powercfg")
                .args(["/setactive", guid])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(windows))]
        {
            let _ = guid;
            Ok(true)
        }
    }

    /// Habilita el plan oculto de Windows "Máximo Rendimiento" (Ultimate Performance)
    pub fn unlock_ultimate_performance() -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let status = Command::new("powercfg")
                .args(["-duplicatescheme", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(windows))]
        {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_power_schemes() {
        let _ = PowerPlanOptimizer::list_schemes();
    }
}
