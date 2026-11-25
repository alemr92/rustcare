//! Controles interactivos y widgets personalizados para RustCare
//! Prohibido el uso directo de ui.button(), ui.checkbox() o ui.radio_value() en las vistas.
use crate::icons::{draw_icon, AppIcon};
use crate::theme::RustCareTheme;
use eframe::egui::{
    self, epaint::Shape, Align2, Color32, CornerRadius, FontId, Id, Pos2, Rect, Response, Sense,
    Stroke, StrokeKind, Ui, Vec2,
};
use std::f32::consts::TAU;

/// 1. Turbina Holográfica Multicapa 2027 (Núcleo Central de Cuidar)
pub fn care_radar_button(
    ui: &mut Ui,
    title: &str,
    subtitle: &str,
    progress: f32,
    is_scanning: bool,
    health_score: Option<u8>,
) -> Response {
    let size = Vec2::splat(260.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let painter = ui.painter();
    let center = rect.center();
    let time = ui.input(|i| i.time) as f32;
    let is_hover = response.hovered();

    let speed_mult = if is_scanning { 3.2 } else if is_hover { 1.6 } else { 0.45 };
    let cw_angle = (time * speed_mult) % TAU;
    let ccw_angle = (-time * speed_mult * 0.75) % TAU;

    // 0. Halos Volumétricos Difusos
    let pulse = (time * 2.2).sin() * 4.0;
    painter.circle_filled(
        center,
        124.0 + pulse,
        Color32::from_rgba_unmultiplied(0, 229, 168, if is_hover { 18 } else { 10 }),
    );
    painter.circle_filled(
        center,
        106.0 + pulse * 0.5,
        Color32::from_rgba_unmultiplied(37, 99, 235, 18),
    );
    painter.circle_filled(
        center,
        92.0,
        Color32::from_rgba_unmultiplied(0, 229, 168, 24),
    );

    // 1. Capa 1: Dial exterior técnico de 48 marcas (r = 120.0)
    let dial_r = 120.0;
    for i in 0..48 {
        let angle = (i as f32) * (TAU / 48.0);
        let is_major = i % 4 == 0;
        let len = if is_major { 5.5 } else { 2.5 };
        let col = if is_major {
            Color32::from_rgba_unmultiplied(0, 229, 168, 140)
        } else {
            Color32::from_rgba_unmultiplied(34, 46, 66, 180)
        };
        let p1 = Pos2::new(center.x + (dial_r - len) * angle.cos(), center.y + (dial_r - len) * angle.sin());
        let p2 = Pos2::new(center.x + dial_r * angle.cos(), center.y + dial_r * angle.sin());
        painter.line_segment([p1, p2], Stroke::new(if is_major { 1.5 } else { 1.0 }, col));
    }

    // 2. Capa 2: Vórtice Galáctico de Partículas Neón y Espirales Holográficas (r = 96.0 .. 132.0)
    for arm in 0..3 {
        let arm_offset = (arm as f32) * (TAU / 3.0);
        for i in 0..36 {
            let frac = i as f32 / 36.0;
            let spiral_r = 96.0 + frac * 34.0;
            let angle = cw_angle + arm_offset + frac * (TAU * 0.75);
            let alpha = ((frac * std::f32::consts::PI).sin() * 220.0).clamp(15.0, 240.0) as u8;
            let dot_pos = Pos2::new(center.x + spiral_r * angle.cos(), center.y + spiral_r * angle.sin());
            let col = if i % 3 == 0 {
                Color32::from_rgba_unmultiplied(56, 189, 248, alpha)
            } else if i % 3 == 1 {
                Color32::from_rgba_unmultiplied(0, 229, 168, alpha)
            } else {
                Color32::from_rgba_unmultiplied(255, 255, 255, alpha.saturating_sub(40))
            };
            let sz = if i % 4 == 0 { 1.8 } else if i % 2 == 0 { 1.2 } else { 0.8 };
            painter.circle_filled(dot_pos, sz, col);
        }
    }

    // 3. Capa 3: Corona Mecánica / Dientes de Engranaje Holográfico Antihorario (r = 90.0)
    let gear_r = 90.0;
    let teeth = 24;
    for i in 0..teeth {
        let a = ccw_angle + (i as f32) * (TAU / teeth as f32);
        let p_in = Pos2::new(center.x + (gear_r - 5.0) * a.cos(), center.y + (gear_r - 5.0) * a.sin());
        let p_out = Pos2::new(center.x + (gear_r + 4.0) * a.cos(), center.y + (gear_r + 4.0) * a.sin());
        painter.line_segment([p_in, p_out], Stroke::new(3.2, Color32::from_rgba_unmultiplied(56, 189, 248, 160)));
    }
    painter.circle_stroke(center, gear_r + 4.0, Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 229, 168, 180)));
    painter.circle_stroke(center, gear_r - 5.0, Stroke::new(1.4, Color32::from_rgba_unmultiplied(37, 99, 235, 200)));

    // 4. Capa 4: Arco de flujo de energía exterior y progreso activo (r = 82.0)
    let flow_r = 82.0;
    painter.circle_stroke(center, flow_r, Stroke::new(4.0, Color32::from_rgba_unmultiplied(26, 35, 52, 160)));
    if progress > 0.001 {
        let active_span = (progress.clamp(0.0, 1.0) * TAU).min(TAU);
        let start_a = -0.25 * TAU;
        let steps = (active_span * 28.0).max(4.0) as usize;
        let mut prev = Pos2::new(center.x + flow_r * start_a.cos(), center.y + flow_r * start_a.sin());
        for s in 1..=steps {
            let frac = s as f32 / steps as f32;
            let a = start_a + frac * active_span;
            let next = Pos2::new(center.x + flow_r * a.cos(), center.y + flow_r * a.sin());
            painter.line_segment([prev, next], Stroke::new(4.5, Color32::from_rgb(0, 229, 168)));
            prev = next;
        }
    } else {
        // 3 arcos de flujo en movimiento
        for i in 0..3 {
            let a_start = cw_angle + (i as f32) * (TAU / 3.0);
            let arc_len = 0.6;
            let mut prev = Pos2::new(center.x + flow_r * a_start.cos(), center.y + flow_r * a_start.sin());
            for s in 1..=8 {
                let frac = s as f32 / 8.0;
                let a = a_start + frac * arc_len;
                let next = Pos2::new(center.x + flow_r * a.cos(), center.y + flow_r * a.sin());
                let alpha = (frac * 240.0) as u8;
                painter.line_segment([prev, next], Stroke::new(3.5, Color32::from_rgba_unmultiplied(0, 229, 168, alpha)));
                prev = next;
            }
        }
    }

    // 5. Núcleo central de cristal espacial con respiración senoidal
    let core_pulse = (time * 1.8).sin() * 1.5;
    let core_r = 74.0 + core_pulse;
    let core_color = if let Some(score) = health_score {
        if score < 50 {
            Color32::from_rgb(244, 63, 94)
        } else if score < 80 {
            Color32::from_rgb(245, 158, 11)
        } else {
            Color32::from_rgb(0, 229, 168)
        }
    } else if is_scanning {
        Color32::from_rgb(56, 189, 248)
    } else {
        Color32::from_rgb(0, 229, 168)
    };

    // Relleno profundo de cristal obsidiana
    painter.circle_filled(center, core_r, Color32::from_rgb(10, 15, 24));
    painter.circle_stroke(center, core_r, Stroke::new(2.0, core_color));
    painter.circle_stroke(center, core_r - 4.0, Stroke::new(1.0, Color32::from_rgba_unmultiplied(37, 99, 235, 140)));

    // 6. Texto o Marcador de Salud
    if let Some(score) = health_score {
        let score_color = if score < 50 {
            RustCareTheme::ACCENT_CORAL
        } else if score < 80 {
            RustCareTheme::ACCENT_AMBER
        } else {
            Color32::from_rgb(0, 229, 168)
        };
        painter.text(
            Pos2::new(center.x, center.y - 18.0),
            Align2::CENTER_CENTER,
            format!("{}", score),
            FontId::proportional(34.0),
            score_color,
        );
        painter.text(
            Pos2::new(center.x, center.y + 4.0),
            Align2::CENTER_CENTER,
            "/ 100",
            FontId::proportional(13.0),
            RustCareTheme::TEXT_MUTED,
        );
        let pill_rect = Rect::from_center_size(Pos2::new(center.x, center.y + 26.0), Vec2::new(104.0, 24.0));
        let pill_text = if score < 80 { "REPARAR TODO" } else { "ÓPTIMO" };
        painter.rect_filled(pill_rect, CornerRadius::same(12), Color32::from_rgba_unmultiplied(0, 229, 168, 35));
        painter.rect_stroke(pill_rect, CornerRadius::same(12), Stroke::new(1.2, Color32::from_rgb(0, 229, 168)), StrokeKind::Inside);
        painter.text(pill_rect.center(), Align2::CENTER_CENTER, pill_text, FontId::proportional(11.5), Color32::WHITE);
    } else {
        let text_y = if subtitle.is_empty() { center.y } else { center.y - 7.0 };
        painter.text(
            Pos2::new(center.x, text_y),
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(22.0),
            Color32::WHITE,
        );
        if !subtitle.is_empty() {
            painter.text(
                Pos2::new(center.x, center.y + 14.0),
                Align2::CENTER_CENTER,
                subtitle,
                FontId::proportional(11.0),
                Color32::from_rgb(0, 229, 168),
            );
        }
    }

    if is_scanning || is_hover {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
    }

    response
}

/// 2. Interruptor Gigante de Turbo Boost (136x56px)
pub fn big_turbo_switch(ui: &mut Ui, enabled: &mut bool) -> Response {
    let size = Vec2::new(136.0, 56.0);
    let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());

    if response.clicked() {
        *enabled = !*enabled;
        response.mark_changed();
    }

    let is_on = *enabled;
    let painter = ui.painter();

    // Fondo y borde pista con iluminación suave
    let track_bg = if is_on {
        Color32::from_rgba_unmultiplied(14, 165, 233, 35) // Frosted azure
    } else {
        RustCareTheme::BG_ELEVATED
    };
    let track_stroke = if is_on {
        RustCareTheme::ACCENT_TEAL
    } else {
        RustCareTheme::BORDER_SUBTLE
    };

    painter.rect_filled(rect, CornerRadius::same(28), track_bg);
    painter.rect_stroke(rect, CornerRadius::same(28), Stroke::new(1.5, track_stroke), StrokeKind::Inside);

    // Círculo deslizante animado (r = 21.0)
    let knob_x = if is_on {
        rect.max.x - 28.0
    } else {
        rect.min.x + 28.0
    };
    let knob_center = Pos2::new(knob_x, rect.center().y);

    if is_on {
        // Halo exterior translúcido
        painter.circle_filled(
            knob_center,
            25.0,
            Color32::from_rgba_unmultiplied(14, 165, 233, 40),
        );
        painter.circle_filled(knob_center, 21.0, RustCareTheme::ACCENT_TEAL);
        painter.circle_filled(knob_center, 12.0, Color32::WHITE);

        // Texto ON a la izquierda
        let text_pos = Pos2::new(rect.min.x + 40.0, rect.center().y);
        painter.text(text_pos, Align2::CENTER_CENTER, "ON", FontId::proportional(20.0), RustCareTheme::TEXT_PRIMARY);
    } else {
        painter.circle_filled(knob_center, 21.0, Color32::from_rgb(45, 58, 77));
        painter.circle_stroke(knob_center, 21.0, Stroke::new(1.0, RustCareTheme::BORDER_SUBTLE));

        // Texto OFF a la derecha
        let text_pos = Pos2::new(rect.max.x - 40.0, rect.center().y);
        painter.text(text_pos, Align2::CENTER_CENTER, "OFF", FontId::proportional(20.0), RustCareTheme::TEXT_MUTED);
    }

    response
}

/// 3. Toggle de Píldora Compacto (42x22px)
pub fn pill_toggle(ui: &mut Ui, enabled: &mut bool) -> Response {
    let size = Vec2::new(42.0, 22.0);
    let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());

    if response.clicked() {
        *enabled = !*enabled;
        response.mark_changed();
    }

    let is_on = *enabled;
    let painter = ui.painter();

    let bg = if is_on {
        Color32::from_rgba_unmultiplied(14, 165, 233, 35)
    } else {
        RustCareTheme::BG_VOID
    };
    let stroke_col = if is_on {
        RustCareTheme::ACCENT_TEAL
    } else {
        RustCareTheme::BORDER_SUBTLE
    };

    painter.rect_filled(rect, CornerRadius::same(11), bg);
    painter.rect_stroke(rect, CornerRadius::same(11), Stroke::new(1.0, stroke_col), StrokeKind::Inside);

    let k_x = if is_on {
        rect.max.x - 11.0
    } else {
        rect.min.x + 11.0
    };
    let knob_col = if is_on {
        RustCareTheme::ACCENT_CYAN
    } else {
        RustCareTheme::TEXT_DIM
    };
    painter.circle_filled(Pos2::new(k_x, rect.center().y), 7.5, knob_col);

    response
}

/// 4. Botón Neón Principal (Altura 34px) - Elegante Frosted Azure
pub fn neon_button(ui: &mut Ui, text: &str, width: Option<f32>) -> Response {
    let font_id = FontId::proportional(13.0);
    let text_col = RustCareTheme::TEXT_PRIMARY;
    let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, text_col);
    let w = width.unwrap_or(galley.size().x + 24.0);
    let size = Vec2::new(w, 34.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    let (fill, stroke_col) = if response.is_pointer_button_down_on() {
        (Color32::from_rgba_unmultiplied(14, 165, 233, 80), RustCareTheme::ACCENT_CYAN)
    } else if response.hovered() {
        (Color32::from_rgba_unmultiplied(14, 165, 233, 50), RustCareTheme::ACCENT_CYAN)
    } else {
        (Color32::from_rgba_unmultiplied(14, 165, 233, 30), RustCareTheme::ACCENT_TEAL)
    };

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(8), fill);
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0, stroke_col), StrokeKind::Inside);
    let text_pos = Pos2::new(rect.center().x - galley.size().x / 2.0, rect.center().y - galley.size().y / 2.0);
    painter.galley(text_pos, galley, text_col);

    response
}

/// 5. Botón Ghost Secundario (Altura 32px) - Modern Slate Card Button
pub fn ghost_button(ui: &mut Ui, text: &str, width: Option<f32>) -> Response {
    let font_id = FontId::proportional(12.5);
    let text_col = RustCareTheme::TEXT_PRIMARY;
    let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, text_col);
    let w = width.unwrap_or(galley.size().x + 22.0);
    let size = Vec2::new(w, 32.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    let (fill, stroke_col) = if response.is_pointer_button_down_on() {
        (Color32::from_rgb(20, 28, 42), RustCareTheme::ACCENT_TEAL)
    } else if response.hovered() {
        (RustCareTheme::BG_HOVER, RustCareTheme::ACCENT_CYAN)
    } else {
        (RustCareTheme::BG_ELEVATED, RustCareTheme::BORDER_SUBTLE)
    };

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(8), fill);
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0, stroke_col), StrokeKind::Inside);
    let text_pos = Pos2::new(rect.center().x - galley.size().x / 2.0, rect.center().y - galley.size().y / 2.0);
    painter.galley(text_pos, galley, text_col);

    response
}

/// 6. Botón Peligro / Danger (Altura 28px) - Soft Coral
pub fn danger_button(ui: &mut Ui, text: &str, width: Option<f32>) -> Response {
    let font_id = FontId::proportional(12.0);
    let text_col = Color32::from_rgb(251, 113, 133);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, text_col);
    let w = width.unwrap_or(galley.size().x + 20.0);
    let size = Vec2::new(w, 28.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    let (fill, stroke_col) = if response.is_pointer_button_down_on() {
        (Color32::from_rgba_unmultiplied(244, 63, 94, 70), RustCareTheme::ACCENT_CORAL)
    } else if response.hovered() {
        (Color32::from_rgba_unmultiplied(244, 63, 94, 45), RustCareTheme::ACCENT_CORAL)
    } else {
        (Color32::from_rgba_unmultiplied(244, 63, 94, 25), Color32::from_rgba_unmultiplied(244, 63, 94, 120))
    };

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(6), fill);
    painter.rect_stroke(rect, CornerRadius::same(6), Stroke::new(1.0, stroke_col), StrokeKind::Inside);
    let text_pos = Pos2::new(rect.center().x - galley.size().x / 2.0, rect.center().y - galley.size().y / 2.0);
    painter.galley(text_pos, galley, text_col);

    response
}

/// 7. Tarjeta con Sensor Líquido y Ondas Dinámicas (Bento Sensor Card 2027)
pub fn sparkline_card(
    ui: &mut Ui,
    title: &str,
    hw_subtitle: &str,
    big_value: &str,
    peak_text: &str,
    current_text: &str,
    history: &[f32],
    accent: Color32,
    width: f32,
    height: f32,
) {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let painter = ui.painter();
    let time = ui.input(|i| i.time) as f32;

    // Fondo Bento Glass con profundidad y proyector de luz inferior neón
    RustCareTheme::draw_glass_card(painter, rect, response.hovered(), Some(accent));

    // Fila 1: Título a la izquierda | Valor Grande a la derecha (estilo 2027_tools.png, deja 52px para la onda)
    painter.text(
        Pos2::new(rect.min.x + 14.0, rect.min.y + 18.0),
        Align2::LEFT_CENTER,
        title,
        FontId::proportional(15.5),
        Color32::WHITE,
    );
    painter.text(
        Pos2::new(rect.max.x - 14.0, rect.min.y + 18.0),
        Align2::RIGHT_CENTER,
        big_value,
        FontId::proportional(20.0),
        Color32::WHITE,
    );

    // Subtítulo hardware sutil debajo del título
    painter.text(
        Pos2::new(rect.min.x + 14.0, rect.min.y + 34.0),
        Align2::LEFT_CENTER,
        hw_subtitle,
        FontId::proportional(10.0),
        RustCareTheme::TEXT_MUTED,
    );

    // Fila 3: Área de Ondas Líquidas Generosa (50px de altura real)
    let wave_rect = Rect::from_min_max(
        Pos2::new(rect.min.x + 12.0, rect.min.y + 42.0),
        Pos2::new(rect.max.x - 12.0, rect.max.y - 22.0),
    );

    if wave_rect.width() > 20.0 {
        let steps = 48;
        let step_x = wave_rect.width() / (steps as f32);
        let mut points: Vec<Pos2> = Vec::with_capacity(steps + 1);

        // Muestreo interpolado desde `history` real
        let sample_history = |frac: f32| -> f32 {
            if history.is_empty() {
                return 20.0;
            }
            if history.len() == 1 {
                return history[0];
            }
            let pos = frac.clamp(0.0, 1.0) * (history.len() - 1) as f32;
            let i0 = pos.floor() as usize;
            let i1 = (i0 + 1).min(history.len() - 1);
            let t = pos - i0 as f32;
            // Interpolación coseno suave para evitar aristas bruscas
            let mu = (1.0 - (t * std::f32::consts::PI).cos()) * 0.5;
            history[i0] * (1.0 - mu) + history[i1] * mu
        };

        let mut crest_pos = Pos2::ZERO;
        let mut min_y = f32::MAX;
        let wave_h = (wave_rect.height() - 6.0).max(10.0);

        // Auto-escala relativa al rango del historial para curvas expresivas
        let min_hist = history.iter().copied().fold(f32::MAX, f32::min);
        let max_hist = history.iter().copied().fold(f32::MIN, f32::max);
        let lo = if history.is_empty() { 0.0 } else { (min_hist - 6.0).max(0.0) };
        let hi = if history.is_empty() { 100.0 } else { (max_hist + 6.0).max(lo + 14.0).min(100.0) };
        let span = (hi - lo).max(1.0);

        for i in 0..=steps {
            let frac = i as f32 / steps as f32;
            let x = wave_rect.min.x + (i as f32) * step_x;
            let val = sample_history(frac).clamp(0.0, 100.0);
            let norm = ((val - lo) / span).clamp(0.10, 0.92);
            // Respiración orgánica sutil (calmada, sin oscilaciones nerviosas)
            let breathe = (time * 0.25 + frac * 2.0).sin() * 0.6;
            let y = (wave_rect.max.y - 4.0 - norm * wave_h + breathe)
                .clamp(wave_rect.min.y + 3.0, wave_rect.max.y - 2.0);
            let pt = Pos2::new(x, y);
            if pt.y < min_y {
                min_y = pt.y;
                crest_pos = pt;
            }
            points.push(pt);
        }

        if title.contains("RAM") && !points.is_empty() {
            // Sonda vertical de rastreo en la cresta
            painter.line_segment(
                [Pos2::new(crest_pos.x, wave_rect.max.y), crest_pos],
                Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 180)),
            );
            painter.circle_filled(crest_pos, 2.5, Color32::WHITE);
            painter.circle_stroke(crest_pos, 4.5, Stroke::new(1.0_f32, Color32::from_rgb(56, 189, 248)));
        }

        // Relleno degradado bajo la onda mediante Mesh (soporta curvas cóncavas sin artefactos)
        if points.len() >= 2 {
            let mut mesh = eframe::egui::Mesh::default();
            let top_col = Color32::from_rgba_unmultiplied(
                accent.r(),
                accent.g(),
                accent.b(),
                if response.hovered() { 70 } else { 48 },
            );
            let bot_col = Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 6);
            for (idx, pt) in points.iter().enumerate() {
                mesh.colored_vertex(*pt, top_col);
                mesh.colored_vertex(Pos2::new(pt.x, wave_rect.max.y), bot_col);
                if idx > 0 {
                    let i0 = ((idx - 1) * 2) as u32;
                    mesh.add_triangle(i0, i0 + 1, i0 + 2);
                    mesh.add_triangle(i0 + 1, i0 + 3, i0 + 2);
                }
            }
            painter.add(Shape::mesh(mesh));
        }

        // Curva superior brillante de 2.2px
        for w in points.windows(2) {
            painter.line_segment([w[0], w[1]], Stroke::new(2.2_f32, accent));
        }

        // Tooltip interactivo al pasar el ratón (estilo Recharts)
        if let Some(hover_pos) = response.hover_pos() {
            if wave_rect.contains(hover_pos) && !points.is_empty() {
                let frac = ((hover_pos.x - wave_rect.min.x) / wave_rect.width()).clamp(0.0, 1.0);
                let pt_idx = ((frac * (points.len() - 1) as f32).round() as usize).min(points.len() - 1);
                let snap_pt = points[pt_idx];
                let val_sample = sample_history(frac);

                // Línea guía vertical (Crosshair)
                painter.line_segment(
                    [Pos2::new(snap_pt.x, wave_rect.min.y), Pos2::new(snap_pt.x, wave_rect.max.y)],
                    Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(248, 250, 252, 110)),
                );
                painter.circle_filled(snap_pt, 3.5, Color32::WHITE);
                painter.circle_stroke(snap_pt, 6.0, Stroke::new(1.2_f32, accent));

                // Cápsula tooltip flotante
                let tip_text = format!("{:.1}%", val_sample);
                let tip_w = 50.0_f32;
                let tip_h = 20.0_f32;
                let tip_x = (snap_pt.x - tip_w * 0.5).clamp(rect.min.x + 8.0, rect.max.x - tip_w - 8.0);
                let tip_y = (snap_pt.y - tip_h - 8.0).max(rect.min.y + 6.0);
                let tip_rect = Rect::from_min_size(Pos2::new(tip_x, tip_y), Vec2::new(tip_w, tip_h));
                painter.rect_filled(tip_rect, CornerRadius::same(6), Color32::from_rgba_unmultiplied(11, 15, 23, 235));
                painter.rect_stroke(tip_rect, CornerRadius::same(6), Stroke::new(1.0_f32, accent), StrokeKind::Inside);
                painter.text(tip_rect.center(), Align2::CENTER_CENTER, tip_text, FontId::monospace(10.5), Color32::WHITE);
            }
        }
    }

    // Fila 4: Etiquetas inferiores de pie
    painter.text(
        Pos2::new(rect.min.x + 14.0, rect.max.y - 13.0),
        Align2::LEFT_CENTER,
        peak_text,
        FontId::proportional(10.5),
        RustCareTheme::TEXT_MUTED,
    );
    painter.text(
        Pos2::new(rect.max.x - 14.0, rect.max.y - 13.0),
        Align2::RIGHT_CENTER,
        current_text,
        FontId::proportional(10.5),
        if title.contains("RAM") || title.contains("NVMe") { RustCareTheme::TEXT_MUTED } else { accent },
    );

    if response.hovered() {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(32));
    }
}

/// 8. Tarjeta de Herramienta para Toolbox
pub fn tool_card(
    ui: &mut Ui,
    title: &str,
    desc: &str,
    metric: &str,
    icon: AppIcon,
    accent: Color32,
    action_label: &str,
    width: f32,
    height: f32,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let painter = ui.painter();

    // Fondo tarjeta con profundidad de cristal y borde reactivo
    RustCareTheme::draw_glass_card(painter, rect, response.hovered(), Some(accent));

    // Icono 44x44 a la izquierda en placa elevada con iluminación suave
    let ic_box = Rect::from_min_size(Pos2::new(rect.min.x + 14.0, rect.min.y + 14.0), Vec2::splat(44.0));
    painter.rect_filled(ic_box, CornerRadius::same(10), Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 26));
    painter.rect_stroke(
        ic_box,
        CornerRadius::same(10),
        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 85)),
        StrokeKind::Inside,
    );
    // Luz superior en la placa del icono
    painter.line_segment(
        [Pos2::new(ic_box.min.x + 4.0, ic_box.min.y + 1.0), Pos2::new(ic_box.max.x - 4.0, ic_box.min.y + 1.0)],
        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(255, 255, 255, 35)),
    );
    draw_icon(painter, ic_box, icon, accent, 1.6);

    // Textos a la derecha del icono
    let text_x = rect.min.x + 68.0;
    painter.text(
        Pos2::new(text_x, rect.min.y + 22.0),
        Align2::LEFT_CENTER,
        title,
        FontId::proportional(14.0),
        RustCareTheme::TEXT_PRIMARY,
    );
    painter.text(
        Pos2::new(text_x, rect.min.y + 38.0),
        Align2::LEFT_CENTER,
        desc,
        FontId::proportional(11.5),
        RustCareTheme::TEXT_MUTED,
    );
    painter.text(
        Pos2::new(text_x, rect.min.y + 54.0),
        Align2::LEFT_CENTER,
        metric,
        FontId::proportional(12.0),
        accent,
    );

    // Botón centrado en la parte inferior
    let btn_w = (rect.width() - 28.0).min(140.0);
    let btn_rect = Rect::from_center_size(Pos2::new(rect.center().x, rect.max.y - 24.0), Vec2::new(btn_w, 28.0));
    let id = Id::new(title);
    let btn_resp = ui.interact(btn_rect, id, Sense::click());

    let (btn_fill, btn_stroke) = if btn_resp.is_pointer_button_down_on() {
        (Color32::from_rgb(20, 28, 42), accent)
    } else if btn_resp.hovered() {
        (RustCareTheme::BG_HOVER, accent)
    } else {
        (RustCareTheme::BG_ELEVATED, RustCareTheme::BORDER_SUBTLE)
    };

    painter.rect_filled(btn_rect, CornerRadius::same(6), btn_fill);
    painter.rect_stroke(btn_rect, CornerRadius::same(6), Stroke::new(1.0_f32, btn_stroke), StrokeKind::Inside);
    painter.text(
        btn_rect.center(),
        Align2::CENTER_CENTER,
        action_label,
        FontId::proportional(12.0),
        RustCareTheme::TEXT_PRIMARY,
    );

    btn_resp
}

#[derive(Clone, Debug)]
pub struct TreemapItem {
    pub label: String,
    pub size_text: String,
    pub weight: f32,
    pub color: Color32,
}

/// 9. Treemap Canvas interactivo
pub fn treemap_canvas(ui: &mut Ui, size: Vec2, items: &[TreemapItem]) {
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter();

    if items.is_empty() {
        painter.rect_filled(rect, CornerRadius::same(8), RustCareTheme::BG_SURFACE);
        return;
    }

    // Algoritmo slice-and-dice con tooltips
    fn layout_items(
        painter: &eframe::egui::Painter,
        ui: &Ui,
        rect: Rect,
        items: &[TreemapItem],
        horizontal: bool,
    ) {
        if items.is_empty() || rect.width() < 2.0 || rect.height() < 2.0 {
            return;
        }
        if items.len() == 1 {
            let item = &items[0];
            painter.rect_filled(rect, CornerRadius::ZERO, item.color);
            painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.5, RustCareTheme::BG_VOID), StrokeKind::Inside);

            let hover = ui.rect_contains_pointer(rect);
            if hover {
                painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(2.0, Color32::WHITE), StrokeKind::Inside);
                egui::show_tooltip(ui.ctx(), ui.layer_id(), Id::new(&item.label), |ui| {
                    ui.label(format!("{}: {}", item.label, item.size_text));
                });
            }
            return;
        }

        let total_weight: f32 = items.iter().map(|it| it.weight).sum();
        if total_weight <= 0.0 {
            return;
        }

        let mut half_weight = 0.0;
        let mut split_idx = 1;
        for (i, it) in items.iter().enumerate() {
            half_weight += it.weight;
            if half_weight >= total_weight / 2.0 || i == items.len() - 2 {
                split_idx = i + 1;
                break;
            }
        }

        let left_weight = items[..split_idx].iter().map(|it| it.weight).sum::<f32>();
        let frac = (left_weight / total_weight).clamp(0.05, 0.95);

        let (r1, r2) = if horizontal {
            let split_x = rect.min.x + rect.width() * frac;
            (
                Rect::from_min_max(rect.min, Pos2::new(split_x, rect.max.y)),
                Rect::from_min_max(Pos2::new(split_x, rect.min.y), rect.max),
            )
        } else {
            let split_y = rect.min.y + rect.height() * frac;
            (
                Rect::from_min_max(rect.min, Pos2::new(rect.max.x, split_y)),
                Rect::from_min_max(Pos2::new(rect.min.x, split_y), rect.max),
            )
        };

        layout_items(painter, ui, r1, &items[..split_idx], !horizontal);
        layout_items(painter, ui, r2, &items[split_idx..], !horizontal);
    }

    layout_items(painter, ui, rect, items, true);
}

/// 10. Status Badge
pub fn status_badge(ui: &mut Ui, text: &str, fg: Color32, bg: Color32) {
    RustCareTheme::draw_badge(ui, text, fg, bg);
}

/// 11. Checkbox Estilizado con acento Neón Teal
pub fn checkbox_item(ui: &mut Ui, checked: &mut bool, label: &str) -> Response {
    let font_id = FontId::proportional(12.5);
    let galley = ui.painter().layout_no_wrap(label.to_string(), font_id, RustCareTheme::TEXT_PRIMARY);
    let size = Vec2::new(galley.size().x + 28.0, 22.0);
    let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());

    if response.clicked() {
        *checked = !*checked;
        response.mark_changed();
    }

    let p = ui.painter();
    let box_rect = Rect::from_center_size(Pos2::new(rect.min.x + 9.0, rect.center().y), Vec2::splat(15.0));
    let (bg, stroke_col) = if *checked {
        (Color32::from_rgba_unmultiplied(14, 165, 233, 45), RustCareTheme::ACCENT_TEAL)
    } else if response.hovered() {
        (RustCareTheme::BG_HOVER, RustCareTheme::ACCENT_CYAN)
    } else {
        (RustCareTheme::BG_ELEVATED, RustCareTheme::BORDER_SUBTLE)
    };

    p.rect_filled(box_rect, CornerRadius::same(4), bg);
    p.rect_stroke(box_rect, CornerRadius::same(4), Stroke::new(1.0, stroke_col), StrokeKind::Inside);

    if *checked {
        let chk = [
            Pos2::new(box_rect.min.x + 3.5, box_rect.center().y),
            Pos2::new(box_rect.center().x - 0.5, box_rect.max.y - 4.0),
            Pos2::new(box_rect.max.x - 3.5, box_rect.min.y + 4.0),
        ];
        p.line_segment([chk[0], chk[1]], Stroke::new(2.0, RustCareTheme::ACCENT_CYAN));
        p.line_segment([chk[1], chk[2]], Stroke::new(2.0, RustCareTheme::ACCENT_CYAN));
    }

    p.galley(Pos2::new(rect.min.x + 24.0, rect.center().y - galley.size().y / 2.0), galley, RustCareTheme::TEXT_PRIMARY);
    response
}

/// 12. Panel lateral deslizable (Slide-Over Drawer estilo emilkowalski/vaul)
pub fn slide_over_drawer(
    ctx: &eframe::egui::Context,
    is_open: &mut bool,
    title: &str,
    subtitle: &str,
    accent: Color32,
    width: f32,
    body: impl FnOnce(&mut Ui),
) {
    if !*is_open {
        return;
    }

    if ctx.input(|i| i.key_pressed(eframe::egui::Key::Escape)) {
        *is_open = false;
        return;
    }

    let screen = ctx.screen_rect();
    let drawer_w = width.min(screen.width() - 48.0);

    // Backdrop atenuado clicable para cerrar
    eframe::egui::Area::new(eframe::egui::Id::new("vaul_drawer_backdrop"))
        .order(eframe::egui::Order::Middle)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let (bg_rect, bg_resp) = ui.allocate_exact_size(
                Vec2::new((screen.width() - drawer_w).max(0.0), screen.height()),
                Sense::click(),
            );
            ui.painter().rect_filled(
                bg_rect,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(6, 9, 15, 145),
            );
            if bg_resp.clicked() {
                *is_open = false;
            }
        });

    let drawer_pos = Pos2::new(screen.max.x - drawer_w, screen.min.y);
    eframe::egui::Area::new(eframe::egui::Id::new("vaul_drawer_panel"))
        .order(eframe::egui::Order::Foreground)
        .fixed_pos(drawer_pos)
        .show(ctx, |ui| {
            let frame = eframe::egui::Frame::NONE
                .fill(Color32::from_rgba_unmultiplied(13, 18, 28, 248))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 100)))
                .inner_margin(eframe::egui::Margin::same(20));

            frame.show(ui, |ui| {
                ui.set_width(drawer_w - 40.0);
                ui.set_min_height(screen.height() - 40.0);
                ui.set_max_height(screen.height() - 40.0);

                // Cabecera del Drawer
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(eframe::egui::RichText::new(title).size(17.0).strong().color(Color32::WHITE));
                        ui.label(eframe::egui::RichText::new(subtitle).size(11.5).color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.with_layout(eframe::egui::Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                        let (c_rect, c_resp) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::click());
                        let p = ui.painter();
                        p.rect_filled(
                            c_rect,
                            CornerRadius::same(7),
                            if c_resp.hovered() { RustCareTheme::BG_HOVER } else { RustCareTheme::BG_ELEVATED },
                        );
                        let cc = c_rect.center();
                        let col = if c_resp.hovered() { Color32::WHITE } else { RustCareTheme::TEXT_MUTED };
                        p.line_segment([Pos2::new(cc.x - 4.0, cc.y - 4.0), Pos2::new(cc.x + 4.0, cc.y + 4.0)], Stroke::new(1.4_f32, col));
                        p.line_segment([Pos2::new(cc.x + 4.0, cc.y - 4.0), Pos2::new(cc.x - 4.0, cc.y + 4.0)], Stroke::new(1.4_f32, col));
                        if c_resp.clicked() {
                            *is_open = false;
                        }
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                    body(ui);
                });
            });
        });
}

/// 13. Anillo circular de estado estilo AVG TuneUp con insignia ✓ / !
pub fn draw_status_ring(
    painter: &eframe::egui::Painter,
    center: Pos2,
    radius: f32,
    progress: f32,
    ring_color: Color32,
    is_ok: bool,
) {
    use std::f32::consts::TAU;
    // Pista de fondo
    painter.circle_stroke(center, radius, Stroke::new(3.0_f32, Color32::from_rgb(26, 35, 52)));

    // Arco de progreso coloreado
    let span = progress.clamp(0.08, 1.0) * TAU;
    let start_a = -0.25 * TAU;
    let steps = 32usize;
    let mut prev = Pos2::new(center.x + radius * start_a.cos(), center.y + radius * start_a.sin());
    for s in 1..=steps {
        let frac = s as f32 / steps as f32;
        let a = start_a + frac * span;
        let next = Pos2::new(center.x + radius * a.cos(), center.y + radius * a.sin());
        painter.line_segment([prev, next], Stroke::new(3.2_f32, ring_color));
        prev = next;
    }

    // Insignia circular pequeña en la esquina superior derecha del anillo (✓ o !)
    let badge_angle = -0.12 * TAU;
    let badge_c = Pos2::new(
        center.x + radius * badge_angle.cos(),
        center.y + radius * badge_angle.sin(),
    );
    painter.circle_filled(badge_c, 7.5, ring_color);
    painter.circle_stroke(badge_c, 7.5, Stroke::new(1.5_f32, Color32::from_rgb(13, 18, 27)));
    if is_ok {
        let p1 = Pos2::new(badge_c.x - 3.2, badge_c.y + 0.3);
        let p2 = Pos2::new(badge_c.x - 0.8, badge_c.y + 2.7);
        let p3 = Pos2::new(badge_c.x + 3.4, badge_c.y - 2.3);
        painter.line_segment([p1, p2], Stroke::new(1.8_f32, Color32::WHITE));
        painter.line_segment([p2, p3], Stroke::new(1.8_f32, Color32::WHITE));
    } else {
        painter.text(
            badge_c,
            Align2::CENTER_CENTER,
            "!",
            FontId::proportional(10.0),
            Color32::WHITE,
        );
    }
}

/// 14. Barra de progreso estilizada (reemplaza egui::ProgressBar estándar)
pub fn styled_progress_bar(
    ui: &mut Ui,
    ratio: f32,
    fill_color: Color32,
    label: Option<&str>,
) {
    let w = ui.available_width().max(80.0);
    let h = if label.is_some() { 18.0 } else { 8.0 };
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, h), Sense::hover());
    let p = ui.painter();

    p.rect_filled(rect, CornerRadius::same(6), RustCareTheme::BG_ELEVATED);
    p.rect_stroke(rect, CornerRadius::same(6), Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE), StrokeKind::Inside);

    let clamped = ratio.clamp(0.0, 1.0);
    if clamped > 0.01 {
        let fill_w = (rect.width() * clamped).max(6.0);
        p.rect_filled(
            Rect::from_min_size(rect.min, Vec2::new(fill_w, rect.height())),
            CornerRadius::same(6),
            Color32::from_rgba_unmultiplied(fill_color.r(), fill_color.g(), fill_color.b(), 200),
        );
    }

    if let Some(txt) = label {
        p.text(rect.center(), Align2::CENTER_CENTER, txt, FontId::proportional(11.0), Color32::WHITE);
    }
}
