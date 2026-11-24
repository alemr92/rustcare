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

    // 2. Capa 2: Estela de partículas y puntos horarios (r = 110.0)
    let particle_r = 110.0;
    for i in 0..28 {
        let frac = i as f32 / 28.0;
        let angle = cw_angle + frac * TAU;
        let alpha = ((frac * std::f32::consts::PI).sin() * 220.0).clamp(10.0, 240.0) as u8;
        let dot_pos = Pos2::new(center.x + particle_r * angle.cos(), center.y + particle_r * angle.sin());
        painter.circle_filled(
            dot_pos,
            if i % 2 == 0 { 1.8 } else { 1.2 },
            Color32::from_rgba_unmultiplied(0, 229, 168, alpha),
        );
    }

    // 3. Capa 3: Corona Mecánica / Dientes de Engranaje Holográfico Antihorario (r = 98.0)
    let gear_r = 98.0;
    let teeth = 20;
    for i in 0..teeth {
        let a = ccw_angle + (i as f32) * (TAU / teeth as f32);
        let p_in = Pos2::new(center.x + (gear_r - 4.0) * a.cos(), center.y + (gear_r - 4.0) * a.sin());
        let p_out = Pos2::new(center.x + (gear_r + 3.0) * a.cos(), center.y + (gear_r + 3.0) * a.sin());
        painter.line_segment([p_in, p_out], Stroke::new(2.0, Color32::from_rgba_unmultiplied(56, 189, 248, 120)));
    }
    painter.circle_stroke(center, gear_r - 4.0, Stroke::new(1.2, Color32::from_rgba_unmultiplied(34, 46, 66, 200)));

    // 4. Capa 4: Arco de flujo de energía exterior y progreso activo (r = 86.0)
    let flow_r = 86.0;
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
        painter.text(
            Pos2::new(center.x, center.y - 6.0),
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(23.0),
            Color32::WHITE,
        );
        if !subtitle.is_empty() {
            painter.text(
                Pos2::new(center.x, center.y + 16.0),
                Align2::CENTER_CENTER,
                subtitle,
                FontId::proportional(11.5),
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

    // Fondo Bento Glass con profundidad y rim light
    RustCareTheme::draw_glass_card(painter, rect, response.hovered(), Some(accent));

    // Fila 1: Título a la izquierda | Subtítulo hardware a la derecha
    painter.text(
        Pos2::new(rect.min.x + 14.0, rect.min.y + 18.0),
        Align2::LEFT_CENTER,
        title,
        FontId::proportional(15.0),
        Color32::WHITE,
    );
    painter.text(
        Pos2::new(rect.max.x - 14.0, rect.min.y + 18.0),
        Align2::RIGHT_CENTER,
        hw_subtitle,
        FontId::proportional(11.0),
        RustCareTheme::TEXT_MUTED,
    );

    // Fila 2: Valor Gigante (34px Bold con alto contraste)
    painter.text(
        Pos2::new(rect.min.x + 14.0, rect.min.y + 48.0),
        Align2::LEFT_CENTER,
        big_value,
        FontId::proportional(34.0),
        Color32::WHITE,
    );

    // Fila 3: Área de Ondas Líquidas Dinámicas (height: 54px)
    let wave_rect = Rect::from_min_max(
        Pos2::new(rect.min.x + 14.0, rect.min.y + 68.0),
        Pos2::new(rect.max.x - 14.0, rect.max.y - 28.0),
    );

    if wave_rect.width() > 20.0 {
        let steps = 40;
        let step_x = wave_rect.width() / (steps as f32);
        let base_val = if !history.is_empty() {
            *history.last().unwrap_or(&25.0)
        } else {
            30.0
        };
        let norm_h = (base_val / 100.0).clamp(0.15, 0.85) * wave_rect.height();

        let mut points: Vec<Pos2> = Vec::with_capacity(steps + 1);
        let freq = if title.contains("NVMe") || title.contains("Disco") { 0.18 } else { 0.08 };
        let speed = if title.contains("NVMe") { 3.5 } else { 1.8 };

        for i in 0..=steps {
            let x = wave_rect.min.x + (i as f32) * step_x;
            let rel_x = i as f32;
            let wave1 = ((rel_x * freq + time * speed).sin()) * 5.5;
            let wave2 = ((rel_x * (freq * 1.8) - time * (speed * 0.7)).sin()) * 2.5;
            let y = (wave_rect.max.y - norm_h + wave1 + wave2).clamp(wave_rect.min.y + 2.0, wave_rect.max.y - 2.0);
            points.push(Pos2::new(x, y));
        }

        // Polígono relleno translúcido hacia la base
        let mut poly = points.clone();
        poly.push(Pos2::new(wave_rect.max.x, wave_rect.max.y));
        poly.push(Pos2::new(wave_rect.min.x, wave_rect.max.y));
        painter.add(Shape::convex_polygon(
            poly,
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), if response.hovered() { 40 } else { 24 }),
            Stroke::NONE,
        ));

        // Curva superior brillante de 2.2px
        for w in points.windows(2) {
            painter.line_segment([w[0], w[1]], Stroke::new(2.2, accent));
        }
    }

    // Fila 4: Pico a la izquierda | Métricas activas a la derecha
    painter.text(
        Pos2::new(rect.min.x + 14.0, rect.max.y - 14.0),
        Align2::LEFT_CENTER,
        peak_text,
        FontId::proportional(11.0),
        RustCareTheme::TEXT_MUTED,
    );
    painter.text(
        Pos2::new(rect.max.x - 14.0, rect.max.y - 14.0),
        Align2::RIGHT_CENTER,
        current_text,
        FontId::proportional(11.0),
        accent,
    );

    if response.hovered() {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
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
