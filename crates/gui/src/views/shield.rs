//! Vista de Seguridad y Antivirus (RustCare Shield)
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, ProgressBar, RichText, ScrollArea, Ui};
use rustcare_core::shield::{ShieldScanResult, ThreatLevel};

pub struct ShieldView {
    pub scan_result: Option<ShieldScanResult>,
    pub is_scanning: bool,
}

impl Default for ShieldView {
    fn default() -> Self {
        Self {
            scan_result: None,
            is_scanning: false,
        }
    }
}

impl ShieldView {
    pub fn render(&mut self, ui: &mut Ui, on_scan: impl FnOnce()) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new("🛡️ RustCare Shield - Seguridad & Malware").size(22.0).color(RustCareTheme::TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !self.is_scanning {
                    if ui
                        .add_sized(
                            [180.0, 36.0],
                            egui::Button::new(RichText::new("🔍 Escanear Amenazas").strong().color(Color32::WHITE))
                                .fill(RustCareTheme::ACCENT_BLUE),
                        )
                        .clicked()
                    {
                        on_scan();
                    }
                }
            });
        });

        ui.add_space(8.0);
        ui.label(RichText::new("Inspecciona procesos en memoria, binarios en ejecución desde carpetas temporales e interceptores de registro (IFEO / AppInit_DLLs).").color(RustCareTheme::TEXT_MUTED));
        ui.separator();
        ui.add_space(10.0);

        if self.is_scanning {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.add(ProgressBar::new(0.5).animate(true));
                ui.add_space(10.0);
                ui.label(RichText::new("Inspeccionando memoria de procesos y persistencias del registro...").size(15.0).color(RustCareTheme::ACCENT_CYAN));
            });
            return;
        }

        if let Some(res) = &self.scan_result {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Procesos analizados: {} | Claves del registro auditadas: {}",
                        res.processes_scanned, res.registry_keys_scanned
                    ))
                    .small()
                    .color(RustCareTheme::TEXT_MUTED),
                );
            });
            ui.add_space(10.0);

            if res.threats.is_empty() {
                egui::Frame::NONE
                    .fill(RustCareTheme::CARD_BG)
                    .corner_radius(8)
                    .inner_margin(20.0)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("✅ Sistema Seguro").size(20.0).strong().color(RustCareTheme::SUCCESS_GREEN));
                            ui.add_space(5.0);
                            ui.label("No se han detectado procesos sospechosos en carpetas temporales ni secuestros en el registro.");
                        });
                    });
            } else {
                ui.label(
                    RichText::new(format!("⚠️ Se han detectado {} alertas de seguridad:", res.threats.len()))
                        .size(16.0)
                        .strong()
                        .color(RustCareTheme::DANGER_RED),
                );
                ui.add_space(8.0);

                ScrollArea::vertical().show(ui, |ui| {
                    for threat in &res.threats {
                        let badge_color = match threat.level {
                            ThreatLevel::Critical => RustCareTheme::DANGER_RED,
                            ThreatLevel::High => RustCareTheme::WARNING_AMBER,
                            ThreatLevel::Warning => Color32::from_rgb(234, 179, 8),
                        };

                        egui::Frame::NONE
                            .fill(RustCareTheme::CARD_BG)
                            .corner_radius(8)
                            .inner_margin(12.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let level_str = match threat.level {
                                        ThreatLevel::Critical => "[CRÍTICO]",
                                        ThreatLevel::High => "[ALTO]",
                                        ThreatLevel::Warning => "[ADVERTENCIA]",
                                    };
                                    ui.label(RichText::new(level_str).strong().color(badge_color));
                                    ui.label(RichText::new(&threat.title).strong().size(15.0));
                                });
                                ui.add_space(4.0);
                                ui.label(RichText::new(&threat.description).small());
                                ui.label(RichText::new(format!("Ruta: {}", threat.path.display())).small().color(RustCareTheme::TEXT_MUTED));
                            });
                        ui.add_space(6.0);
                    }
                });
            }
        } else {
            egui::Frame::NONE
                .fill(RustCareTheme::CARD_BG)
                .corner_radius(8)
                .inner_margin(30.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("🛡️ Escudo Desactivado").size(18.0).color(RustCareTheme::TEXT_MUTED));
                        ui.add_space(8.0);
                        ui.label("Haz clic en 'Escanear Amenazas' para iniciar una auditoría completa del sistema.");
                    });
                });
        }
    }
}
