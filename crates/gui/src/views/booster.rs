//! Vista de Aceleración (Turbo Boost estilo IObit ASC) - 2026 Edition
use crate::theme::RustCareTheme;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, ProgressBar, RichText, ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2};
use rustcare_core::backend::{SystemBackend, TurboBoostMode, TurboBoostResult};
use rustcare_core::booster::RamCleanResult;
use rustcare_core::cleaner::CleanerEngine;
use rustcare_core::startup::StartupEntry;
use std::sync::Arc;

pub struct BoosterView {
    pub selected_mode: TurboBoostMode,
    pub is_boost_active: bool,
    pub boost_result: Option<TurboBoostResult>,
    pub startup_entries: Vec<StartupEntry>,
    pub ram_result: Option<RamCleanResult>,
    pub dns_message: Option<String>,
}

impl Default for BoosterView {
    fn default() -> Self {
        Self {
            selected_mode: TurboBoostMode::Game,
            is_boost_active: false,
            boost_result: None,
            startup_entries: Vec::new(),
            ram_result: None,
            dns_message: None,
        }
    }
}

impl BoosterView {
    pub fn render(&mut self, ui: &mut Ui, backend: &Arc<dyn SystemBackend>) {
        let _time = ui.input(|i| i.time) as f32;

        // Cargar entradas de inicio si la lista está vacía
        if self.startup_entries.is_empty() {
            self.startup_entries = backend.list_startup_entries();
        }
        self.is_boost_active = backend.is_turbo_boost_active();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(
                    RichText::new("⚡ Aceleración de Sistema & Turbo Boost")
                        .size(22.0)
                        .strong()
                        .color(RustCareTheme::TEXT_PRIMARY),
                );
                ui.label(
                    RichText::new("Detén servicios innecesarios y optimiza los hilos de CPU/RAM para máxima respuesta.")
                        .color(RustCareTheme::TEXT_MUTED),
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.is_boost_active {
                    RustCareTheme::draw_badge(
                        ui,
                        "🚀 TURBO BOOST ACTIVADO",
                        RustCareTheme::NEON_TEAL,
                        Color32::from_rgba_premultiplied(0, 229, 168, 30),
                    );
                } else {
                    RustCareTheme::draw_badge(
                        ui,
                        "⏸️ MODO NORMAL",
                        RustCareTheme::TEXT_MUTED,
                        Color32::from_rgba_premultiplied(100, 116, 139, 20),
                    );
                }
            });
        });

        ui.add_space(14.0);

        // ==========================================
        // HERO PANEL: INTERRUPTOR GIGANTE TURBO BOOST + 3 MODOS
        // ==========================================
        RustCareTheme::card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                // Columna izquierda: Interruptor Gigante ON/OFF
                ui.allocate_ui_with_layout(Vec2::new(260.0, 160.0), egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("ESTADO DEL ACELERADOR")
                            .size(11.0)
                            .strong()
                            .color(RustCareTheme::TEXT_MUTED),
                    );
                    ui.add_space(6.0);

                    // Botón de palanca gigante interactivo
                    let switch_size = Vec2::new(140.0, 56.0);
                    let (rect, response) = ui.allocate_exact_size(switch_size, Sense::click());
                    let painter = ui.painter();

                    let is_active = self.is_boost_active;
                    let bg_color = if is_active {
                        RustCareTheme::NEON_TEAL
                    } else {
                        Color32::from_rgb(34, 43, 62)
                    };

                    painter.rect_filled(rect, CornerRadius::same(28), bg_color);
                    painter.rect_stroke(rect, CornerRadius::same(28), Stroke::new(1.5, RustCareTheme::CARD_BORDER), StrokeKind::Inside);

                    let knob_radius = 22.0;
                    let knob_x = if is_active {
                        rect.max.x - knob_radius - 6.0
                    } else {
                        rect.min.x + knob_radius + 6.0
                    };
                    let knob_center = Pos2::new(knob_x, rect.center().y);

                    painter.circle_filled(knob_center, knob_radius, Color32::WHITE);
                    painter.circle_stroke(knob_center, knob_radius, Stroke::new(1.0, Color32::from_rgb(200, 200, 200)));

                    let state_text = if is_active { "ON" } else { "OFF" };
                    let text_x = if is_active { rect.min.x + 32.0 } else { rect.max.x - 32.0 };
                    painter.text(
                        Pos2::new(text_x, rect.center().y),
                        Align2::CENTER_CENTER,
                        state_text,
                        FontId::proportional(16.0),
                        if is_active { Color32::BLACK } else { RustCareTheme::TEXT_MUTED },
                    );

                    if response.clicked() {
                        if self.is_boost_active {
                            let _ = backend.disable_turbo_boost();
                            self.boost_result = None;
                        } else {
                            let res = backend.apply_turbo_boost(self.selected_mode);
                            self.boost_result = Some(res);
                        }
                        self.is_boost_active = backend.is_turbo_boost_active();
                    }

                    ui.add_space(8.0);
                    let status_desc = if is_active {
                        "Modo Turbo activo. Rendimiento maximizado."
                    } else {
                        "Pulsa el interruptor para acelerar."
                    };
                    ui.label(RichText::new(status_desc).size(11.0).color(if is_active { RustCareTheme::NEON_TEAL } else { RustCareTheme::TEXT_MUTED }));
                });

                ui.separator();
                ui.add_space(8.0);

                // Columna derecha: 3 Modos Interactivas de Turbo Boost
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("SELECCIONA EL PERFIL DE OPTIMIZACIÓN:")
                            .size(12.0)
                            .strong()
                            .color(RustCareTheme::TEXT_MUTED),
                    );
                    ui.add_space(6.0);

                    let card_width = (ui.available_width() - 16.0) / 3.0;

                    ui.horizontal(|ui| {
                        // MODO 1: Modo Trabajo
                        let mode_card = |ui: &mut Ui, mode: TurboBoostMode, title: &str, subtitle: &str, details: &str, current_mode: TurboBoostMode| -> bool {
                            let is_selected = current_mode == mode;
                            let desired_size = Vec2::new(card_width, 95.0);
                            let (rect, resp) = ui.allocate_exact_size(desired_size, Sense::click());
                            let painter = ui.painter();

                            let fill_col = if is_selected {
                                Color32::from_rgb(28, 42, 64)
                            } else if resp.hovered() {
                                Color32::from_rgb(26, 34, 50)
                            } else {
                                Color32::from_rgb(20, 26, 38)
                            };

                            let border_col = if is_selected {
                                RustCareTheme::NEON_TEAL
                            } else {
                                RustCareTheme::CARD_BORDER
                            };

                            painter.rect_filled(rect, CornerRadius::same(8), fill_col);
                            painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(if is_selected { 2.0 } else { 1.0 }, border_col), StrokeKind::Inside);

                            let text_start_x = rect.min.x + 10.0;
                            painter.text(
                                Pos2::new(text_start_x, rect.min.y + 16.0),
                                Align2::LEFT_TOP,
                                title,
                                FontId::proportional(14.0),
                                if is_selected { RustCareTheme::NEON_TEAL } else { Color32::WHITE },
                            );
                            painter.text(
                                Pos2::new(text_start_x, rect.min.y + 38.0),
                                Align2::LEFT_TOP,
                                subtitle,
                                FontId::proportional(11.0),
                                RustCareTheme::TEXT_MUTED,
                            );
                            painter.text(
                                Pos2::new(text_start_x, rect.min.y + 68.0),
                                Align2::LEFT_TOP,
                                details,
                                FontId::proportional(10.5),
                                RustCareTheme::ACCENT_CYAN,
                            );

                            resp.clicked()
                        };

                        if mode_card(ui, TurboBoostMode::Work, "💼 Modo Trabajo", "Productividad y Ofimática", "4 serv. • ~180 MB liberados", self.selected_mode) {
                            self.selected_mode = TurboBoostMode::Work;
                            if self.is_boost_active {
                                self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                            }
                        }

                        if mode_card(ui, TurboBoostMode::Game, "🎮 Modo Juego", "Máximos FPS y Baja Latencia", "6 serv. • ~350 MB liberados", self.selected_mode) {
                            self.selected_mode = TurboBoostMode::Game;
                            if self.is_boost_active {
                                self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                            }
                        }

                        if mode_card(ui, TurboBoostMode::Economy, "🌱 Ahorro de Energía", "Bajo consumo en portátiles", "3 serv. • ~120 MB liberados", self.selected_mode) {
                            self.selected_mode = TurboBoostMode::Economy;
                            if self.is_boost_active {
                                self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                            }
                        }
                    });

                    if let Some(res) = &self.boost_result {
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(&res.message)
                                .size(12.0)
                                .strong()
                                .color(RustCareTheme::NEON_TEAL),
                        );
                    }
                });
            });
        });

        ui.add_space(12.0);

        // ==========================================
        // FILA INFERIOR: LIBERADOR DE RAM + FLUSH DNS
        // ==========================================
        let half_width = (ui.available_width() - 16.0) / 2.0;

        ui.horizontal(|ui| {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(half_width);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("🧠 Aceleración de Memoria RAM")
                            .strong()
                            .size(15.0)
                            .color(RustCareTheme::ACCENT_CYAN),
                    );
                    ui.label(RichText::new("Vaciado nativo de Working Sets para liberar memoria no reclamada.").small().color(RustCareTheme::TEXT_MUTED));
                    ui.add_space(6.0);

                    let metrics = backend.get_metrics();
                    let total = metrics.memory_total_bytes;
                    let used = metrics.memory_used_bytes;
                    let pct = metrics.memory_usage_percent;

                    ui.add(ProgressBar::new(pct / 100.0).text(format!("{:.1}% en uso ({} de {})", pct, CleanerEngine::format_bytes(used), CleanerEngine::format_bytes(total))).corner_radius(CornerRadius::same(6)));
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [180.0, 36.0],
                                egui::Button::new(
                                    RichText::new("🚀 Liberar RAM Ahora")
                                        .strong()
                                        .color(Color32::WHITE),
                                )
                                .fill(RustCareTheme::ACCENT_BLUE)
                                .corner_radius(CornerRadius::same(8)),
                            )
                            .clicked()
                        {
                            let res = backend.clean_ram();
                            self.ram_result = Some(res);
                        }

                        if let Some(res) = &self.ram_result {
                            RustCareTheme::draw_badge(
                                ui,
                                &format!("✔ {} procesos optimizados", res.processes_cleaned),
                                RustCareTheme::NEON_TEAL,
                                Color32::from_rgba_premultiplied(0, 229, 168, 25),
                            );
                        }
                    });
                });
            });

            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(half_width);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("🌐 Acelerador de Red & DNS")
                            .strong()
                            .size(15.0)
                            .color(RustCareTheme::ACCENT_CYAN),
                    );
                    ui.label(RichText::new("Vacía la caché DNS para resolver problemas de conexión y latencia.").small().color(RustCareTheme::TEXT_MUTED));
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [180.0, 36.0],
                                egui::Button::new(
                                    RichText::new("🔄 Vaciar Caché DNS")
                                        .strong()
                                        .color(Color32::WHITE),
                                )
                                .fill(RustCareTheme::CARD_HOVER_BG)
                                .corner_radius(CornerRadius::same(8)),
                            )
                            .clicked()
                        {
                            let ok = rustcare_core::booster::BoosterEngine::flush_dns_cache();
                            self.dns_message = Some(if ok {
                                "Caché DNS vaciada con éxito.".into()
                            } else {
                                "No se pudo vaciar la caché DNS.".into()
                            });
                        }

                        if let Some(msg) = &self.dns_message {
                            RustCareTheme::draw_badge(
                                ui,
                                msg,
                                RustCareTheme::NEON_TEAL,
                                Color32::from_rgba_premultiplied(0, 229, 168, 25),
                            );
                        }
                    });
                });
            });
        });

        ui.add_space(14.0);
        ui.separator();
        ui.add_space(8.0);

        // ==========================================
        // GESTOR DE ARRANQUE CONECTADO AL BACKEND
        // ==========================================
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("🚀 Gestor de Programas de Inicio")
                    .size(16.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new("🔄 Actualizar")
                            .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    self.startup_entries = backend.list_startup_entries();
                }
            });
        });
        ui.label(
            RichText::new("Desactiva programas de inicio innecesarios para reducir el tiempo de encendido del equipo.")
                .small()
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.add_space(6.0);

        ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
            RustCareTheme::card_frame().show(ui, |ui| {
                if self.startup_entries.is_empty() {
                    ui.label("No se detectaron programas en el inicio.");
                } else {
                    let mut toggles: Vec<(String, bool)> = Vec::new();

                    for entry in &self.startup_entries {
                        ui.horizontal(|ui| {
                            let status_color = if entry.is_enabled {
                                RustCareTheme::NEON_TEAL
                            } else {
                                RustCareTheme::TEXT_MUTED
                            };

                            let status_bg = if entry.is_enabled {
                                Color32::from_rgba_premultiplied(0, 229, 168, 25)
                            } else {
                                Color32::from_rgba_premultiplied(100, 116, 139, 25)
                            };

                            RustCareTheme::draw_badge(
                                ui,
                                if entry.is_enabled { "Habilitado" } else { "Deshabilitado" },
                                status_color,
                                status_bg,
                            );

                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(&entry.name)
                                        .strong()
                                        .color(RustCareTheme::TEXT_PRIMARY),
                                );
                                ui.label(
                                    RichText::new(&entry.command)
                                        .small()
                                        .color(RustCareTheme::TEXT_DIM),
                                );
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let btn_label = if entry.is_enabled { "Deshabilitar" } else { "Habilitar" };
                                let btn_color = if entry.is_enabled {
                                    RustCareTheme::WARNING_AMBER
                                } else {
                                    RustCareTheme::SUCCESS_GREEN
                                };

                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(btn_label).small().color(Color32::WHITE),
                                        )
                                        .fill(btn_color)
                                        .corner_radius(CornerRadius::same(6)),
                                    )
                                    .clicked()
                                {
                                    toggles.push((entry.id.clone(), !entry.is_enabled));
                                }
                            });
                        });
                        ui.separator();
                    }

                    for (id, new_state) in toggles {
                        let _ = backend.toggle_startup_entry(&id, new_state);
                        self.startup_entries = backend.list_startup_entries();
                    }
                }
            });
        });
    }
}
