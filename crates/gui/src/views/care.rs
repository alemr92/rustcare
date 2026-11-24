//! Vista 1: Cuidar (Care) - Basada en 01_care.png con fidelidad pixel-perfect
use crate::icons::{draw_icon, AppIcon};
use crate::theme::RustCareTheme;
use crate::widgets;
use eframe::egui::{
    Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Ui, Vec2,
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
            // 2. BOTÓN CIRCULAR RADAR (260x260px) + SCI-FI HUD BACKGROUND
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
                ("ANALIZAR".to_string(), String::new(), None)
            };

            // Overlay Sci-Fi HUD en el fondo del reactor
            let cursor_pos = ui.cursor().min;
            let hud_center = Pos2::new(cursor_pos.x + ui.available_width() / 2.0, cursor_pos.y + 130.0);
            Self::draw_scifi_hud_overlay(ui.painter(), hud_center);

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
            // 3. FILA INFERIOR: 4 BENTO GLASS CARDS CON UNDERGLOW NEÓN 2027
            // =================================================================
            let avail_w = ui.available_width();
            let card_spacing = RustCareTheme::ITEM_SPACING;
            let card_w = (avail_w - (card_spacing * 3.0)) / 4.0;
            let card_h = 112.0;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = card_spacing;

                // TARJETA 1: ARCHIVOS BASURA
                let c1_accent = Color32::from_rgb(0, 229, 168);
                let (c1_rect, c1_resp) = ui.allocate_exact_size(Vec2::new(card_w, card_h), Sense::click());
                RustCareTheme::draw_glass_card(ui.painter(), c1_rect, c1_resp.hovered(), Some(c1_accent));

                let p1 = ui.painter();
                p1.text(
                    Pos2::new(c1_rect.min.x + 16.0, c1_rect.min.y + 16.0),
                    Align2::LEFT_TOP,
                    "ARCHIVOS BASURA",
                    FontId::proportional(11.0),
                    RustCareTheme::TEXT_MUTED,
                );
                let ic1_box = Rect::from_min_size(Pos2::new(c1_rect.min.x + 16.0, c1_rect.min.y + 44.0), Vec2::splat(44.0));
                p1.rect_filled(ic1_box, CornerRadius::same(10), Color32::from_rgba_unmultiplied(0, 229, 168, 22));
                p1.rect_stroke(ic1_box, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 229, 168, 90)), StrokeKind::Inside);
                draw_icon(p1, ic1_box, AppIcon::Trash, c1_accent, 1.6);

                let size_text = if self.scan_completed && self.total_bytes_found > 0 {
                    CleanerEngine::format_bytes(self.total_bytes_found)
                } else {
                    "0 GB".to_string()
                };
                p1.text(
                    Pos2::new(c1_rect.min.x + 72.0, c1_rect.min.y + 64.0),
                    Align2::LEFT_CENTER,
                    size_text,
                    FontId::proportional(19.0),
                    Color32::WHITE,
                );

                if c1_resp.clicked() {
                    start_scan_clicked = true;
                }

                // TARJETA 2: ESCUDO PRIVACIDAD
                let c2_accent = Color32::from_rgb(56, 189, 248);
                let (c2_rect, c2_resp) = ui.allocate_exact_size(Vec2::new(card_w, card_h), Sense::click());
                RustCareTheme::draw_glass_card(ui.painter(), c2_rect, c2_resp.hovered(), Some(c2_accent));

                let p2 = ui.painter();
                p2.text(
                    Pos2::new(c2_rect.min.x + 16.0, c2_rect.min.y + 16.0),
                    Align2::LEFT_TOP,
                    "ESCUDO PRIVACIDAD",
                    FontId::proportional(11.0),
                    RustCareTheme::TEXT_MUTED,
                );
                let ic2_box = Rect::from_min_size(Pos2::new(c2_rect.min.x + 16.0, c2_rect.min.y + 44.0), Vec2::splat(44.0));
                p2.rect_filled(ic2_box, CornerRadius::same(10), Color32::from_rgba_unmultiplied(56, 189, 248, 22));
                p2.rect_stroke(ic2_box, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(56, 189, 248, 90)), StrokeKind::Inside);
                draw_icon(p2, ic2_box, AppIcon::Eye, c2_accent, 1.6);

                let priv_status = if self.scan_completed && self.threats_found > 0 {
                    "14 rastros"
                } else {
                    "Seguro"
                };
                p2.text(
                    Pos2::new(c2_rect.min.x + 72.0, c2_rect.min.y + 64.0),
                    Align2::LEFT_CENTER,
                    priv_status,
                    FontId::proportional(19.0),
                    Color32::WHITE,
                );

                // TARJETA 3: RAM TURBO
                let c3_accent = Color32::from_rgb(45, 212, 191);
                let (c3_rect, c3_resp) = ui.allocate_exact_size(Vec2::new(card_w, card_h), Sense::click());
                RustCareTheme::draw_glass_card(ui.painter(), c3_rect, c3_resp.hovered(), Some(c3_accent));

                let p3 = ui.painter();
                p3.text(
                    Pos2::new(c3_rect.min.x + 16.0, c3_rect.min.y + 16.0),
                    Align2::LEFT_TOP,
                    "RAM TURBO",
                    FontId::proportional(11.0),
                    RustCareTheme::TEXT_MUTED,
                );
                let ic3_box = Rect::from_min_size(Pos2::new(c3_rect.min.x + 16.0, c3_rect.min.y + 44.0), Vec2::splat(44.0));
                p3.rect_filled(ic3_box, CornerRadius::same(10), Color32::from_rgba_unmultiplied(45, 212, 191, 22));
                p3.rect_stroke(ic3_box, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(45, 212, 191, 90)), StrokeKind::Inside);
                draw_icon(p3, ic3_box, AppIcon::RamChip, c3_accent, 1.6);

                p3.text(
                    Pos2::new(c3_rect.min.x + 72.0, c3_rect.min.y + 64.0),
                    Align2::LEFT_CENTER,
                    "92% Libre",
                    FontId::proportional(19.0),
                    Color32::WHITE,
                );

                // TARJETA 4: SALUD DEL SISTEMA IA
                let c4_accent = Color32::from_rgb(0, 229, 168);
                let (c4_rect, c4_resp) = ui.allocate_exact_size(Vec2::new(card_w, card_h), Sense::click());
                RustCareTheme::draw_glass_card(ui.painter(), c4_rect, c4_resp.hovered(), Some(c4_accent));

                let p4 = ui.painter();
                p4.text(
                    Pos2::new(c4_rect.min.x + 16.0, c4_rect.min.y + 16.0),
                    Align2::LEFT_TOP,
                    "SALUD DEL SISTEMA IA",
                    FontId::proportional(11.0),
                    RustCareTheme::TEXT_MUTED,
                );
                let ic4_box = Rect::from_min_size(Pos2::new(c4_rect.min.x + 16.0, c4_rect.min.y + 44.0), Vec2::splat(44.0));
                p4.rect_filled(ic4_box, CornerRadius::same(10), Color32::from_rgba_unmultiplied(0, 229, 168, 22));
                p4.rect_stroke(ic4_box, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 229, 168, 90)), StrokeKind::Inside);
                draw_icon(p4, ic4_box, AppIcon::Brain, c4_accent, 1.6);

                let health_str = if self.scan_completed {
                    format!("{}% Bueno", self.health_score)
                } else {
                    "Excelente".to_string()
                };
                p4.text(
                    Pos2::new(c4_rect.min.x + 72.0, c4_rect.min.y + 64.0),
                    Align2::LEFT_CENTER,
                    health_str,
                    FontId::proportional(19.0),
                    RustCareTheme::ACCENT_CYAN,
                );
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

    fn draw_scifi_hud_overlay(painter: &eframe::egui::Painter, center: Pos2) {
        let hud_cyan = Color32::from_rgba_unmultiplied(56, 189, 248, 55);
        let hud_cyan_bright = Color32::from_rgba_unmultiplied(56, 189, 248, 120);
        let text_dim = Color32::from_rgba_unmultiplied(160, 185, 215, 115);

        // 1. 4 Soportes de esquina HUD (Corner Brackets)
        let sx = 225.0_f32;
        let sy = 135.0_f32;
        let blen = 22.0_f32;

        // Top-Left
        let tl = Pos2::new(center.x - sx, center.y - sy);
        painter.line_segment([tl, Pos2::new(tl.x + blen, tl.y)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([tl, Pos2::new(tl.x, tl.y + blen)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([Pos2::new(tl.x - 4.0, tl.y), Pos2::new(tl.x + 4.0, tl.y)], Stroke::new(1.0_f32, hud_cyan_bright));
        painter.line_segment([Pos2::new(tl.x, tl.y - 4.0), Pos2::new(tl.x, tl.y + 4.0)], Stroke::new(1.0_f32, hud_cyan_bright));

        // Top-Right
        let tr = Pos2::new(center.x + sx, center.y - sy);
        painter.line_segment([tr, Pos2::new(tr.x - blen, tr.y)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([tr, Pos2::new(tr.x, tr.y + blen)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([Pos2::new(tr.x - 4.0, tr.y), Pos2::new(tr.x + 4.0, tr.y)], Stroke::new(1.0_f32, hud_cyan_bright));
        painter.line_segment([Pos2::new(tr.x, tr.y - 4.0), Pos2::new(tr.x, tr.y + 4.0)], Stroke::new(1.0_f32, hud_cyan_bright));

        // Bottom-Left
        let bl = Pos2::new(center.x - sx, center.y + sy);
        painter.line_segment([bl, Pos2::new(bl.x + blen, bl.y)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([bl, Pos2::new(bl.x, bl.y - blen)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([Pos2::new(bl.x - 4.0, bl.y), Pos2::new(bl.x + 4.0, bl.y)], Stroke::new(1.0_f32, hud_cyan_bright));
        painter.line_segment([Pos2::new(bl.x, bl.y - 4.0), Pos2::new(bl.x, bl.y + 4.0)], Stroke::new(1.0_f32, hud_cyan_bright));

        // Bottom-Right
        let br = Pos2::new(center.x + sx, center.y + sy);
        painter.line_segment([br, Pos2::new(br.x - blen, br.y)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([br, Pos2::new(br.x, br.y - blen)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([Pos2::new(br.x - 4.0, br.y), Pos2::new(br.x + 4.0, br.y)], Stroke::new(1.0_f32, hud_cyan_bright));
        painter.line_segment([Pos2::new(br.x, br.y - 4.0), Pos2::new(br.x, br.y + 4.0)], Stroke::new(1.0_f32, hud_cyan_bright));

        // 2. Anillo orbital HUD difuso exterior (r = 168.0)
        let orb_r = 168.0_f32;
        painter.circle_stroke(center, orb_r, Stroke::new(0.8_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 22)));

        // 3. Reglas verticales de telemetría y muescas (Escala izquierda y derecha)
        let scale_lx = center.x - 172.0_f32;
        let scale_rx = center.x + 172.0_f32;
        for i in -4..=4 {
            let ny = center.y + (i as f32) * 16.0_f32;
            let nw = if i == 0 { 10.0_f32 } else if i % 2 == 0 { 6.0_f32 } else { 3.5_f32 };
            painter.line_segment([Pos2::new(scale_lx, ny), Pos2::new(scale_lx + nw, ny)], Stroke::new(0.9_f32, hud_cyan));
            painter.line_segment([Pos2::new(scale_rx - nw, ny), Pos2::new(scale_rx, ny)], Stroke::new(0.9_f32, hud_cyan));
        }

        // 4. Telemetría izquierda (exacta a 2027_care.png)
        let font_nano = FontId::monospace(8.5);
        painter.text(Pos2::new(center.x - 198.0, center.y - 88.0), Align2::LEFT_CENTER, "33.6000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x - 222.0, center.y - 8.0), Align2::LEFT_CENTER, "360000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x - 204.0, center.y + 68.0), Align2::LEFT_CENTER, "0.025", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x - 198.0, center.y + 82.0), Align2::LEFT_CENTER, "-491500", font_nano.clone(), text_dim);

        // Caja de barras holográfica izquierda
        let box_l = Rect::from_min_size(Pos2::new(center.x - 224.0, center.y + 20.0), Vec2::new(26.0, 18.0));
        painter.rect_stroke(box_l, CornerRadius::ZERO, Stroke::new(0.8_f32, hud_cyan), StrokeKind::Inside);
        for b in 0..3 {
            let bar_y = box_l.min.y + 3.0 + (b as f32) * 4.5;
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(box_l.min.x + 3.0, bar_y), Vec2::new(14.0 - (b as f32) * 3.0, 2.5)),
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(0, 229, 168, 140),
            );
        }

        // 5. Telemetría derecha (exacta a 2027_care.png)
        painter.text(Pos2::new(center.x + 168.0, center.y - 96.0), Align2::LEFT_CENTER, "3.60000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 178.0, center.y - 84.0), Align2::LEFT_CENTER, "70000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 188.0, center.y - 72.0), Align2::LEFT_CENTER, "15000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 218.0, center.y - 24.0), Align2::LEFT_CENTER, "+30000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 226.0, center.y + 18.0), Align2::LEFT_CENTER, "52000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 178.0, center.y + 76.0), Align2::LEFT_CENTER, "0.020", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 178.0, center.y + 88.0), Align2::LEFT_CENTER, "0.300", font_nano, text_dim);

        // Segmento holográfico curvo derecho
        for a_deg in (15..=45).step_by(5) {
            let rad = (a_deg as f32).to_radians();
            let p_arc = Pos2::new(center.x + 154.0 * rad.cos(), center.y + 154.0 * rad.sin());
            painter.circle_filled(p_arc, 1.2_f32, Color32::from_rgba_unmultiplied(0, 229, 168, 110));
        }
    }
}
