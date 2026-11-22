//! Vista de Seguridad y Antivirus (RustCare Shield) - 2026 Edition
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, CornerRadius, ProgressBar, RichText, ScrollArea, Ui};
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
            ui.heading(
                RichText::new("RustCare Shield - Seguridad & Malware")
                    .size(22.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !self.is_scanning {
                    if RustCareTheme::custom_button(
                        ui,
                        "Escanear Amenazas",
                        RustCareTheme::ACCENT_TEAL,
                        RustCareTheme::ACCENT_TEAL,
                        Color32::from_rgba_premultiplied(0, 229, 168, 20),
                        32.0,
                    ).clicked() {
                        self.is_scanning = true;
                        on_scan();
                    }
                }
            });
        });

        ui.add_space(6.0);
        ui.label(
            RichText::new("Auditoría heurística: detecta binarios ejecutándose desde %TEMP%, suplantadores de svchost.exe y secuestros de depuración (IFEO / AppInit_DLLs).")
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.separator();
        ui.add_space(10.0);

        if self.is_scanning {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.add(ProgressBar::new(0.6).animate(true).corner_radius(CornerRadius::same(6)));
                ui.add_space(10.0);
                ui.label(
                    RichText::new("Inspeccionando memoria de procesos y persistencias del registro...")
                        .size(15.0)
                        .strong()
                        .color(RustCareTheme::ACCENT_CYAN),
                );
            });
            return;
        }

        if let Some(res) = &self.scan_result {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("{} Procesos Verificados", res.processes_scanned),
                        RustCareTheme::ACCENT_CYAN,
                        Color32::from_rgba_premultiplied(0, 229, 255, 25),
                    );
                    RustCareTheme::draw_badge(
                        ui,
                        &format!("{} Claves de Registro Auditadas", res.registry_keys_scanned),
                        RustCareTheme::TEXT_PRIMARY,
                        Color32::from_rgba_premultiplied(148, 163, 184, 25),
                    );
                });
            });
            ui.add_space(10.0);

            if res.threats.is_empty() {
                RustCareTheme::hero_frame(false).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new("✅ Sistema Seguro y Protegido")
                                .size(20.0)
                                .strong()
                                .color(RustCareTheme::SUCCESS_GREEN),
                        );
                        ui.add_space(6.0);
                        ui.label("No se han detectado procesos maliciosos en carpetas temporales ni secuestros en el registro.");
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

                        let badge_label = match threat.level {
                            ThreatLevel::Critical => "Crítica",
                            ThreatLevel::High => "Alta",
                            ThreatLevel::Warning => "Advertencia",
                        };

                        RustCareTheme::card_frame().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                RustCareTheme::draw_badge(
                                    ui,
                                    badge_label,
                                    badge_color,
                                    Color32::from_rgba_premultiplied(badge_color.r(), badge_color.g(), badge_color.b(), 25),
                                );
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(&threat.title)
                                            .strong()
                                            .color(RustCareTheme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new(&threat.description)
                                            .small()
                                            .color(RustCareTheme::TEXT_MUTED),
                                    );
                                    ui.label(
                                        RichText::new(format!("Ruta: {}", threat.path.display()))
                                            .small()
                                            .color(RustCareTheme::TEXT_DIM),
                                    );
                                });
                            });
                        });
                        ui.add_space(6.0);
                    }
                });
            }
        } else {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("🛡️ Escudo de Protección en Tiempo Real y Heurística")
                            .size(17.0)
                            .strong()
                            .color(RustCareTheme::TEXT_PRIMARY),
                    );
                    ui.add_space(6.0);
                    ui.label("Haz clic en 'Escanear Amenazas' para iniciar una verificación completa de memoria y persistencias.");
                });
            });
        }
    }
}
