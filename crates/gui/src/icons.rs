//! Primitivas de dibujo vectorial 100% nativas para RustCare
//! Prohibido el uso de emojis del sistema operativo. Trazo estándar de 1.5px.
use crate::theme::RustCareTheme;
use eframe::egui::{Color32, Painter, Pos2, Rect, Stroke};
use std::f32::consts::TAU;

pub struct VectorIcons;

impl VectorIcons {
    /// Icono de Radar / Casa para Cuidar / Inicio (18x18)
    pub fn draw_radar(painter: &Painter, rect: Rect, color: Color32) {
        let center = rect.center();
        let stroke = Stroke::new(1.5_f32, color);
        let r = rect.width().min(rect.height()) * 0.42;

        // Círculo exterior
        painter.circle_stroke(center, r, stroke);
        // Círculo concéntrico menor
        painter.circle_stroke(center, r * 0.55, Stroke::new(1.0_f32, color));
        // Línea de barrido / radio
        painter.line_segment([center, Pos2::new(center.x + r * 0.707, center.y - r * 0.707)], stroke);
        // Punto central
        painter.circle_filled(center, 1.8, color);
    }

    /// Icono de Rayo para Acelerar / Turbo Boost (18x18)
    pub fn draw_lightning(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let p = [
            Pos2::new(rect.center().x + 1.0, rect.min.y + 2.0),
            Pos2::new(rect.min.x + 4.0, rect.center().y + 1.0),
            Pos2::new(rect.center().x - 1.0, rect.center().y + 1.0),
            Pos2::new(rect.center().x - 1.0, rect.max.y - 2.0),
            Pos2::new(rect.max.x - 4.0, rect.center().y - 1.0),
            Pos2::new(rect.center().x + 1.0, rect.center().y - 1.0),
        ];
        for w in p.windows(2) {
            painter.line_segment([w[0], w[1]], stroke);
        }
        painter.line_segment([p[5], p[0]], stroke);
    }

    /// Icono de Escoba / Limpieza (18x18)
    pub fn draw_broom(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        // Mango diagonal
        painter.line_segment(
            [Pos2::new(rect.max.x - 3.0, rect.min.y + 3.0), Pos2::new(rect.center().x, rect.center().y)],
            stroke,
        );
        // Cerdas trapezoidales
        let b1 = Pos2::new(rect.center().x - 3.0, rect.center().y - 1.0);
        let b2 = Pos2::new(rect.center().x + 1.0, rect.center().y + 3.0);
        let b3 = Pos2::new(rect.min.x + 3.0, rect.max.y - 2.0);
        let b4 = Pos2::new(rect.min.x + 7.0, rect.max.y - 1.0);
        painter.line_segment([b1, b2], stroke);
        painter.line_segment([b2, b4], stroke);
        painter.line_segment([b4, b3], stroke);
        painter.line_segment([b3, b1], stroke);
    }

    /// Icono de Escudo para Seguridad (18x18)
    pub fn draw_shield(painter: &Painter, rect: Rect, color: Color32, with_check: bool) {
        let stroke = Stroke::new(1.5_f32, color);
        let c = rect.center();
        let top = rect.min.y + 2.5;
        let left = rect.min.x + 3.0;
        let right = rect.max.x - 3.0;
        let bot = rect.max.y - 2.5;

        let points = [
            Pos2::new(left, top),
            Pos2::new(right, top),
            Pos2::new(right, c.y + 1.0),
            Pos2::new(c.x, bot),
            Pos2::new(left, c.y + 1.0),
            Pos2::new(left, top),
        ];
        for w in points.windows(2) {
            painter.line_segment([w[0], w[1]], stroke);
        }

        if with_check {
            let chk = [
                Pos2::new(c.x - 3.5, c.y),
                Pos2::new(c.x - 1.0, c.y + 2.5),
                Pos2::new(c.x + 3.5, c.y - 2.0),
            ];
            painter.line_segment([chk[0], chk[1]], stroke);
            painter.line_segment([chk[1], chk[2]], stroke);
        }
    }

    /// Icono de Herramientas / Llaves Cruzadas (18x18)
    pub fn draw_tools(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        // Barra 1
        painter.line_segment(
            [Pos2::new(rect.min.x + 4.0, rect.min.y + 4.0), Pos2::new(rect.max.x - 4.0, rect.max.y - 4.0)],
            stroke,
        );
        // Barra 2
        painter.line_segment(
            [Pos2::new(rect.max.x - 4.0, rect.min.y + 4.0), Pos2::new(rect.min.x + 4.0, rect.max.y - 4.0)],
            stroke,
        );
        // Cabezales
        painter.circle_stroke(Pos2::new(rect.min.x + 4.0, rect.min.y + 4.0), 2.2, stroke);
        painter.circle_stroke(Pos2::new(rect.max.x - 4.0, rect.max.y - 4.0), 2.2, stroke);
    }

    /// Icono de Monitor / Gráfica de pulso ECG (18x18)
    pub fn draw_monitor(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let cy = rect.center().y;
        let points = [
            Pos2::new(rect.min.x + 2.0, cy),
            Pos2::new(rect.min.x + 5.0, cy),
            Pos2::new(rect.min.x + 7.5, cy - 6.0),
            Pos2::new(rect.min.x + 10.5, cy + 6.0),
            Pos2::new(rect.min.x + 13.0, cy),
            Pos2::new(rect.max.x - 2.0, cy),
        ];
        for w in points.windows(2) {
            painter.line_segment([w[0], w[1]], stroke);
        }
    }

    /// Icono de Engranaje para Ajustes (18x18)
    pub fn draw_gear(painter: &Painter, rect: Rect, color: Color32) {
        let center = rect.center();
        let stroke = Stroke::new(1.5_f32, color);
        let r = rect.width().min(rect.height()) * 0.38;

        painter.circle_stroke(center, r * 0.5, stroke);
        painter.circle_stroke(center, r, Stroke::new(1.0_f32, color));

        for i in 0..6 {
            let a = (i as f32 / 6.0) * TAU;
            let p1 = Pos2::new(center.x + (r - 1.0) * a.cos(), center.y + (r - 1.0) * a.sin());
            let p2 = Pos2::new(center.x + (r + 2.5) * a.cos(), center.y + (r + 2.5) * a.sin());
            painter.line_segment([p1, p2], stroke);
        }
    }

    /// Icono Papelera / Archivos Basura (24x24 o superior)
    pub fn draw_trash(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let top = rect.min.y + rect.height() * 0.22;
        let bot = rect.max.y - rect.height() * 0.15;
        let left = rect.min.x + rect.width() * 0.25;
        let right = rect.max.x - rect.width() * 0.25;

        // Tapa
        painter.line_segment([Pos2::new(left - 3.0, top), Pos2::new(right + 3.0, top)], stroke);
        painter.line_segment(
            [Pos2::new(rect.center().x - 3.0, top - 3.0), Pos2::new(rect.center().x + 3.0, top - 3.0)],
            stroke,
        );

        // Cuerpo trapezoidal
        painter.line_segment([Pos2::new(left, top), Pos2::new(left + 2.0, bot)], stroke);
        painter.line_segment([Pos2::new(right, top), Pos2::new(right - 2.0, bot)], stroke);
        painter.line_segment([Pos2::new(left + 2.0, bot), Pos2::new(right - 2.0, bot)], stroke);

        // Ranuras verticales
        let cx = rect.center().x;
        painter.line_segment([Pos2::new(cx, top + 4.0), Pos2::new(cx, bot - 3.0)], Stroke::new(1.2_f32, color));
        painter.line_segment([Pos2::new(cx - 3.5, top + 4.0), Pos2::new(cx - 3.0, bot - 3.0)], Stroke::new(1.2_f32, color));
        painter.line_segment([Pos2::new(cx + 3.5, top + 4.0), Pos2::new(cx + 3.0, bot - 3.0)], Stroke::new(1.2_f32, color));
    }

    /// Icono Ojo / Privacidad (24x24)
    pub fn draw_eye(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let c = rect.center();
        let w = rect.width() * 0.40;
        let h = rect.height() * 0.24;

        // Puntos contorno de ojo
        let p1 = Pos2::new(c.x - w, c.y);
        let p2 = Pos2::new(c.x, c.y - h);
        let p3 = Pos2::new(c.x + w, c.y);
        let p4 = Pos2::new(c.x, c.y + h);

        painter.line_segment([p1, p2], stroke);
        painter.line_segment([p2, p3], stroke);
        painter.line_segment([p3, p4], stroke);
        painter.line_segment([p4, p1], stroke);

        // Pupila central
        painter.circle_filled(c, 2.8, color);
    }

    /// Icono Chip RAM (24x24)
    pub fn draw_ram_chip(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let inner = rect.shrink(rect.width() * 0.22);

        painter.rect_stroke(inner, eframe::egui::CornerRadius::same(2), stroke, eframe::egui::StrokeKind::Inside);

        // Pines exteriores horizontales
        for i in 0..3 {
            let y = inner.min.y + inner.height() * (0.25 + i as f32 * 0.25);
            painter.line_segment([Pos2::new(rect.min.x + 1.0, y), Pos2::new(inner.min.x, y)], stroke);
            painter.line_segment([Pos2::new(inner.max.x, y), Pos2::new(rect.max.x - 1.0, y)], stroke);
        }

        // Pines exteriores verticales
        for i in 0..3 {
            let x = inner.min.x + inner.width() * (0.25 + i as f32 * 0.25);
            painter.line_segment([Pos2::new(x, rect.min.y + 1.0), Pos2::new(x, inner.min.y)], stroke);
            painter.line_segment([Pos2::new(x, inner.max.y), Pos2::new(x, rect.max.y - 1.0)], stroke);
        }
    }

    /// Icono Maletín / Modo Trabajo (32x32)
    pub fn draw_briefcase(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let body = Rect::from_min_max(
            Pos2::new(rect.min.x + 3.0, rect.min.y + 8.0),
            Pos2::new(rect.max.x - 3.0, rect.max.y - 4.0),
        );
        painter.rect_stroke(body, eframe::egui::CornerRadius::same(4), stroke, eframe::egui::StrokeKind::Inside);

        // Asa
        let h_left = rect.center().x - 6.0;
        let h_right = rect.center().x + 6.0;
        painter.line_segment([Pos2::new(h_left, body.min.y), Pos2::new(h_left, rect.min.y + 4.0)], stroke);
        painter.line_segment([Pos2::new(h_left, rect.min.y + 4.0), Pos2::new(h_right, rect.min.y + 4.0)], stroke);
        painter.line_segment([Pos2::new(h_right, rect.min.y + 4.0), Pos2::new(h_right, body.min.y)], stroke);

        // Línea central
        painter.line_segment([Pos2::new(body.min.x, body.center().y), Pos2::new(body.max.x, body.center().y)], Stroke::new(1.0_f32, color));
    }

    /// Icono Gamepad / Modo Juego (32x32)
    pub fn draw_gamepad(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let body = Rect::from_min_max(
            Pos2::new(rect.min.x + 3.0, rect.min.y + 7.0),
            Pos2::new(rect.max.x - 3.0, rect.max.y - 7.0),
        );
        painter.rect_stroke(body, eframe::egui::CornerRadius::same(7), stroke, eframe::egui::StrokeKind::Inside);

        // D-Pad a la izquierda
        let d_center = Pos2::new(body.min.x + 8.0, body.center().y);
        painter.line_segment([Pos2::new(d_center.x - 3.5, d_center.y), Pos2::new(d_center.x + 3.5, d_center.y)], stroke);
        painter.line_segment([Pos2::new(d_center.x, d_center.y - 3.5), Pos2::new(d_center.x, d_center.y + 3.5)], stroke);

        // 2 Botones a la derecha
        let b1 = Pos2::new(body.max.x - 9.0, body.center().y - 2.5);
        let b2 = Pos2::new(body.max.x - 6.0, body.center().y + 2.5);
        painter.circle_filled(b1, 1.8, color);
        painter.circle_filled(b2, 1.8, color);
    }

    /// Icono Batería / Ahorro de Energía (32x32)
    pub fn draw_battery(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let body = Rect::from_min_max(
            Pos2::new(rect.min.x + 5.0, rect.min.y + 7.0),
            Pos2::new(rect.max.x - 7.0, rect.max.y - 7.0),
        );
        painter.rect_stroke(body, eframe::egui::CornerRadius::same(3), stroke, eframe::egui::StrokeKind::Inside);

        // Terminal positivo
        let term = Rect::from_min_max(
            Pos2::new(body.max.x, body.center().y - 3.5),
            Pos2::new(body.max.x + 2.5, body.center().y + 3.5),
        );
        painter.rect_filled(term, eframe::egui::CornerRadius::same(1), color);

        // Rayo dentro de la batería
        let c = body.center();
        let pts = [
            Pos2::new(c.x + 1.0, c.y - 4.5),
            Pos2::new(c.x - 2.5, c.y),
            Pos2::new(c.x + 0.5, c.y),
            Pos2::new(c.x - 1.0, c.y + 4.5),
        ];
        painter.line_segment([pts[0], pts[1]], stroke);
        painter.line_segment([pts[1], pts[2]], stroke);
        painter.line_segment([pts[2], pts[3]], stroke);
    }

    /// Icono Destructor de Archivos (32x32)
    pub fn draw_shredder(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let c = rect.center();
        // Barra ranura horizontal
        painter.line_segment([Pos2::new(rect.min.x + 3.0, c.y - 2.0), Pos2::new(rect.max.x - 3.0, c.y - 2.0)], stroke);
        painter.line_segment([Pos2::new(rect.min.x + 3.0, c.y + 2.0), Pos2::new(rect.max.x - 3.0, c.y + 2.0)], stroke);

        // Hoja arriba
        let page = Rect::from_min_max(Pos2::new(c.x - 7.0, rect.min.y + 3.0), Pos2::new(c.x + 7.0, c.y - 2.0));
        painter.rect_stroke(page, eframe::egui::CornerRadius::same(1), stroke, eframe::egui::StrokeKind::Inside);

        // Tiras de papel triturado abajo
        for i in 0..4 {
            let x = c.x - 6.0 + i as f32 * 4.0;
            painter.line_segment([Pos2::new(x, c.y + 2.0), Pos2::new(x, rect.max.y - 3.0)], Stroke::new(1.2_f32, color));
        }
    }

    /// Icono Buscador de Duplicados (32x32)
    pub fn draw_duplicate(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        // Página trasera
        let p_back = Rect::from_min_max(Pos2::new(rect.min.x + 7.0, rect.min.y + 3.0), Pos2::new(rect.max.x - 3.0, rect.max.y - 7.0));
        painter.rect_stroke(p_back, eframe::egui::CornerRadius::same(2), Stroke::new(1.0_f32, color), eframe::egui::StrokeKind::Inside);
        // Página delantera
        let p_front = Rect::from_min_max(Pos2::new(rect.min.x + 3.0, rect.min.y + 7.0), Pos2::new(rect.max.x - 7.0, rect.max.y - 3.0));
        painter.rect_filled(p_front, eframe::egui::CornerRadius::same(2), RustCareTheme::BG_SURFACE);
        painter.rect_stroke(p_front, eframe::egui::CornerRadius::same(2), stroke, eframe::egui::StrokeKind::Inside);
    }

    /// Icono Avatar Usuario (Topbar)
    pub fn draw_user(painter: &Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5_f32, color);
        let c = rect.center();
        // Cabeza
        painter.circle_stroke(Pos2::new(c.x, c.y - 3.0), 3.5, stroke);
        // Hombros
        let s_left = Pos2::new(c.x - 6.0, c.y + 6.0);
        let s_top = Pos2::new(c.x, c.y + 2.0);
        let s_right = Pos2::new(c.x + 6.0, c.y + 6.0);
        painter.line_segment([s_left, s_top], stroke);
        painter.line_segment([s_top, s_right], stroke);
    }
}
