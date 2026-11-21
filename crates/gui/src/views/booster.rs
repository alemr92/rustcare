//! Vista de Aceleración (Booster, RAM, DNS y Gestor de Arranque) - 2026 Edition
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, CornerRadius, RichText, ScrollArea, Ui};
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
        ui.heading(
            RichText::new("⚡ Aceleración y Optimización de Rendimiento")
                .size(22.0)
                .strong()
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.label(
            RichText::new("Vaciado nativo de conjuntos de trabajo (RAM Working Sets) y control total del inicio de Windows.")
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.add_space(15.0);

        // Panel de Acciones Rápidas (RAM & DNS) en tarjetas gemelas de cristal
        let half_width = (ui.available_width() - 16.0) / 2.0;

        ui.horizontal(|ui| {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(half_width);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("🧠 Vaciado de Memoria RAM")
                            .strong()
                            .size(16.0)
                            .color(RustCareTheme::ACCENT_CYAN),
                    );
                    ui.label("Libera memoria no utilizada vaciando los Working Sets de los procesos.");
                    ui.add_space(10.0);

                    if ui
                        .add_sized(
                            [190.0, 38.0],
                            egui::Button::new(
                                RichText::new("⚡ Liberar RAM Ahora")
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(RustCareTheme::ACCENT_BLUE)
                            .corner_radius(CornerRadius::same(8)),
                        )
                        .clicked()
                    {
                        self.ram_result = Some(BoosterEngine::clean_ram());
                    }

                    if let Some(res) = &self.ram_result {
                        ui.add_space(6.0);
                        RustCareTheme::draw_badge(
                            ui,
                            &format!("✔ {} procesos optimizados", res.processes_cleaned),
                            RustCareTheme::SUCCESS_GREEN,
                            Color32::from_rgba_premultiplied(16, 185, 129, 25),
                        );
                    }
                });
            });

            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(half_width);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("🌐 Caché de Red / DNS")
                            .strong()
                            .size(16.0)
                            .color(RustCareTheme::ACCENT_CYAN),
                    );
                    ui.label("Vacía la caché de resolución DNS del sistema para optimizar latencia.");
                    ui.add_space(10.0);

                    if ui
                        .add_sized(
                            [190.0, 38.0],
                            egui::Button::new(
                                RichText::new("🔄 Vaciar Caché DNS")
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(RustCareTheme::ACCENT_BLUE)
                            .corner_radius(CornerRadius::same(8)),
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
                        ui.add_space(6.0);
                        RustCareTheme::draw_badge(
                            ui,
                            msg,
                            RustCareTheme::SUCCESS_GREEN,
                            Color32::from_rgba_premultiplied(16, 185, 129, 25),
                        );
                    }
                });
            });
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // Gestor de Arranque Reversible
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("🚀 Gestor de Arranque Reversible")
                    .size(18.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new("🔄 Actualizar Lista")
                            .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    self.startup_entries = StartupEngine::list_entries();
                }
            });
        });
        ui.label(
            RichText::new("Deshabilita programas de inicio de forma segura: se guardan en copia de respaldo para reactivarlos con 1 clic.")
                .small()
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.add_space(10.0);

        ScrollArea::vertical().show(ui, |ui| {
            RustCareTheme::card_frame().show(ui, |ui| {
                if self.startup_entries.is_empty() {
                    ui.label("No se detectaron programas en el inicio del usuario.");
                } else {
                    let mut toggles: Vec<(usize, bool)> = Vec::new();

                    for (idx, entry) in self.startup_entries.iter().enumerate() {
                        ui.horizontal(|ui| {
                            let status_color = if entry.is_enabled {
                                RustCareTheme::SUCCESS_GREEN
                            } else {
                                RustCareTheme::TEXT_MUTED
                            };

                            let status_bg = if entry.is_enabled {
                                Color32::from_rgba_premultiplied(16, 185, 129, 25)
                            } else {
                                Color32::from_rgba_premultiplied(100, 116, 139, 25)
                            };

                            RustCareTheme::draw_badge(
                                ui,
                                if entry.is_enabled { "Habilitado" } else { "Deshabilitado" },
                                status_color,
                                status_bg,
                            );

                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(&entry.name)
                                        .strong()
                                        .color(RustCareTheme::TEXT_PRIMARY),
                                );
                                ui.label(
                                    RichText::new(&entry.command)
                                        .small()
                                        .color(RustCareTheme::TEXT_DIM),
                                );
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let btn_label = if entry.is_enabled { "Deshabilitar" } else { "Habilitar" };
                                let btn_color = if entry.is_enabled {
                                    RustCareTheme::WARNING_AMBER
                                } else {
                                    RustCareTheme::SUCCESS_GREEN
                                };

                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(btn_label).small().color(Color32::WHITE),
                                        )
                                        .fill(btn_color)
                                        .corner_radius(CornerRadius::same(6)),
                                    )
                                    .clicked()
                                {
                                    toggles.push((idx, !entry.is_enabled));
                                }
                            });
                        });
                        ui.separator();
                    }

                    for (idx, new_state) in toggles {
                        let entry = &mut self.startup_entries[idx];
                        if new_state {
                            let _ = StartupEngine::enable_entry(entry);
                        } else {
                            let _ = StartupEngine::disable_entry(entry);
                        }
                    }
                }
            });
        });
    }
}
