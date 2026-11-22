//! Limpiador del Almacén de Componentes de Windows (WinSxS) vía DISM Oficial
//! Elimina paquetes antiguos de Windows Update que ya han sido reemplazados, liberando de 5 a 20 GB.
use serde::{Deserialize, Serialize};
#[cfg_attr(not(windows), allow(unused_imports))]
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DismCleanupResult {
    pub success: bool,
    pub output_message: String,
    pub requires_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DismStatus {
    pub is_supported: bool,
    pub recommended_action: String,
}

pub struct DismEngine;

impl DismEngine {
    /// Comprueba si el sistema operativo admite la limpieza DISM de componentes
    pub fn check_status() -> DismStatus {
        #[cfg(windows)]
        {
            DismStatus {
                is_supported: true,
                recommended_action: "La limpieza de WinSxS elimina copias de seguridad de parches antiguos obsoletos.".into(),
            }
        }

        #[cfg(not(windows))]
        {
            DismStatus {
                is_supported: false,
                recommended_action: "DISM es una utilidad exclusiva del kernel de Microsoft Windows.".into(),
            }
        }
    }

    /// Ejecuta la limpieza de WinSxS usando la herramienta oficial de Microsoft (Dism.exe)
    /// Requiere permisos de administrador para ejecutar StartComponentCleanup.
    pub fn start_component_cleanup(reset_base: bool) -> DismCleanupResult {
        #[cfg(windows)]
        {
            if !crate::security::is_elevated() {
                return DismCleanupResult {
                    success: false,
                    output_message: "La limpieza profunda de WinSxS requiere ejecutar RustCare con privilegios de Administrador.".to_string(),
                    requires_admin: true,
                };
            }

            let mut args = vec!["/Online", "/Cleanup-Image", "/StartComponentCleanup"];
            if reset_base {
                args.push("/ResetBase");
            }

            let output = Command::new("Dism.exe").args(&args).output();

            match output {
                Ok(out) => {
                    let msg = String::from_utf8_lossy(&out.stdout).to_string();
                    let success = out.status.success();
                    DismCleanupResult {
                        success,
                        output_message: if success {
                            "Limpieza de almacén de componentes WinSxS completada con éxito. Se liberó espacio de actualizaciones obsoletas.".to_string()
                        } else {
                            format!("DISM finalizó con advertencias: {}", msg)
                        },
                        requires_admin: false,
                    }
                }
                Err(e) => DismCleanupResult {
                    success: false,
                    output_message: format!("Error al invocar Dism.exe: {}", e),
                    requires_admin: false,
                },
            }
        }

        #[cfg(not(windows))]
        {
            let _ = reset_base;
            DismCleanupResult {
                success: true,
                output_message: "DISM no es necesario en sistemas Unix/macOS.".to_string(),
                requires_admin: false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dism_status_structure() {
        let status = DismEngine::check_status();
        #[cfg(windows)]
        assert!(status.is_supported);
        #[cfg(not(windows))]
        assert!(!status.is_supported);
    }
}
