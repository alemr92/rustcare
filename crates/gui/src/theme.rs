//! Tema visual moderno 2026 oscuro para RustCare
use eframe::egui::{self, Color32, CornerRadius, FontId, Margin, Rect, Stroke, StrokeKind, Ui, Vec2, Visuals};

pub struct RustCareTheme;

impl RustCareTheme {
    // Paleta Cyber-Dark & Neon (Inspirada en IObit Advanced SystemCare)
    pub const BG_DARK: Color32 = Color32::from_rgb(15, 19, 26); // #0F131A
    pub const SIDEBAR_BG: Color32 = Color32::from_rgb(17, 21, 30);
    pub const TOPBAR_BG: Color32 = Color32::from_rgb(19, 24, 34);
    pub const CARD_BG: Color32 = Color32::from_rgb(26, 33, 46); // #1A212E
    pub const CARD_HOVER_BG: Color32 = Color32::from_rgb(33, 42, 58);
    pub const CARD_BORDER: Color32 = Color32::from_rgb(38, 48, 68);

    // Acentos Neón / Cyber
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0, 229, 255);
    pub const NEON_TEAL: Color32 = Color32::from_rgb(0, 229, 168); // #00E5A8 Óptimo
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(59, 130, 246);
    pub const ACCENT_PURPLE: Color32 = Color32::from_rgb(168, 85, 247);
    pub const SUCCESS_GREEN: Color32 = Color32::from_rgb(0, 229, 168); // #00E5A8
    pub const WARNING_AMBER: Color32 = Color32::from_rgb(255, 176, 32); // #FFB020
    pub const DANGER_RED: Color32 = Color32::from_rgb(255, 77, 109); // #FF4D6D Crítico

    // Tipografía
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(248, 250, 252);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(100, 116, 139);

    /// Configura las fuentes del sistema (Segoe UI, Segoe UI Symbol y Segoe UI Emoji)
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

            if let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\seguisym.ttf") {
                fonts.font_data.insert(
                    "segoe_symbol".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(bytes)),
                );
                fonts.families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .push("segoe_symbol".to_owned());
            }

            if let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\seguiemj.ttf") {
                fonts.font_data.insert(
                    "segoe_emoji".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(bytes)),
                );
                fonts.families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .push("segoe_emoji".to_owned());
            }
        }

        ctx.set_fonts(fonts);
    }

    /// Configura el tema oscuro y estilo refinado de widgets
    pub fn apply(ctx: &egui::Context) {
        Self::configure_fonts(ctx);
        let mut visuals = Visuals::dark();

        visuals.panel_fill = Self::BG_DARK;
        visuals.window_fill = Self::CARD_BG;
        visuals.extreme_bg_color = Self::SIDEBAR_BG;

        visuals.widgets.noninteractive.bg_fill = Self::CARD_BG;
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(10);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);

        visuals.widgets.inactive.bg_fill = Self::CARD_BG;
        visuals.widgets.inactive.corner_radius = CornerRadius::same(8);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);

        visuals.widgets.hovered.bg_fill = Self::CARD_HOVER_BG;
        visuals.widgets.hovered.corner_radius = CornerRadius::same(8);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        visuals.widgets.active.bg_fill = Self::ACCENT_BLUE;
        visuals.widgets.active.corner_radius = CornerRadius::same(8);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        visuals.selection.bg_fill = Self::ACCENT_BLUE;
        visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);

        ctx.set_visuals(visuals);
    }

    /// Marco estándar de tarjeta de cristal con borde sutil
    pub fn card_frame() -> egui::Frame {
        egui::Frame::NONE
            .fill(Self::CARD_BG)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0_f32, Self::CARD_BORDER))
            .inner_margin(Margin::same(14))
    }

    /// Marco destacado con acento
    pub fn hero_frame(accent_border: bool) -> egui::Frame {
        let border_color = if accent_border {
            Self::ACCENT_CYAN
        } else {
            Self::CARD_BORDER
        };
        egui::Frame::NONE
            .fill(Self::CARD_BG)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.5_f32, border_color))
            .inner_margin(Margin::same(16))
    }

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
}
