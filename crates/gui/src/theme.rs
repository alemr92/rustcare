//! Tema visual moderno oscuro para RustCare
use eframe::egui::{self, Color32, CornerRadius, Stroke, Visuals};

pub struct RustCareTheme;

impl RustCareTheme {
    // Paleta de colores moderna
    pub const BG_DARK: Color32 = Color32::from_rgb(18, 20, 26);
    pub const SIDEBAR_BG: Color32 = Color32::from_rgb(24, 27, 36);
    pub const CARD_BG: Color32 = Color32::from_rgb(30, 34, 46);
    pub const CARD_HOVER_BG: Color32 = Color32::from_rgb(38, 44, 58);
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(59, 130, 246);
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(6, 182, 212);
    pub const SUCCESS_GREEN: Color32 = Color32::from_rgb(16, 185, 129);
    pub const WARNING_AMBER: Color32 = Color32::from_rgb(245, 158, 11);
    pub const DANGER_RED: Color32 = Color32::from_rgb(239, 68, 68);
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(241, 245, 249);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);

    /// Configura el tema oscuro y estilo refinado de widgets
    pub fn apply(ctx: &egui::Context) {
        let mut visuals = Visuals::dark();

        visuals.panel_fill = Self::BG_DARK;
        visuals.window_fill = Self::CARD_BG;
        visuals.extreme_bg_color = Self::SIDEBAR_BG;

        visuals.widgets.noninteractive.bg_fill = Self::CARD_BG;
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);

        visuals.widgets.inactive.bg_fill = Self::CARD_BG;
        visuals.widgets.inactive.corner_radius = CornerRadius::same(6);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);

        visuals.widgets.hovered.bg_fill = Self::CARD_HOVER_BG;
        visuals.widgets.hovered.corner_radius = CornerRadius::same(6);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        visuals.widgets.active.bg_fill = Self::ACCENT_BLUE;
        visuals.widgets.active.corner_radius = CornerRadius::same(6);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        visuals.selection.bg_fill = Self::ACCENT_BLUE;
        visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);

        ctx.set_visuals(visuals);
    }
}
