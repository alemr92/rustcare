//! Vista de Limpieza y Privacidad Granular
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
                // Por defecto seleccionamos todos los que no requieran admin obligatorio
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
            ui.heading(RichText::new("🧹 Limpieza & Privacidad Granular").size(22.0).color(RustCareTheme::TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(res) = &self.scan_result {
                    if res.total_bytes > 0 {
                        if ui
                            .add_sized(
                                [160.0, 36.0],
                                egui::Button::new(RichText::new("🗑️ Limpiar Ahora").strong().color(Color32::WHITE))
                                    .fill(RustCareTheme::DANGER_RED),
                            )
                            .clicked()
                        {
                            on_clean();
                        }
                    }
                }

                if ui
                    .add_sized(
                        [140.0, 36.0],
                        egui::Button::new(RichText::new("🔍 Escanear").strong().color(Color32::WHITE))
                            .fill(RustCareTheme::ACCENT_BLUE),
                    )
                    .clicked()
                {
                    let targets: Vec<String> = self.selected_targets.iter().cloned().collect();
                    on_scan(targets);
                }
            });
        });

        ui.add_space(8.0);
        ui.label(RichText::new("Selecciona de forma personalizada qué elementos del sistema, navegadores o cachés dev deseas limpiar.").color(RustCareTheme::TEXT_MUTED));
        ui.separator();

        if let Some(res) = &self.scan_result {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Archivos encontrados: {} | Espacio recuperable: {}",
                        res.total_files,
                        CleanerEngine::format_bytes(res.total_bytes)
                    ))
                    .strong()
                    .color(RustCareTheme::WARNING_AMBER),
                );
            });
            ui.add_space(5.0);
        }

        if let Some(clean_res) = &self.clean_result {
            ui.label(
                RichText::new(format!(
                    "¡Limpieza completada! Se liberaron {} ({} archivos eliminados).",
                    CleanerEngine::format_bytes(clean_res.total_bytes_freed),
                    clean_res.total_files_deleted
                ))
                .strong()
                .color(RustCareTheme::SUCCESS_GREEN),
            );
            ui.add_space(5.0);
        }

        ScrollArea::vertical().show(ui, |ui| {
            for group in &self.config.groups {
                ui.add_space(10.0);
                ui.label(RichText::new(&group.name).size(16.0).strong().color(RustCareTheme::ACCENT_CYAN));
                ui.label(RichText::new(&group.description).small().color(RustCareTheme::TEXT_MUTED));
                ui.add_space(6.0);

                egui::Frame::NONE
                    .fill(RustCareTheme::CARD_BG)
                    .corner_radius(8)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
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
                                    ui.label(RichText::new("[Admin]").small().color(RustCareTheme::WARNING_AMBER));
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if let Some(res) = &self.scan_result {
                                        if let Some(tr) = res.targets.iter().find(|t| t.target_id == target.id) {
                                            if tr.skipped_unprivileged {
                                                ui.label(RichText::new("Requiere Admin").small().color(RustCareTheme::TEXT_MUTED));
                                            } else {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{} ({} archivos)",
                                                        CleanerEngine::format_bytes(tr.total_bytes),
                                                        tr.file_count
                                                    ))
                                                    .strong()
                                                    .color(if tr.total_bytes > 0 { RustCareTheme::WARNING_AMBER } else { RustCareTheme::TEXT_MUTED }),
                                                );
                                            }
                                        }
                                    } else {
                                        ui.label(RichText::new(&target.description).small().color(RustCareTheme::TEXT_MUTED));
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
