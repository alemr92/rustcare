//! Vista 1: Cuidar (Care) - Basada en 01_care.png con fidelidad pixel-perfect
use crate::icons::{draw_icon, AppIcon};
use crate::theme::RustCareTheme;
use crate::widgets;
use eframe::egui::{
    Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Ui,
    Vec2,
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
                "El sistema requiere análisis"
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
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
                            ui.painter().rect_filled(box_rect, CornerRadius::same(6), Color32::from_rgba_unmultiplied(14, 165, 233, 20));
                            ui.painter().rect_stroke(box_rect, CornerRadius::same(6), Stroke::new(1.0, Color32::from_rgba_unmultiplied(14, 165, 233, 40)), StrokeKind::Inside);
                            draw_icon(ui.painter(), box_rect, AppIcon::Trash, RustCareTheme::ACCENT_CYAN, 1.5);
                            ui.add_space(6.0);

                            let size_text = if self.scan_completed && self.total_bytes_found > 0 {
                                CleanerEngine::format_bytes(self.total_bytes_found)
                            } else {
                                "0 GB".to_string()
                            };
                            let val_col = if self.scan_completed && self.total_bytes_found > 0 {
                                RustCareTheme::ACCENT_CORAL
                            } else {
                                RustCareTheme::TEXT_PRIMARY
                            };
                            ui.label(
                                RichText::new(size_text)
                                    .size(16.0)
                                    .strong()
                                    .color(val_col),
                            );
                        });
                        ui.add_space(6.0);
                        // Barra de progreso de 4px
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
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
                            ui.painter().rect_filled(box_rect, CornerRadius::same(6), Color32::from_rgba_unmultiplied(14, 165, 233, 20));
                            ui.painter().rect_stroke(box_rect, CornerRadius::same(6), Stroke::new(1.0, Color32::from_rgba_unmultiplied(14, 165, 233, 40)), StrokeKind::Inside);
                            draw_icon(ui.painter(), box_rect, AppIcon::Eye, RustCareTheme::ACCENT_CYAN, 1.5);
                            ui.add_space(6.0);
                            let (status, col) = if self.scan_completed && self.threats_found > 0 {
                                ("14 rastros", RustCareTheme::ACCENT_AMBER)
                            } else {
                                ("Seguro", RustCareTheme::ACCENT_EMERALD)
                            };
                            ui.label(
                                RichText::new(status)
                                    .size(15.0)
                                    .strong()
                                    .color(col),
                            );
                        });
                        ui.add_space(4.0);
                        let link_rect = Rect::from_min_size(ui.cursor().min, Vec2::new(60.0, 16.0));
                        let link_resp = ui.interact(link_rect, ui.id().with("link_analizar"), Sense::click());
                        ui.painter().text(
                            link_rect.min,
                            Align2::LEFT_TOP,
                            "Analizar",
                            FontId::proportional(11.5),
                            RustCareTheme::TEXT_MUTED,
                        );
                        if link_resp.clicked() {
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
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
                            ui.painter().rect_filled(box_rect, CornerRadius::same(6), Color32::from_rgba_unmultiplied(14, 165, 233, 20));
                            ui.painter().rect_stroke(box_rect, CornerRadius::same(6), Stroke::new(1.0, Color32::from_rgba_unmultiplied(14, 165, 233, 40)), StrokeKind::Inside);
                            draw_icon(ui.painter(), box_rect, AppIcon::RamChip, RustCareTheme::ACCENT_CYAN, 1.5);
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("Optimizado")
                                    .size(15.0)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                        });
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("2.05 GB libres")
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
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
                            ui.painter().rect_filled(box_rect, CornerRadius::same(6), Color32::from_rgba_unmultiplied(14, 165, 233, 20));
                            ui.painter().rect_stroke(box_rect, CornerRadius::same(6), Stroke::new(1.0, Color32::from_rgba_unmultiplied(14, 165, 233, 40)), StrokeKind::Inside);
                            draw_icon(ui.painter(), box_rect, AppIcon::ShieldCheck, RustCareTheme::ACCENT_EMERALD, 1.5);
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("Protegido")
                                    .size(15.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_EMERALD),
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
