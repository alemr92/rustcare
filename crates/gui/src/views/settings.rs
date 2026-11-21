//! Vista de Configuración, Automatización y Notificaciones ("AutoCare")
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, CornerRadius, RichText, ScrollArea, Slider, Ui};
use rustcare_core::automation::AutomationEngine;
use rustcare_core::config::{ConfigManager, UserPreferences};

pub struct SettingsView {
    pub prefs: UserPreferences,
    pub status_message: Option<String>,
}

impl Default for SettingsView {
    fn default() -> Self {
        let mut prefs = ConfigManager::load();
        prefs.auto_start_with_system = AutomationEngine::is_autostart_enabled();

        Self {
            prefs,
            status_message: None,
        }
    }
}

impl SettingsView {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.heading(
            RichText::new("⚙️ Configuración, Automatización y Notificaciones")
                .size(22.0)
                .strong()
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.label(
            RichText::new("Personaliza el comportamiento en segundo plano de RustCare: auto-limpieza, liberador de RAM inteligente y avisos de sistema.")
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.separator();
        ui.add_space(10.0);

        if let Some(msg) = &self.status_message {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    RustCareTheme::draw_badge(
                        ui,
                        msg,
                        RustCareTheme::SUCCESS_GREEN,
                        Color32::from_rgba_premultiplied(16, 185, 129, 25),
                    );
                });
            });
            ui.add_space(10.0);
        }

        ScrollArea::vertical().show(ui, |ui| {
            // Sección 1: Inicio y Segundo Plano
            ui.label(
                RichText::new("🚀 INICIO Y SEGUNDO PLANO")
                    .size(13.0)
                    .strong()
                    .color(RustCareTheme::ACCENT_CYAN),
            );
            ui.add_space(6.0);

            RustCareTheme::card_frame().show(ui, |ui| {
                ui.vertical(|ui| {
                    let mut autostart = self.prefs.auto_start_with_system;
                    if ui.checkbox(&mut autostart, "Iniciar RustCare automáticamente con el sistema").changed() {
                        self.prefs.auto_start_with_system = autostart;
                        let _ = AutomationEngine::set_autostart(autostart);
                        self.status_message = Some(if autostart {
                            "Inicio automático habilitado correctamente.".to_string()
                        } else {
                            "Inicio automático deshabilitado.".to_string()
                        });
                    }
                    ui.label(
                        RichText::new("Inicia minimizado para monitorizar recursos y aplicar mantenimiento en segundo plano sin ralentizar el arranque.")
                            .small()
                            .color(RustCareTheme::TEXT_MUTED),
                    );
                });
            });

            ui.add_space(16.0);

            // Sección 2: Optimización Inteligente de Memoria RAM
            ui.label(
                RichText::new("🧠 OPTIMIZACIÓN INTELIGENTE DE MEMORIA RAM")
                    .size(13.0)
                    .strong()
                    .color(RustCareTheme::ACCENT_CYAN),
            );
            ui.add_space(6.0);

            RustCareTheme::card_frame().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut self.prefs.auto_ram_boost_enabled, "Vaciado automático de memoria RAM en segundo plano");
                    ui.label(
                        RichText::new("Aplica K32EmptyWorkingSet de forma automática y silenciosa cuando la memoria supere el umbral establecido.")
                            .small()
                            .color(RustCareTheme::TEXT_MUTED),
                    );
                    ui.add_space(8.0);

                    if self.prefs.auto_ram_boost_enabled {
                        ui.horizontal(|ui| {
                            ui.label("Umbral de activación de RAM:");
                            ui.add(Slider::new(&mut self.prefs.auto_ram_threshold_percent, 50.0..=95.0).suffix("% de uso"));
                        });
                    }
                });
            });

            ui.add_space(16.0);

            // Sección 3: Alertas y Auto-Limpieza de Basura
            ui.label(
                RichText::new("🧹 AUTO-LIMPIEZA & VOLUMEN DE BASURA")
                    .size(13.0)
                    .strong()
                    .color(RustCareTheme::ACCENT_CYAN),
            );
            ui.add_space(6.0);

            RustCareTheme::card_frame().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut self.prefs.auto_clean_enabled, "Avisar o limpiar automáticamente cuando se acumule basura");
                    ui.label(
                        RichText::new("Inspecciona cachés temporales y archivos residuales periódicamente sin molestar al usuario.")
                            .small()
                            .color(RustCareTheme::TEXT_MUTED),
                    );
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Umbral de basura acumulada:");
                        ui.add(Slider::new(&mut self.prefs.auto_clean_threshold_mb, 256..=4096).suffix(" MB"));
                    });
                });
            });

            ui.add_space(16.0);

            // Sección 4: Notificaciones de Escritorio
            ui.label(
                RichText::new("🔔 NOTIFICACIONES DEL SISTEMA")
                    .size(13.0)
                    .strong()
                    .color(RustCareTheme::ACCENT_CYAN),
            );
            ui.add_space(6.0);

            RustCareTheme::card_frame().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut self.prefs.notifications_enabled, "Habilitar notificaciones de escritorio");
                    ui.add_space(4.0);

                    if self.prefs.notifications_enabled {
                        ui.checkbox(&mut self.prefs.notify_on_updates, "Avisar cuando haya actualizaciones de software pendientes (WinGet / Flatpak / Brew)");
                        ui.checkbox(&mut self.prefs.notify_on_high_junk, "Avisar cuando el volumen de archivos basura supere el umbral configurado");
                        ui.checkbox(&mut self.prefs.notify_on_drivers, "Avisar cuando se detecten controladores desactualizados o anómalos");
                        ui.add_space(8.0);

                        if ui
                            .add(
                                egui::Button::new("🔔 Probar Notificación de Escritorio")
                                    .fill(RustCareTheme::ACCENT_BLUE)
                                    .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            let sent = AutomationEngine::send_notification(
                                "RustCare 2026",
                                "¡El sistema de notificaciones en tiempo real está configurado y funcionando!",
                            );
                            self.status_message = Some(if sent {
                                "Notificación enviada al sistema con éxito.".to_string()
                            } else {
                                "La notificación se ejecutó (puede requerir permisos en el centro de control).".to_string()
                            });
                        }
                    }
                });
            });

            ui.add_space(20.0);

            // Botón de Guardado
            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        [220.0, 44.0],
                        egui::Button::new(
                            RichText::new("💾 Guardar Configuración")
                                .size(16.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(RustCareTheme::SUCCESS_GREEN)
                        .corner_radius(CornerRadius::same(8)),
                    )
                    .clicked()
                {
                    if let Ok(()) = ConfigManager::save(&self.prefs) {
                        self.status_message = Some("Preferencias guardadas correctamente en disco.".to_string());
                    }
                }
            });
        });
    }
}
