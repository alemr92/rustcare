//! Vista de Análisis 1-Clic (Care)
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, ProgressBar, RichText, Ui};
use rustcare_core::cleaner::CleanerEngine;

#[derive(Debug, Clone)]
pub struct CareScanOptions {
    pub clean_junk: bool,
    pub clean_browsers: bool,
    pub privacy_telemetry: bool,
    pub ram_boost: bool,
    pub malware_shield: bool,
}

impl Default for CareScanOptions {
    fn default() -> Self {
        Self {
            clean_junk: true,
            clean_browsers: true,
            privacy_telemetry: true,
            ram_boost: true,
            malware_shield: true,
        }
    }
}

pub struct CareView {
    pub options: CareScanOptions,
    pub is_scanning: bool,
    pub is_cleaning: bool,
    pub progress: f32,
    pub status_text: String,
    pub total_bytes_found: u64,
    pub total_items_found: usize,
    pub threats_found: usize,
    pub scan_completed: bool,
    pub clean_completed: bool,
}

impl Default for CareView {
    fn default() -> Self {
        Self {
            options: CareScanOptions::default(),
            is_scanning: false,
            is_cleaning: false,
            progress: 0.0,
            status_text: "Listo para analizar el sistema.".to_string(),
            total_bytes_found: 0,
            total_items_found: 0,
            threats_found: 0,
            scan_completed: false,
            clean_completed: false,
        }
    }
}

impl CareView {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        on_start_scan: impl FnOnce(CareScanOptions),
        on_start_clean: impl FnOnce(),
    ) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading(RichText::new("🚀 Análisis 1-Clic de RustCare").size(24.0).color(RustCareTheme::TEXT_PRIMARY));
            ui.label(RichText::new("Optimización integral de limpieza, privacidad, rendimiento y seguridad con un solo botón.").color(RustCareTheme::TEXT_MUTED));
            ui.add_space(20.0);

            // Botón central grande de análisis / reparación
            if self.is_scanning {
                ui.add(ProgressBar::new(self.progress).show_percentage().animate(true));
                ui.add_space(10.0);
                ui.label(RichText::new(&self.status_text).size(15.0).color(RustCareTheme::ACCENT_CYAN));
            } else if self.is_cleaning {
                ui.add(ProgressBar::new(self.progress).show_percentage().animate(true));
                ui.add_space(10.0);
                ui.label(RichText::new("Optimizando y liberando espacio...").size(15.0).color(RustCareTheme::SUCCESS_GREEN));
            } else if self.scan_completed && !self.clean_completed {
                ui.label(
                    RichText::new(format!(
                        "¡Análisis Finalizado! Se encontraron {} archivos basura ({}) y {} alertas de seguridad.",
                        self.total_items_found,
                        CleanerEngine::format_bytes(self.total_bytes_found),
                        self.threats_found
                    ))
                    .size(16.0)
                    .color(RustCareTheme::WARNING_AMBER),
                );
                ui.add_space(15.0);

                if ui
                    .add_sized(
                        [220.0, 50.0],
                        egui::Button::new(RichText::new("✨ Reparar y Optimizar Ahora").size(18.0).color(Color32::WHITE))
                            .fill(RustCareTheme::SUCCESS_GREEN),
                    )
                    .clicked()
                {
                    on_start_clean();
                }
            } else if self.clean_completed {
                ui.label(
                    RichText::new("¡Sistema Optimizado con Éxito! Se ha liberado espacio y aplicado mejoras de rendimiento.")
                        .size(18.0)
                        .color(RustCareTheme::SUCCESS_GREEN),
                );
                ui.add_space(15.0);

                if ui
                    .add_sized(
                        [200.0, 45.0],
                        egui::Button::new(RichText::new("🔄 Nuevo Análisis").size(16.0).color(Color32::WHITE))
                            .fill(RustCareTheme::ACCENT_BLUE),
                    )
                    .clicked()
                {
                    self.scan_completed = false;
                    self.clean_completed = false;
                    self.total_bytes_found = 0;
                    self.total_items_found = 0;
                    self.threats_found = 0;
                }
            } else {
                if ui
                    .add_sized(
                        [240.0, 60.0],
                        egui::Button::new(RichText::new("🔍 ESCANEAR AHORA").size(20.0).strong().color(Color32::WHITE))
                            .fill(RustCareTheme::ACCENT_BLUE),
                    )
                    .clicked()
                {
                    on_start_scan(self.options.clone());
                }
            }
        });

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(15.0);

        // Opciones de qué incluir en el escaneo
        ui.label(RichText::new("Áreas de optimización incluidas en el análisis:").strong().size(15.0));
        ui.add_space(10.0);

        egui::Grid::new("care_options_grid")
            .num_columns(2)
            .spacing([40.0, 15.0])
            .show(ui, |ui| {
                ui.checkbox(
                    &mut self.options.clean_junk,
                    RichText::new("🗑️ Archivos Basura y Temporales").strong(),
                );
                ui.label("Limpia %TEMP%, volcados de error y cachés del sistema.");
                ui.end_row();

                ui.checkbox(
                    &mut self.options.clean_browsers,
                    RichText::new("🌐 Privacidad y Navegadores").strong(),
                );
                ui.label("Caché web, cookies y sesiones de Chrome, Edge, Firefox y Brave.");
                ui.end_row();

                ui.checkbox(
                    &mut self.options.privacy_telemetry,
                    RichText::new("🛡️ Telemetría y Políticas de Windows").strong(),
                );
                ui.label("Minimiza recolección de diagnósticos, ID publicitario y búsquedas Bing.");
                ui.end_row();

                ui.checkbox(
                    &mut self.options.ram_boost,
                    RichText::new("⚡ Optimización de Memoria RAM").strong(),
                );
                ui.label("Vaciado seguro de conjuntos de trabajo (Working Sets) de aplicaciones.");
                ui.end_row();

                ui.checkbox(
                    &mut self.options.malware_shield,
                    RichText::new("🛡️ Escáner de Amenazas (Shield)").strong(),
                );
                ui.label("Revisa procesos en carpetas temporales y secuestros en el registro.");
                ui.end_row();
            });
    }
}
