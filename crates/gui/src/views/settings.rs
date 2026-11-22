//! Vista de Configuración, Automatización y Notificaciones ("AutoCare") - Estilo ASC Pro
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, CornerRadius, RichText, ScrollArea, Slider, Ui};
use rustcare_core::automation::AutomationEngine;
use rustcare_core::config::{ConfigManager, UserPreferences};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    General,
    Browsers,
    JunkFiles,
    Registry,
    Exclusions,
}

pub struct SettingsView {
    pub prefs: UserPreferences,
    pub current_tab: SettingsTab,
    pub new_exclusion_path: String,
    pub status_message: Option<String>,
}

impl Default for SettingsView {
    fn default() -> Self {
        let mut prefs = ConfigManager::load();
        prefs.auto_start_with_system = AutomationEngine::is_autostart_enabled();

        Self {
            prefs,
            current_tab: SettingsTab::General,
            new_exclusion_path: String::new(),
            status_message: None,
        }
    }
}

impl SettingsView {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.heading(
            RichText::new("⚙️ Centro de Configuración & Opciones PRO")
                .size(22.0)
                .strong()
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.label(
            RichText::new("Ajusta de forma granular qué elementos se limpian en navegadores, registro, temporales y reglas de segundo plano.")
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.add_space(8.0);

        // Barra de Pestañas de Configuración Estilo Advanced SystemCare
        ui.horizontal(|ui| {
            let tabs = [
                (SettingsTab::General, "⚙️ General & AutoCare"),
                (SettingsTab::Browsers, "🌐 Privacidad & Navegadores"),
                (SettingsTab::JunkFiles, "🧹 Archivos Basura"),
                (SettingsTab::Registry, "🗃️ Registro de Windows"),
                (SettingsTab::Exclusions, "🛡️ Lista Blanca & Exclusiones"),
            ];

            for (tab, label) in tabs {
                if ui.selectable_label(self.current_tab == tab, label).clicked() {
                    self.current_tab = tab;
                }
            }
        });
        ui.separator();
        ui.add_space(8.0);

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
            match self.current_tab {
                SettingsTab::General => self.render_general_tab(ui),
                SettingsTab::Browsers => self.render_browsers_tab(ui),
                SettingsTab::JunkFiles => self.render_junk_tab(ui),
                SettingsTab::Registry => self.render_registry_tab(ui),
                SettingsTab::Exclusions => self.render_exclusions_tab(ui),
            }

            ui.add_space(20.0);

            // Botón de Guardado Prominente
            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        [240.0, 42.0],
                        egui::Button::new(
                            RichText::new("💾 Guardar Todas las Preferencias")
                                .size(15.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(RustCareTheme::SUCCESS_GREEN)
                        .corner_radius(CornerRadius::same(8)),
                    )
                    .clicked()
                {
                    if let Ok(()) = ConfigManager::save(&self.prefs) {
                        self.status_message = Some("¡Configuración guardada con éxito en settings.json!".to_string());
                    }
                }
            });
            ui.add_space(16.0);
        });
    }

    fn render_general_tab(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("🚀 INICIO Y SEGUNDO PLANO").size(13.0).strong().color(RustCareTheme::ACCENT_CYAN));
        ui.add_space(6.0);
        RustCareTheme::card_frame().show(ui, |ui| {
            ui.vertical(|ui| {
                let mut autostart = self.prefs.auto_start_with_system;
                if ui.checkbox(&mut autostart, "Iniciar RustCare automáticamente con Windows").changed() {
                    self.prefs.auto_start_with_system = autostart;
                    let _ = AutomationEngine::set_autostart(autostart);
                }
                ui.label(RichText::new("Carga en segundo plano para monitorización de RAM y mantenimiento sin ralentizar el arranque.").small().color(RustCareTheme::TEXT_MUTED));
            });
        });

        ui.add_space(14.0);
        ui.label(RichText::new("🧠 OPTIMIZACIÓN DE MEMORIA RAM (AutoCare)").size(13.0).strong().color(RustCareTheme::ACCENT_CYAN));
        ui.add_space(6.0);
        RustCareTheme::card_frame().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.checkbox(&mut self.prefs.auto_ram_boost_enabled, "Vaciado automático de memoria de trabajo (Working Set)");
                ui.label(RichText::new("Invoca de forma no destructiva K32EmptyWorkingSet cuando se supere el porcentaje límite.").small().color(RustCareTheme::TEXT_MUTED));
                if self.prefs.auto_ram_boost_enabled {
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label("Umbral de activación de RAM:");
                        ui.add(Slider::new(&mut self.prefs.auto_ram_threshold_percent, 50.0..=95.0).suffix("%"));
                    });
                }
            });
        });

        ui.add_space(14.0);
        ui.label(RichText::new("🔔 NOTIFICACIONES Y ALERTAS").size(13.0).strong().color(RustCareTheme::ACCENT_CYAN));
        ui.add_space(6.0);
        RustCareTheme::card_frame().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.checkbox(&mut self.prefs.notifications_enabled, "Habilitar notificaciones nativas de escritorio");
                if self.prefs.notifications_enabled {
                    ui.checkbox(&mut self.prefs.notify_on_updates, "Avisar cuando haya parches de aplicaciones pendientes (WinGet)");
                    ui.checkbox(&mut self.prefs.notify_on_high_junk, "Avisar cuando se acumule un volumen alto de basura");
                    ui.checkbox(&mut self.prefs.notify_on_drivers, "Avisar si se detectan controladores con conflicto de hardware");
                    ui.add_space(8.0);
                    if ui.button("🔔 Probar Notificación de Escritorio").clicked() {
                        let _ = AutomationEngine::send_notification("RustCare PRO", "Las notificaciones están funcionando correctamente.");
                        self.status_message = Some("Notificación de prueba enviada al sistema.".into());
                    }
                }
            });
        });
    }

    fn render_browsers_tab(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("🌐 LIMPIEZA DE PRIVACIDAD POR NAVEGADOR").size(13.0).strong().color(RustCareTheme::ACCENT_CYAN));
        ui.label(RichText::new("Elige con precisión qué datos eliminar en cada navegador. Las contraseñas guardadas siempre están excluidas por omisión.").color(RustCareTheme::TEXT_MUTED));
        ui.add_space(8.0);

        // Google Chrome
        RustCareTheme::card_frame().show(ui, |ui| {
            ui.label(RichText::new("Google Chrome").strong().color(Color32::WHITE));
            ui.add_space(4.0);
            ui.checkbox(&mut self.prefs.browser_clean_chrome_cache, "Caché de internet y archivos temporales web");
            ui.checkbox(&mut self.prefs.browser_clean_chrome_history, "Historial de navegación");
            ui.checkbox(&mut self.prefs.browser_clean_chrome_downloads, "Historial de descargas");
            ui.checkbox(&mut self.prefs.browser_clean_chrome_cookies, "Cookies y sesiones de inicio de sesión (Cierra sesiones abiertas)");
        });
        ui.add_space(10.0);

        // Microsoft Edge
        RustCareTheme::card_frame().show(ui, |ui| {
            ui.label(RichText::new("Microsoft Edge").strong().color(Color32::WHITE));
            ui.add_space(4.0);
            ui.checkbox(&mut self.prefs.browser_clean_edge_cache, "Caché de páginas web y miniaturas");
            ui.checkbox(&mut self.prefs.browser_clean_edge_history, "Historial de navegación");
            ui.checkbox(&mut self.prefs.browser_clean_edge_downloads, "Historial de descargas");
            ui.checkbox(&mut self.prefs.browser_clean_edge_cookies, "Cookies y datos de sesión");
        });
        ui.add_space(10.0);

        // Mozilla Firefox
        RustCareTheme::card_frame().show(ui, |ui| {
            ui.label(RichText::new("Mozilla Firefox").strong().color(Color32::WHITE));
            ui.add_space(4.0);
            ui.checkbox(&mut self.prefs.browser_clean_firefox_cache, "Caché de red y datos de almacenamiento offline");
            ui.checkbox(&mut self.prefs.browser_clean_firefox_history, "Historial de navegación");
            ui.checkbox(&mut self.prefs.browser_clean_firefox_downloads, "Historial de descargas");
            ui.checkbox(&mut self.prefs.browser_clean_firefox_cookies, "Cookies y datos de sitio");
        });
        ui.add_space(10.0);

        // Brave Browser & SQLite VACUUM
        RustCareTheme::card_frame().show(ui, |ui| {
            ui.label(RichText::new("Brave Browser & Optimización SQLite").strong().color(Color32::WHITE));
            ui.add_space(4.0);
            ui.checkbox(&mut self.prefs.browser_clean_brave_cache, "Caché web de Brave");
            ui.checkbox(&mut self.prefs.browser_clean_brave_history, "Historial de navegación de Brave");
            ui.separator();
            ui.checkbox(&mut self.prefs.browser_vacuum_sqlite_auto, "Desfragmentar y compactar bases de datos SQLite (VACUUM sin borrar historial)");
        });
    }

    fn render_junk_tab(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("🧹 ARCHIVOS BASURA Y SISTEMA OPERATIVO").size(13.0).strong().color(RustCareTheme::ACCENT_CYAN));
        ui.label(RichText::new("Configura qué áreas del disco se limpian en los análisis 1-Clic y programados.").color(RustCareTheme::TEXT_MUTED));
        ui.add_space(8.0);

        RustCareTheme::card_frame().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.checkbox(&mut self.prefs.junk_clean_user_temp, "Archivos temporales del usuario (%TEMP%)");
                ui.checkbox(&mut self.prefs.junk_clean_system_temp, "Archivos temporales del sistema (C:\\Windows\\Temp - Requiere Admin)");
                ui.checkbox(&mut self.prefs.junk_clean_recycle_bin, "Papelera de reciclaje de todas las unidades");
                ui.checkbox(&mut self.prefs.junk_clean_windows_logs, "Archivos de registro e informes de error de Windows (.log, .wer)");
                ui.checkbox(&mut self.prefs.junk_clean_crash_dumps, "Volcados de memoria de fallo del sistema (Crash Dumps / BSOD)");
                ui.checkbox(&mut self.prefs.junk_clean_windows_update, "Caché residual de descargas de Windows Update (SoftwareDistribution)");
                ui.checkbox(&mut self.prefs.junk_clean_thumbnails, "Caché de miniaturas de imágenes y vídeos de Explorer");
                ui.checkbox(&mut self.prefs.junk_clean_dev_caches, "Cachés de herramientas de desarrollo (Cargo, npm, pip, NuGet, Gradle)");
            });
        });
    }

    fn render_registry_tab(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("🗃️ LIMPIEZA Y REPARACIÓN DEL REGISTRO DE WINDOWS").size(13.0).strong().color(RustCareTheme::ACCENT_CYAN));
        ui.label(RichText::new("Reglas de auditoría del registro con garantía de reversibilidad obligatoria.").color(RustCareTheme::TEXT_MUTED));
        ui.add_space(8.0);

        RustCareTheme::card_frame().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.checkbox(&mut self.prefs.registry_clean_uninstalled, "Entradas de programas desinstalados y desinstaladores huérfanos");
                ui.checkbox(&mut self.prefs.registry_clean_file_extensions, "Asociaciones de extensión de archivo huérfanas");
                ui.checkbox(&mut self.prefs.registry_clean_app_paths, "Rutas de aplicación y comandos obsoletos en App Paths");
                ui.separator();
                ui.checkbox(&mut self.prefs.registry_auto_backup, "🛡️ Crear siempre copia de seguridad automática (.reg) antes de modificar el registro");
                ui.label(RichText::new("Permite deshacer cualquier cambio en 1 clic desde el Centro de Rescate.").small().color(RustCareTheme::SUCCESS_GREEN));
            });
        });
    }

    fn render_exclusions_tab(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("🛡️ LISTA BLANCA DE EXCLUSIONES").size(13.0).strong().color(RustCareTheme::ACCENT_CYAN));
        ui.label(RichText::new("Las rutas y carpetas aquí añadidas nunca serán tocadas ni borradas por ningún motor de limpieza.").color(RustCareTheme::TEXT_MUTED));
        ui.add_space(8.0);

        RustCareTheme::card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ruta a excluir:");
                ui.text_edit_singleline(&mut self.new_exclusion_path);
                if ui.button("➕ Añadir a Exclusiones").clicked() && !self.new_exclusion_path.trim().is_empty() {
                    let path = self.new_exclusion_path.trim().to_string();
                    if !self.prefs.excluded_paths.contains(&path) {
                        self.prefs.excluded_paths.push(path);
                        self.new_exclusion_path.clear();
                    }
                }
            });
            ui.add_space(10.0);

            if self.prefs.excluded_paths.is_empty() {
                ui.label(RichText::new("No hay rutas excluidas actualmente.").color(RustCareTheme::TEXT_MUTED));
            } else {
                let mut remove_idx = None;
                for (idx, p) in self.prefs.excluded_paths.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("• {}", p));
                        if ui.small_button("❌ Quitar").clicked() {
                            remove_idx = Some(idx);
                        }
                    });
                }
                if let Some(i) = remove_idx {
                    self.prefs.excluded_paths.remove(i);
                }
            }
        });
    }
}
