//! Ficha Técnica Completa de Hardware y Sistema ("System Information Hub")
use crate::cleaner::CleanerEngine;
use crate::drivers::DriverManager;
use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHardwareReport {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub host_name: String,
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub memory_total: String,
    pub memory_free: String,
    pub gpus: Vec<String>,
    pub storage_units: Vec<String>,
}

pub struct SystemInfoHub;

impl SystemInfoHub {
    /// Genera la ficha técnica integral del equipo
    pub fn get_report() -> SystemHardwareReport {
        let mut sys = System::new_all();
        sys.refresh_all();

        let os_name = System::name().unwrap_or_else(|| "Windows".to_string());
        let os_version = System::os_version().unwrap_or_default();
        let kernel_version = System::kernel_version().unwrap_or_default();
        let host_name = System::host_name().unwrap_or_default();

        let cpu_brand = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Procesador Genérico".to_string());
        let cpu_cores = sys.cpus().len();

        let memory_total = CleanerEngine::format_bytes(sys.total_memory());
        let memory_free = CleanerEngine::format_bytes(sys.available_memory());

        let drivers = DriverManager::scan_installed_drivers();
        let gpus: Vec<String> = drivers
            .iter()
            .filter(|d| d.class_name.contains("Pantalla") || d.class_name.contains("GPU"))
            .map(|d| format!("{} (v{})", d.name, d.version))
            .collect();

        let storage_units = sysinfo::Disks::new_with_refreshed_list()
            .iter()
            .map(|d| {
                format!(
                    "{} [{}] - {}",
                    d.name().to_string_lossy(),
                    d.mount_point().to_string_lossy(),
                    CleanerEngine::format_bytes(d.total_space())
                )
            })
            .collect();

        SystemHardwareReport {
            os_name,
            os_version,
            kernel_version,
            host_name,
            cpu_brand,
            cpu_cores,
            memory_total,
            memory_free,
            gpus,
            storage_units,
        }
    }

    /// Exporta la ficha técnica en formato de texto legible para diagnósticos o soporte técnico
    pub fn export_text_report() -> String {
        let r = Self::get_report();
        let mut s = String::new();
        s.push_str("====================================================\n");
        s.push_str("          INFORME DE HARDWARE - RUSTCARE 2026       \n");
        s.push_str("====================================================\n\n");
        s.push_str(&format!("Equipo / Host: {}\n", r.host_name));
        s.push_str(&format!("Sistema Operativo: {} {}\n", r.os_name, r.os_version));
        s.push_str(&format!("Versión de Kernel / Build: {}\n", r.kernel_version));
        s.push_str(&format!("Procesador: {} ({} hilos lógicos)\n", r.cpu_brand, r.cpu_cores));
        s.push_str(&format!("Memoria RAM Total: {} (Disponible: {})\n", r.memory_total, r.memory_free));
        s.push_str("\n--- Tarjetas Gráficas y Controladores Visuales ---\n");
        for gpu in &r.gpus {
            s.push_str(&format!("• {}\n", gpu));
        }
        s.push_str("\n--- Unidades de Almacenamiento ---\n");
        for disk in &r.storage_units {
            s.push_str(&format!("• {}\n", disk));
        }
        s.push_str("\n====================================================\n");
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_report() {
        let rep = SystemInfoHub::get_report();
        assert!(!rep.cpu_brand.is_empty());
        assert!(!rep.memory_total.is_empty());
    }
}
