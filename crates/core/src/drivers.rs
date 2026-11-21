//! Auditor y Gestor de Controladores de Hardware (RustCare Driver Booster)
use serde::{Deserialize, Serialize};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDriverInfo {
    pub class_name: String,
    pub name: String,
    pub version: String,
    pub date: String,
    pub provider: String,
    pub hardware_id: String,
}

pub struct DriverManager;

impl DriverManager {
    const CONTROL_CLASS_KEY: &'static str = "SYSTEM\\CurrentControlSet\\Control\\Class";

    /// Traduce el GUID de clase de Windows a un nombre amigable
    fn class_guid_to_name(guid: &str) -> &'static str {
        match guid.to_uppercase().as_str() {
            "{4D36E968-E325-11CE-BFC1-08002BE10318}" => "Adaptador de Pantalla (GPU)",
            "{4D36E972-E325-11CE-BFC1-08002BE10318}" => "Adaptador de Red (Ethernet / WiFi)",
            "{4D36E96C-E325-11CE-BFC1-08002BE10318}" => "Dispositivos de Audio y Sonido",
            "{E0CBF06C-CD8B-4647-BB8A-263B43F0F974}" => "Bluetooth",
            "{4D36E967-E325-11CE-BFC1-08002BE10318}" => "Controladores de Almacenamiento",
            "{4D36E978-E325-11CE-BFC1-08002BE10318}" => "Puertos y Controladores USB",
            "{4D36E97D-E325-11CE-BFC1-08002BE10318}" => "Dispositivos del Sistema / Placa",
            "{4D36E96F-E325-11CE-BFC1-08002BE10318}" => "Ratones y Dispositivos Señaladores",
            "{4D36E96B-E325-11CE-BFC1-08002BE10318}" => "Teclados",
            "{6BDD1FC6-810F-11D0-BEC7-08002BE2092F}" => "Cámaras e Imagen",
            _ => "Otros Dispositivos",
        }
    }

    /// Escanea todos los controladores de hardware instalados leyendo de forma segura HKLM
    /// (100% lectura permitida sin privilegios de administrador).
    pub fn scan_installed_drivers() -> Vec<DeviceDriverInfo> {
        let mut drivers = Vec::new();
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        if let Ok(class_root) = hklm.open_subkey(Self::CONTROL_CLASS_KEY) {
            for class_guid in class_root.enum_keys().filter_map(|k| k.ok()) {
                let friendly_class = Self::class_guid_to_name(&class_guid);

                // Filtrar clases no relevantes para drivers principales si se desea, o procesar
                if let Ok(guid_key) = class_root.open_subkey(&class_guid) {
                    for dev_num in guid_key.enum_keys().filter_map(|k| k.ok()) {
                        // Las subclaves numéricas (0000, 0001, etc.) contienen los dispositivos reales
                        if dev_num.chars().all(|c| c.is_ascii_digit()) {
                            if let Ok(dev_key) = guid_key.open_subkey(&dev_num) {
                                let driver_desc: String = dev_key
                                    .get_value("DriverDesc")
                                    .unwrap_or_default();

                                if driver_desc.trim().is_empty() {
                                    continue;
                                }

                                let driver_version: String = dev_key
                                    .get_value("DriverVersion")
                                    .unwrap_or_else(|_| "Desconocida".to_string());

                                let driver_date: String = dev_key
                                    .get_value("DriverDate")
                                    .unwrap_or_else(|_| "Desconocida".to_string());

                                let provider_name: String = dev_key
                                    .get_value("ProviderName")
                                    .unwrap_or_else(|_| "Microsoft".to_string());

                                let matching_id: String = dev_key
                                    .get_value("MatchingDeviceId")
                                    .unwrap_or_default();

                                drivers.push(DeviceDriverInfo {
                                    class_name: friendly_class.to_string(),
                                    name: driver_desc,
                                    version: driver_version,
                                    date: driver_date,
                                    provider: provider_name,
                                    hardware_id: matching_id,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Ordenar por tipo de dispositivo y nombre
        drivers.sort_by(|a, b| {
            a.class_name
                .cmp(&b.class_name)
                .then_with(|| a.name.cmp(&b.name))
        });

        drivers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_drivers() {
        let drivers = DriverManager::scan_installed_drivers();
        println!("Controladores detectados: {}", drivers.len());
        // En cualquier máquina con Windows habrá controladores de GPU/Red/Sistema
        assert!(!drivers.is_empty());
    }
}
