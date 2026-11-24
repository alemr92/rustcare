//! Tema visual determinante y elegante para RustCare
//! Paleta moderna inspirada en interfaces premium (Obsidian Dark & Electric Azure/Emerald)
use eframe::egui::{
    self, Color32, CornerRadius, FontId, Margin, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2, Visuals,
};

#[allow(dead_code)]
pub struct RustCareTheme;

impl RustCareTheme {
    // =========================================================================
    // TOKENS DE COLOR PREMIUM (DARK OBSIDIAN & REFINED ACCENTS)
    // =========================================================================
    pub const BG_VOID: Color32 = Color32::from_rgb(11, 15, 23);          // #0B0F17 - Fondo base
    pub const BG_SIDEBAR: Color32 = Color32::from_rgb(13, 18, 27);      // #0D121B - Fondo barra lateral
    pub const BG_SURFACE: Color32 = Color32::from_rgb(18, 24, 37);      // #121825 - Fondo tarjetas
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(26, 35, 52);     // #1A2334 - Fondo inputs, filas y botones
    pub const BG_HOVER: Color32 = Color32::from_rgb(34, 46, 68);        // #222E44 - Estado hover interactivo
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(32, 43, 62);   // #202B3E - Borde estándar 1.0px

    // Acentos Armoniosos y Refinados (Paleta Electric Azure & Emerald)
    pub const ACCENT_TEAL: Color32 = Color32::from_rgb(14, 165, 233);   // #0EA5E9 - Electric Azure principal
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(56, 189, 248);   // #38BDF8 - Cian suave secundario
    pub const ACCENT_EMERALD: Color32 = Color32::from_rgb(16, 185, 129);// #10B981 - Verde esmeralda (Salud óptima / OK)
    pub const ACCENT_INDIGO: Color32 = Color32::from_rgb(129, 140, 248);// #818CF8 - Índigo suave (Disco en monitor)
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(245, 158, 11);  // #F59E0B - Ámbar cálido (Advertencias / Sandbox)
    pub const ACCENT_CORAL: Color32 = Color32::from_rgb(244, 63, 94);   // #F43F5E - Coral / Rose (Peligro / Riesgo)

    // Tipografía
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(248, 250, 252); // #F8FAFC - Blanco tiza de alta legibilidad
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);   // #94A3B8 - Texto secundario slate
    pub const TEXT_DIM: Color32 = Color32::from_rgb(100, 116, 139);     // #64748B - Texto terciario

    // Alias semánticos
    pub const CARD_BG: Color32 = Self::BG_SURFACE;
    pub const CARD_HOVER_BG: Color32 = Self::BG_ELEVATED;
    pub const CARD_BORDER: Color32 = Self::BORDER_SUBTLE;
    pub const SIDEBAR_BG: Color32 = Self::BG_SIDEBAR;
    pub const SUCCESS_GREEN: Color32 = Self::ACCENT_EMERALD;
    pub const DANGER_RED: Color32 = Self::ACCENT_CORAL;
    pub const WARNING_AMBER: Color32 = Self::ACCENT_AMBER;
    pub const NEON_TEAL: Color32 = Self::ACCENT_TEAL;
    pub const ACCENT_BLUE: Color32 = Self::ACCENT_CYAN;
    pub const ACCENT_PURPLE: Color32 = Self::ACCENT_INDIGO;

    // Radios y Espaciados Geométricos
    pub const RADIUS_CARD: f32 = 14.0;
    pub const RADIUS_BUTTON: f32 = 8.0;
    pub const RADIUS_PILL: f32 = 21.0;
    pub const PADDING_CARD: f32 = 16.0;
    pub const ITEM_SPACING: f32 = 14.0;

    /// Dibuja la luz ambiental difusa en el fondo de la ventana (Ambient Mesh Glow 2027)
    pub fn draw_ambient_mesh_glow(painter: &eframe::egui::Painter, screen_rect: Rect, time: f32) {
        let c = screen_rect.center();
        let pulse = (time * 0.8).sin() * 20.0;
        // Halo central cian (#00E5A8) de radio 440px
        painter.circle_filled(
            egui::pos2(c.x + 60.0, c.y - 40.0),
            420.0 + pulse,
            Color32::from_rgba_unmultiplied(0, 229, 168, 11), // alpha 0.045
        );
        // Halo superior azul cobalto (#2563EB) de radio 420px
        painter.circle_filled(
            egui::pos2(screen_rect.min.x + 140.0, screen_rect.min.y + 90.0),
            420.0,
            Color32::from_rgba_unmultiplied(37, 99, 235, 13), // alpha 0.05
        );
        // Halo inferior-derecho cian suave (#38BDF8)
        painter.circle_filled(
            egui::pos2(screen_rect.max.x - 100.0, screen_rect.max.y - 80.0),
            380.0,
            Color32::from_rgba_unmultiplied(56, 189, 248, 10), // alpha 0.04
        );
    }

    /// Dibuja una tarjeta con profundidad tridimensional, luz de cristal superior y borde reactivo (Bento Glass Card 2027)
    pub fn draw_glass_card(
        painter: &eframe::egui::Painter,
        rect: Rect,
        is_hovered: bool,
        accent: Option<Color32>,
    ) {
        let bg = if is_hovered {
            Color32::from_rgba_unmultiplied(26, 36, 54, 235)
        } else {
            Color32::from_rgba_unmultiplied(18, 24, 36, 220)
        };
        let radius = CornerRadius::same(Self::RADIUS_CARD as u8);
        painter.rect_filled(rect, radius, bg);

        // Borde perimetral con iluminación al hover
        let border_col = if is_hovered {
            accent.unwrap_or(Color32::from_rgb(0, 229, 168))
        } else {
            Color32::from_rgb(34, 46, 66)
        };
        painter.rect_stroke(rect, radius, Stroke::new(1.0_f32, border_col), StrokeKind::Inside);

        // Luz superior especular (Rim Light de 1px, blanco alpha 38 / 60)
        let top_highlight = if is_hovered {
            Color32::from_rgba_unmultiplied(255, 255, 255, 65)
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 38)
        };
        painter.line_segment(
            [
                egui::pos2(rect.min.x + Self::RADIUS_CARD, rect.min.y + 1.0),
                egui::pos2(rect.max.x - Self::RADIUS_CARD, rect.min.y + 1.0),
            ],
            Stroke::new(1.0_f32, top_highlight),
        );

        // Proyector de luz inferior neón (Signature Underglow Bar 2027)
        let glow_accent = accent.unwrap_or(Color32::from_rgb(0, 229, 168));
        let glow_w = (rect.width() * 0.48).clamp(38.0, 110.0);
        let glow_center_x = rect.center().x;
        let glow_y = rect.max.y;

        // Halo difuso exterior debajo de la tarjeta
        painter.rect_filled(
            Rect::from_center_size(Pos2::new(glow_center_x, glow_y + 1.0), Vec2::new(glow_w + 22.0, 7.0)),
            CornerRadius::same(3),
            Color32::from_rgba_unmultiplied(glow_accent.r(), glow_accent.g(), glow_accent.b(), if is_hovered { 70 } else { 42 }),
        );
        // Barra intensa de neón horizontal
        painter.rect_filled(
            Rect::from_center_size(Pos2::new(glow_center_x, glow_y - 0.5), Vec2::new(glow_w, 3.2)),
            CornerRadius::same(2),
            Color32::from_rgba_unmultiplied(glow_accent.r(), glow_accent.g(), glow_accent.b(), if is_hovered { 255 } else { 205 }),
        );
        // Núcleo blanco brillante en el centro del proyector
        painter.rect_filled(
            Rect::from_center_size(Pos2::new(glow_center_x, glow_y - 0.5), Vec2::new(glow_w * 0.45, 1.8)),
            CornerRadius::same(1),
            Color32::from_rgba_unmultiplied(255, 255, 255, if is_hovered { 220 } else { 160 }),
        );
    }

    /// Dibuja una píldora / etiqueta de estado compacta
    pub fn draw_badge(ui: &mut Ui, text: &str, fg: Color32, bg: Color32) {
        let font_id = FontId::proportional(11.0);
        let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, fg);
        let badge_width = galley.size().x + 16.0;
        let (rect, _) = ui.allocate_exact_size(Vec2::new(badge_width, 20.0), egui::Sense::hover());

        ui.painter().rect_filled(rect, CornerRadius::same(10), bg);
        ui.painter().rect_stroke(
            rect,
            CornerRadius::same(10),
            Stroke::new(1.0_f32, fg),
            StrokeKind::Inside,
        );
        let text_pos = egui::pos2(rect.min.x + 8.0, rect.min.y + 2.5);
        ui.painter().galley(text_pos, galley, fg);
    }

    /// Configura las fuentes del sistema
    pub fn configure_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        #[cfg(windows)]
        {
            if let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\segoeui.ttf") {
                fonts.font_data.insert(
                    "segoe_ui".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(bytes)),
                );
                fonts.families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "segoe_ui".to_owned());
            }

            if let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\seguisb.ttf") {
                fonts.font_data.insert(
                    "segoe_ui_semibold".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(bytes)),
                );
            }
        }

        ctx.set_fonts(fonts);
    }

    /// Configura el tema oscuro global de widgets
    pub fn apply(ctx: &egui::Context) {
        Self::configure_fonts(ctx);
        let mut visuals = Visuals::dark();

        visuals.panel_fill = Self::BG_VOID;
        visuals.window_fill = Self::BG_SURFACE;
        visuals.extreme_bg_color = Self::BG_VOID;

        visuals.widgets.noninteractive.bg_fill = Self::BG_SURFACE;
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(Self::RADIUS_BUTTON as u8);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);

        visuals.widgets.inactive.bg_fill = Self::BG_SURFACE;
        visuals.widgets.inactive.corner_radius = CornerRadius::same(Self::RADIUS_BUTTON as u8);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);

        visuals.widgets.hovered.bg_fill = Self::BG_ELEVATED;
        visuals.widgets.hovered.corner_radius = CornerRadius::same(Self::RADIUS_BUTTON as u8);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        visuals.widgets.active.bg_fill = Self::BG_ELEVATED;
        visuals.widgets.active.corner_radius = CornerRadius::same(Self::RADIUS_BUTTON as u8);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Self::ACCENT_TEAL);

        visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(14, 165, 233, 40);
        visuals.selection.stroke = Stroke::new(1.0_f32, Self::ACCENT_TEAL);

        ctx.set_visuals(visuals);
    }

    /// Marco estándar de tarjeta con radio 12px, relleno BG_SURFACE y borde 1px BORDER_SUBTLE
    pub fn card_frame() -> egui::Frame {
        egui::Frame::NONE
            .fill(Self::BG_SURFACE)
            .corner_radius(CornerRadius::same(Self::RADIUS_CARD as u8))
            .stroke(Stroke::new(1.0_f32, Self::BORDER_SUBTLE))
            .inner_margin(Margin::same(Self::PADDING_CARD as i8))
    }

    /// Marco de tarjeta de cristal semitransparente con radio 14px (Bento Glass Card 2027)
    pub fn glass_card_frame() -> egui::Frame {
        egui::Frame::NONE
            .fill(Color32::from_rgba_unmultiplied(18, 24, 36, 220))
            .corner_radius(CornerRadius::same(14))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(34, 46, 66)))
            .inner_margin(Margin::same(Self::PADDING_CARD as i8))
    }

    /// Marco de tarjeta elevado para hover o elementos destacados
    pub fn elevated_frame() -> egui::Frame {
        egui::Frame::NONE
            .fill(Self::BG_ELEVATED)
            .corner_radius(CornerRadius::same(Self::RADIUS_CARD as u8))
            .stroke(Stroke::new(1.0_f32, Self::BORDER_SUBTLE))
            .inner_margin(Margin::same(Self::PADDING_CARD as i8))
    }

    /// Marco hero destacado con doble borde / glow opcional
    pub fn hero_frame(is_active: bool) -> egui::Frame {
        let border_color = if is_active {
            Self::ACCENT_TEAL
        } else {
            Self::BORDER_SUBTLE
        };

        egui::Frame::NONE
            .fill(Self::BG_SURFACE)
            .corner_radius(CornerRadius::same(Self::RADIUS_CARD as u8))
            .stroke(Stroke::new(if is_active { 1.5_f32 } else { 1.0_f32 }, border_color))
            .inner_margin(Margin::same(20))
    }

    /// Botón personalizado
    pub fn custom_button(
        ui: &mut Ui,
        text: &str,
        border_color: Color32,
        text_color: Color32,
        _bg_color: Color32,
        height: f32,
    ) -> egui::Response {
        let font_id = FontId::proportional(12.5);
        let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, text_color);
        let btn_width = galley.size().x + 24.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(btn_width, height), egui::Sense::click());

        let (fill, stroke_col) = if response.is_pointer_button_down_on() {
            (Self::BG_HOVER, border_color)
        } else if response.hovered() {
            (Self::BG_HOVER, border_color)
        } else {
            (Self::BG_ELEVATED, border_color)
        };

        ui.painter().rect_filled(rect, CornerRadius::same(Self::RADIUS_BUTTON as u8), fill);
        ui.painter().rect_stroke(
            rect,
            CornerRadius::same(Self::RADIUS_BUTTON as u8),
            Stroke::new(1.0_f32, stroke_col),
            StrokeKind::Inside,
        );

        let text_pos = egui::pos2(
            rect.center().x - galley.size().x / 2.0,
            rect.center().y - galley.size().y / 2.0,
        );
        ui.painter().galley(text_pos, galley, text_color);

        response
    }
}
