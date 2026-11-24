//! Vista 1: Cuidar (Care) - Basada en 01_care.png con fidelidad pixel-perfect
use crate::icons::{draw_icon, AppIcon};
use crate::theme::RustCareTheme;
use crate::widgets;
use eframe::egui::{
    Color32, CornerRadius, RichText, Sense, Stroke, StrokeKind, Ui, Vec2,
};
use rustcare_core::cleaner::CleanerEngine;

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
    pub is_scanning: bool,
    pub is_cleaning: bool,
    pub scan_completed: bool,
    pub clean_completed: bool,
    pub progress: f32,
    pub status_text: String,
    pub total_bytes_found: u64,
    pub total_items_found: usize,
    pub threats_found: usize,
    pub options: CareScanOptions,
    pub health_score: u8,
}

impl Default for CareView {
    fn default() -> Self {
        Self {
            is_scanning: false,
            is_cleaning: false,
            scan_completed: false,
            clean_completed: false,
            progress: 0.0,
            status_text: "Listo para analizar".to_string(),
            total_bytes_found: 0,
            total_items_found: 0,
            threats_found: 0,
            options: CareScanOptions::default(),
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
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);

            // =================================================================
            // 1. CABECERA CENTRADA
            // =================================================================
            let header_text = if self.is_scanning {
                "Analizando sistema..."
            } else if self.is_cleaning {
                "Optimizando y reparando el sistema..."
            } else if self.scan_completed {
                "Diagnóstico completado"
            } else {
                "El sistema requiere optimización"
            };

            ui.label(
                RichText::new(header_text)
                    .size(14.0)
                    .color(RustCareTheme::TEXT_MUTED),
            );

            ui.add_space(16.0);

            // =================================================================
            // 2. BOTÓN CIRCULAR RADAR (240x240px con widgets::care_radar_button)
            // =================================================================
            let mut start_scan_clicked = false;
            let mut start_clean_clicked = false;

            let (title, subtitle, health_opt) = if self.is_scanning {
                (format!("{:.0}%", self.progress * 100.0), self.status_text.clone(), None)
            } else if self.is_cleaning {
                ("LIMPIANDO".to_string(), "Reparando el sistema...".to_string(), None)
            } else if self.scan_completed && !self.clean_completed {
                ("REPARAR".to_string(), "Problemas detectados".to_string(), Some(self.health_score))
            } else if self.clean_completed {
                ("ÓPTIMO".to_string(), "Sistema optimizado".to_string(), Some(self.health_score))
            } else {
                ("ANALIZAR".to_string(), "Escaneo Inteligente".to_string(), None)
            };

            let radar_resp = widgets::care_radar_button(
                ui,
                &title,
                &subtitle,
                self.progress,
                self.is_scanning || self.is_cleaning,
                health_opt,
            );

            if radar_resp.clicked() {
                if self.scan_completed && !self.clean_completed {
                    start_clean_clicked = true;
                } else {
                    start_scan_clicked = true;
                }
            }

            ui.add_space(24.0);

            // =================================================================
            // 3. FILA INFERIOR: 4 TARJETAS IGUALES EN GRID DE 4 COLUMNAS (height: 112px, gap: 14px)
            // =================================================================
            let avail_w = ui.available_width();
            let card_spacing = RustCareTheme::ITEM_SPACING;
            let card_w = (avail_w - (card_spacing * 3.0)) / 4.0;
            let card_h = 112.0;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = card_spacing;

                // TARJETA 1: ARCHIVOS BASURA (Acento Cian Neón)
                let c1_accent = Color32::from_rgb(0, 229, 168);
                RustCareTheme::glass_card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    let card_rect = ui.max_rect();
                    ui.painter().line_segment(
                        [egui::pos2(card_rect.min.x + 8.0, card_rect.min.y + 1.0), egui::pos2(card_rect.max.x - 8.0, card_rect.min.y + 1.0)],
                        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(0, 229, 168, 70)),
                    );

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("ARCHIVOS BASURA")
                                .size(11.0)
                                .strong()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(36.0), Sense::hover());
                            ui.painter().rect_filled(box_rect, CornerRadius::same(10), Color32::from_rgba_unmultiplied(0, 229, 168, 28));
                            ui.painter().rect_stroke(box_rect, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 229, 168, 110)), StrokeKind::Inside);
                            draw_icon(ui.painter(), box_rect, AppIcon::Trash, c1_accent, 1.6);
                            ui.add_space(6.0);

                            let size_text = if self.scan_completed && self.total_bytes_found > 0 {
                                CleanerEngine::format_bytes(self.total_bytes_found)
                            } else {
                                "0 GB".to_string()
                            };
                            ui.label(
                                RichText::new(size_text)
                                    .size(17.0)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                        });
                    });
                });

                // TARJETA 2: ESCUDO PRIVACIDAD (Acento Azure)
                let c2_accent = Color32::from_rgb(56, 189, 248);
                RustCareTheme::glass_card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    let card_rect = ui.max_rect();
                    ui.painter().line_segment(
                        [egui::pos2(card_rect.min.x + 8.0, card_rect.min.y + 1.0), egui::pos2(card_rect.max.x - 8.0, card_rect.min.y + 1.0)],
                        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 70)),
                    );

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("ESCUDO PRIVACIDAD")
                                .size(11.0)
                                .strong()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(36.0), Sense::hover());
                            ui.painter().rect_filled(box_rect, CornerRadius::same(10), Color32::from_rgba_unmultiplied(56, 189, 248, 28));
                            ui.painter().rect_stroke(box_rect, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(56, 189, 248, 110)), StrokeKind::Inside);
                            draw_icon(ui.painter(), box_rect, AppIcon::Eye, c2_accent, 1.6);
                            ui.add_space(6.0);
                            let status = if self.scan_completed && self.threats_found > 0 {
                                "14 rastros"
                            } else {
                                "Seguro"
                            };
                            ui.label(
                                RichText::new(status)
                                    .size(17.0)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                        });
                    });
                });

                // TARJETA 3: RAM TURBO (Acento Teal)
                let c3_accent = Color32::from_rgb(45, 212, 191);
                RustCareTheme::glass_card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    let card_rect = ui.max_rect();
                    ui.painter().line_segment(
                        [egui::pos2(card_rect.min.x + 8.0, card_rect.min.y + 1.0), egui::pos2(card_rect.max.x - 8.0, card_rect.min.y + 1.0)],
                        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(45, 212, 191, 70)),
                    );

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("RAM TURBO")
                                .size(11.0)
                                .strong()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(36.0), Sense::hover());
                            ui.painter().rect_filled(box_rect, CornerRadius::same(10), Color32::from_rgba_unmultiplied(45, 212, 191, 28));
                            ui.painter().rect_stroke(box_rect, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(45, 212, 191, 110)), StrokeKind::Inside);
                            draw_icon(ui.painter(), box_rect, AppIcon::RamChip, c3_accent, 1.6);
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("92% Libre")
                                    .size(17.0)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                        });
                    });
                });

                // TARJETA 4: SALUD DEL SISTEMA IA (Acento Cian)
                let c4_accent = Color32::from_rgb(0, 229, 168);
                RustCareTheme::glass_card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    let card_rect = ui.max_rect();
                    ui.painter().line_segment(
                        [egui::pos2(card_rect.min.x + 8.0, card_rect.min.y + 1.0), egui::pos2(card_rect.max.x - 8.0, card_rect.min.y + 1.0)],
                        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(0, 229, 168, 70)),
                    );

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("SALUD DEL SISTEMA IA")
                                .size(11.0)
                                .strong()
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(36.0), Sense::hover());
                            ui.painter().rect_filled(box_rect, CornerRadius::same(10), Color32::from_rgba_unmultiplied(0, 229, 168, 28));
                            ui.painter().rect_stroke(box_rect, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 229, 168, 110)), StrokeKind::Inside);
                            draw_icon(ui.painter(), box_rect, AppIcon::Brain, c4_accent, 1.6);
                            ui.add_space(6.0);
                            let score_text = if self.scan_completed {
                                format!("{}% Bueno", self.health_score)
                            } else {
                                "Excelente".to_string()
                            };
                            ui.label(
                                RichText::new(score_text)
                                    .size(17.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_CYAN),
                            );
                        });
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
