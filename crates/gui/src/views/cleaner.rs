//! Vista de Limpieza y Privacidad Granular - 2026 Edition
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, RichText, ScrollArea, Ui};
use rustcare_core::cleaner::{CleanerEngine, FullCleanResult, FullScanResult};
use rustcare_core::rules::CleanerConfig;
use rustcare_core::security::is_elevated;

pub struct CleanerView {
    pub config: CleanerConfig,
    pub selected_targets: std::collections::HashSet<String>,
    pub scan_result: Option<FullScanResult>,
    pub clean_result: Option<FullCleanResult>,
    pub is_scanning: bool,
    pub is_cleaning: bool,
}

impl CleanerView {
    pub fn new(config: CleanerConfig) -> Self {
        let mut selected = std::collections::HashSet::new();
        for group in &config.groups {
            for target in &group.targets {
                if !target.requires_admin || is_elevated() {
                    selected.insert(target.id.clone());
                }
            }
        }

        Self {
            config,
            selected_targets: selected,
            scan_result: None,
            clean_result: None,
            is_scanning: false,
            is_cleaning: false,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut Ui,
        on_scan: impl FnOnce(Vec<String>),
        on_clean: impl FnOnce(),
    ) {
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new("Limpieza & Privacidad Granular")
                    .size(22.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(res) = &self.scan_result {
                    if res.total_bytes > 0 {
                        if RustCareTheme::custom_button(
                            ui,
                            "Limpiar Selección",
                            RustCareTheme::ACCENT_CORAL,
                            RustCareTheme::ACCENT_CORAL,
                            Color32::from_rgba_premultiplied(255, 77, 109, 20),
                            32.0,
                        ).clicked() {
                            on_clean();
                        }
                    }
                }

                if RustCareTheme::custom_button(
                    ui,
                    "Escanear",
                    RustCareTheme::ACCENT_TEAL,
                    RustCareTheme::ACCENT_TEAL,
                    Color32::from_rgba_premultiplied(0, 229, 168, 20),
                    32.0,
                ).clicked() {
                    let targets: Vec<String> = self.selected_targets.iter().cloned().collect();
                    on_scan(targets);
                }

                if RustCareTheme::custom_button(ui, "Marcar Todo", RustCareTheme::BORDER_SUBTLE, RustCareTheme::TEXT_PRIMARY, RustCareTheme::BG_SURFACE, 28.0).clicked() {
                    for group in &self.config.groups {
                        for target in &group.targets {
                            if !target.requires_admin || is_elevated() {
                                self.selected_targets.insert(target.id.clone());
                            }
                        }
                    }
                }

                if RustCareTheme::custom_button(ui, "Desmarcar", RustCareTheme::BORDER_SUBTLE, RustCareTheme::TEXT_MUTED, RustCareTheme::BG_SURFACE, 28.0).clicked() {
                    self.selected_targets.clear();
                }
            });
        });

        ui.add_space(6.0);
        ui.label(
            RichText::new("Selecciona con precisión los objetivos del sistema operativo, cachés de desarrollo y navegadores web a auditar.")
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.separator();
        ui.add_space(8.0);

        if let Some(res) = &self.scan_result {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("{} Archivos Basura", res.total_files),
                        RustCareTheme::WARNING_AMBER,
                        Color32::from_rgba_premultiplied(245, 158, 11, 25),
                    );
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("{} Recuperables", CleanerEngine::format_bytes(res.total_bytes)),
                        RustCareTheme::ACCENT_CYAN,
                        Color32::from_rgba_premultiplied(0, 229, 255, 25),
                    );
                });
            });
            ui.add_space(8.0);
        }

        if let Some(clean_res) = &self.clean_result {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("✔ {} Liberados", CleanerEngine::format_bytes(clean_res.total_bytes_freed)),
                        RustCareTheme::SUCCESS_GREEN,
                        Color32::from_rgba_premultiplied(16, 185, 129, 25),
                    );
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("{} Archivos Eliminados", clean_res.total_files_deleted),
                        RustCareTheme::TEXT_PRIMARY,
                        Color32::from_rgba_premultiplied(148, 163, 184, 25),
                    );
                });
            });
            ui.add_space(8.0);
        }

        ScrollArea::vertical().show(ui, |ui| {
            for group in &self.config.groups {
                ui.add_space(10.0);
                ui.label(
                    RichText::new(&group.name)
                        .size(16.0)
                        .strong()
                        .color(RustCareTheme::ACCENT_CYAN),
                );
                ui.label(
                    RichText::new(&group.description)
                        .small()
                        .color(RustCareTheme::TEXT_MUTED),
                );
                ui.add_space(6.0);

                RustCareTheme::card_frame().show(ui, |ui| {
                    for target in &group.targets {
                        ui.horizontal(|ui| {
                            let mut is_checked = self.selected_targets.contains(&target.id);
                            if ui.checkbox(&mut is_checked, &target.name).changed() {
                                if is_checked {
                                    self.selected_targets.insert(target.id.clone());
                                } else {
                                    self.selected_targets.remove(&target.id);
                                }
                            }

                            if target.requires_admin {
                                RustCareTheme::draw_badge(
                                    ui,
                                    "Admin",
                                    RustCareTheme::WARNING_AMBER,
                                    Color32::from_rgba_premultiplied(245, 158, 11, 20),
                                );
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if let Some(res) = &self.scan_result {
                                    if let Some(tr) = res.targets.iter().find(|t| t.target_id == target.id) {
                                        if tr.skipped_unprivileged {
                                            RustCareTheme::draw_badge(
                                                ui,
                                                "Omitido (Requiere Admin)",
                                                RustCareTheme::TEXT_DIM,
                                                Color32::from_rgba_premultiplied(100, 116, 139, 20),
                                            );
                                        } else {
                                            let badge_color = if tr.total_bytes > 0 {
                                                RustCareTheme::WARNING_AMBER
                                            } else {
                                                RustCareTheme::TEXT_DIM
                                            };
                                            RustCareTheme::draw_badge(
                                                ui,
                                                &format!(
                                                    "{} ({} archivos)",
                                                    CleanerEngine::format_bytes(tr.total_bytes),
                                                    tr.file_count
                                                ),
                                                badge_color,
                                                Color32::from_rgba_premultiplied(245, 158, 11, 15),
                                            );
                                        }
                                    }
                                } else {
                                    ui.label(
                                        RichText::new(&target.description)
                                            .small()
                                            .color(RustCareTheme::TEXT_MUTED),
                                    );
                                }
                            });
                        });
                        ui.separator();
                    }
                });
            }
        });
    }
}
