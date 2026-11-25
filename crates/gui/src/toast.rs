//! Sistema de Notificaciones Toast (inspirado en Sonner) para RustCare
use crate::theme::RustCareTheme;
use eframe::egui::{
    self, Align2, Area, Color32, CornerRadius, FontId, Id, Order, Pos2, Rect, Sense, Stroke,
    StrokeKind, Vec2,
};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToastKind {
    Success,
    Info,
    Warning,
    Error,
}

impl ToastKind {
    fn accent_color(self) -> Color32 {
        match self {
            ToastKind::Success => RustCareTheme::ACCENT_EMERALD,
            ToastKind::Info => RustCareTheme::ACCENT_CYAN,
            ToastKind::Warning => RustCareTheme::ACCENT_AMBER,
            ToastKind::Error => RustCareTheme::ACCENT_CORAL,
        }
    }

    fn badge_label(self) -> &'static str {
        match self {
            ToastKind::Success => "OK",
            ToastKind::Info => "INFO",
            ToastKind::Warning => "AVISO",
            ToastKind::Error => "ERROR",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToastItem {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub kind: ToastKind,
    pub created_at: Instant,
    pub duration_secs: f32,
}

pub struct ToastManager {
    items: Vec<ToastItem>,
    next_id: u64,
}

impl Default for ToastManager {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }
}

#[allow(dead_code)]
impl ToastManager {
    pub fn push(
        &mut self,
        kind: ToastKind,
        title: impl Into<String>,
        description: impl Into<String>,
        duration_secs: f32,
    ) {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(ToastItem {
            id,
            title: title.into(),
            description: description.into(),
            kind,
            created_at: Instant::now(),
            duration_secs,
        });
        // Mantener un máximo de 4 toasts visibles simultáneamente
        if self.items.len() > 4 {
            self.items.remove(0);
        }
    }

    pub fn success(&mut self, title: impl Into<String>, description: impl Into<String>) {
        self.push(ToastKind::Success, title, description, 4.0);
    }

    pub fn info(&mut self, title: impl Into<String>, description: impl Into<String>) {
        self.push(ToastKind::Info, title, description, 3.8);
    }

    pub fn warning(&mut self, title: impl Into<String>, description: impl Into<String>) {
        self.push(ToastKind::Warning, title, description, 5.0);
    }

    pub fn error(&mut self, title: impl Into<String>, description: impl Into<String>) {
        self.push(ToastKind::Error, title, description, 6.0);
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        self.items
            .retain(|t| t.created_at.elapsed().as_secs_f32() < t.duration_secs);

        if self.items.is_empty() {
            return;
        }

        let screen_rect = ctx.screen_rect();
        let toast_w = 320.0_f32;
        let toast_h = 66.0_f32;
        let gap = 10.0_f32;
        let margin_right = 20.0_f32;
        let margin_bottom = 20.0_f32;

        let mut dismissed_id: Option<u64> = None;

        for (idx, toast) in self.items.iter().rev().enumerate() {
            let elapsed = toast.created_at.elapsed().as_secs_f32();
            let remaining = (toast.duration_secs - elapsed).max(0.0);
            let alpha_factor = if elapsed < 0.18 {
                elapsed / 0.18
            } else if remaining < 0.35 {
                remaining / 0.35
            } else {
                1.0
            }
            .clamp(0.0, 1.0);

            let offset_y = (idx as f32) * (toast_h + gap);
            let pos = Pos2::new(
                screen_rect.max.x - toast_w - margin_right,
                screen_rect.max.y - toast_h - margin_bottom - offset_y,
            );

            let area_id = Id::new("rustcare_toast").with(toast.id);
            let accent = toast.kind.accent_color();
            let a_bg = (232.0 * alpha_factor) as u8;
            let a_fg = (255.0 * alpha_factor) as u8;

            Area::new(area_id)
                .order(Order::Foreground)
                .fixed_pos(pos)
                .show(ctx, |ui| {
                    let (rect, _resp) =
                        ui.allocate_exact_size(Vec2::new(toast_w, toast_h), Sense::hover());
                    let p = ui.painter();

                    // Fondo de cristal obsidiana elevado
                    p.rect_filled(
                        rect,
                        CornerRadius::same(10),
                        Color32::from_rgba_unmultiplied(15, 22, 34, a_bg),
                    );
                    p.rect_stroke(
                        rect,
                        CornerRadius::same(10),
                        Stroke::new(
                            1.0_f32,
                            Color32::from_rgba_unmultiplied(
                                accent.r(),
                                accent.g(),
                                accent.b(),
                                (130.0 * alpha_factor) as u8,
                            ),
                        ),
                        StrokeKind::Inside,
                    );

                    // Barra lateral de acento neón
                    p.rect_filled(
                        Rect::from_min_size(
                            Pos2::new(rect.min.x + 1.0, rect.min.y + 8.0),
                            Vec2::new(3.0, rect.height() - 16.0),
                        ),
                        CornerRadius::same(2),
                        Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), a_fg),
                    );

                    // Etiqueta de estado
                    let badge_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x + 14.0, rect.min.y + 12.0),
                        Vec2::new(42.0, 18.0),
                    );
                    p.rect_filled(
                        badge_rect,
                        CornerRadius::same(5),
                        Color32::from_rgba_unmultiplied(
                            accent.r(),
                            accent.g(),
                            accent.b(),
                            (34.0 * alpha_factor) as u8,
                        ),
                    );
                    p.text(
                        badge_rect.center(),
                        Align2::CENTER_CENTER,
                        toast.kind.badge_label(),
                        FontId::proportional(9.5),
                        Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), a_fg),
                    );

                    // Título
                    p.text(
                        Pos2::new(rect.min.x + 64.0, rect.min.y + 21.0),
                        Align2::LEFT_CENTER,
                        &toast.title,
                        FontId::proportional(13.0),
                        Color32::from_rgba_unmultiplied(248, 250, 252, a_fg),
                    );

                    // Descripción
                    p.text(
                        Pos2::new(rect.min.x + 14.0, rect.min.y + 44.0),
                        Align2::LEFT_CENTER,
                        &toast.description,
                        FontId::proportional(11.5),
                        Color32::from_rgba_unmultiplied(148, 163, 184, a_fg),
                    );

                    // Botón cerrar (×)
                    let close_rect = Rect::from_min_size(
                        Pos2::new(rect.max.x - 26.0, rect.min.y + 8.0),
                        Vec2::splat(18.0),
                    );
                    let close_resp =
                        ui.interact(close_rect, area_id.with("close"), Sense::click());
                    let c_col = if close_resp.hovered() {
                        Color32::from_rgba_unmultiplied(248, 250, 252, a_fg)
                    } else {
                        Color32::from_rgba_unmultiplied(148, 163, 184, a_fg)
                    };
                    let cc = close_rect.center();
                    p.line_segment(
                        [Pos2::new(cc.x - 3.5, cc.y - 3.5), Pos2::new(cc.x + 3.5, cc.y + 3.5)],
                        Stroke::new(1.3_f32, c_col),
                    );
                    p.line_segment(
                        [Pos2::new(cc.x + 3.5, cc.y - 3.5), Pos2::new(cc.x - 3.5, cc.y + 3.5)],
                        Stroke::new(1.3_f32, c_col),
                    );
                    if close_resp.clicked() {
                        dismissed_id = Some(toast.id);
                    }

                    // Barra de progreso inferior de expiración
                    let prog = (remaining / toast.duration_secs).clamp(0.0, 1.0);
                    p.rect_filled(
                        Rect::from_min_size(
                            Pos2::new(rect.min.x + 8.0, rect.max.y - 2.5),
                            Vec2::new((rect.width() - 16.0) * prog, 1.5),
                        ),
                        CornerRadius::same(1),
                        Color32::from_rgba_unmultiplied(
                            accent.r(),
                            accent.g(),
                            accent.b(),
                            (140.0 * alpha_factor) as u8,
                        ),
                    );
                });
        }

        if let Some(id) = dismissed_id {
            self.items.retain(|t| t.id != id);
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}
