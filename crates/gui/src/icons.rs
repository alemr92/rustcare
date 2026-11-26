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
            let s = w.min(h);
            let top = c.y - s * 0.24;
            let bot = c.y + s * 0.38;
            let left = c.x - s * 0.28;
            let right = c.x + s * 0.22;

            // Halo volumétrico difuso de fondo
            painter.circle_filled(c, (s * 0.48).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 26));

            // Micro-fragmentos luminosos asomando en la boca del cubo (firma 2027_care.png)
            let shard1 = Rect::from_min_max(Pos2::new(left + s * 0.06, top - s * 0.12), Pos2::new(left + s * 0.20, top - s * 0.02));
            let shard2 = Rect::from_min_max(Pos2::new(c.x - s * 0.02, top - s * 0.16), Pos2::new(c.x + s * 0.12, top - s * 0.03));
            painter.rect_filled(shard1, eframe::egui::CornerRadius::same(1), Color32::from_rgba_unmultiplied(56, 189, 248, 170));
            painter.rect_filled(shard2, eframe::egui::CornerRadius::same(1), Color32::from_rgba_unmultiplied(0, 229, 168, 190));

            // Tapa inclinada holográfica
            painter.line_segment([Pos2::new(left - s * 0.06, top - s * 0.03), Pos2::new(right + s * 0.06, top - s * 0.09)], stroke);
            painter.line_segment([Pos2::new(c.x - s * 0.09, top - s * 0.11), Pos2::new(c.x + s * 0.05, top - s * 0.14)], Stroke::new(1.3_f32, color));

            // Cuerpo trapezoidal con relleno vítreo translúcido
            let p_body = vec![
                Pos2::new(left, top),
                Pos2::new(right, top),
                Pos2::new(right - s * 0.05, bot),
                Pos2::new(left + s * 0.05, bot),
            ];
            painter.add(eframe::egui::Shape::convex_polygon(
                p_body.clone(),
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 38),
                Stroke::NONE,
            ));
            // Bisel superior luminoso del cubo
            painter.line_segment([p_body[0], p_body[1]], Stroke::new(1.4_f32, Color32::from_rgba_unmultiplied(255, 255, 255, 160)));
            painter.line_segment([p_body[0], p_body[3]], stroke);
            painter.line_segment([p_body[1], p_body[2]], stroke);
            painter.line_segment([p_body[2], p_body[3]], stroke);

            // 3 costillas verticales luminosas
            for rx in [-0.11_f32, -0.02, 0.07] {
                let x_top = c.x + s * rx;
                let x_bot = c.x + s * rx * 0.85;
                painter.line_segment(
                    [Pos2::new(x_top, top + s * 0.08), Pos2::new(x_bot, bot - s * 0.07)],
                    Stroke::new(1.2_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 200)),
                );
            }

            // Engranaje técnico luminoso en esquina inferior derecha (Firma 2027)
            let g_c = Pos2::new(right + s * 0.04, bot - s * 0.06);
            let g_r = (s * 0.13).max(3.5);
            painter.circle_filled(g_c, g_r, Color32::from_rgb(10, 18, 28));
            painter.circle_stroke(g_c, g_r, Stroke::new(1.3_f32, Color32::from_rgb(56, 189, 248)));
            painter.circle_filled(g_c, (g_r * 0.38).max(1.5), Color32::from_rgb(0, 229, 168));
            for i in 0..6 {
                let a = (i as f32) * (TAU / 6.0);
                let p1 = Pos2::new(g_c.x + (g_r - 0.5) * a.cos(), g_c.y + (g_r - 0.5) * a.sin());
                let p2 = Pos2::new(g_c.x + (g_r + 2.4) * a.cos(), g_c.y + (g_r + 2.4) * a.sin());
                painter.line_segment([p1, p2], Stroke::new(1.3_f32, Color32::from_rgb(56, 189, 248)));
            }
        }

        AppIcon::Eye => {
            // Escudo volumétrico 3D con ojo holográfico (Escudo Privacidad 2027)
            let s = w.min(h);
            let top = c.y - s * 0.38;
            let bot = c.y + s * 0.40;
            let left = c.x - s * 0.34;
            let right = c.x + s * 0.34;

            // Halo difuso
            painter.circle_filled(c, (s * 0.48).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 26));

            // Silueta del Escudo exterior con relleno vítreo
            let s_pts = vec![
                Pos2::new(left, top + s * 0.05),
                Pos2::new(c.x, top - s * 0.02),
                Pos2::new(right, top + s * 0.05),
                Pos2::new(right - s * 0.03, c.y + s * 0.08),
                Pos2::new(c.x, bot),
                Pos2::new(left + s * 0.03, c.y + s * 0.08),
            ];
            painter.add(eframe::egui::Shape::convex_polygon(
                s_pts.clone(),
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 38),
                stroke,
            ));

            // Bisel interior luminoso
            let inner_pts = vec![
                Pos2::new(left + s * 0.07, top + s * 0.10),
                Pos2::new(c.x, top + s * 0.04),
                Pos2::new(right - s * 0.07, top + s * 0.10),
                Pos2::new(right - s * 0.09, c.y + s * 0.06),
                Pos2::new(c.x, bot - s * 0.08),
                Pos2::new(left + s * 0.09, c.y + s * 0.06),
            ];
            for i in 0..inner_pts.len() {
                painter.line_segment(
                    [inner_pts[i], inner_pts[(i + 1) % inner_pts.len()]],
                    Stroke::new(0.9_f32, Color32::from_rgba_unmultiplied(255, 255, 255, 75)),
                );
            }

            // Ojo almendrado holográfico interior
            let eye_c = Pos2::new(c.x, c.y - s * 0.02);
            let ew = s * 0.21;
            let eh = s * 0.13;
            let eye_pts = vec![
                Pos2::new(eye_c.x - ew, eye_c.y),
                Pos2::new(eye_c.x - ew * 0.45, eye_c.y - eh * 0.85),
                Pos2::new(eye_c.x + ew * 0.45, eye_c.y - eh * 0.85),
                Pos2::new(eye_c.x + ew, eye_c.y),
                Pos2::new(eye_c.x + ew * 0.45, eye_c.y + eh * 0.85),
                Pos2::new(eye_c.x - ew * 0.45, eye_c.y + eh * 0.85),
            ];
            painter.add(eframe::egui::Shape::convex_polygon(
                eye_pts,
                Color32::from_rgba_unmultiplied(10, 20, 34, 210),
                Stroke::new(1.4_f32, Color32::from_rgb(180, 240, 255)),
            ));
            painter.circle_stroke(eye_c, (s * 0.095).max(2.5), Stroke::new(1.4_f32, color));
            painter.circle_filled(eye_c, (s * 0.045).max(1.5), Color32::from_rgb(0, 229, 168));
            painter.circle_filled(Pos2::new(eye_c.x - s * 0.025, eye_c.y - s * 0.025), 1.2, Color32::WHITE);
        }

        AppIcon::RamChip => {
            // Chip RAM isométrico 3D (estilo 2027_care.png) con pines en las 4 aristas y núcleo brillante
            let s = w.min(h);
            painter.circle_filled(c, (s * 0.46).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 26));

            let rx = s * 0.30;
            let ry = s * 0.21;
            let depth = s * 0.075;

            // Cara superior isométrica (rombo)
            let top_n = Pos2::new(c.x, c.y - ry - depth * 0.35);
            let top_e = Pos2::new(c.x + rx, c.y - depth * 0.35);
            let top_s = Pos2::new(c.x, c.y + ry - depth * 0.35);
            let top_w = Pos2::new(c.x - rx, c.y - depth * 0.35);

            // Pines isométricos saliendo de los 4 lados
            let pin_col = Color32::from_rgb(56, 189, 248);
            let pin_len = s * 0.085;
            for i in 1..=4 {
                let t = i as f32 / 5.0;
                // Borde NW -> salida (-pin_len, -pin_len*0.65)
                let p_nw = top_w.lerp(top_n, t);
                painter.line_segment([p_nw, Pos2::new(p_nw.x - pin_len, p_nw.y - pin_len * 0.65)], Stroke::new(1.3_f32, pin_col));
                // Borde NE -> salida (+pin_len, -pin_len*0.65)
                let p_ne = top_n.lerp(top_e, t);
                painter.line_segment([p_ne, Pos2::new(p_ne.x + pin_len, p_ne.y - pin_len * 0.65)], Stroke::new(1.3_f32, pin_col));
                // Borde SW -> salida (-pin_len, +pin_len*0.65)
                let p_sw = top_w.lerp(top_s, t);
                painter.line_segment([p_sw, Pos2::new(p_sw.x - pin_len, p_sw.y + pin_len * 0.65)], Stroke::new(1.3_f32, pin_col));
                // Borde SE -> salida (+pin_len, +pin_len*0.65)
                let p_se = top_s.lerp(top_e, t);
                painter.line_segment([p_se, Pos2::new(p_se.x + pin_len, p_se.y + pin_len * 0.65)], Stroke::new(1.3_f32, pin_col));
            }

            // Base 3D extruida (caras inferiores)
            let bot_w = Pos2::new(top_w.x, top_w.y + depth);
            let bot_s = Pos2::new(top_s.x, top_s.y + depth);
            let bot_e = Pos2::new(top_e.x, top_e.y + depth);
            painter.add(eframe::egui::Shape::convex_polygon(
                vec![top_w, top_s, bot_s, bot_w],
                Color32::from_rgba_unmultiplied(15, 35, 52, 230),
                Stroke::new(1.1_f32, color),
            ));
            painter.add(eframe::egui::Shape::convex_polygon(
                vec![top_s, top_e, bot_e, bot_s],
                Color32::from_rgba_unmultiplied(10, 24, 38, 240),
                Stroke::new(1.1_f32, color),
            ));

            // Placa superior obsidiana con borde luminoso
            painter.add(eframe::egui::Shape::convex_polygon(
                vec![top_n, top_e, top_s, top_w],
                Color32::from_rgba_unmultiplied(14, 24, 38, 240),
                stroke,
            ));

            // Circuito interior isométrico brillante
            let ic_c = Pos2::new(c.x, c.y - depth * 0.35);
            let irx = rx * 0.50;
            let iry = ry * 0.50;
            let inner = vec![
                Pos2::new(ic_c.x, ic_c.y - iry),
                Pos2::new(ic_c.x + irx, ic_c.y),
                Pos2::new(ic_c.x, ic_c.y + iry),
                Pos2::new(ic_c.x - irx, ic_c.y),
            ];
            painter.add(eframe::egui::Shape::convex_polygon(
                inner,
                Color32::from_rgba_unmultiplied(0, 229, 168, 45),
                Stroke::new(1.1_f32, Color32::from_rgb(0, 229, 168)),
            ));
            painter.circle_filled(ic_c, (s * 0.055).max(1.8), Color32::from_rgb(0, 229, 168));
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
            let s = w.min(h);
            let left = c.x - s * 0.34;
            let right = c.x + s * 0.34;
            let mid_y = c.y - s * 0.04;

            painter.circle_filled(c, (s * 0.45).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 24));

            // Hoja entrando por arriba con relleno translúcido
            let doc_r = Rect::from_min_max(
                Pos2::new(c.x - s * 0.20, c.y - s * 0.38),
                Pos2::new(c.x + s * 0.20, mid_y - s * 0.08),
            );
            painter.rect_filled(doc_r, eframe::egui::CornerRadius::same(2), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 45));
            painter.rect_stroke(doc_r, eframe::egui::CornerRadius::same(2), stroke, eframe::egui::StrokeKind::Inside);

            // Cabezal del destructor con ranura luminosa
            let head_r = Rect::from_min_max(
                Pos2::new(left, mid_y - s * 0.10),
                Pos2::new(right, mid_y + s * 0.10),
            );
            painter.rect_filled(head_r, eframe::egui::CornerRadius::same(3), Color32::from_rgba_unmultiplied(14, 24, 38, 235));
            painter.rect_stroke(head_r, eframe::egui::CornerRadius::same(3), stroke, eframe::egui::StrokeKind::Inside);
            painter.line_segment(
                [Pos2::new(left + s * 0.06, mid_y), Pos2::new(right - s * 0.06, mid_y)],
                Stroke::new(1.3_f32, Color32::from_rgb(0, 229, 168)),
            );

            // 5 tiras de papel triturado saliendo abajo + fragmentos cayendo
            let strip_w = right - left - s * 0.14;
            for i in 0..5 {
                let x = left + s * 0.07 + (i as f32) * (strip_w / 4.0);
                let len = s * (0.24 + ((i % 2) as f32) * 0.08);
                painter.line_segment(
                    [Pos2::new(x, mid_y + s * 0.10), Pos2::new(x + ((i as f32 - 2.0) * 0.8), mid_y + s * 0.10 + len)],
                    Stroke::new(1.4_f32, color),
                );
            }
            // Confeti de partículas trituradas
            painter.rect_filled(Rect::from_center_size(Pos2::new(c.x - s * 0.14, c.y + s * 0.36), eframe::egui::vec2(2.2, 2.2)), eframe::egui::CornerRadius::ZERO, Color32::from_rgb(0, 229, 168));
            painter.rect_filled(Rect::from_center_size(Pos2::new(c.x + s * 0.12, c.y + s * 0.39), eframe::egui::vec2(2.2, 2.2)), eframe::egui::CornerRadius::ZERO, color);
        }

        AppIcon::Gauge => {
            let s = w.min(h);
            let r = (s * 0.38).max(5.0);
            painter.circle_filled(c, (s * 0.44).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 24));
            let start = 0.75 * TAU; // Arriba
            let span = 0.67 * TAU;  // 240 deg
            let arc_start = start - span / 2.0;
            let steps = 18;
            let mut prev = Pos2::new(c.x + r * arc_start.cos(), c.y + r * arc_start.sin());

            for i in 1..=steps {
                let frac = i as f32 / steps as f32;
                let a = arc_start + frac * span;
                let next = Pos2::new(c.x + r * a.cos(), c.y + r * a.sin());
                painter.line_segment([prev, next], Stroke::new(stroke_width + 0.3, color));
                prev = next;
            }

            // Marcas radiales de escala
            for i in 0..=4 {
                let frac = i as f32 / 4.0;
                let a = arc_start + frac * span;
                let p1 = Pos2::new(c.x + (r - s * 0.08) * a.cos(), c.y + (r - s * 0.08) * a.sin());
                let p2 = Pos2::new(c.x + (r - 1.0) * a.cos(), c.y + (r - 1.0) * a.sin());
                painter.line_segment([p1, p2], Stroke::new(1.1_f32, Color32::from_rgba_unmultiplied(255, 255, 255, 160)));
            }

            // Aguja apuntando a alto rendimiento
            let needle_angle: f32 = -0.55_f32;
            let needle_end = Pos2::new(c.x + (r - 2.5) * needle_angle.cos(), c.y + (r - 2.5) * needle_angle.sin());
            painter.line_segment([c, needle_end], Stroke::new(1.8_f32, Color32::from_rgb(0, 229, 168)));
            painter.circle_filled(c, (s * 0.07).max(2.0), Color32::WHITE);
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
            // Cerebro IA 2027 esculpido con 2 hemisferios vítreos, pliegues y pulso ECG
            let s = w.min(h);
            painter.circle_filled(c, (s * 0.46).max(6.0), Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 26));

            let cy = c.y - s * 0.02;
            let rx = s * 0.36;
            let ry = s * 0.32;

            // Hemisferio izquierdo (polígono convexo translúcido + contorno)
            let left_hemi = vec![
                Pos2::new(c.x - 1.5, cy - ry * 0.92),
                Pos2::new(c.x - rx * 0.58, cy - ry),
                Pos2::new(c.x - rx * 0.96, cy - ry * 0.45),
                Pos2::new(c.x - rx, cy + ry * 0.18),
                Pos2::new(c.x - rx * 0.68, cy + ry * 0.78),
                Pos2::new(c.x - 1.5, cy + ry * 0.85),
            ];
            let right_hemi = vec![
                Pos2::new(c.x + 1.5, cy - ry * 0.92),
                Pos2::new(c.x + rx * 0.58, cy - ry),
                Pos2::new(c.x + rx * 0.96, cy - ry * 0.45),
                Pos2::new(c.x + rx, cy + ry * 0.18),
                Pos2::new(c.x + rx * 0.68, cy + ry * 0.78),
                Pos2::new(c.x + 1.5, cy + ry * 0.85),
            ];
            let fill_col = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 40);
            painter.add(eframe::egui::Shape::convex_polygon(left_hemi, fill_col, stroke));
            painter.add(eframe::egui::Shape::convex_polygon(right_hemi, fill_col, stroke));

            // Pliegues cerebrales internos (gyri)
            let gyri_col = Stroke::new(1.1_f32, Color32::from_rgba_unmultiplied(180, 248, 255, 160));
            painter.line_segment([Pos2::new(c.x - rx * 0.65, cy - ry * 0.42), Pos2::new(c.x - rx * 0.25, cy - ry * 0.52)], gyri_col);
            painter.line_segment([Pos2::new(c.x + rx * 0.65, cy - ry * 0.42), Pos2::new(c.x + rx * 0.25, cy - ry * 0.52)], gyri_col);
            painter.line_segment([Pos2::new(c.x - rx * 0.62, cy + ry * 0.35), Pos2::new(c.x - rx * 0.24, cy + ry * 0.44)], gyri_col);
            painter.line_segment([Pos2::new(c.x + rx * 0.62, cy + ry * 0.35), Pos2::new(c.x + rx * 0.24, cy + ry * 0.44)], gyri_col);

            // Onda ECG luminosa cruzando el centro del cerebro
            let ecg = [
                Pos2::new(c.x - rx * 0.85, cy),
                Pos2::new(c.x - rx * 0.36, cy),
                Pos2::new(c.x - rx * 0.18, cy - ry * 0.36),
                Pos2::new(c.x, cy + ry * 0.34),
                Pos2::new(c.x + rx * 0.20, cy - ry * 0.24),
                Pos2::new(c.x + rx * 0.36, cy),
                Pos2::new(c.x + rx * 0.85, cy),
            ];
            for win in ecg.windows(2) {
                painter.line_segment([win[0], win[1]], Stroke::new(1.5_f32, Color32::from_rgb(0, 229, 168)));
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
