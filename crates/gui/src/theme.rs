//! Tema visual determinante 100% pixel-perfect para RustCare
use eframe::egui::{self, Color32, CornerRadius, FontId, Margin, Rect, Stroke, StrokeKind, Ui, Vec2, Visuals};

#[allow(dead_code)]
pub struct RustCareTheme;

impl RustCareTheme {
    // =========================================================================
    // TOKENS DE COLOR EXACTOS (ESPECIFICACIÓN DETERMINISTA)
    // =========================================================================
    pub const BG_VOID: Color32 = Color32::from_rgb(9, 12, 16);          // #090C10 - Fondo ventana y sidebar
    pub const BG_SIDEBAR: Color32 = Color32::from_rgb(13, 17, 24);      // #0D1118 - Fondo sidebar
    pub const BG_SURFACE: Color32 = Color32::from_rgb(17, 22, 31);      // #11161F - Fondo tarjetas principales
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(24, 32, 44);     // #18202C - Filas hover, inputs, píldoras inactivas
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(35, 45, 63);   // #232D3F - Borde 1.0px de todas las tarjetas

    // Acentos Neón / Cyber
    pub const ACCENT_TEAL: Color32 = Color32::from_rgb(0, 229, 168);    // #00E5A8 - Neón principal (ANALIZAR, Turbo ON, CPU)
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(56, 189, 248);   // #38BDF8 - Cian secundario (RAM, Modo Trabajo)
    pub const ACCENT_INDIGO: Color32 = Color32::from_rgb(129, 140, 248);// #818CF8 - Índigo (Disco en monitor)
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(255, 176, 32);  // #FFB020 - Ámbar (Badge MODO SANDBOX, Red)
    pub const ACCENT_CORAL: Color32 = Color32::from_rgb(255, 77, 109);  // #FF4D6D - Coral (Salud Crítica <50, Finalizar Tarea)

    // Tipografía
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(240, 244, 248); // #F0F4F8 - Texto blanco principal
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(138, 153, 173);   // #8A99AD - Texto secundario gris azulado

    // Alias de compatibilidad semántica unificada
    pub const CARD_BG: Color32 = Self::BG_SURFACE;
    pub const CARD_HOVER_BG: Color32 = Self::BG_ELEVATED;
    pub const CARD_BORDER: Color32 = Self::BORDER_SUBTLE;
    pub const SIDEBAR_BG: Color32 = Self::BG_SIDEBAR;
    pub const SUCCESS_GREEN: Color32 = Self::ACCENT_TEAL;
    pub const DANGER_RED: Color32 = Self::ACCENT_CORAL;
    pub const WARNING_AMBER: Color32 = Self::ACCENT_AMBER;
    pub const NEON_TEAL: Color32 = Self::ACCENT_TEAL;
    pub const ACCENT_BLUE: Color32 = Self::ACCENT_CYAN;
    pub const ACCENT_PURPLE: Color32 = Self::ACCENT_INDIGO;
    pub const TEXT_DIM: Color32 = Self::TEXT_MUTED;

    /// Dibuja una píldora / etiqueta de estado compacta
    pub fn draw_badge(ui: &mut Ui, text: &str, fg: Color32, bg: Color32) {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(0.0, 20.0), egui::Sense::hover());
        let font_id = FontId::proportional(11.0);
        let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, fg);
        let badge_width = galley.size().x + 16.0;
        let badge_rect = Rect::from_min_size(rect.min, Vec2::new(badge_width, 20.0));

        ui.painter().rect_filled(badge_rect, CornerRadius::same(10), bg);
        ui.painter().rect_stroke(
            badge_rect,
            CornerRadius::same(10),
            Stroke::new(1.0_f32, fg),
            StrokeKind::Inside,
        );
        let text_pos = egui::pos2(badge_rect.min.x + 8.0, badge_rect.min.y + 2.5);
        ui.painter().galley(text_pos, galley, fg);
        ui.allocate_rect(badge_rect, egui::Sense::hover());
    }

    // Radios y Espaciados Geométricos
    pub const RADIUS_CARD: f32 = 12.0;
    pub const RADIUS_BUTTON: f32 = 8.0;
    pub const RADIUS_PILL: f32 = 21.0;
    pub const PADDING_CARD: f32 = 16.0;
    pub const ITEM_SPACING: f32 = 14.0;

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

        visuals.selection.bg_fill = Color32::from_rgba_premultiplied(0, 229, 168, 60);
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
            .inner_margin(Margin::same(Self::PADDING_CARD as i8))
    }

    /// Botón personalizado con borde fino y relleno oscuro (prohibido botón gris estándar de egui)
    pub fn custom_button(ui: &mut Ui, text: &str, border_color: Color32, text_color: Color32, fill: Color32, height: f32) -> egui::Response {
        let font_id = FontId::proportional(12.5);
        let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, text_color);
        let desired_size = Vec2::new(galley.size().x + 22.0, height);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        let (fill_col, stroke_col) = if response.is_pointer_button_down_on() {
            (Color32::from_rgba_premultiplied(border_color.r(), border_color.g(), border_color.b(), 60), border_color)
        } else if response.hovered() {
            (Color32::from_rgba_premultiplied(border_color.r(), border_color.g(), border_color.b(), 35), border_color)
        } else {
            (fill, border_color)
        };

        ui.painter().rect_filled(rect, CornerRadius::same(Self::RADIUS_BUTTON as u8), fill_col);
        ui.painter().rect_stroke(rect, CornerRadius::same(Self::RADIUS_BUTTON as u8), Stroke::new(1.0_f32, stroke_col), StrokeKind::Inside);
        let text_pos = egui::pos2(rect.center().x - galley.size().x / 2.0, rect.center().y - galley.size().y / 2.0);
        ui.painter().galley(text_pos, galley, text_color);

        response
    }
}
