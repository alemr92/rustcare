//! Vista 2: Acelerar (Booster) - Basada en 02_turbo.png con fidelidad pixel-perfect
use crate::icons::{draw_icon, AppIcon};
use crate::theme::RustCareTheme;
use crate::widgets;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Pos2, RichText, ScrollArea, Sense,
    Stroke, StrokeKind, Ui, Vec2,
};
use rustcare_core::backend::{SystemBackend, TurboBoostMode, TurboBoostResult};
use rustcare_core::booster::RamCleanResult;
use rustcare_core::context_menu::{ContextMenuItem, ContextMenuManager};
use rustcare_core::scheduled_tasks::{ScheduledTaskItem, ScheduledTaskManager};
use rustcare_core::sleep_mode::{SleepEngine, SleepProgramItem};
use rustcare_core::startup::StartupEntry;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupSubTab {
    StartupApps,
    SleepMode,
    ScheduledTasks,
    ContextMenu,
}

pub struct BoosterView {
    pub selected_mode: TurboBoostMode,
    pub is_boost_active: bool,
    pub boost_result: Option<TurboBoostResult>,
    pub startup_entries: Vec<StartupEntry>,
    pub sleep_programs: Vec<SleepProgramItem>,
    pub startup_tab: StartupSubTab,
    pub scheduled_tasks: Vec<ScheduledTaskItem>,
    pub context_handlers: Vec<ContextMenuItem>,
    #[allow(dead_code)]
    pub ram_result: Option<RamCleanResult>,
    #[allow(dead_code)]
    pub dns_message: Option<String>,
}

impl Default for BoosterView {
    fn default() -> Self {
        Self {
            selected_mode: TurboBoostMode::Game,
            is_boost_active: false,
            boost_result: None,
            startup_entries: Vec::new(),
            sleep_programs: Vec::new(),
            startup_tab: StartupSubTab::StartupApps,
            scheduled_tasks: Vec::new(),
            context_handlers: Vec::new(),
            ram_result: None,
            dns_message: None,
        }
    }
}

impl BoosterView {
    pub fn render(&mut self, ui: &mut Ui, backend: &Arc<dyn SystemBackend>) {
        if self.startup_entries.is_empty() {
            self.startup_entries = backend.list_startup_entries();
        }
        self.is_boost_active = backend.is_turbo_boost_active();

        ui.vertical(|ui| {
            // =================================================================
            // 1. TARJETA HERO SUPERIOR (height: 124px)
            // =================================================================
            let is_on = self.is_boost_active;
            let hero_border = if is_on {
                RustCareTheme::ACCENT_TEAL
            } else {
                RustCareTheme::BORDER_SUBTLE
            };

            egui::Frame::NONE
                .fill(RustCareTheme::BG_SURFACE)
                .corner_radius(CornerRadius::same(RustCareTheme::RADIUS_CARD as u8))
                .stroke(Stroke::new(if is_on { 1.5 } else { 1.0 }, hero_border))
                .inner_margin(egui::Margin::symmetric(24, 20))
                .show(ui, |ui| {
                    ui.set_height(84.0);
                    ui.horizontal_centered(|ui| {
                        // Izquierda: Título TURBO BOOST + ESTADO
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("TURBO BOOST")
                                    .size(26.0)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                            ui.add_space(4.0);
                            let status_text = if is_on { "ESTADO: ACTIVO" } else { "ESTADO: INACTIVO" };
                            let status_col = if is_on { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::TEXT_MUTED };
                            ui.label(
                                RichText::new(status_text)
                                    .size(15.0)
                                    .strong()
                                    .color(status_col),
                            );
                        });

                        // Derecha: INTERRUPTOR GIGANTE widgets::big_turbo_switch (136x56px)
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut switch_state = self.is_boost_active;
                            if widgets::big_turbo_switch(ui, &mut switch_state).changed() {
                                if switch_state {
                                    self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                                    self.is_boost_active = true;
                                } else {
                                    let _ = backend.disable_turbo_boost();
                                    self.is_boost_active = false;
                                    self.boost_result = None;
                                }
                            }
                        });
                    });
                });

            ui.add_space(RustCareTheme::ITEM_SPACING);

            // =================================================================
            // 2. TRES TARJETAS DE MODO (height: 156px, gap: 14px)
            // =================================================================
            let avail_w = ui.available_width();
            let card_spacing = RustCareTheme::ITEM_SPACING;
            let col_w = (avail_w - (card_spacing * 2.0)) / 3.0;
            let col_h = 156.0;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = card_spacing;

                // TARJETA 1: MODO TRABAJO
                let is_sel1 = self.selected_mode == TurboBoostMode::Work;
                let border1 = if is_sel1 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE };
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(CornerRadius::same(RustCareTheme::RADIUS_CARD as u8))
                    .stroke(Stroke::new(1.0, border1))
                    .inner_margin(egui::Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(col_w - 32.0);
                        ui.set_height(col_h - 32.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let (ic_rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
                                ui.painter().rect_filled(ic_rect, CornerRadius::same(8), Color32::from_rgba_unmultiplied(14, 165, 233, 20));
                                ui.painter().rect_stroke(ic_rect, CornerRadius::same(8), Stroke::new(1.0, Color32::from_rgba_unmultiplied(14, 165, 233, 40)), StrokeKind::Inside);
                                draw_icon(ui.painter(), ic_rect, AppIcon::Briefcase, RustCareTheme::ACCENT_CYAN, 1.5);

                                ui.add_space(4.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new("MODO TRABAJO")
                                            .size(13.5)
                                            .strong()
                                            .color(RustCareTheme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new("4 Servicios detenidos")
                                            .size(11.0)
                                            .color(RustCareTheme::TEXT_MUTED),
                                    );
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
                                    ui.painter().circle_filled(dot_rect.center(), 3.5, if is_sel1 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE });
                                });
                            });

                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("180 MB liberados")
                                    .size(13.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_CYAN),
                            );

                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if widgets::ghost_button(ui, if is_sel1 { "Activo" } else { "Activar" }, Some(72.0)).clicked() {
                                    self.selected_mode = TurboBoostMode::Work;
                                    if self.is_boost_active {
                                        self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                                    }
                                }
                                if widgets::ghost_button(ui, "Configurar", Some(88.0)).clicked() {}
                            });
                        });
                    });

                // TARJETA 2: MODO JUEGO
                let is_sel2 = self.selected_mode == TurboBoostMode::Game;
                let border2 = if is_sel2 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE };
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(CornerRadius::same(RustCareTheme::RADIUS_CARD as u8))
                    .stroke(Stroke::new(1.0, border2))
                    .inner_margin(egui::Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(col_w - 32.0);
                        ui.set_height(col_h - 32.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let (ic_rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
                                ui.painter().rect_filled(ic_rect, CornerRadius::same(8), Color32::from_rgba_unmultiplied(14, 165, 233, 20));
                                ui.painter().rect_stroke(ic_rect, CornerRadius::same(8), Stroke::new(1.0, Color32::from_rgba_unmultiplied(14, 165, 233, 40)), StrokeKind::Inside);
                                draw_icon(ui.painter(), ic_rect, AppIcon::Gamepad, RustCareTheme::ACCENT_CYAN, 1.5);

                                ui.add_space(4.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new("MODO JUEGO")
                                            .size(13.5)
                                            .strong()
                                            .color(RustCareTheme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new("6 Servicios detenidos")
                                            .size(11.0)
                                            .color(RustCareTheme::TEXT_MUTED),
                                    );
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
                                    ui.painter().circle_filled(dot_rect.center(), 3.5, if is_sel2 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE });
                                });
                            });

                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("350 MB liberados")
                                    .size(13.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_TEAL),
                            );

                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if widgets::ghost_button(ui, if is_sel2 { "Activo" } else { "Activar" }, Some(72.0)).clicked() {
                                    self.selected_mode = TurboBoostMode::Game;
                                    if self.is_boost_active {
                                        self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                                    }
                                }
                                if widgets::ghost_button(ui, "Configurar", Some(88.0)).clicked() {}
                            });
                        });
                    });

                // TARJETA 3: AHORRO ENERGÍA
                let is_sel3 = self.selected_mode == TurboBoostMode::Economy;
                let border3 = if is_sel3 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE };
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(CornerRadius::same(RustCareTheme::RADIUS_CARD as u8))
                    .stroke(Stroke::new(1.0, border3))
                    .inner_margin(egui::Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(col_w - 32.0);
                        ui.set_height(col_h - 32.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let (ic_rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
                                ui.painter().rect_filled(ic_rect, CornerRadius::same(8), Color32::from_rgba_unmultiplied(14, 165, 233, 20));
                                ui.painter().rect_stroke(ic_rect, CornerRadius::same(8), Stroke::new(1.0, Color32::from_rgba_unmultiplied(14, 165, 233, 40)), StrokeKind::Inside);
                                draw_icon(ui.painter(), ic_rect, AppIcon::BatteryBolt, RustCareTheme::ACCENT_CYAN, 1.5);

                                ui.add_space(4.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new("AHORRO ENERGÍA")
                                            .size(13.5)
                                            .strong()
                                            .color(RustCareTheme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new("3 Servicios detenidos")
                                            .size(11.0)
                                            .color(RustCareTheme::TEXT_MUTED),
                                    );
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
                                    ui.painter().circle_filled(dot_rect.center(), 3.5, if is_sel3 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE });
                                });
                            });

                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("120 MB liberados")
                                    .size(13.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_AMBER),
                            );

                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if widgets::ghost_button(ui, if is_sel3 { "Activo" } else { "Activar" }, Some(72.0)).clicked() {
                                    self.selected_mode = TurboBoostMode::Economy;
                                    if self.is_boost_active {
                                        self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                                    }
                                }
                                if widgets::ghost_button(ui, "Configurar", Some(88.0)).clicked() {}
                            });
                        });
                    });
            });

            ui.add_space(RustCareTheme::ITEM_SPACING);

            // =================================================================
            // 3. PANEL INFERIOR UNIFICADO: GESTOR DE ARRANQUE Y SHELL (3 Subpestañas)
            // =================================================================
            RustCareTheme::glass_card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("GESTOR INTEGRAL DE ARRANQUE Y SHELL")
                            .size(13.5)
                            .strong()
                            .color(RustCareTheme::TEXT_PRIMARY),
                    );

                    // Badge de ganancia de tiempo
                    let (badge_r, _) = ui.allocate_exact_size(Vec2::new(125.0, 20.0), Sense::hover());
                    ui.painter().rect_filled(badge_r, CornerRadius::same(10), Color32::from_rgba_unmultiplied(0, 229, 168, 25));
                    ui.painter().rect_stroke(badge_r, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 229, 168, 90)), StrokeKind::Inside);
                    ui.painter().text(badge_r.center(), Align2::CENTER_CENTER, "-4.2 seg ganados", FontId::proportional(11.0), RustCareTheme::ACCENT_CYAN);

                    // 3 Subpestañas
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let is_ctx = self.startup_tab == StartupSubTab::ContextMenu;
                        if widgets::ghost_button(ui, if is_ctx { "[Menú Contextual]" } else { "Menú Contextual" }, Some(130.0)).clicked() {
                            if self.context_handlers.is_empty() {
                                self.context_handlers = ContextMenuManager::list_handlers();
                            }
                            self.startup_tab = StartupSubTab::ContextMenu;
                        }

                        let is_tasks = self.startup_tab == StartupSubTab::ScheduledTasks;
                        if widgets::ghost_button(ui, if is_tasks { "[Tareas Programadas]" } else { "Tareas Programadas" }, Some(140.0)).clicked() {
                            if self.scheduled_tasks.is_empty() {
                                self.scheduled_tasks = ScheduledTaskManager::list_tasks();
                            }
                            self.startup_tab = StartupSubTab::ScheduledTasks;
                        }

                        let is_sleep = self.startup_tab == StartupSubTab::SleepMode;
                        if widgets::ghost_button(ui, if is_sleep { "[Modo Suspensión]" } else { "Modo Suspensión" }, Some(135.0)).clicked() {
                            if self.sleep_programs.is_empty() {
                                let snap = backend.get_metrics();
                                self.sleep_programs = SleepEngine::scan_programs(&self.startup_entries, &snap);
                            }
                            self.startup_tab = StartupSubTab::SleepMode;
                        }

                        let is_apps = self.startup_tab == StartupSubTab::StartupApps;
                        if widgets::ghost_button(ui, if is_apps { "[Apps al Inicio]" } else { "Apps al Inicio" }, Some(120.0)).clicked() {
                            self.startup_tab = StartupSubTab::StartupApps;
                        }
                    });
                });

                ui.add_space(8.0);

                let total_w = ui.available_width();
                let col_prog_w = total_w * 0.42;
                let col_impact_w = total_w * 0.20;
                let col_state_w = total_w * 0.18;
                let col_action_w = total_w * 0.20;

                match self.startup_tab {
                    StartupSubTab::StartupApps => {
                        // Cabecera de la tabla
                        ui.horizontal(|ui| {
                            ui.add_space(8.0);
                            ui.allocate_ui_with_layout(Vec2::new(col_prog_w, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(RichText::new("Programa").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                            });
                            ui.allocate_ui_with_layout(Vec2::new(col_impact_w, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(RichText::new("Impacto").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                            });
                            ui.allocate_ui_with_layout(Vec2::new(col_state_w, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(RichText::new("Estado").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                            });
                            ui.allocate_ui_with_layout(Vec2::new(col_action_w - 16.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(RichText::new("Acción").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                            });
                        });

                        ui.add_space(4.0);
                        ui.painter().line_segment(
                            [Pos2::new(ui.min_rect().min.x, ui.cursor().min.y), Pos2::new(ui.min_rect().max.x, ui.cursor().min.y)],
                            Stroke::new(1.0, RustCareTheme::BORDER_SUBTLE),
                        );
                        ui.add_space(6.0);

                        ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                            let mut toggles: Vec<(String, bool)> = Vec::new();

                            for (idx, entry) in self.startup_entries.iter().enumerate() {
                                let row_bg = if idx % 2 == 0 {
                                    RustCareTheme::BG_SURFACE
                                } else {
                                    Color32::from_rgba_unmultiplied(26, 35, 52, 120)
                                };

                                egui::Frame::NONE
                                    .fill(row_bg)
                                    .corner_radius(CornerRadius::same(6))
                                    .inner_margin(egui::Margin::symmetric(8, 4))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                                            let dot_col = if entry.is_enabled { RustCareTheme::ACCENT_CYAN } else { RustCareTheme::BORDER_SUBTLE };
                                            ui.painter().circle_filled(dot_rect.center(), 3.5, dot_col);

                                            ui.add_space(4.0);

                                            let name_clean = if entry.name.len() > 26 {
                                                format!("{}...", &entry.name[..24])
                                            } else {
                                                entry.name.clone()
                                            };
                                            ui.allocate_ui_with_layout(Vec2::new(col_prog_w - 24.0, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(name_clean).size(12.5).color(RustCareTheme::TEXT_PRIMARY));
                                            });

                                            let (imp_text, imp_color) = if idx == 0 {
                                                ("Alto", RustCareTheme::ACCENT_CORAL)
                                            } else if idx == 1 {
                                                ("Medio", RustCareTheme::ACCENT_AMBER)
                                            } else {
                                                ("Bajo", RustCareTheme::ACCENT_CYAN)
                                            };
                                            ui.allocate_ui_with_layout(Vec2::new(col_impact_w, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                let (imp_rect, _) = ui.allocate_exact_size(Vec2::new(60.0, 20.0), Sense::hover());
                                                ui.painter().rect_filled(imp_rect, CornerRadius::same(10), RustCareTheme::BG_ELEVATED);
                                                ui.painter().rect_stroke(imp_rect, CornerRadius::same(10), Stroke::new(1.0, imp_color), StrokeKind::Inside);
                                                ui.painter().text(imp_rect.center(), Align2::CENTER_CENTER, imp_text, FontId::proportional(11.0), imp_color);
                                            });

                                            ui.allocate_ui_with_layout(Vec2::new(col_state_w, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                let mut state_copy = entry.is_enabled;
                                                if widgets::pill_toggle(ui, &mut state_copy).changed() {
                                                    toggles.push((entry.id.clone(), state_copy));
                                                }
                                            });

                                            let btn_label = if entry.is_enabled { "Desactivar" } else { "Activar" };
                                            if widgets::ghost_button(ui, btn_label, Some(84.0)).clicked() {
                                                toggles.push((entry.id.clone(), !entry.is_enabled));
                                            }
                                        });
                                    });
                                ui.add_space(2.0);
                            }

                            for (id, new_state) in toggles {
                                let _ = backend.toggle_startup_entry(&id, new_state);
                                self.startup_entries = backend.list_startup_entries();
                            }
                        });
                    }
                    StartupSubTab::ScheduledTasks => {
                        ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                            if self.scheduled_tasks.is_empty() {
                                ui.label(RichText::new("Haz clic en Tareas Programadas para auditar las tareas activas.").color(RustCareTheme::TEXT_MUTED));
                            } else {
                                for task in &self.scheduled_tasks {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(&task.task_name).strong().color(RustCareTheme::TEXT_PRIMARY));
                                        ui.label(RichText::new(&task.status).color(RustCareTheme::ACCENT_CYAN));
                                        ui.label(RichText::new(&task.action_exec).small().color(RustCareTheme::TEXT_MUTED));
                                    });
                                }
                            }
                        });
                    }
                    StartupSubTab::SleepMode => {
                        ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                            if self.sleep_programs.is_empty() {
                                let snap = backend.get_metrics();
                                self.sleep_programs = SleepEngine::scan_programs(&self.startup_entries, &snap);
                            }
                            for idx in 0..self.sleep_programs.len() {
                                let name = self.sleep_programs[idx].name.clone();
                                let imp_lbl = self.sleep_programs[idx].impact.label();
                                let is_slp = self.sleep_programs[idx].is_sleeping;
                                let mem_mb = self.sleep_programs[idx].memory_mb;
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(&name).strong().color(RustCareTheme::TEXT_PRIMARY));
                                    ui.label(RichText::new(format!("Impacto: {} · {:.0} MB", imp_lbl, mem_mb)).small().color(RustCareTheme::TEXT_MUTED));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if is_slp {
                                            if widgets::ghost_button(ui, "Despertar", Some(90.0)).clicked() {
                                                let _ = SleepEngine::wake_program(&mut self.sleep_programs[idx], backend);
                                            }
                                        } else if widgets::neon_button(ui, "Suspender", Some(90.0)).clicked() {
                                            let _ = SleepEngine::sleep_program(&mut self.sleep_programs[idx], backend);
                                        }
                                    });
                                });
                                ui.separator();
                            }
                        });
                    }
                    StartupSubTab::ContextMenu => {
                        ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                            if self.context_handlers.is_empty() {
                                ui.label(RichText::new("Haz clic en Menú Contextual para auditar extensiones de clic derecho.").color(RustCareTheme::TEXT_MUTED));
                            } else {
                                for handler in &self.context_handlers {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(&handler.name).strong().color(RustCareTheme::TEXT_PRIMARY));
                                        ui.label(RichText::new(if handler.is_enabled { "Habilitado" } else { "Deshabilitado" }).color(RustCareTheme::ACCENT_AMBER));
                                        ui.label(RichText::new(&handler.key_path).small().color(RustCareTheme::TEXT_MUTED));
                                    });
                                }
                            }
                        });
                    }
                }
            });
        });
    }
}
