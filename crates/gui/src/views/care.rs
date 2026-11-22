//! Vista 1: Cuidar (Care) - Basada en 01_care.png con fidelidad pixel-perfect
use crate::icons::VectorIcons;
use crate::theme::RustCareTheme;
use eframe::egui::{
    Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke,
    Ui, Vec2,
};
use rustcare_core::cleaner::CleanerEngine;
use std::f32::consts::TAU;

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
            status_text: "C:\\Windows\\Temp\\~df294.tmp".to_string(),
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
            ui.add_space(8.0);

            // =================================================================
            // 1. CABECERA CENTRADA
            // =================================================================
            let header_text = if self.is_scanning {
                "Analizando el sistema..."
            } else if self.is_cleaning {
                "Optimizando y reparando el sistema..."
            } else if self.scan_completed {
                "Análisis completado"
            } else {
                "El sistema requiere análisis"
            };

            ui.label(
                RichText::new(header_text)
                    .size(14.0)
                    .color(RustCareTheme::TEXT_MUTED),
            );

            ui.add_space(20.0);

            // =================================================================
            // 2. BOTÓN CIRCULAR RADAR (230x230px con egui::Painter)
            // =================================================================
            let radar_size = Vec2::splat(230.0);
            let (rect, response) = ui.allocate_exact_size(radar_size, Sense::click());
            let painter = ui.painter();
            let center = rect.center();

            let mut start_scan_clicked = false;
            let mut start_clean_clicked = false;

            // Velocidad de rotación: time * 2.5 en escaneo, time * 0.4 en reposo
            let rot_speed = if self.is_scanning || self.is_cleaning { 2.5 } else { 0.4 };
            let rot_angle = (time * rot_speed) % TAU;

            // Halo exterior difuso en #00E5A8 (alpha 10% y 18%)
            painter.circle_filled(
                center,
                114.0,
                Color32::from_rgba_premultiplied(0, 229, 168, 18),
            );
            painter.circle_filled(
                center,
                104.0,
                Color32::from_rgba_premultiplied(0, 229, 168, 30),
            );

            // Anillo base exterior tenue
            painter.circle_stroke(
                center,
                100.0,
                Stroke::new(1.0_f32, Color32::from_rgb(26, 36, 52)),
            );

            // Marcas radiales tipo dial (36 ticks finos)
            for i in 0..36 {
                let a = (i as f32 / 36.0) * TAU;
                let is_major = i % 9 == 0;
                let tick_len = if is_major { 6.0 } else { 3.5 };
                let p1 = Pos2::new(center.x + (100.0 - tick_len) * a.cos(), center.y + (100.0 - tick_len) * a.sin());
                let p2 = Pos2::new(center.x + 100.0 * a.cos(), center.y + 100.0 * a.sin());
                let tick_col = if is_major {
                    RustCareTheme::ACCENT_TEAL
                } else {
                    Color32::from_rgb(40, 54, 76)
                };
                painter.line_segment([p1, p2], Stroke::new(1.0_f32, tick_col));
            }

            // 3 arcos segmentados de grosor 4.0px en #00E5A8 y #38BDF8 que rotan continuamente
            let arc_colors = [RustCareTheme::ACCENT_TEAL, RustCareTheme::ACCENT_CYAN, RustCareTheme::ACCENT_TEAL];
            for seg in 0..3 {
                let seg_base = rot_angle + (seg as f32 / 3.0) * TAU;
                let seg_color = arc_colors[seg];
                let num_points = 20;
                let sweep = TAU * 0.20; // 72 grados
                let mut points = Vec::with_capacity(num_points + 1);
                for i in 0..=num_points {
                    let a = seg_base + (i as f32 / num_points as f32) * sweep;
                    points.push(Pos2::new(center.x + 94.0 * a.cos(), center.y + 94.0 * a.sin()));
                }
                for w in points.windows(2) {
                    painter.line_segment([w[0], w[1]], Stroke::new(4.0_f32, seg_color));
                }
            }

            // Anillo interior de contención
            painter.circle_stroke(
                center,
                82.0,
                Stroke::new(1.0_f32, Color32::from_rgb(26, 38, 54)),
            );

            // Barrido de radar cónico semitransparente girando dentro del círculo interior
            let sweep_angle = rot_angle * 1.5;
            let cone_steps = 18;
            for i in 0..cone_steps {
                let a = sweep_angle - (i as f32 / cone_steps as f32) * 0.85;
                let alpha = (120.0 * (1.0 - (i as f32 / cone_steps as f32))) as u8;
                let p = Pos2::new(center.x + 75.0 * a.cos(), center.y + 75.0 * a.sin());
                painter.line_segment([center, p], Stroke::new(1.8_f32, Color32::from_rgba_premultiplied(0, 229, 168, alpha)));
            }

            // Núcleo circular (r = 76px, relleno #0F1722, borde 2.0px #00E5A8)
            let core_fill = Color32::from_rgb(15, 23, 34); // #0F1722
            let core_stroke_color = if self.scan_completed && !self.clean_completed {
                RustCareTheme::ACCENT_CORAL
            } else {
                RustCareTheme::ACCENT_TEAL
            };

            painter.circle_filled(center, 76.0, core_fill);
            painter.circle_stroke(center, 76.0, Stroke::new(2.0_f32, core_stroke_color));

            // Texto o progreso central en el núcleo
            if self.is_scanning || self.is_cleaning {
                let pct = format!("{:.0}%", self.progress * 100.0);
                painter.text(
                    Pos2::new(center.x, center.y - 12.0),
                    Align2::CENTER_CENTER,
                    pct,
                    FontId::proportional(34.0),
                    RustCareTheme::TEXT_PRIMARY,
                );
                // Ruta actual truncada (11px #00E5A8)
                let display_path = if self.status_text.len() > 22 {
                    format!("{}...", &self.status_text[..20])
                } else {
                    self.status_text.clone()
                };
                painter.text(
                    Pos2::new(center.x, center.y + 18.0),
                    Align2::CENTER_CENTER,
                    display_path,
                    FontId::proportional(11.0),
                    RustCareTheme::ACCENT_TEAL,
                );
            } else if self.scan_completed && !self.clean_completed {
                painter.text(
                    Pos2::new(center.x, center.y - 14.0),
                    Align2::CENTER_CENTER,
                    "42 / 100",
                    FontId::proportional(22.0),
                    RustCareTheme::ACCENT_CORAL,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 12.0),
                    Align2::CENTER_CENTER,
                    "REPARAR",
                    FontId::proportional(17.0),
                    Color32::WHITE,
                );
                if response.clicked() {
                    start_clean_clicked = true;
                }
            } else if self.clean_completed {
                painter.text(
                    Pos2::new(center.x, center.y - 14.0),
                    Align2::CENTER_CENTER,
                    "98 / 100",
                    FontId::proportional(24.0),
                    RustCareTheme::ACCENT_TEAL,
                );
                painter.text(
                    Pos2::new(center.x, center.y + 14.0),
                    Align2::CENTER_CENTER,
                    "ÓPTIMO",
                    FontId::proportional(14.0),
                    Color32::WHITE,
                );
                if response.clicked() {
                    start_scan_clicked = true;
                }
            } else {
                // Modo reposo: ANALIZAR (22px Bold #F0F4F8)
                painter.text(
                    center,
                    Align2::CENTER_CENTER,
                    "ANALIZAR",
                    FontId::proportional(22.0),
                    RustCareTheme::TEXT_PRIMARY,
                );
                if response.clicked() {
                    start_scan_clicked = true;
                }
            }

            ui.add_space(42.0);

            // =================================================================
            // 3. FILA INFERIOR: 4 TARJETAS IGUALES EN GRID DE 4 COLUMNAS (height: 105px)
            // =================================================================
            let avail_w = ui.available_width();
            let card_spacing = RustCareTheme::ITEM_SPACING;
            let card_w = (avail_w - (card_spacing * 3.0)) / 4.0;
            let card_h = 105.0;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = card_spacing;

                // TARJETA 1: ARCHIVOS BASURA
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("ARCHIVOS BASURA")
                                .size(11.5)
                                .strong()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::hover());
                            VectorIcons::draw_trash(ui.painter(), icon_rect, RustCareTheme::ACCENT_TEAL);
                            ui.add_space(4.0);

                            let size_text = if self.scan_completed && self.total_bytes_found > 0 {
                                CleanerEngine::format_bytes(self.total_bytes_found)
                            } else {
                                "0 GB".to_string()
                            };
                            ui.label(
                                RichText::new(size_text)
                                    .size(16.0)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                        });
                        ui.add_space(6.0);
                        // Barra de progreso fina (4px)
                        let (bar_rect, _) = ui.allocate_exact_size(Vec2::new(card_w - 34.0, 4.0), Sense::hover());
                        ui.painter().rect_filled(bar_rect, CornerRadius::same(2), RustCareTheme::BG_ELEVATED);
                        let bar_fill = if self.scan_completed && self.total_bytes_found > 0 {
                            (bar_rect.width() * 0.65).min(bar_rect.width())
                        } else {
                            12.0
                        };
                        ui.painter().rect_filled(
                            Rect::from_min_size(bar_rect.min, Vec2::new(bar_fill, 4.0)),
                            CornerRadius::same(2),
                            RustCareTheme::ACCENT_TEAL,
                        );
                    });
                });

                // TARJETA 2: PRIVACIDAD
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("PRIVACIDAD")
                                .size(11.5)
                                .strong()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::hover());
                            VectorIcons::draw_eye(ui.painter(), icon_rect, RustCareTheme::ACCENT_TEAL);
                            ui.add_space(4.0);
                            let status = if self.scan_completed && self.threats_found > 0 {
                                "Riesgos"
                            } else {
                                "Seguro"
                            };
                            ui.label(
                                RichText::new(status)
                                    .size(15.0)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                        });
                        ui.add_space(4.0);
                        if ui.link(RichText::new("Analizar").size(11.5).color(RustCareTheme::TEXT_MUTED)).clicked() {
                            start_scan_clicked = true;
                        }
                    });
                });

                // TARJETA 3: IMPULSO RAM
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("IMPULSO RAM")
                                .size(11.5)
                                .strong()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::hover());
                            VectorIcons::draw_ram_chip(ui.painter(), icon_rect, RustCareTheme::ACCENT_TEAL);
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new("Optimizado")
                                    .size(15.0)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                        });
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("2.006 MB Libres")
                                .size(11.0)
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                    });
                });

                // TARJETA 4: ESCUDO ACTIVO
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("ESCUDO ACTIVO")
                                .size(11.5)
                                .strong()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::hover());
                            VectorIcons::draw_shield(ui.painter(), icon_rect, RustCareTheme::ACCENT_TEAL, true);
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new("Protegido")
                                    .size(15.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_TEAL),
                            );
                        });
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("Protección en tiempo real")
                                .size(11.0)
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                    });
                });
            });

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
        });
    }
}
