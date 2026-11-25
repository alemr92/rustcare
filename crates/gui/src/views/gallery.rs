//! Galería de Componentes UI (estilo Storybook) para RustCare
//! Accesible mediante `--gallery` o desde la paleta Ctrl+K
use crate::icons::{draw_icon, AppIcon};
use crate::theme::RustCareTheme;
use crate::toast::ToastManager;
use crate::widgets;
use eframe::egui::{
    self, Color32, CornerRadius, Rect, RichText, ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2,
};

pub struct GalleryView {
    pub toggle_a: bool,
    pub toggle_b: bool,
    pub check_a: bool,
    pub check_b: bool,
    pub drawer_open: bool,
    pub radar_scanning: bool,
    pub sample_history: Vec<f32>,
}

impl Default for GalleryView {
    fn default() -> Self {
        Self {
            toggle_a: true,
            toggle_b: false,
            check_a: true,
            check_b: false,
            drawer_open: false,
            radar_scanning: false,
            sample_history: vec![12.0, 18.0, 24.0, 19.0, 28.0, 22.0, 17.0, 21.0, 16.0, 18.0],
        }
    }
}

impl GalleryView {
    pub fn render(&mut self, ui: &mut Ui, toasts: &mut ToastManager) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.heading(
                RichText::new("Galería de Componentes RustCare (Storybook)")
                    .size(22.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.label(
                RichText::new("Catálogo interactivo de primitivas, estados, gráficas, toasts, drawer e iconografía vectorial.")
                    .color(RustCareTheme::TEXT_MUTED),
            );
            ui.add_space(12.0);

            // 1. BOTONES Y CONTROLES
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new("1. Botones, Toggles y Checkboxes").size(15.0).strong().color(RustCareTheme::ACCENT_CYAN));
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    if widgets::neon_button(ui, "Botón Neón", Some(130.0)).clicked() {
                        toasts.success("Botón Neón", "Acción primaria ejecutada");
                    }
                    if widgets::ghost_button(ui, "Botón Ghost", Some(130.0)).clicked() {
                        toasts.info("Botón Ghost", "Acción secundaria ejecutada");
                    }
                    if widgets::danger_button(ui, "Botón Peligro", Some(130.0)).clicked() {
                        toasts.error("Botón Peligro", "Acción destructiva simulada");
                    }
                    ui.add_space(12.0);
                    widgets::pill_toggle(ui, &mut self.toggle_a);
                    widgets::pill_toggle(ui, &mut self.toggle_b);
                    ui.add_space(12.0);
                    widgets::checkbox_item(ui, &mut self.check_a, "Opción activa");
                    widgets::checkbox_item(ui, &mut self.check_b, "Opción inactiva");
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    RustCareTheme::draw_badge(ui, "ÓPTIMO", RustCareTheme::ACCENT_EMERALD, Color32::from_rgba_unmultiplied(16, 185, 129, 25));
                    RustCareTheme::draw_badge(ui, "TELEMETRÍA", RustCareTheme::ACCENT_CYAN, Color32::from_rgba_unmultiplied(56, 189, 248, 25));
                    RustCareTheme::draw_badge(ui, "SANDBOX", RustCareTheme::ACCENT_AMBER, Color32::from_rgba_unmultiplied(245, 158, 11, 25));
                    RustCareTheme::draw_badge(ui, "CRÍTICO", RustCareTheme::ACCENT_CORAL, Color32::from_rgba_unmultiplied(244, 63, 94, 25));
                });
            });

            ui.add_space(12.0);

            // 2. TOASTS, DRAWER Y TRACKER BAR
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new("2. Toasts (Sonner), Drawer (Vaul) y Tracker Bar (Tremor)").size(15.0).strong().color(RustCareTheme::ACCENT_CYAN));
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    if widgets::ghost_button(ui, "Toast Éxito", Some(120.0)).clicked() {
                        toasts.success("Operación completada", "1.4 GB liberados correctamente");
                    }
                    if widgets::ghost_button(ui, "Toast Info", Some(120.0)).clicked() {
                        toasts.info("Escaneo iniciado", "Analizando archivos temporales");
                    }
                    if widgets::ghost_button(ui, "Toast Aviso", Some(120.0)).clicked() {
                        toasts.warning("Permisos limitados", "Ejecuta como Administrador para DISM");
                    }
                    if widgets::neon_button(ui, "Abrir Slide-Over Drawer", Some(185.0)).clicked() {
                        self.drawer_open = true;
                    }
                });
                ui.add_space(10.0);
                ui.label(RichText::new("Tracker Bar de Salud Histórica (24 bloques):").size(12.0).color(RustCareTheme::TEXT_MUTED));
                ui.add_space(4.0);
                let tracker_states = [
                    1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1,
                ];
                let avail = ui.available_width().min(480.0);
                let (t_rect, _) = ui.allocate_exact_size(Vec2::new(avail, 16.0), Sense::hover());
                let gap = 3.0_f32;
                let block_w = ((t_rect.width() - gap * (tracker_states.len() as f32 - 1.0)) / tracker_states.len() as f32).max(4.0);
                let p = ui.painter();
                for (i, st) in tracker_states.iter().enumerate() {
                    let bx = t_rect.min.x + (i as f32) * (block_w + gap);
                    let col = match st {
                        1 => RustCareTheme::ACCENT_EMERALD,
                        2 => RustCareTheme::ACCENT_AMBER,
                        _ => RustCareTheme::ACCENT_CORAL,
                    };
                    p.rect_filled(
                        Rect::from_min_size(egui::pos2(bx, t_rect.min.y), Vec2::new(block_w, 16.0)),
                        CornerRadius::same(3),
                        col,
                    );
                }
            });

            ui.add_space(12.0);

            // 3. SPARKLINES INTERACTIVOS CON TOOLTIP (RECHARTS)
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new("3. Sensores Líquidos con Tooltip Interactivo (pasa el ratón por la onda)").size(15.0).strong().color(RustCareTheme::ACCENT_CYAN));
                ui.add_space(8.0);
                let w = ((ui.available_width() - 14.0) * 0.5).max(200.0);
                ui.horizontal(|ui| {
                    widgets::sparkline_card(
                        ui,
                        "CPU Demo",
                        "Suavizado EMA",
                        "18%",
                        "Pico: 28%",
                        "18%",
                        &self.sample_history,
                        RustCareTheme::ACCENT_CYAN,
                        w,
                        108.0,
                    );
                    widgets::sparkline_card(
                        ui,
                        "RAM Demo",
                        "32 GB Total",
                        "42% - 13.4 GB",
                        "Capacidad",
                        "Carga 42%",
                        &[40.0, 41.0, 43.0, 42.0, 44.0, 42.0, 41.0, 42.0],
                        RustCareTheme::ACCENT_INDIGO,
                        w,
                        108.0,
                    );
                });
            });

            ui.add_space(12.0);

            // 4. ICONOGRAFÍA VECTORIAL
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new("4. Catálogo de Iconografía Vectorial (icons.rs)").size(15.0).strong().color(RustCareTheme::ACCENT_CYAN));
                ui.add_space(8.0);
                let icons = [
                    ("LogoShield", AppIcon::LogoShield),
                    ("Home", AppIcon::Home),
                    ("Bolt", AppIcon::Bolt),
                    ("Broom", AppIcon::Broom),
                    ("ShieldCheck", AppIcon::ShieldCheck),
                    ("Tools", AppIcon::Tools),
                    ("Pulse", AppIcon::Pulse),
                    ("Gear", AppIcon::Gear),
                    ("Trash", AppIcon::Trash),
                    ("Eye", AppIcon::Eye),
                    ("RamChip", AppIcon::RamChip),
                    ("Brain", AppIcon::Brain),
                    ("Gauge", AppIcon::Gauge),
                    ("Shredder", AppIcon::Shredder),
                    ("Folder", AppIcon::Folder),
                ];
                ui.horizontal_wrapped(|ui| {
                    for (name, ic) in icons {
                        let (box_r, _) = ui.allocate_exact_size(Vec2::new(72.0, 62.0), Sense::hover());
                        let p = ui.painter();
                        p.rect_filled(box_r, CornerRadius::same(8), RustCareTheme::BG_ELEVATED);
                        p.rect_stroke(box_r, CornerRadius::same(8), Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE), StrokeKind::Inside);
                        let ic_r = Rect::from_center_size(egui::pos2(box_r.center().x, box_r.min.y + 22.0), Vec2::splat(28.0));
                        draw_icon(p, ic_r, ic, RustCareTheme::ACCENT_CYAN, 1.5);
                        p.text(
                            egui::pos2(box_r.center().x, box_r.max.y - 11.0),
                            egui::Align2::CENTER_CENTER,
                            name,
                            egui::FontId::proportional(9.5),
                            RustCareTheme::TEXT_MUTED,
                        );
                    }
                });
            });

            ui.add_space(12.0);

            // 5. TURBINA RADAR
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new("5. Turbina Holográfica Central").size(15.0).strong().color(RustCareTheme::ACCENT_CYAN));
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    let label = if self.radar_scanning { "64%" } else { "ANALIZAR" };
                    let sub = if self.radar_scanning { "Escaneando..." } else { "" };
                    if widgets::care_radar_button(ui, label, sub, if self.radar_scanning { 0.64 } else { 0.0 }, self.radar_scanning, None).clicked() {
                        self.radar_scanning = !self.radar_scanning;
                    }
                });
            });
        });

        widgets::slide_over_drawer(
            ui.ctx(),
            &mut self.drawer_open,
            "Drawer de Ejemplo (Vaul)",
            "Panel lateral deslizable sin perder el contexto",
            RustCareTheme::ACCENT_CYAN,
            380.0,
            |ui| {
                ui.label(RichText::new("Este panel permite inspeccionar archivos residuales, controladores o amenazas sin salir de la vista principal.").color(RustCareTheme::TEXT_PRIMARY));
                ui.add_space(12.0);
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.label(RichText::new("Estado del componente: Verificado").color(RustCareTheme::ACCENT_EMERALD));
                    ui.label(RichText::new("Pulsa ESC o haz clic fuera para cerrar.").size(11.5).color(RustCareTheme::TEXT_MUTED));
                });
            },
        );
    }
}
