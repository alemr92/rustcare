//! Vista de Limpieza y Privacidad Granular - 2026 Edition
//! Inspirada en la transparencia y claridad de Roomy (Explicaciones en lenguaje humano y niveles de seguridad)
use crate::theme::RustCareTheme;
use crate::widgets::{checkbox_item, danger_button, ghost_button, neon_button};
use eframe::egui::{self, Color32, RichText, ScrollArea, Ui};
use rustcare_core::cleaner::{CleanerEngine, FullCleanResult, FullScanResult};
use rustcare_core::rules::CleanerConfig;
use rustcare_core::security::is_elevated;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanerFilterTab {
    All,
    SystemJunk,
    BrowserVacuum,
    WinSxsDism,
    EmptyFoldersShortcuts,
}

pub struct CleanerView {
    pub config: CleanerConfig,
    pub selected_targets: std::collections::HashSet<String>,
    pub scan_result: Option<FullScanResult>,
    pub clean_result: Option<FullCleanResult>,
    pub is_scanning: bool,
    pub is_cleaning: bool,
    pub show_explanations: bool,
    pub current_tab: CleanerFilterTab,
}

impl CleanerView {
    pub fn new(config: CleanerConfig) -> Self {
        let mut selected = std::collections::HashSet::new();
        for group in &config.groups {
            for target in &group.targets {
                let is_safe = target.safety_level == "safe" || target.safety_level == "recommended";
                let can_run = !target.requires_admin || is_elevated();
                if is_safe && can_run {
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
            show_explanations: true,
            current_tab: CleanerFilterTab::All,
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
                        if danger_button(ui, "Limpiar Selección", Some(140.0)).clicked() {
                            on_clean();
                        }
                    }
                }

                if neon_button(ui, "Escaneo Paralelo", Some(135.0)).clicked() {
                    let targets: Vec<String> = self.selected_targets.iter().cloned().collect();
                    on_scan(targets);
                }

                if ghost_button(ui, "Marcar Todo", Some(95.0)).clicked() {
                    for group in &self.config.groups {
                        for target in &group.targets {
                            if !target.requires_admin || is_elevated() {
                                self.selected_targets.insert(target.id.clone());
                            }
                        }
                    }
                }

                if ghost_button(ui, "Solo Seguros", Some(95.0)).clicked() {
                    self.selected_targets.clear();
                    for group in &self.config.groups {
                        for target in &group.targets {
                            let is_safe = target.safety_level == "safe" || target.safety_level == "recommended";
                            let can_run = !target.requires_admin || is_elevated();
                            if is_safe && can_run {
                                self.selected_targets.insert(target.id.clone());
                            }
                        }
                    }
                }

                if ghost_button(ui, "Desmarcar", Some(85.0)).clicked() {
                    self.selected_targets.clear();
                }
            });
        });

        ui.add_space(4.0);
        // Barra de Subpestañas Inteligentes de Limpieza
        ui.horizontal(|ui| {
            let tabs = [
                (CleanerFilterTab::All, "Todo en Paralelo"),
                (CleanerFilterTab::SystemJunk, "Basura del Sistema"),
                (CleanerFilterTab::BrowserVacuum, "Navegadores (Vacuum)"),
                (CleanerFilterTab::WinSxsDism, "WinSxS / DISM"),
                (CleanerFilterTab::EmptyFoldersShortcuts, "Carpetas & Accesos Rotos"),
            ];

            for (tab, label) in tabs {
                let is_sel = self.current_tab == tab;
                let btn_lbl = if is_sel { format!("[{}]", label) } else { label.to_string() };
                if ghost_button(ui, &btn_lbl, None).clicked() {
                    self.current_tab = tab;
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let toggle_txt = if self.show_explanations { "Ocultar Guía" } else { "Mostrar Guía" };
                if ghost_button(ui, toggle_txt, Some(110.0)).clicked() {
                    self.show_explanations = !self.show_explanations;
                }
            });
        });

        ui.separator();
        ui.add_space(8.0);

        if let Some(res) = &self.scan_result {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("{} Archivos Basura", res.total_files),
                        RustCareTheme::WARNING_AMBER,
                        Color32::from_rgba_unmultiplied(245, 158, 11, 25),
                    );
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("{} Recuperables", CleanerEngine::format_bytes(res.total_bytes)),
                        RustCareTheme::ACCENT_CYAN,
                        Color32::from_rgba_unmultiplied(56, 189, 248, 25),
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
                        &format!("{} Liberados", CleanerEngine::format_bytes(clean_res.total_bytes_freed)),
                        RustCareTheme::SUCCESS_GREEN,
                        Color32::from_rgba_unmultiplied(16, 185, 129, 25),
                    );
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("{} Archivos Eliminados", clean_res.total_files_deleted),
                        RustCareTheme::TEXT_PRIMARY,
                        Color32::from_rgba_unmultiplied(148, 163, 184, 25),
                    );
                });
            });
            ui.add_space(8.0);
        }

        ScrollArea::vertical().show(ui, |ui| {
            for group in &self.config.groups {
                let show_group = match self.current_tab {
                    CleanerFilterTab::All => true,
                    CleanerFilterTab::SystemJunk => {
                        let n = group.name.to_lowercase();
                        n.contains("sistema") || n.contains("windows") || n.contains("temp") || n.contains("shader") || n.contains("caché") || n.contains("cache")
                    }
                    CleanerFilterTab::BrowserVacuum => {
                        let n = group.name.to_lowercase();
                        n.contains("navegador") || n.contains("browser") || n.contains("chrome") || n.contains("edge") || n.contains("sqlite")
                    }
                    CleanerFilterTab::WinSxsDism => {
                        let n = group.name.to_lowercase();
                        n.contains("winsxs") || n.contains("dism") || n.contains("actualiz") || n.contains("paquete")
                    }
                    CleanerFilterTab::EmptyFoldersShortcuts => {
                        let n = group.name.to_lowercase();
                        n.contains("carpeta") || n.contains("acceso") || n.contains("directo") || n.contains("huérfan") || n.contains("vaci")
                    }
                };
                if !show_group {
                    continue;
                }

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
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let mut is_checked = self.selected_targets.contains(&target.id);
                                if checkbox_item(ui, &mut is_checked, &target.name).changed() {
                                    if is_checked {
                                        self.selected_targets.insert(target.id.clone());
                                    } else {
                                        self.selected_targets.remove(&target.id);
                                    }
                                }

                                // Insignia de seguridad estilo Roomy
                                match target.safety_level.as_str() {
                                    "safe" => {
                                        RustCareTheme::draw_badge(
                                            ui,
                                            "Seguro",
                                            RustCareTheme::ACCENT_EMERALD,
                                            Color32::from_rgba_unmultiplied(16, 185, 129, 25),
                                        );
                                    }
                                    "recommended" => {
                                        RustCareTheme::draw_badge(
                                            ui,
                                            "Recomendado",
                                            RustCareTheme::ACCENT_CYAN,
                                            Color32::from_rgba_unmultiplied(56, 189, 248, 25),
                                        );
                                    }
                                    _ => {
                                        RustCareTheme::draw_badge(
                                            ui,
                                            "Opcional",
                                            RustCareTheme::ACCENT_AMBER,
                                            Color32::from_rgba_unmultiplied(245, 158, 11, 25),
                                        );
                                    }
                                }

                                if target.requires_admin {
                                    RustCareTheme::draw_badge(
                                        ui,
                                        "Admin",
                                        RustCareTheme::ACCENT_AMBER,
                                        Color32::from_rgba_unmultiplied(245, 158, 11, 25),
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
                                                    Color32::from_rgba_unmultiplied(100, 116, 139, 25),
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
                                                    Color32::from_rgba_unmultiplied(245, 158, 11, 20),
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

                            // Ficha explicativa en lenguaje claro estilo Roomy (desplegable con el botón de Guía)
                            if self.show_explanations && (!target.why_it_exists.is_empty() || !target.safe_to_delete_reason.is_empty()) {
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(26.0);
                                    ui.vertical(|ui| {
                                        if !target.why_it_exists.is_empty() {
                                            ui.label(
                                                RichText::new(format!("Por que existe: {}", target.why_it_exists))
                                                    .size(11.0)
                                                    .color(RustCareTheme::TEXT_MUTED),
                                            );
                                        }
                                        if !target.safe_to_delete_reason.is_empty() {
                                            ui.label(
                                                RichText::new(format!("Seguridad: {}", target.safe_to_delete_reason))
                                                    .size(11.0)
                                                    .color(RustCareTheme::ACCENT_TEAL),
                                            );
                                        }
                                    });
                                });
                                ui.add_space(4.0);
                            }
                        });
                        ui.separator();
                    }
                });
            }
        });
    }
}
