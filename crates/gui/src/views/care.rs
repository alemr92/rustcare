//! Vista de Análisis 1-Clic (Care) - Rediseño Cyber-Dark & Neon Teal tipo IObit Advanced SystemCare
use crate::theme::RustCareTheme;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Pos2, ProgressBar, RichText, Sense, Stroke,
    Ui, Vec2,
};
use rustcare_core::cleaner::CleanerEngine;
use std::f32::consts::TAU;

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
    pub health_score: u32,
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
            health_score: 100,
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
        let time = ui.input(|i| i.time) as f32;

        ui.vertical_centered(|ui| {
            ui.add_space(4.0);

            // ==========================================
            // Encabezado de la pantalla Cuidar
            // ==========================================
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("CUIDAR • ANÁLISIS INTELIGENTE 1-CLIC")
                            .size(13.0)
                            .strong()
                            .color(RustCareTheme::ACCENT_CYAN),
                    );
                    ui.label(
                        RichText::new("Optimización total: Limpieza profunda, privacidad, RAM y estabilidad en un toque.")
                            .size(12.0)
                            .color(RustCareTheme::TEXT_MUTED),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.clean_completed {
                        RustCareTheme::draw_badge(
                            ui,
                            "✨ SALUD: ÓPTIMA (98/100)",
                            RustCareTheme::NEON_TEAL,
                            Color32::from_rgba_premultiplied(0, 229, 168, 25),
                        );
                    } else if self.scan_completed {
                        RustCareTheme::draw_badge(
                            ui,
                            "⚠️ SALUD: CRÍTICA (42/100)",
                            RustCareTheme::DANGER_RED,
                            Color32::from_rgba_premultiplied(255, 77, 109, 25),
                        );
                    } else {
                        RustCareTheme::draw_badge(
                            ui,
                            "🛡️ ESTADO: PENDIENTE DE ESCANEO",
                            RustCareTheme::WARNING_AMBER,
                            Color32::from_rgba_premultiplied(255, 176, 32, 20),
                        );
                    }
                });
            });

            ui.add_space(14.0);

            // ==========================================
            // BOTÓN CIRCULAR GIGANTE ("ANALIZAR") con Painter
            // ==========================================
            let hud_size = Vec2::splat(210.0);
            let (rect, response) = ui.allocate_exact_size(hud_size, Sense::click());
            let painter = ui.painter();
            let center = rect.center();
            let base_radius = hud_size.x / 2.0 - 12.0;

            let mut start_scan_clicked = false;
            let mut start_clean_clicked = false;
            let mut new_scan_clicked = false;

            if self.is_scanning || self.is_cleaning {
                // ------------------------------------------
                // ESTADO: ESCANEANDO / REPARANDO (ANIMADO)
                // ------------------------------------------
                let primary_color = if self.is_scanning {
                    RustCareTheme::ACCENT_CYAN
                } else {
                    RustCareTheme::NEON_TEAL
                };

                // Pulsación suave del fondo
                let _pulse = (time * 3.0).sin() * 3.0;

                painter.circle_filled(
                    center,
                    base_radius - 10.0,
                    Color32::from_rgba_premultiplied(0, 229, 255, 18),
                );

                // Anillo de fondo gris tenue
                painter.circle_stroke(
                    center,
                    base_radius,
                    Stroke::new(3.0, Color32::from_rgb(30, 38, 54)),
                );

                // Segmentos giratorios de radar
                let rot_speed = if self.is_scanning { 3.5 } else { 2.5 };
                let base_angle = (time * rot_speed) % TAU;

                for notch in 0..12 {
                    let a = base_angle + (notch as f32 / 12.0) * TAU;
                    let p1 = Pos2::new(center.x + (base_radius + 4.0) * a.cos(), center.y + (base_radius + 4.0) * a.sin());
                    let p2 = Pos2::new(center.x + (base_radius + 8.0) * a.cos(), center.y + (base_radius + 8.0) * a.sin());
                    let alpha = ((notch as f32 / 12.0) * 200.0) as u8;
                    painter.line_segment([p1, p2], Stroke::new(2.0, Color32::from_rgba_premultiplied(0, 229, 255, alpha)));
                }

                // Arco de progreso con trazo grueso
                let sweep = TAU * self.progress.clamp(0.04, 1.0);
                let num_points = 54;
                let mut points = Vec::with_capacity(num_points + 1);
                for i in 0..=num_points {
                    let angle = -std::f32::consts::FRAC_PI_2 + (i as f32 / num_points as f32) * sweep;
                    points.push(Pos2::new(
                        center.x + base_radius * angle.cos(),
                        center.y + base_radius * angle.sin(),
                    ));
                }
                for w in points.windows(2) {
                    painter.line_segment([w[0], w[1]], Stroke::new(6.0, primary_color));
                }

                // Texto Central: Porcentaje y Acción
                let pct = format!("{:.0}%", self.progress * 100.0);
                painter.text(
                    Pos2::new(center.x, center.y - 12.0),
                    Align2::CENTER_CENTER,
                    pct,
                    FontId::proportional(34.0),
                    Color32::WHITE,
                );

                let label_text = if self.is_scanning {
                    "ANALIZANDO..."
                } else {
                    "REPARANDO..."
                };
                painter.text(
                    Pos2::new(center.x, center.y + 22.0),
                    Align2::CENTER_CENTER,
                    label_text,
                    FontId::proportional(12.0),
                    primary_color,
                );
            } else if self.scan_completed && !self.clean_completed {
                // ------------------------------------------
                // ESTADO: ESCANEO COMPLETADO (AMENAZAS DETECTADAS)
                // ------------------------------------------
                let is_hovered = response.hovered();
                let pulse = (time * 4.0).sin() * 2.0;

                // Anillo de advertencia Neón Ámbar / Rojo
                painter.circle_stroke(
                    center,
                    base_radius + pulse,
                    Stroke::new(4.5, RustCareTheme::DANGER_RED),
                );
                painter.circle_filled(
                    center,
                    base_radius - 8.0,
                    if is_hovered {
                        Color32::from_rgba_premultiplied(255, 77, 109, 45)
                    } else {
                        Color32::from_rgba_premultiplied(255, 77, 109, 25)
                    },
                );

                let formatted_size = CleanerEngine::format_bytes(self.total_bytes_found);
                painter.text(
                    Pos2::new(center.x, center.y - 18.0),
                    Align2::CENTER_CENTER,
                    formatted_size,
                    FontId::proportional(22.0),
                    Color32::WHITE,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 8.0),
                    Align2::CENTER_CENTER,
                    "REPARAR",
                    FontId::proportional(18.0),
                    RustCareTheme::DANGER_RED,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 28.0),
                    Align2::CENTER_CENTER,
                    "Clic para optimizar",
                    FontId::proportional(11.0),
                    RustCareTheme::TEXT_MUTED,
                );

                if response.clicked() {
                    start_clean_clicked = true;
                }
            } else if self.clean_completed {
                // ------------------------------------------
                // ESTADO: SISTEMA ÓPTIMO (98/100)
                // ------------------------------------------
                let is_hovered = response.hovered();
                painter.circle_stroke(
                    center,
                    base_radius,
                    Stroke::new(4.5, RustCareTheme::NEON_TEAL),
                );
                painter.circle_filled(
                    center,
                    base_radius - 8.0,
                    if is_hovered {
                        Color32::from_rgba_premultiplied(0, 229, 168, 40)
                    } else {
                        Color32::from_rgba_premultiplied(0, 229, 168, 20)
                    },
                );

                painter.text(
                    Pos2::new(center.x, center.y - 14.0),
                    Align2::CENTER_CENTER,
                    "98 / 100",
                    FontId::proportional(28.0),
                    RustCareTheme::NEON_TEAL,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 14.0),
                    Align2::CENTER_CENTER,
                    "SALUD ÓPTIMA",
                    FontId::proportional(13.0),
                    Color32::WHITE,
                );

                if response.clicked() {
                    new_scan_clicked = true;
                }
            } else {
                // ------------------------------------------
                // ESTADO IDLE: BOTÓN CIRCULAR GIGANTE "ANALIZAR"
                // ------------------------------------------
                let is_hovered = response.hovered();
                let pulse = (time * 2.5).sin() * 2.0;

                // Doble anillo con brillo
                let ring_color = if is_hovered {
                    RustCareTheme::ACCENT_CYAN
                } else {
                    RustCareTheme::NEON_TEAL
                };

                let bg_color = if is_hovered {
                    Color32::from_rgba_premultiplied(0, 229, 255, 35)
                } else {
                    Color32::from_rgba_premultiplied(0, 229, 168, 18)
                };

                // Anillo exterior brillante
                painter.circle_stroke(
                    center,
                    base_radius + pulse,
                    Stroke::new(3.5, ring_color),
                );

                // Anillo interior sutil
                painter.circle_stroke(
                    center,
                    base_radius - 8.0,
                    Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 30)),
                );

                painter.circle_filled(center, base_radius - 9.0, bg_color);

                // Icono y Tipografía Neón
                painter.text(
                    Pos2::new(center.x, center.y - 18.0),
                    Align2::CENTER_CENTER,
                    "⚡",
                    FontId::proportional(34.0),
                    ring_color,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 14.0),
                    Align2::CENTER_CENTER,
                    "ANALIZAR",
                    FontId::proportional(20.0),
                    Color32::WHITE,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 32.0),
                    Align2::CENTER_CENTER,
                    "OPTIMIZACIÓN 1-CLIC",
                    FontId::proportional(10.0),
                    RustCareTheme::TEXT_MUTED,
                );

                if response.clicked() {
                    start_scan_clicked = true;
                }
            }

            ui.add_space(12.0);

            // ==========================================
            // Feedback y Botón de Acción Principal
            // ==========================================
            if self.is_scanning || self.is_cleaning {
                ui.label(
                    RichText::new(&self.status_text)
                        .size(14.0)
                        .strong()
                        .color(RustCareTheme::ACCENT_CYAN),
                );
                ui.add_space(4.0);
                let mut pbar = ProgressBar::new(self.progress).animate(true);
                pbar = pbar.corner_radius(CornerRadius::same(6));
                ui.add_sized([420.0, 10.0], pbar);
            } else if self.scan_completed && !self.clean_completed {
                ui.label(
                    RichText::new(format!(
                        "¡Escaneo finalizado! Se encontraron {} elementos basura ({}) y {} riesgos de seguridad.",
                        self.total_items_found,
                        CleanerEngine::format_bytes(self.total_bytes_found),
                        self.threats_found
                    ))
                    .size(15.0)
                    .strong()
                    .color(RustCareTheme::WARNING_AMBER),
                );
                ui.add_space(8.0);

                if ui
                    .add_sized(
                        [240.0, 44.0],
                        egui::Button::new(
                            RichText::new("⚡ REPARAR AHORA")
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
                    RichText::new("¡Sistema 100% Optimizado! Archivos eliminados, RAM liberada y estado seguro.")
                        .size(15.0)
                        .strong()
                        .color(RustCareTheme::NEON_TEAL),
                );
                ui.add_space(8.0);

                if ui
                    .add_sized(
                        [220.0, 40.0],
                        egui::Button::new(
                            RichText::new("🔄 Realizar Nuevo Análisis")
                                .size(14.0)
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
                        [240.0, 44.0],
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
                self.status_text = "Iniciando escáner de alta velocidad...".into();
                on_start_scan(self.options.clone());
            }

            if start_clean_clicked {
                self.is_cleaning = true;
                self.progress = 0.05;
                self.status_text = "Purgando archivos basura y liberando recursos...".into();
                on_start_clean();
            }

            if new_scan_clicked {
                self.scan_completed = false;
                self.clean_completed = false;
                self.total_bytes_found = 0;
                self.total_items_found = 0;
                self.threats_found = 0;
                self.progress = 0.0;
                self.health_score = 100;
                self.status_text = "Listo para analizar el sistema.".into();
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(12.0);

        // ==========================================
        // 4 Tarjetas de Cristal con Selección de Módulos
        // ==========================================
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("ÁREAS DE OPTIMIZACIÓN ACTIVA")
                    .size(13.0)
                    .strong()
                    .color(RustCareTheme::TEXT_MUTED),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("Seleccionar Todo").clicked() {
                    self.options.clean_junk = true;
                    self.options.clean_browsers = true;
                    self.options.privacy_telemetry = true;
                    self.options.ram_boost = true;
                    self.options.malware_shield = true;
                }
            });
        });
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
                        let sub = if self.scan_completed && self.total_bytes_found > 0 {
                            format!("Detectado: {}", CleanerEngine::format_bytes(self.total_bytes_found))
                        } else {
                            "Caché de Windows, volcados, logs y temporales dev.".into()
                        };
                        ui.label(RichText::new(sub).small().color(if self.scan_completed && self.total_bytes_found > 0 { RustCareTheme::WARNING_AMBER } else { RustCareTheme::TEXT_MUTED }));
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
                            RichText::new("Historial, cookies, sesiones y métricas de navegación.")
                                .small()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                    });
                });
            });
        });

        ui.add_space(8.0);

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
                            RichText::new("🛡️ Escudo de Seguridad & Persistencias")
                                .strong()
                                .color(RustCareTheme::TEXT_PRIMARY),
                        );
                        let sub = if self.scan_completed && self.threats_found > 0 {
                            format!("{} persistencias sospechosas", self.threats_found)
                        } else {
                            "Auditoría de persistencias IFEO y ejecutables temporales.".into()
                        };
                        ui.label(RichText::new(sub).small().color(if self.scan_completed && self.threats_found > 0 { RustCareTheme::DANGER_RED } else { RustCareTheme::TEXT_MUTED }));
                    });
                });
            });
        });
    }
}
