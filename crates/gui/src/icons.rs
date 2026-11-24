//! Primitivas de dibujo vectorial 100% nativas para RustCare
//! Prohibido el uso de emojis del sistema operativo. Trazo estándar de 1.5px.
use eframe::egui::{Color32, Painter, Pos2, Rect, Stroke};
use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppIcon {
    LogoShield,
    Home,
    Bolt,
    Broom,
    ShieldCheck,
    Tools,
    Pulse,
    Gear,
    Trash,
    Eye,
    RamChip,
    Briefcase,
    Gamepad,
    BatteryBolt,
    Refresh,
    SearchDoc,
    Shredder,
    Gauge,
    User,
    Brain,
    Folder,
}

pub fn draw_icon(painter: &Painter, rect: Rect, icon: AppIcon, color: Color32, stroke_width: f32) {
    let stroke = Stroke::new(stroke_width, color);
    let c = rect.center();
    let w = rect.width();
    let h = rect.height();

    match icon {
        AppIcon::LogoShield => {
            // Hexágono exterior
            let r = (w.min(h) * 0.46).max(6.0);
            let mut hex_pts = Vec::with_capacity(6);
            for i in 0..6 {
                let angle = (i as f32) * (TAU / 6.0) - (TAU / 12.0);
                hex_pts.push(Pos2::new(c.x + r * angle.cos(), c.y + r * angle.sin()));
            }
            for i in 0..6 {
                painter.line_segment([hex_pts[i], hex_pts[(i + 1) % 6]], stroke);
            }
            // Rayo interno estilizado
            let bolt = [
                Pos2::new(c.x + 1.0, c.y - r * 0.55),
                Pos2::new(c.x - r * 0.35, c.y + 0.5),
                Pos2::new(c.x, c.y + 0.5),
                Pos2::new(c.x - 1.0, c.y + r * 0.55),
                Pos2::new(c.x + r * 0.35, c.y - 0.5),
                Pos2::new(c.x, c.y - 0.5),
            ];
            for i in 0..6 {
                painter.line_segment([bolt[i], bolt[(i + 1) % 6]], Stroke::new(1.2_f32, color));
            }
        }

        AppIcon::Home => {
            let left = rect.min.x + 3.0;
            let right = rect.max.x - 3.0;
            let top = rect.min.y + 2.5;
            let bot = rect.max.y - 2.5;
            let mid_y = c.y - 0.5;

            // Tejado triangular
            painter.line_segment([Pos2::new(left - 0.5, mid_y + 1.0), Pos2::new(c.x, top)], stroke);
            painter.line_segment([Pos2::new(c.x, top), Pos2::new(right + 0.5, mid_y + 1.0)], stroke);

            // Base rectangular
            painter.line_segment([Pos2::new(left + 2.0, mid_y + 1.0), Pos2::new(left + 2.0, bot)], stroke);
            painter.line_segment([Pos2::new(right - 2.0, mid_y + 1.0), Pos2::new(right - 2.0, bot)], stroke);
            painter.line_segment([Pos2::new(left + 2.0, bot), Pos2::new(right - 2.0, bot)], stroke);

            // Puerta central
            painter.line_segment([Pos2::new(c.x - 2.5, bot), Pos2::new(c.x - 2.5, bot - 5.0)], stroke);
            painter.line_segment([Pos2::new(c.x - 2.5, bot - 5.0), Pos2::new(c.x + 2.5, bot - 5.0)], stroke);
            painter.line_segment([Pos2::new(c.x + 2.5, bot - 5.0), Pos2::new(c.x + 2.5, bot)], stroke);
        }

        AppIcon::Bolt => {
            let p = [
                Pos2::new(c.x + 1.0, rect.min.y + 2.0),
                Pos2::new(rect.min.x + 4.0, c.y + 1.0),
                Pos2::new(c.x - 1.0, c.y + 1.0),
                Pos2::new(c.x - 1.0, rect.max.y - 2.0),
                Pos2::new(rect.max.x - 4.0, c.y - 1.0),
                Pos2::new(c.x + 1.0, c.y - 1.0),
            ];
            for i in 0..6 {
                painter.line_segment([p[i], p[(i + 1) % 6]], stroke);
            }
        }

        AppIcon::Broom => {
            // Mango diagonal
            painter.line_segment(
                [Pos2::new(rect.max.x - 3.0, rect.min.y + 3.0), Pos2::new(c.x, c.y)],
                stroke,
            );
            // Cerdas trapezoidales
            let b1 = Pos2::new(c.x - 3.0, c.y - 1.0);
            let b2 = Pos2::new(c.x + 1.0, c.y + 3.0);
            let b3 = Pos2::new(rect.min.x + 3.0, rect.max.y - 2.0);
            let b4 = Pos2::new(rect.min.x + 7.5, rect.max.y - 1.0);
            painter.line_segment([b1, b2], stroke);
            painter.line_segment([b2, b4], stroke);
            painter.line_segment([b4, b3], stroke);
            painter.line_segment([b3, b1], stroke);

            // 3 líneas de cerdas internas
            painter.line_segment([Pos2::new(c.x - 1.0, c.y + 1.0), Pos2::new(rect.min.x + 4.5, rect.max.y - 1.5)], stroke);
            painter.line_segment([Pos2::new(c.x, c.y + 2.0), Pos2::new(rect.min.x + 6.0, rect.max.y - 1.2)], stroke);
        }

        AppIcon::ShieldCheck => {
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

            let chk = [
                Pos2::new(c.x - 3.5, c.y),
                Pos2::new(c.x - 1.0, c.y + 2.5),
                Pos2::new(c.x + 3.5, c.y - 2.0),
            ];
            painter.line_segment([chk[0], chk[1]], stroke);
            painter.line_segment([chk[1], chk[2]], stroke);
        }

        AppIcon::Tools => {
            // Dos líneas cruzadas con topes/llaves
            painter.line_segment(
                [Pos2::new(rect.min.x + 4.0, rect.min.y + 4.0), Pos2::new(rect.max.x - 4.0, rect.max.y - 4.0)],
                stroke,
            );
            painter.line_segment(
                [Pos2::new(rect.max.x - 4.0, rect.min.y + 4.0), Pos2::new(rect.min.x + 4.0, rect.max.y - 4.0)],
                stroke,
            );
            // Topes ensanchados
            painter.line_segment([Pos2::new(rect.min.x + 2.5, rect.min.y + 5.5), Pos2::new(rect.min.x + 5.5, rect.min.y + 2.5)], stroke);
            painter.line_segment([Pos2::new(rect.max.x - 2.5, rect.max.y - 5.5), Pos2::new(rect.max.x - 5.5, rect.max.y - 2.5)], stroke);
            painter.line_segment([Pos2::new(rect.max.x - 5.5, rect.min.y + 2.5), Pos2::new(rect.max.x - 2.5, rect.min.y + 5.5)], stroke);
            painter.line_segment([Pos2::new(rect.min.x + 5.5, rect.max.y - 2.5), Pos2::new(rect.min.x + 2.5, rect.max.y - 5.5)], stroke);
        }

        AppIcon::Pulse => {
            let left = rect.min.x + 2.0;
            let right = rect.max.x - 2.0;
            let mid_y = c.y;

            let pts = [
                Pos2::new(left, mid_y),
                Pos2::new(c.x - 5.0, mid_y),
                Pos2::new(c.x - 3.0, mid_y - 4.5),
                Pos2::new(c.x - 1.0, mid_y + 4.5),
                Pos2::new(c.x + 1.0, mid_y - 7.0),
                Pos2::new(c.x + 3.0, mid_y + 3.0),
                Pos2::new(c.x + 5.0, mid_y),
                Pos2::new(right, mid_y),
            ];
            for w in pts.windows(2) {
                painter.line_segment([w[0], w[1]], stroke);
            }
        }

        AppIcon::Gear => {
            let r = (w.min(h) * 0.32).max(4.0);
            painter.circle_stroke(c, r, stroke);
            painter.circle_filled(c, 1.8, color);

            // 8 dientes radiales
            for i in 0..8 {
                let angle = (i as f32) * (TAU / 8.0);
                let p1 = Pos2::new(c.x + r * angle.cos(), c.y + r * angle.sin());
                let p2 = Pos2::new(c.x + (r + 2.8) * angle.cos(), c.y + (r + 2.8) * angle.sin());
                painter.line_segment([p1, p2], stroke);
            }
        }

        AppIcon::Trash => {
            let top = rect.min.y + 4.0;
            let bot = rect.max.y - 4.0;
            let left = rect.min.x + 5.0;
            let right = rect.max.x - 7.0;

            // Halo volumétrico difuso de fondo
            painter.circle_filled(c, (w.min(h) * 0.45).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 25));

            // Asa y tapa del cubo
            painter.line_segment([Pos2::new(c.x - 3.0, top - 2.0), Pos2::new(c.x + 3.0, top - 2.0)], stroke);
            painter.line_segment([Pos2::new(left - 2.0, top), Pos2::new(right + 2.0, top)], stroke);

            // Cuerpo trapezoidal del cubo
            let p_body = [
                Pos2::new(left, top),
                Pos2::new(right, top),
                Pos2::new(right - 2.0, bot),
                Pos2::new(left + 2.0, bot),
            ];
            painter.line_segment([p_body[0], p_body[3]], stroke);
            painter.line_segment([p_body[1], p_body[2]], stroke);
            painter.line_segment([p_body[2], p_body[3]], stroke);

            // Líneas de ranura vertical
            painter.line_segment([Pos2::new(c.x - 2.5, top + 3.0), Pos2::new(c.x - 2.0, bot - 2.5)], stroke);
            painter.line_segment([Pos2::new(c.x + 0.5, top + 3.0), Pos2::new(c.x + 0.5, bot - 2.5)], stroke);

            // Engranaje técnico luminoso en esquina inferior derecha (Firma 2027)
            let g_c = Pos2::new(rect.max.x - 6.0, rect.max.y - 6.0);
            let g_r = 5.0;
            painter.circle_stroke(g_c, g_r, Stroke::new(1.2, Color32::from_rgb(56, 189, 248)));
            painter.circle_filled(g_c, 1.8, Color32::from_rgb(0, 229, 168));
            for i in 0..6 {
                let a = (i as f32) * (TAU / 6.0);
                let p1 = Pos2::new(g_c.x + (g_r - 1.0) * a.cos(), g_c.y + (g_r - 1.0) * a.sin());
                let p2 = Pos2::new(g_c.x + (g_r + 2.2) * a.cos(), g_c.y + (g_r + 2.2) * a.sin());
                painter.line_segment([p1, p2], Stroke::new(1.2, Color32::from_rgb(56, 189, 248)));
            }
        }

        AppIcon::Eye => {
            // Escudo con ojo holográfico (Escudo Privacidad 2027)
            let top = rect.min.y + 2.5;
            let bot = rect.max.y - 2.5;
            let left = rect.min.x + 3.5;
            let right = rect.max.x - 3.5;

            // Halo difuso
            painter.circle_filled(c, (w.min(h) * 0.44).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 25));

            // Silueta del Escudo exterior
            let s_pts = [
                Pos2::new(left, top),
                Pos2::new(right, top),
                Pos2::new(right, c.y + 1.0),
                Pos2::new(c.x, bot),
                Pos2::new(left, c.y + 1.0),
                Pos2::new(left, top),
            ];
            for win in s_pts.windows(2) {
                painter.line_segment([win[0], win[1]], stroke);
            }

            // Ojo holográfico interior
            let eye_c = Pos2::new(c.x, c.y - 1.0);
            let e_l = Pos2::new(eye_c.x - 5.5, eye_c.y);
            let e_r = Pos2::new(eye_c.x + 5.5, eye_c.y);
            let e_t = Pos2::new(eye_c.x, eye_c.y - 3.5);
            let e_b = Pos2::new(eye_c.x, eye_c.y + 3.5);

            painter.line_segment([e_l, e_t], Stroke::new(1.2, color));
            painter.line_segment([e_t, e_r], Stroke::new(1.2, color));
            painter.line_segment([e_r, e_b], Stroke::new(1.2, color));
            painter.line_segment([e_b, e_l], Stroke::new(1.2, color));

            painter.circle_filled(eye_c, 1.8, Color32::from_rgb(0, 229, 168));
        }

        AppIcon::RamChip => {
            // Procesador / Chip RAM con pines y trazas de circuito luminosas
            let left = rect.min.x + 5.5;
            let right = rect.max.x - 5.5;
            let top = rect.min.y + 5.5;
            let bot = rect.max.y - 5.5;

            painter.circle_filled(c, (w.min(h) * 0.42).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 25));

            // Cuerpo del microchip con bisel
            painter.rect_filled(
                Rect::from_min_max(Pos2::new(left, top), Pos2::new(right, bot)),
                eframe::egui::CornerRadius::same(3),
                Color32::from_rgba_unmultiplied(0, 229, 168, 30),
            );
            painter.rect_stroke(
                Rect::from_min_max(Pos2::new(left, top), Pos2::new(right, bot)),
                eframe::egui::CornerRadius::same(3),
                stroke,
                eframe::egui::StrokeKind::Inside,
            );

            // Pines de circuito
            let count = 4;
            let step = (right - left) / (count as f32 + 1.0);
            for i in 1..=count {
                let x = left + step * (i as f32);
                painter.line_segment([Pos2::new(x, rect.min.y + 2.0), Pos2::new(x, top)], Stroke::new(1.2, Color32::from_rgb(56, 189, 248)));
                painter.line_segment([Pos2::new(x, bot), Pos2::new(x, rect.max.y - 2.0)], Stroke::new(1.2, Color32::from_rgb(56, 189, 248)));
            }
            let step_y = (bot - top) / (count as f32 + 1.0);
            for i in 1..=count {
                let y = top + step_y * (i as f32);
                painter.line_segment([Pos2::new(rect.min.x + 2.0, y), Pos2::new(left, y)], Stroke::new(1.2, Color32::from_rgb(56, 189, 248)));
                painter.line_segment([Pos2::new(right, y), Pos2::new(rect.max.x - 2.0, y)], Stroke::new(1.2, Color32::from_rgb(56, 189, 248)));
            }

            // Núcleo central luminoso
            painter.circle_filled(c, 2.2, Color32::from_rgb(0, 229, 168));
        }

        AppIcon::Briefcase => {
            let left = rect.min.x + 3.0;
            let right = rect.max.x - 3.0;
            let top = rect.min.y + 5.0;
            let bot = rect.max.y - 3.0;

            // Cuerpo maletín
            painter.rect_stroke(
                Rect::from_min_max(Pos2::new(left, top), Pos2::new(right, bot)),
                eframe::egui::CornerRadius::same(3),
                stroke,
                eframe::egui::StrokeKind::Inside,
            );

            // Asa superior
            painter.line_segment([Pos2::new(c.x - 4.0, top), Pos2::new(c.x - 4.0, rect.min.y + 2.5)], stroke);
            painter.line_segment([Pos2::new(c.x - 4.0, rect.min.y + 2.5), Pos2::new(c.x + 4.0, rect.min.y + 2.5)], stroke);
            painter.line_segment([Pos2::new(c.x + 4.0, rect.min.y + 2.5), Pos2::new(c.x + 4.0, top)], stroke);

            // Línea central y cerradura
            painter.line_segment([Pos2::new(left, c.y), Pos2::new(right, c.y)], Stroke::new(1.0_f32, color));
            painter.rect_filled(Rect::from_center_size(c, eframe::egui::vec2(3.0, 2.5)), eframe::egui::CornerRadius::same(1), color);
        }

        AppIcon::Gamepad => {
            let left = rect.min.x + 2.5;
            let right = rect.max.x - 2.5;
            let top = rect.min.y + 4.5;
            let bot = rect.max.y - 3.5;

            // Contorno mando
            painter.rect_stroke(
                Rect::from_min_max(Pos2::new(left, top), Pos2::new(right, bot)),
                eframe::egui::CornerRadius::same(6),
                stroke,
                eframe::egui::StrokeKind::Inside,
            );

            // Cruceta izquierda
            let pad_c = Pos2::new(left + 5.0, c.y + 0.5);
            painter.line_segment([Pos2::new(pad_c.x - 2.5, pad_c.y), Pos2::new(pad_c.x + 2.5, pad_c.y)], stroke);
            painter.line_segment([Pos2::new(pad_c.x, pad_c.y - 2.5), Pos2::new(pad_c.x, pad_c.y + 2.5)], stroke);

            // 2 botones derecha
            painter.circle_filled(Pos2::new(right - 6.5, c.y - 0.5), 1.3, color);
            painter.circle_filled(Pos2::new(right - 3.5, c.y + 1.5), 1.3, color);
        }

        AppIcon::BatteryBolt => {
            let left = rect.min.x + 4.5;
            let right = rect.max.x - 4.5;
            let top = rect.min.y + 4.5;
            let bot = rect.max.y - 2.5;

            // Borne superior
            painter.line_segment([Pos2::new(c.x - 2.0, rect.min.y + 2.5), Pos2::new(c.x + 2.0, rect.min.y + 2.5)], stroke);
            painter.line_segment([Pos2::new(c.x - 2.0, rect.min.y + 2.5), Pos2::new(c.x - 2.0, top)], stroke);
            painter.line_segment([Pos2::new(c.x + 2.0, rect.min.y + 2.5), Pos2::new(c.x + 2.0, top)], stroke);

            // Cuerpo batería
            painter.rect_stroke(
                Rect::from_min_max(Pos2::new(left, top), Pos2::new(right, bot)),
                eframe::egui::CornerRadius::same(2),
                stroke,
                eframe::egui::StrokeKind::Inside,
            );

            // Rayo central
            let b_pts = [
                Pos2::new(c.x + 0.5, top + 2.0),
                Pos2::new(c.x - 2.0, c.y + 0.5),
                Pos2::new(c.x + 0.5, c.y + 0.5),
                Pos2::new(c.x - 0.5, bot - 2.0),
                Pos2::new(c.x + 2.0, c.y - 0.5),
                Pos2::new(c.x - 0.5, c.y - 0.5),
            ];
            for w in b_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(1.0_f32, color));
            }
        }

        AppIcon::Refresh => {
            let r = (w.min(h) * 0.36).max(5.0);
            let start_angle: f32 = -0.3_f32;
            let end_angle: f32 = start_angle + 4.8_f32; // ~275 deg
            let steps = 18;
            let mut prev = Pos2::new(c.x + r * start_angle.cos(), c.y + r * start_angle.sin());

            for i in 1..=steps {
                let frac = i as f32 / steps as f32;
                let a = start_angle + frac * (end_angle - start_angle);
                let next = Pos2::new(c.x + r * a.cos(), c.y + r * a.sin());
                painter.line_segment([prev, next], stroke);
                prev = next;
            }

            // Flecha triangular en el extremo final
            let arrow_tip = prev;
            painter.line_segment([arrow_tip, Pos2::new(arrow_tip.x + 3.0, arrow_tip.y - 2.0)], stroke);
            painter.line_segment([arrow_tip, Pos2::new(arrow_tip.x - 1.0, arrow_tip.y - 3.5)], stroke);
        }

        AppIcon::SearchDoc => {
            let left = rect.min.x + 3.0;
            let right = rect.max.x - 6.0;
            let top = rect.min.y + 2.5;
            let bot = rect.max.y - 4.5;
            let fold = 3.5;

            // Hoja doblada
            painter.line_segment([Pos2::new(left, top), Pos2::new(right - fold, top)], stroke);
            painter.line_segment([Pos2::new(right - fold, top), Pos2::new(right, top + fold)], stroke);
            painter.line_segment([Pos2::new(right, top + fold), Pos2::new(right, bot)], stroke);
            painter.line_segment([Pos2::new(right, bot), Pos2::new(left, bot)], stroke);
            painter.line_segment([Pos2::new(left, bot), Pos2::new(left, top)], stroke);

            // Pliegue
            painter.line_segment([Pos2::new(right - fold, top), Pos2::new(right - fold, top + fold)], stroke);
            painter.line_segment([Pos2::new(right - fold, top + fold), Pos2::new(right, top + fold)], stroke);

            // Lupa superpuesta abajo derecha
            let m_c = Pos2::new(rect.max.x - 4.5, rect.max.y - 4.5);
            painter.circle_stroke(m_c, 3.0, stroke);
            painter.line_segment([Pos2::new(m_c.x + 2.0, m_c.y + 2.0), Pos2::new(rect.max.x - 1.0, rect.max.y - 1.0)], stroke);
        }

        AppIcon::Shredder => {
            let left = rect.min.x + 3.0;
            let right = rect.max.x - 3.0;
            let mid_y = c.y - 1.0;

            // Ranura superior
            painter.rect_stroke(
                Rect::from_min_max(Pos2::new(left, mid_y - 2.5), Pos2::new(right, mid_y + 2.5)),
                eframe::egui::CornerRadius::same(2),
                stroke,
                eframe::egui::StrokeKind::Inside,
            );

            // Hoja entrando por arriba
            painter.line_segment([Pos2::new(left + 2.5, rect.min.y + 2.5), Pos2::new(right - 2.5, rect.min.y + 2.5)], stroke);
            painter.line_segment([Pos2::new(left + 2.5, rect.min.y + 2.5), Pos2::new(left + 2.5, mid_y - 2.5)], stroke);
            painter.line_segment([Pos2::new(right - 2.5, rect.min.y + 2.5), Pos2::new(right - 2.5, mid_y - 2.5)], stroke);

            // 4 tiras de papel saliendo abajo
            let step = (right - left - 4.0) / 3.0;
            for i in 0..4 {
                let x = left + 2.0 + (i as f32) * step;
                painter.line_segment([Pos2::new(x, mid_y + 2.5), Pos2::new(x, rect.max.y - 2.0)], stroke);
            }
        }

        AppIcon::Gauge => {
            let r = (w.min(h) * 0.40).max(5.0);
            let start = 0.75 * TAU; // Arriba
            let span = 0.67 * TAU;  // 240 deg
            let arc_start = start - span / 2.0;
            let steps = 16;
            let mut prev = Pos2::new(c.x + r * arc_start.cos(), c.y + r * arc_start.sin());

            for i in 1..=steps {
                let frac = i as f32 / steps as f32;
                let a = arc_start + frac * span;
                let next = Pos2::new(c.x + r * a.cos(), c.y + r * a.sin());
                painter.line_segment([prev, next], stroke);
                prev = next;
            }

            // Aguja apuntando a las 2 en punto (~30 deg / 0.5 rad)
            let needle_angle: f32 = -0.55_f32;
            let needle_end = Pos2::new(c.x + (r - 2.0) * needle_angle.cos(), c.y + (r - 2.0) * needle_angle.sin());
            painter.line_segment([c, needle_end], stroke);
            painter.circle_filled(c, 1.8, color);
        }

        AppIcon::User => {
            let head_r = (w.min(h) * 0.18).max(2.5);
            let head_c = Pos2::new(c.x, c.y - 2.5);
            painter.circle_stroke(head_c, head_r, stroke);

            let body_top = Pos2::new(c.x, c.y + head_r);
            let shoulder_w = head_r * 2.2;
            let shoulder_y = body_top.y + 4.0;
            painter.line_segment([Pos2::new(c.x - shoulder_w, shoulder_y), Pos2::new(c.x + shoulder_w, shoulder_y)], stroke);
            painter.line_segment([Pos2::new(c.x - shoulder_w, shoulder_y), Pos2::new(c.x - shoulder_w + 1.0, rect.max.y - 2.0)], stroke);
            painter.line_segment([Pos2::new(c.x + shoulder_w, shoulder_y), Pos2::new(c.x + shoulder_w - 1.0, rect.max.y - 2.0)], stroke);
        }

        AppIcon::Brain => {
            // Cerebro IA 2027 con halo volumétrico y onda de pulso neuronal
            let r_lobe = (w.min(h) * 0.36).max(5.0);
            let left_cx = c.x - r_lobe * 0.38;
            let right_cx = c.x + r_lobe * 0.38;

            // Halo difuso
            painter.circle_filled(c, (w.min(h) * 0.44).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 25));

            // Lóbulo superior izquierdo y derecho
            painter.circle_stroke(Pos2::new(left_cx, c.y - r_lobe * 0.35), r_lobe * 0.52, Stroke::new(stroke_width, color));
            painter.circle_stroke(Pos2::new(right_cx, c.y - r_lobe * 0.35), r_lobe * 0.52, Stroke::new(stroke_width, color));

            // Lóbulo inferior izquierdo y derecho
            painter.circle_stroke(Pos2::new(left_cx + 1.0, c.y + r_lobe * 0.28), r_lobe * 0.44, Stroke::new(stroke_width, color));
            painter.circle_stroke(Pos2::new(right_cx - 1.0, c.y + r_lobe * 0.28), r_lobe * 0.44, Stroke::new(stroke_width, color));

            // Hendidura central
            painter.line_segment([Pos2::new(c.x, c.y - r_lobe * 0.85), Pos2::new(c.x, c.y + r_lobe * 0.65)], stroke);

            // Nodos y sinapsis neuronales luminosas
            painter.circle_filled(Pos2::new(left_cx, c.y - 1.0), 1.8, Color32::from_rgb(0, 229, 168));
            painter.circle_filled(Pos2::new(right_cx, c.y - 1.0), 1.8, Color32::from_rgb(0, 229, 168));
            painter.line_segment([Pos2::new(left_cx, c.y - 1.0), Pos2::new(c.x, c.y - 1.0)], Stroke::new(1.2, color));
            painter.line_segment([Pos2::new(right_cx, c.y - 1.0), Pos2::new(c.x, c.y - 1.0)], Stroke::new(1.2, color));

            // Onda de electrocardiograma / pulso vital en la parte inferior
            let py = c.y + r_lobe * 0.85;
            let ecg = [
                Pos2::new(c.x - r_lobe * 0.8, py),
                Pos2::new(c.x - r_lobe * 0.3, py),
                Pos2::new(c.x - r_lobe * 0.15, py - 3.5),
                Pos2::new(c.x, py + 3.0),
                Pos2::new(c.x + r_lobe * 0.15, py - 2.0),
                Pos2::new(c.x + r_lobe * 0.3, py),
                Pos2::new(c.x + r_lobe * 0.8, py),
            ];
            for w in ecg.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(1.2, Color32::from_rgb(0, 229, 168)));
            }
        }

        AppIcon::Folder => {
            let left = rect.min.x + 2.5;
            let right = rect.max.x - 2.5;
            let top = rect.min.y + 3.0;
            let bot = rect.max.y - 2.5;
            let tab_w = (right - left) * 0.45;

            // Pestaña superior de la carpeta
            painter.line_segment([Pos2::new(left, top + 3.0), Pos2::new(left, top)], stroke);
            painter.line_segment([Pos2::new(left, top), Pos2::new(left + tab_w, top)], stroke);
            painter.line_segment([Pos2::new(left + tab_w, top), Pos2::new(left + tab_w + 3.0, top + 3.0)], stroke);
            painter.line_segment([Pos2::new(left + tab_w + 3.0, top + 3.0), Pos2::new(right, top + 3.0)], stroke);

            // Cuerpo de la carpeta
            painter.line_segment([Pos2::new(right, top + 3.0), Pos2::new(right, bot)], stroke);
            painter.line_segment([Pos2::new(right, bot), Pos2::new(left, bot)], stroke);
            painter.line_segment([Pos2::new(left, bot), Pos2::new(left, top + 3.0)], stroke);
        }
    }
}

pub struct VectorIcons;

impl VectorIcons {
    pub fn draw_radar(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Home, color, 1.5);
    }
    pub fn draw_lightning(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Bolt, color, 1.5);
    }
    pub fn draw_broom(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Broom, color, 1.5);
    }
    pub fn draw_shield(painter: &Painter, rect: Rect, color: Color32, with_check: bool) {
        if with_check {
            draw_icon(painter, rect, AppIcon::ShieldCheck, color, 1.5);
        } else {
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
        }
    }
    pub fn draw_tools(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Tools, color, 1.5);
    }
    pub fn draw_monitor(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Pulse, color, 1.5);
    }
    pub fn draw_gear(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Gear, color, 1.5);
    }
    pub fn draw_trash(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Trash, color, 1.5);
    }
    pub fn draw_eye(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Eye, color, 1.5);
    }
    pub fn draw_ram(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::RamChip, color, 1.5);
    }
    pub fn draw_briefcase(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Briefcase, color, 1.5);
    }
    pub fn draw_gamepad(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Gamepad, color, 1.5);
    }
    pub fn draw_battery(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::BatteryBolt, color, 1.5);
    }
    pub fn draw_shredder(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::Shredder, color, 1.5);
    }
    pub fn draw_duplicate(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::SearchDoc, color, 1.5);
    }
    pub fn draw_user(painter: &Painter, rect: Rect, color: Color32) {
        draw_icon(painter, rect, AppIcon::User, color, 1.5);
    }
}
