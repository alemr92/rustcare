//! Vista de Aceleración (Booster, RAM, DNS y Gestor de Arranque)
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, RichText, ScrollArea, Ui};
use rustcare_core::booster::{BoosterEngine, RamCleanResult};
use rustcare_core::startup::{StartupEngine, StartupEntry};

pub struct BoosterView {
    pub startup_entries: Vec<StartupEntry>,
    pub ram_result: Option<RamCleanResult>,
    pub dns_message: Option<String>,
}

impl Default for BoosterView {
    fn default() -> Self {
        Self {
            startup_entries: StartupEngine::list_entries(),
            ram_result: None,
            dns_message: None,
        }
    }
}

impl BoosterView {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("⚡ Aceleración y Optimización de Rendimiento").size(22.0).color(RustCareTheme::TEXT_PRIMARY));
        ui.label(RichText::new("Libera memoria RAM instantáneamente y gestiona las aplicaciones que se inician con Windows.").color(RustCareTheme::TEXT_MUTED));
        ui.add_space(15.0);

        // Panel de Acciones Rápidas (RAM & DNS)
        ui.horizontal(|ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::CARD_BG)
                .corner_radius(8)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("🧠 Vaciado de Memoria RAM").strong().size(16.0).color(RustCareTheme::ACCENT_CYAN));
                        ui.label("Libera memoria no utilizada vaciando los Working Sets de los procesos.");
                        ui.add_space(8.0);

                        if ui
                            .add_sized(
                                [180.0, 36.0],
                                egui::Button::new(RichText::new("⚡ Liberar RAM Ahora").strong().color(Color32::WHITE))
                                    .fill(RustCareTheme::ACCENT_BLUE),
                            )
                            .clicked()
                        {
                            self.ram_result = Some(BoosterEngine::clean_ram());
                        }

                        if let Some(res) = &self.ram_result {
                            ui.label(
                                RichText::new(format!(
                                    "¡Éxito! Se optimizaron {} procesos.",
                                    res.processes_cleaned
                                ))
                                .small()
                                .color(RustCareTheme::SUCCESS_GREEN),
                            );
                        }
                    });
                });

            ui.add_space(15.0);

            egui::Frame::NONE
                .fill(RustCareTheme::CARD_BG)
                .corner_radius(8)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("🌐 Caché de Red / DNS").strong().size(16.0).color(RustCareTheme::ACCENT_CYAN));
                        ui.label("Vacía la caché de resolución DNS para solucionar problemas de conexión.");
                        ui.add_space(8.0);

                        if ui
                            .add_sized(
                                [180.0, 36.0],
                                egui::Button::new(RichText::new("🔄 Vaciar Caché DNS").strong().color(Color32::WHITE))
                                    .fill(RustCareTheme::ACCENT_BLUE),
                            )
                            .clicked()
                        {
                            let ok = BoosterEngine::flush_dns_cache();
                            self.dns_message = Some(if ok {
                                "Caché DNS vaciada con éxito.".to_string()
                            } else {
                                "No se pudo vaciar la caché DNS.".to_string()
                            });
                        }

                        if let Some(msg) = &self.dns_message {
                            ui.label(RichText::new(msg).small().color(RustCareTheme::SUCCESS_GREEN));
                        }
                    });
                });
        });

        ui.add_space(25.0);
        ui.separator();
        ui.add_space(10.0);

        // Gestor de Arranque Reversible
        ui.horizontal(|ui| {
            ui.label(RichText::new("🚀 Gestor de Arranque Reversible").size(18.0).strong().color(RustCareTheme::TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄 Actualizar Lista").clicked() {
                    self.startup_entries = StartupEngine::list_entries();
                }
            });
        });
        ui.label(RichText::new("Deshabilita programas de inicio de forma segura: se guardan en copia de respaldo para reactivarlos con 1 clic.").small().color(RustCareTheme::TEXT_MUTED));
        ui.add_space(10.0);

        ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::CARD_BG)
                .corner_radius(8)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    if self.startup_entries.is_empty() {
                        ui.label("No se detectaron entradas de inicio.");
                        return;
                    }

                    for entry in &self.startup_entries.clone() {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&entry.name).strong());
                                ui.label(RichText::new(&entry.command).small().color(RustCareTheme::TEXT_MUTED));
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if entry.is_enabled {
                                    if ui
                                        .add_sized(
                                            [110.0, 28.0],
                                            egui::Button::new(RichText::new("Deshabilitar").color(Color32::WHITE))
                                                .fill(RustCareTheme::WARNING_AMBER),
                                        )
                                        .clicked()
                                    {
                                        let _ = StartupEngine::disable_entry(entry);
                                        self.startup_entries = StartupEngine::list_entries();
                                    }
                                } else {
                                    if ui
                                        .add_sized(
                                            [110.0, 28.0],
                                            egui::Button::new(RichText::new("Habilitar").color(Color32::WHITE))
                                                .fill(RustCareTheme::SUCCESS_GREEN),
                                        )
                                        .clicked()
                                    {
                                        let _ = StartupEngine::enable_entry(entry);
                                        self.startup_entries = StartupEngine::list_entries();
                                    }
                                }
                            });
                        });
                        ui.separator();
                    }
                });
        });
    }
}
