//! Vista 2: Acelerar (Booster) - Basada en 02_turbo.png con fidelidad pixel-perfect
use crate::icons::VectorIcons;
use crate::theme::RustCareTheme;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, RichText, ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2};
use rustcare_core::backend::{SystemBackend, TurboBoostMode, TurboBoostResult};
use rustcare_core::booster::RamCleanResult;
use rustcare_core::startup::StartupEntry;
use std::sync::Arc;

pub struct BoosterView {
    pub selected_mode: TurboBoostMode,
    pub is_boost_active: bool,
    pub boost_result: Option<TurboBoostResult>,
    pub startup_entries: Vec<StartupEntry>,
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
            ram_result: None,
            dns_message: None,
        }
    }
}

impl BoosterView {
    pub fn render(&mut self, ui: &mut Ui, backend: &Arc<dyn SystemBackend>) {
        // Cargar entradas de inicio si la lista está vacía
        if self.startup_entries.is_empty() {
            self.startup_entries = backend.list_startup_entries();
        }
        self.is_boost_active = backend.is_turbo_boost_active();

        ui.vertical(|ui| {
            // =================================================================
            // 1. TARJETA HERO SUPERIOR (height: 120px, doble borde / glow #00E5A8 cuando ON)
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
                .stroke(Stroke::new(if is_on { 1.5_f32 } else { 1.0_f32 }, hero_border))
                .inner_margin(egui::Margin::symmetric(24, 20))
                .show(ui, |ui| {
                    ui.set_height(80.0);
                    ui.horizontal_centered(|ui| {
                        // Izquierda: Título TURBO BOOST + ESTADO: ACTIVO / INACTIVO
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
                                    .size(14.0)
                                    .strong()
                                    .color(status_col),
                            );
                        });

                        // Derecha: INTERRUPTOR PÍLDORA GIGANTE (130px x 54px)
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let switch_size = Vec2::new(130.0, 54.0);
                            let (switch_rect, switch_resp) = ui.allocate_exact_size(switch_size, Sense::click());
                            let p = ui.painter();

                            let track_fill = if is_on {
                                Color32::from_rgb(14, 38, 48) // Teal oscuro
                            } else {
                                RustCareTheme::BG_ELEVATED
                            };
                            let track_border = if is_on {
                                RustCareTheme::ACCENT_TEAL
                            } else {
                                RustCareTheme::BORDER_SUBTLE
                            };

                            // Pista redondeada (radius: 27px)
                            p.rect_filled(switch_rect, CornerRadius::same(27), track_fill);
                            p.rect_stroke(switch_rect, CornerRadius::same(27), Stroke::new(2.0_f32, track_border), StrokeKind::Inside);

                            // Círculo deslizante (r = 21px)
                            let knob_r = 21.0;
                            let knob_x = if is_on {
                                switch_rect.max.x - knob_r - 6.0
                            } else {
                                switch_rect.min.x + knob_r + 6.0
                            };
                            let knob_center = Pos2::new(knob_x, switch_rect.center().y);

                            let knob_color = if is_on {
                                RustCareTheme::ACCENT_TEAL
                            } else {
                                Color32::from_rgb(45, 58, 78)
                            };
                            p.circle_filled(knob_center, knob_r, knob_color);

                            // Texto ON / OFF
                            let label_text = if is_on { "ON" } else { "OFF" };
                            let label_x = if is_on {
                                switch_rect.min.x + 36.0
                            } else {
                                switch_rect.max.x - 36.0
                            };
                            p.text(
                                Pos2::new(label_x, switch_rect.center().y),
                                Align2::CENTER_CENTER,
                                label_text,
                                FontId::proportional(20.0),
                                if is_on { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::TEXT_MUTED },
                            );

                            if switch_resp.clicked() {
                                if self.is_boost_active {
                                    let _ = backend.disable_turbo_boost();
                                    self.boost_result = None;
                                } else {
                                    let res = backend.apply_turbo_boost(self.selected_mode);
                                    self.boost_result = Some(res);
                                }
                                self.is_boost_active = backend.is_turbo_boost_active();
                            }
                        });
                    });
                });

            ui.add_space(RustCareTheme::ITEM_SPACING);

            // =================================================================
            // 2. FILA MEDIA (3 TARJETAS IGUALES EN GRID DE 3 COLUMNAS, height: 150px)
            // =================================================================
            let avail_w = ui.available_width();
            let card_spacing = RustCareTheme::ITEM_SPACING;
            let col_w = (avail_w - (card_spacing * 2.0)) / 3.0;
            let col_h = 150.0;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = card_spacing;

                // TARJETA 1: MODO TRABAJO
                let is_sel1 = self.selected_mode == TurboBoostMode::Work;
                let border1 = if is_sel1 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE };
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(CornerRadius::same(RustCareTheme::RADIUS_CARD as u8))
                    .stroke(Stroke::new(1.0_f32, border1))
                    .inner_margin(egui::Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(col_w - 32.0);
                        ui.set_height(col_h - 32.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                // Icono cuadrado redondeado 42x42px
                                let (ic_rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
                                ui.painter().rect_filled(ic_rect, CornerRadius::same(8), Color32::from_rgba_premultiplied(0, 229, 168, 30));
                                VectorIcons::draw_briefcase(ui.painter(), ic_rect, RustCareTheme::ACCENT_TEAL);

                                ui.add_space(4.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new("MODO TRABAJO")
                                            .size(13.5)
                                            .strong()
                                            .color(RustCareTheme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new("54 Servicios detenidos")
                                            .size(11.0)
                                            .color(RustCareTheme::TEXT_MUTED),
                                    );
                                });

                                // Punto indicador esquina superior derecha
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
                                    ui.painter().circle_filled(dot_rect.center(), 3.5, if is_sel1 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE });
                                });
                            });

                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("820 MB liberados")
                                    .size(13.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_CYAN),
                            );

                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if RustCareTheme::custom_button(
                                    ui,
                                    if is_sel1 { "Activo" } else { "Activar" },
                                    if is_sel1 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE },
                                    if is_sel1 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::TEXT_MUTED },
                                    Color32::TRANSPARENT,
                                    26.0,
                                ).clicked() {
                                    self.selected_mode = TurboBoostMode::Work;
                                    if self.is_boost_active {
                                        self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                                    }
                                }
                                if RustCareTheme::custom_button(
                                    ui,
                                    "Configurar",
                                    RustCareTheme::BORDER_SUBTLE,
                                    RustCareTheme::TEXT_MUTED,
                                    Color32::TRANSPARENT,
                                    26.0,
                                ).clicked() {}
                            });
                        });
                    });

                // TARJETA 2: MODO JUEGO
                let is_sel2 = self.selected_mode == TurboBoostMode::Game;
                let border2 = if is_sel2 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE };
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(CornerRadius::same(RustCareTheme::RADIUS_CARD as u8))
                    .stroke(Stroke::new(1.0_f32, border2))
                    .inner_margin(egui::Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(col_w - 32.0);
                        ui.set_height(col_h - 32.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let (ic_rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
                                ui.painter().rect_filled(ic_rect, CornerRadius::same(8), Color32::from_rgba_premultiplied(0, 229, 168, 30));
                                VectorIcons::draw_gamepad(ui.painter(), ic_rect, RustCareTheme::ACCENT_TEAL);

                                ui.add_space(4.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new("MODO JUEGO")
                                            .size(13.5)
                                            .strong()
                                            .color(RustCareTheme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new("68 Servicios detenidos")
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
                                RichText::new("1.2 GB liberados")
                                    .size(13.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_TEAL),
                            );

                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if RustCareTheme::custom_button(
                                    ui,
                                    if is_sel2 { "Activo" } else { "Activar" },
                                    if is_sel2 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE },
                                    if is_sel2 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::TEXT_MUTED },
                                    Color32::TRANSPARENT,
                                    26.0,
                                ).clicked() {
                                    self.selected_mode = TurboBoostMode::Game;
                                    if self.is_boost_active {
                                        self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                                    }
                                }
                                if RustCareTheme::custom_button(
                                    ui,
                                    "Configurar",
                                    RustCareTheme::BORDER_SUBTLE,
                                    RustCareTheme::TEXT_MUTED,
                                    Color32::TRANSPARENT,
                                    26.0,
                                ).clicked() {}
                            });
                        });
                    });

                // TARJETA 3: AHORRO ENERGÍA
                let is_sel3 = self.selected_mode == TurboBoostMode::Economy;
                let border3 = if is_sel3 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE };
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(CornerRadius::same(RustCareTheme::RADIUS_CARD as u8))
                    .stroke(Stroke::new(1.0_f32, border3))
                    .inner_margin(egui::Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(col_w - 32.0);
                        ui.set_height(col_h - 32.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let (ic_rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
                                ui.painter().rect_filled(ic_rect, CornerRadius::same(8), Color32::from_rgba_premultiplied(0, 229, 168, 30));
                                VectorIcons::draw_battery(ui.painter(), ic_rect, RustCareTheme::ACCENT_TEAL);

                                ui.add_space(4.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new("AHORRO ENERGÍA")
                                            .size(13.5)
                                            .strong()
                                            .color(RustCareTheme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new("12 Servicios detenidos")
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
                                RichText::new("150 MB liberados")
                                    .size(13.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_AMBER),
                            );

                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if RustCareTheme::custom_button(
                                    ui,
                                    if is_sel3 { "Activo" } else { "Activar" },
                                    if is_sel3 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::BORDER_SUBTLE },
                                    if is_sel3 { RustCareTheme::ACCENT_TEAL } else { RustCareTheme::TEXT_MUTED },
                                    Color32::TRANSPARENT,
                                    26.0,
                                ).clicked() {
                                    self.selected_mode = TurboBoostMode::Economy;
                                    if self.is_boost_active {
                                        self.boost_result = Some(backend.apply_turbo_boost(self.selected_mode));
                                    }
                                }
                                if RustCareTheme::custom_button(
                                    ui,
                                    "Configurar",
                                    RustCareTheme::BORDER_SUBTLE,
                                    RustCareTheme::TEXT_MUTED,
                                    Color32::TRANSPARENT,
                                    26.0,
                                ).clicked() {}
                            });
                        });
                    });
            });

            ui.add_space(RustCareTheme::ITEM_SPACING);

            // =================================================================
            // 3. PANEL INFERIOR: GESTOR DE INICIO (Tabla Estilizada)
            // =================================================================
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("GESTOR DE INICIO")
                            .size(14.0)
                            .strong()
                            .color(RustCareTheme::TEXT_PRIMARY),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
                        ui.painter().circle_filled(dot_rect.center(), 3.5, RustCareTheme::ACCENT_TEAL);
                    });
                });

                ui.add_space(8.0);

                // Cabecera de la tabla
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.allocate_ui_with_layout(Vec2::new(230.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Programa").size(11.5).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(110.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Impacto de Inicio").size(11.5).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(70.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Estado").size(11.5).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.label(RichText::new("Acción").size(11.5).strong().color(RustCareTheme::TEXT_MUTED));
                });

                ui.add_space(4.0);
                ui.painter().line_segment(
                    [Pos2::new(ui.min_rect().min.x, ui.cursor().min.y), Pos2::new(ui.min_rect().max.x, ui.cursor().min.y)],
                    Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE),
                );
                ui.add_space(6.0);

                // Filas de programas de inicio
                ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                    let mut toggles: Vec<(String, bool)> = Vec::new();

                    for (idx, entry) in self.startup_entries.iter().enumerate() {
                        let row_bg = if idx % 2 == 0 {
                            RustCareTheme::BG_SURFACE
                        } else {
                            RustCareTheme::BG_ELEVATED
                        };

                        egui::Frame::NONE
                            .fill(row_bg)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(egui::Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Mini switch indicador de estado
                                    let (sw_rect, sw_resp) = ui.allocate_exact_size(Vec2::new(26.0, 14.0), Sense::click());
                                    let p = ui.painter();
                                    let sw_color = if entry.is_enabled { RustCareTheme::ACCENT_TEAL } else { Color32::from_rgb(50, 62, 82) };
                                    p.rect_filled(sw_rect, CornerRadius::same(7), if entry.is_enabled { Color32::from_rgb(14, 40, 50) } else { RustCareTheme::BG_VOID });
                                    p.rect_stroke(sw_rect, CornerRadius::same(7), Stroke::new(1.0_f32, sw_color), StrokeKind::Inside);
                                    let k_x = if entry.is_enabled { sw_rect.max.x - 7.0 } else { sw_rect.min.x + 7.0 };
                                    p.circle_filled(Pos2::new(k_x, sw_rect.center().y), 5.0, sw_color);

                                    if sw_resp.clicked() {
                                        toggles.push((entry.id.clone(), !entry.is_enabled));
                                    }

                                    ui.add_space(8.0);
                                    // Nombre del programa (ancho fijo ~200px)
                                    let name_clean = if entry.name.len() > 22 {
                                        format!("{}...", &entry.name[..20])
                                    } else {
                                        entry.name.clone()
                                    };
                                    ui.allocate_ui_with_layout(Vec2::new(190.0, 20.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(name_clean).size(12.5).color(RustCareTheme::TEXT_PRIMARY));
                                    });

                                    ui.add_space(10.0);

                                    // Píldora de Impacto de Inicio
                                    let (imp_text, imp_color) = if idx == 0 {
                                        ("Alto", RustCareTheme::ACCENT_CORAL)
                                    } else if idx == 1 {
                                        ("Medio", RustCareTheme::ACCENT_AMBER)
                                    } else {
                                        ("Bajo", RustCareTheme::TEXT_MUTED)
                                    };
                                    let (imp_rect, _) = ui.allocate_exact_size(Vec2::new(60.0, 20.0), Sense::hover());
                                    ui.painter().rect_filled(imp_rect, CornerRadius::same(10), RustCareTheme::BG_ELEVATED);
                                    ui.painter().rect_stroke(imp_rect, CornerRadius::same(10), Stroke::new(1.0_f32, imp_color), StrokeKind::Inside);
                                    ui.painter().text(imp_rect.center(), Align2::CENTER_CENTER, imp_text, FontId::proportional(11.0), imp_color);

                                    ui.add_space(20.0);

                                    // Mini-toggle switch de estado
                                    let (sw2_rect, sw2_resp) = ui.allocate_exact_size(Vec2::new(26.0, 14.0), Sense::click());
                                    let p = ui.painter();
                                    p.rect_filled(sw2_rect, CornerRadius::same(7), if entry.is_enabled { Color32::from_rgb(14, 40, 50) } else { RustCareTheme::BG_VOID });
                                    p.rect_stroke(sw2_rect, CornerRadius::same(7), Stroke::new(1.0_f32, sw_color), StrokeKind::Inside);
                                    p.circle_filled(Pos2::new(k_x, sw2_rect.center().y), 5.0, sw_color);

                                    if sw2_resp.clicked() {
                                        toggles.push((entry.id.clone(), !entry.is_enabled));
                                    }

                                    ui.add_space(20.0);

                                    // Botón Activar / Desactivar
                                    let btn_label = if entry.is_enabled { "Desactivar" } else { "Activar" };
                                    let btn_color = if entry.is_enabled { RustCareTheme::ACCENT_AMBER } else { RustCareTheme::ACCENT_TEAL };
                                    if RustCareTheme::custom_button(
                                        ui,
                                        btn_label,
                                        RustCareTheme::BORDER_SUBTLE,
                                        btn_color,
                                        RustCareTheme::BG_VOID,
                                        24.0,
                                    ).clicked() {
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
            });
        });
    }
}
