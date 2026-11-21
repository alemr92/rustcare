//! Vista de Análisis 1-Clic (Care) - Diseño Futurista 2026
use crate::theme::RustCareTheme;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Pos2, ProgressBar, RichText, Sense, Stroke,
    Ui, Vec2,
};
use rustcare_core::cleaner::CleanerEngine;

#[derive(Debug, Clone)]
pub struct CareScanOptions {
    pub clean_junk: bool,
    pub clean_browsers: bool,
    #[allow(dead_code)]
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
            ui.add_space(6.0);
            ui.heading(
                RichText::new("🚀 Centro de Optimización 1-Clic")
                    .size(24.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.label(
                RichText::new("Mantenimiento integral 2026: limpieza profunda, privacidad, RAM y protección en un solo toque.")
                    .color(RustCareTheme::TEXT_MUTED),
            );
            ui.add_space(16.0);

            // ==========================================
            // Widget Hero Circular HUD 2026
            // ==========================================
            let hud_size = Vec2::splat(180.0);
            let (rect, response) = ui.allocate_exact_size(hud_size, Sense::click());
            let painter = ui.painter();
            let center = rect.center();
            let radius = hud_size.x / 2.0 - 8.0;

            // Anillos base concéntricos
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(2.0_f32, RustCareTheme::CARD_BORDER),
            );
            painter.circle_stroke(
                center,
                radius - 8.0,
                Stroke::new(1.0_f32, Color32::from_rgb(25, 30, 44)),
            );

            let mut start_scan_clicked = false;
            let mut start_clean_clicked = false;
            let mut new_scan_clicked = false;

            if self.is_scanning || self.is_cleaning {
                // Modo escaneo activo: rotación y arco de progreso
                let arc_color = if self.is_scanning {
                    RustCareTheme::ACCENT_CYAN
                } else {
                    RustCareTheme::SUCCESS_GREEN
                };

                // Arco de progreso
                let sweep_angle = std::f32::consts::TAU * self.progress.clamp(0.05, 1.0);
                let num_points = 48;
                let mut points = Vec::with_capacity(num_points + 1);
                for i in 0..=num_points {
                    let angle = -std::f32::consts::FRAC_PI_2 + (i as f32 / num_points as f32) * sweep_angle;
                    points.push(Pos2::new(
                        center.x + radius * angle.cos(),
                        center.y + radius * angle.sin(),
                    ));
                }
                for w in points.windows(2) {
                    painter.line_segment([w[0], w[1]], Stroke::new(4.0_f32, arc_color));
                }

                // Círculo interno con brillo
                painter.circle_filled(
                    center,
                    radius - 12.0,
                    Color32::from_rgba_premultiplied(0, 229, 255, 15),
                );

                // Porcentaje central
                let pct_text = format!("{:.0}%", self.progress * 100.0);
                painter.text(
                    Pos2::new(center.x, center.y - 10.0),
                    Align2::CENTER_CENTER,
                    pct_text,
                    FontId::proportional(28.0),
                    RustCareTheme::TEXT_PRIMARY,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 18.0),
                    Align2::CENTER_CENTER,
                    if self.is_scanning { "ANALIZANDO" } else { "OPTIMIZANDO" },
                    FontId::proportional(11.0),
                    arc_color,
                );
            } else if self.scan_completed && !self.clean_completed {
                // Escaneo finalizado: anillo ámbar y botón reparar
                painter.circle_stroke(
                    center,
                    radius,
                    Stroke::new(3.5_f32, RustCareTheme::WARNING_AMBER),
                );
                painter.circle_filled(
                    center,
                    radius - 10.0,
                    Color32::from_rgba_premultiplied(245, 158, 11, 20),
                );

                let formatted_size = CleanerEngine::format_bytes(self.total_bytes_found);
                painter.text(
                    Pos2::new(center.x, center.y - 12.0),
                    Align2::CENTER_CENTER,
                    formatted_size,
                    FontId::proportional(20.0),
                    RustCareTheme::WARNING_AMBER,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 12.0),
                    Align2::CENTER_CENTER,
                    "REPARAR AHORA",
                    FontId::proportional(12.0),
                    Color32::WHITE,
                );

                if response.clicked() {
                    start_clean_clicked = true;
                }
            } else if self.clean_completed {
                // Limpieza completada: anillo verde esmeralda
                painter.circle_stroke(
                    center,
                    radius,
                    Stroke::new(3.5_f32, RustCareTheme::SUCCESS_GREEN),
                );
                painter.circle_filled(
                    center,
                    radius - 10.0,
                    Color32::from_rgba_premultiplied(16, 185, 129, 20),
                );

                painter.text(
                    Pos2::new(center.x, center.y - 10.0),
                    Align2::CENTER_CENTER,
                    "✔ 100%",
                    FontId::proportional(24.0),
                    RustCareTheme::SUCCESS_GREEN,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 14.0),
                    Align2::CENTER_CENTER,
                    "SISTEMA ÓPTIMO",
                    FontId::proportional(11.0),
                    RustCareTheme::TEXT_PRIMARY,
                );

                if response.clicked() {
                    new_scan_clicked = true;
                }
            } else {
                // Modo Idle: Radar interactivo
                let is_hovered = response.hovered();
                let ring_color = if is_hovered {
                    RustCareTheme::ACCENT_CYAN
                } else {
                    RustCareTheme::ACCENT_BLUE
                };
                let bg_color = if is_hovered {
                    Color32::from_rgba_premultiplied(0, 229, 255, 30)
                } else {
                    Color32::from_rgba_premultiplied(59, 130, 246, 20)
                };

                painter.circle_stroke(center, radius, Stroke::new(3.0_f32, ring_color));
                painter.circle_filled(center, radius - 10.0, bg_color);

                painter.text(
                    Pos2::new(center.x, center.y - 12.0),
                    Align2::CENTER_CENTER,
                    "⚡",
                    FontId::proportional(28.0),
                    RustCareTheme::ACCENT_CYAN,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 14.0),
                    Align2::CENTER_CENTER,
                    "ESCANEAR",
                    FontId::proportional(14.0),
                    Color32::WHITE,
                );

                if response.clicked() {
                    start_scan_clicked = true;
                }
            }

            ui.add_space(14.0);

            // Subtítulo de estado bajo el HUD
            if self.is_scanning || self.is_cleaning {
                ui.label(
                    RichText::new(&self.status_text)
                        .size(14.0)
                        .strong()
                        .color(RustCareTheme::ACCENT_CYAN),
                );
                ui.add_space(6.0);
                ui.add(ProgressBar::new(self.progress).animate(true));
            } else if self.scan_completed && !self.clean_completed {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "Se detectaron {} elementos ({}) y {} alertas de seguridad.",
                            self.total_items_found,
                            CleanerEngine::format_bytes(self.total_bytes_found),
                            self.threats_found
                        ))
                        .size(15.0)
                        .strong()
                        .color(RustCareTheme::WARNING_AMBER),
                    );
                });
                ui.add_space(10.0);

                if ui
                    .add_sized(
                        [220.0, 44.0],
                        egui::Button::new(
                            RichText::new("✨ Reparar y Optimizar Ahora")
                                .size(16.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(RustCareTheme::SUCCESS_GREEN)
                        .corner_radius(CornerRadius::same(8)),
                    )
                    .clicked()
                {
                    start_clean_clicked = true;
                }
            } else if self.clean_completed {
                ui.label(
                    RichText::new("¡Sistema Optimizado! Espacio recuperado y memoria acelerada con éxito.")
                        .size(15.0)
                        .strong()
                        .color(RustCareTheme::SUCCESS_GREEN),
                );
                ui.add_space(10.0);

                if ui
                    .add_sized(
                        [200.0, 40.0],
                        egui::Button::new(
                            RichText::new("🔄 Nuevo Análisis")
                                .size(15.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(RustCareTheme::ACCENT_BLUE)
                        .corner_radius(CornerRadius::same(8)),
                    )
                    .clicked()
                {
                    new_scan_clicked = true;
                }
            } else {
                if ui
                    .add_sized(
                        [220.0, 44.0],
                        egui::Button::new(
                            RichText::new("🔍 Iniciar Análisis Completo")
                                .size(16.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(RustCareTheme::ACCENT_BLUE)
                        .corner_radius(CornerRadius::same(8)),
                    )
                    .clicked()
                {
                    start_scan_clicked = true;
                }
            }

            if start_scan_clicked {
                self.is_scanning = true;
                self.scan_completed = false;
                self.clean_completed = false;
                self.progress = 0.05;
                on_start_scan(self.options.clone());
            }

            if start_clean_clicked {
                self.is_cleaning = true;
                self.progress = 0.05;
                on_start_clean();
            }

            if new_scan_clicked {
                self.scan_completed = false;
                self.clean_completed = false;
                self.total_bytes_found = 0;
                self.total_items_found = 0;
                self.threats_found = 0;
                self.progress = 0.0;
            }
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(14.0);

        // ==========================================
        // 4 Tarjetas de Cristal de Módulos (2x2 Grid)
        // ==========================================
        ui.label(
            RichText::new("COMPONENTES DE MANTENIMIENTO INTEGRADO")
                .size(13.0)
                .strong()
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.add_space(8.0);

        let half_width = (ui.available_width() - 16.0) / 2.0;

        ui.horizontal(|ui| {
            // Tarjeta 1: Basura del Sistema
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(half_width);
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.options.clean_junk, "");
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("🗑️ Archivos Basura & Temporales")
                                .strong()
                                .color(RustCareTheme::TEXT_PRIMARY),
                        );
                        ui.label(
                            RichText::new("Caché de Windows, volcados, logs y cachés dev.")
                                .small()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                    });
                });
            });

            // Tarjeta 2: Privacidad & Navegadores
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(half_width);
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.options.clean_browsers, "");
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("🔒 Privacidad & Rastreo Web")
                                .strong()
                                .color(RustCareTheme::TEXT_PRIMARY),
                        );
                        ui.label(
                            RichText::new("Historial, cookies, sesiones y telemetría de Windows.")
                                .small()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                    });
                });
            });
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // Tarjeta 3: Optimización de RAM
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(half_width);
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.options.ram_boost, "");
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("⚡ Aceleración de Memoria RAM")
                                .strong()
                                .color(RustCareTheme::TEXT_PRIMARY),
                        );
                        ui.label(
                            RichText::new("Vaciado de Working Sets y purga de caché DNS.")
                                .small()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                    });
                });
            });

            // Tarjeta 4: Escudo de Seguridad
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(half_width);
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.options.malware_shield, "");
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("🛡️ Escudo de Seguridad & Malware")
                                .strong()
                                .color(RustCareTheme::TEXT_PRIMARY),
                        );
                        ui.label(
                            RichText::new("Auditoría de persistencias IFEO y ejecutables temporales.")
                                .small()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                    });
                });
            });
        });
    }
}
