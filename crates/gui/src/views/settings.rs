//! Vista de Centro de Configuración & Opciones PRO
//! Reproducción 1:1 de las opciones de Advanced SystemCare (IObit) con árbol de navegación
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, CornerRadius, Margin, RichText, ScrollArea, Slider, Stroke, Ui, Vec2};
use rustcare_core::automation::AutomationEngine;
use rustcare_core::config::{ConfigManager, UserPreferences};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsSection {
    General,
    AntiSpyware,
    // Limpiar y optimizar (Subitems)
    SpywareRemover,
    RegistryCleaner,
    PrivacyCleaner,
    JunkFilesCleaner,
    SystemOptimization,
    InternetBooster,
    SecurityReinforce,
    VulnerabilityFixes,
    DiskOptimization,
    // Servicio automático (Subitems)
    AutoCare,
    AutoCleaner,
    AutoUpdater,
    AutoRamCleaner,
    CustomCleaning,
    IgnoreList,
    LogsAndBackups,
    Notifications,
}

pub struct SettingsView {
    pub prefs: UserPreferences,
    pub selected_section: SettingsSection,
    pub expand_clean_optimize: bool,
    pub expand_auto_service: bool,
    pub new_exclusion_path: String,
    pub status_message: Option<String>,
    pub current_username: String,
}

impl Default for SettingsView {
    fn default() -> Self {
        let mut prefs = ConfigManager::load();
        prefs.auto_start_with_system = AutomationEngine::is_autostart_enabled();

        let username = std::env::var("USERNAME")
            .or_else(|_| std::env::var("USER"))
            .unwrap_or_else(|_| "Alejandro".to_string());

        Self {
            prefs,
            selected_section: SettingsSection::General,
            expand_clean_optimize: true,
            expand_auto_service: true,
            new_exclusion_path: String::new(),
            status_message: None,
            current_username: username,
        }
    }
}

impl SettingsView {
    pub fn render(&mut self, ui: &mut Ui) {
        let available_size = ui.available_size();
        let sidebar_width = 240.0;
        let content_width = (available_size.x - sidebar_width - 16.0).max(450.0);
        let panel_height = (available_size.y - 45.0).max(480.0);

        // Encabezado superior de la ventana de opciones
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new("⚙ RustCare - Opciones")
                    .size(20.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(msg) = &self.status_message {
                    RustCareTheme::draw_badge(
                        ui,
                        msg,
                        RustCareTheme::SUCCESS_GREEN,
                        Color32::from_rgba_premultiplied(16, 185, 129, 25),
                    );
                }
            });
        });
        ui.separator();
        ui.add_space(4.0);

        // Disposición en Dos Columnas: Árbol de Navegación Izquierdo (240px) + Panel de Controles Derecho
        ui.horizontal(|ui| {
            // Columna Izquierda: Árbol de Categorías ASC
            ui.allocate_ui(Vec2::new(sidebar_width, panel_height), |ui| {
                self.render_navigation_sidebar(ui);
            });

            ui.separator();

            // Columna Derecha: Contenido de Opciones
            ui.allocate_ui(Vec2::new(content_width, panel_height), |ui| {
                self.render_right_content_panel(ui);
            });
        });
    }

    /// Árbol jerárquico de navegación izquierdo idéntico a Advanced SystemCare
    fn render_navigation_sidebar(&mut self, ui: &mut Ui) {
        egui::Frame::NONE
            .fill(RustCareTheme::SIDEBAR_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(6))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ScrollArea::vertical()
                    .id_salt("settings_nav_sidebar_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // 1. Ajustes generales
                        self.render_tree_item(
                            ui,
                            "⚙  Ajustes generales",
                            SettingsSection::General,
                            0,
                        );

                        // 2. Anti-Spyware
                        self.render_tree_item(
                            ui,
                            "🛡  Anti-Spyware",
                            SettingsSection::AntiSpyware,
                            0,
                        );

                        // 3. Limpiar y optimizar (Desplegable con flecha)
                        ui.horizontal(|ui| {
                            let arrow = if self.expand_clean_optimize { "▾" } else { "▸" };
                            if ui
                                .selectable_label(
                                    false,
                                    RichText::new(format!("{} 🧹 Limpiar y optimizar", arrow))
                                        .strong()
                                        .color(RustCareTheme::TEXT_PRIMARY),
                                )
                                .clicked()
                            {
                                self.expand_clean_optimize = !self.expand_clean_optimize;
                            }
                        });

                        if self.expand_clean_optimize {
                            self.render_tree_item(ui, "🛡  Eliminador de Spyware", SettingsSection::SpywareRemover, 16);
                            self.render_tree_item(ui, "🗂  Limpiador de registro", SettingsSection::RegistryCleaner, 16);
                            self.render_tree_item(ui, "🔒  Limpieza de privacidad", SettingsSection::PrivacyCleaner, 16);
                            self.render_tree_item(ui, "🗑  Limpiador de archivos basura", SettingsSection::JunkFilesCleaner, 16);
                            self.render_tree_item(ui, "⚡  Optimización del Sistema", SettingsSection::SystemOptimization, 16);
                            self.render_tree_item(ui, "🌐  Acelerador de Internet", SettingsSection::InternetBooster, 16);
                            self.render_tree_item(ui, "🛡  Refuerzo de seguridad", SettingsSection::SecurityReinforce, 16);
                            self.render_tree_item(ui, "🩹  Soluciones de vulnerabilidades", SettingsSection::VulnerabilityFixes, 16);
                            self.render_tree_item(ui, "💾  Optimización de disco", SettingsSection::DiskOptimization, 16);
                        }

                        // 4. Servicio automático (Desplegable con flecha)
                        ui.horizontal(|ui| {
                            let arrow = if self.expand_auto_service { "▾" } else { "▸" };
                            if ui
                                .selectable_label(
                                    false,
                                    RichText::new(format!("{} 🤖 Servicio automático", arrow))
                                        .strong()
                                        .color(RustCareTheme::TEXT_PRIMARY),
                                )
                                .clicked()
                            {
                                self.expand_auto_service = !self.expand_auto_service;
                            }
                        });

                        if self.expand_auto_service {
                            self.render_tree_item(ui, "🕒  Auto Cuidador", SettingsSection::AutoCare, 16);
                            self.render_tree_item(ui, "🧹  Auto Limpiador", SettingsSection::AutoCleaner, 16);
                            self.render_tree_item(ui, "🔄  Auto Actualizador", SettingsSection::AutoUpdater, 16);
                            self.render_tree_item(ui, "🧠  Auto Limpiador de RAM", SettingsSection::AutoRamCleaner, 16);
                            self.render_tree_item(ui, "🎛  Limpieza personalizada", SettingsSection::CustomCleaning, 16);
                            self.render_tree_item(ui, "⛔  Lista de ignorados", SettingsSection::IgnoreList, 16);
                            self.render_tree_item(ui, "🛟  Registros y copias de seguridad", SettingsSection::LogsAndBackups, 16);
                            self.render_tree_item(ui, "🔔  Notificaciones", SettingsSection::Notifications, 16);
                        }
                    });
            });
    }

    /// Renderiza un elemento individual en el árbol de navegación
    fn render_tree_item(
        &mut self,
        ui: &mut Ui,
        title: &str,
        section: SettingsSection,
        indent: usize,
    ) {
        let is_selected = self.selected_section == section;

        let display_text = if indent > 0 {
            format!("{:indent$}{}", "", title, indent = indent / 4)
        } else {
            title.to_string()
        };

        let text_color = if is_selected {
            RustCareTheme::ACCENT_CYAN
        } else {
            RustCareTheme::TEXT_MUTED
        };

        let response = ui.selectable_label(
            is_selected,
            RichText::new(display_text).color(text_color).strong(),
        );

        if response.clicked() {
            self.selected_section = section;
            self.status_message = None;
        }
    }

    /// Panel de Controles Derecho con Scroll y Barra Inferior de Acción
    fn render_right_content_panel(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let max_scroll_h = (ui.available_height() - 50.0).max(300.0);
            ScrollArea::vertical()
                .id_salt("settings_right_panel_scroll")
                .auto_shrink([false, false])
                .max_height(max_scroll_h)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width() - 8.0);
                    match self.selected_section {
                        SettingsSection::General => self.render_general_screen(ui),
                        SettingsSection::RegistryCleaner => self.render_registry_screen(ui),
                        SettingsSection::PrivacyCleaner => self.render_privacy_screen(ui),
                        SettingsSection::JunkFilesCleaner => self.render_junk_screen(ui),
                        SettingsSection::SystemOptimization => self.render_system_opt_screen(ui),
                        SettingsSection::InternetBooster => self.render_internet_booster_screen(ui),
                        SettingsSection::AntiSpyware | SettingsSection::SpywareRemover => {
                            self.render_antispyware_screen(ui)
                        }
                        SettingsSection::SecurityReinforce => self.render_security_screen(ui),
                        SettingsSection::VulnerabilityFixes => self.render_vulnerability_screen(ui),
                        SettingsSection::DiskOptimization => self.render_disk_opt_screen(ui),
                        SettingsSection::AutoCare => self.render_auto_care_screen(ui),
                        SettingsSection::AutoCleaner => self.render_auto_cleaner_screen(ui),
                        SettingsSection::AutoUpdater => self.render_auto_updater_screen(ui),
                        SettingsSection::AutoRamCleaner => self.render_auto_ram_screen(ui),
                        SettingsSection::CustomCleaning => self.render_custom_cleaning_screen(ui),
                        SettingsSection::IgnoreList => self.render_ignore_list_screen(ui),
                        SettingsSection::LogsAndBackups => self.render_backups_screen(ui),
                        SettingsSection::Notifications => self.render_notifications_screen(ui),
                    }
                });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            // Barra de Botones Inferior de Acción (Restablecer, Aplicar, Guardar)
            ui.horizontal(|ui| {
                if ui
                    .button(
                        RichText::new("↺ Restablecer predeterminados")
                            .color(RustCareTheme::TEXT_MUTED),
                    )
                    .clicked()
                {
                    self.prefs = UserPreferences::default();
                    self.status_message = Some("Preferencias restablecidas a los valores originales.".into());
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_sized(
                            [210.0, 36.0],
                            egui::Button::new(
                                RichText::new("💾 Guardar y Aplicar")
                                    .size(14.0)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(RustCareTheme::SUCCESS_GREEN)
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        let _ = AutomationEngine::set_autostart(self.prefs.auto_start_with_system);
                        if let Ok(()) = ConfigManager::save(&self.prefs) {
                            self.status_message =
                                Some("¡Configuración guardada y aplicada con éxito!".into());
                        }
                    }
                });
            });
        });
    }

    // =========================================================================
    // PANTALLA 1: Ajustes generales (Captura 1)
    // =========================================================================
    fn render_general_screen(&mut self, ui: &mut Ui) {
        ui.label(
            RichText::new("Ajustes generales")
                .size(18.0)
                .strong()
                .color(Color32::WHITE),
        );
        ui.add_space(10.0);

        ui.checkbox(
            &mut self.prefs.minimize_to_tray_on_close,
            "Minimizar a Bandeja de Sistema al cerrar programa.",
        );

        ui.horizontal(|ui| {
            let mut tray_boot = self.prefs.autostart_tray_on_boot;
            if ui
                .checkbox(
                    &mut tray_boot,
                    "Cargar Bandeja de Advanced SystemCare al inicio de Windows",
                )
                .changed()
            {
                self.prefs.autostart_tray_on_boot = tray_boot;
                self.prefs.auto_start_with_system = tray_boot;
                let _ = AutomationEngine::set_autostart(tray_boot);
            }
            ui.label(RichText::new("ⓘ").color(RustCareTheme::ACCENT_CYAN));
        });

        ui.horizontal(|ui| {
            ui.checkbox(
                &mut self.prefs.autostart_service_on_boot,
                "Cargar Servicio Advanced SystemCare al inicio de Windows",
            );
            ui.label(RichText::new("ⓘ").color(RustCareTheme::ACCENT_CYAN));
        });

        ui.checkbox(
            &mut self.prefs.skip_uac_warning,
            "Saltar advertencia Control de Cuenta de Usuario.",
        );

        ui.checkbox(
            &mut self.prefs.enable_perf_monitor,
            "Habilitar Monitor de Rendimiento.",
        );

        ui.label(
            RichText::new("Monitorea factores claves que tal vez afecten el rendimiento de su PC en tiempo real y le provee algunas opciones rápidas útiles para mejorar dicho rendimiento.")
                .size(12.0)
                .color(RustCareTheme::TEXT_MUTED),
        );

        ui.add_space(14.0);
        ui.label(
            RichText::new("Menú Contextual:")
                .size(14.0)
                .strong()
                .color(Color32::WHITE),
        );
        ui.add_space(4.0);

        // Cuadro con borde para opciones del menú contextual
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.ctx_recover_file, "Recuperar Archivo");
                ui.checkbox(&mut self.prefs.ctx_destroy_file, "Destruir Archivo");
                ui.checkbox(&mut self.prefs.ctx_fix_shortcut, "Reparar Acceso Directo");
                ui.checkbox(&mut self.prefs.ctx_sweep_privacy, "Barrer Privacidad");
                ui.checkbox(&mut self.prefs.ctx_repair_disk, "Reparar Disco");
                ui.checkbox(&mut self.prefs.ctx_clean_disk, "Limpiar Disco");
                ui.checkbox(&mut self.prefs.ctx_defrag_disk, "Desfragmentar Disco");
            });
    }

    // =========================================================================
    // PANTALLA 2: Limpiador de registro (Captura 2)
    // =========================================================================
    fn render_registry_screen(&mut self, ui: &mut Ui) {
        ui.label(
            RichText::new("Limpiador de registro")
                .size(18.0)
                .strong()
                .color(Color32::WHITE),
        );
        ui.add_space(6.0);
        ui.label(
            RichText::new("Seleccione los objetos que desee analizar:")
                .size(13.0)
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.add_space(6.0);

        // Caja con lista de 13 elementos seleccionables
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .id_salt("registry_items_scroll")
                    .max_height(230.0)
                    .show(ui, |ui| {
                        ui.checkbox(&mut self.prefs.reg_activex, "ActiveX y Componente");
                        ui.checkbox(&mut self.prefs.reg_integrity, "Integridad de Registro");
                        ui.checkbox(&mut self.prefs.reg_app_paths, "Rutas de Aplicaciones");
                        ui.checkbox(&mut self.prefs.reg_fonts, "Fuentes");
                        ui.checkbox(&mut self.prefs.reg_help_files, "Archivos de Ayuda");
                        ui.checkbox(&mut self.prefs.reg_history_lists, "Listas de Historiales");
                        ui.checkbox(&mut self.prefs.reg_shared_files, "Archivos Compartidos");
                        ui.checkbox(&mut self.prefs.reg_shortcuts, "Accesos Directos");
                        ui.checkbox(&mut self.prefs.reg_software, "Software");
                        ui.checkbox(&mut self.prefs.reg_sounds, "Sonidos");
                        ui.checkbox(&mut self.prefs.reg_start_menu, "Menú de Inicio");
                        ui.checkbox(&mut self.prefs.reg_startup_items, "Elementos de Inicio");
                        ui.checkbox(&mut self.prefs.reg_mui_cache, "Caché MUI");
                    });
            });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.prefs.reg_deep_clean, "Limpieza Profunda");
            RustCareTheme::draw_badge(
                ui,
                "PRO",
                Color32::WHITE,
                Color32::from_rgb(217, 119, 6),
            );
        });

        ui.label(
            RichText::new("Se abre paso a través del registro para identificar, sugerir y reparar errores. Limpieza Profunda puede mejorar el rendimiento y la estabilidad del sistema operativo previniendo fallas de Windows y mensajes de errores.")
                .size(12.0)
                .color(RustCareTheme::TEXT_MUTED),
        );
    }

    // =========================================================================
    // PANTALLA 3: Limpieza de privacidad (Captura 3)
    // =========================================================================
    fn render_privacy_screen(&mut self, ui: &mut Ui) {
        ui.label(
            RichText::new("Limpieza de privacidad")
                .size(18.0)
                .strong()
                .color(Color32::WHITE),
        );
        ui.add_space(6.0);
        ui.label(
            RichText::new("Seleccione los objetos que desee limpiar:")
                .size(13.0)
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.add_space(6.0);

        // Cuadro con lista jerárquica de navegadores
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .id_salt("privacy_browsers_scroll")
                    .max_height(220.0)
                    .show(ui, |ui| {
                        // Microsoft Edge
                        ui.collapsing(RichText::new("Edge").strong().color(Color32::WHITE), |ui| {
                            ui.checkbox(&mut self.prefs.edge_cache, "Caché de Internet");
                            ui.checkbox(&mut self.prefs.edge_cookies, "Cookie");
                            ui.checkbox(&mut self.prefs.edge_history, "Historial");
                            ui.checkbox(&mut self.prefs.edge_downloads, "Historial de Descargas");
                            ui.checkbox(&mut self.prefs.edge_session, "Sesión");
                            ui.checkbox(&mut self.prefs.edge_autofill_form, "Formulario de Autocompletado");
                            ui.checkbox(&mut self.prefs.edge_autofill_passwords, "Contraseña de Autocompletado");
                            ui.checkbox(&mut self.prefs.edge_metrics, "Archivos temporales de métrica");
                            ui.checkbox(&mut self.prefs.edge_bookmarks_backup, "Copia de seguridad de los marcadores");
                            ui.checkbox(&mut self.prefs.edge_compact_db, "Base de Datos Compacta");
                            ui.checkbox(&mut self.prefs.edge_site_characteristics, "Site Characteristics Database");
                        });

                        // Google Chrome
                        ui.collapsing(RichText::new("Google Chrome").strong().color(Color32::WHITE), |ui| {
                            ui.checkbox(&mut self.prefs.chrome_cache, "Caché de Internet");
                            ui.checkbox(&mut self.prefs.chrome_cookies, "Cookie");
                            ui.checkbox(&mut self.prefs.chrome_history, "Historial");
                            ui.checkbox(&mut self.prefs.chrome_downloads, "Historial de Descargas");
                            ui.checkbox(&mut self.prefs.chrome_session, "Sesión");
                            ui.checkbox(&mut self.prefs.chrome_autofill_form, "Formulario de Autocompletado");
                            ui.checkbox(&mut self.prefs.chrome_autofill_passwords, "Contraseña de Autocompletado");
                            ui.checkbox(&mut self.prefs.chrome_metrics, "Archivos temporales de métrica");
                            ui.checkbox(&mut self.prefs.chrome_bookmarks_backup, "Copia de seguridad de marcadores");
                            ui.checkbox(&mut self.prefs.chrome_compact_db, "Base de Datos Compacta");
                        });

                        // Mozilla Firefox
                        ui.collapsing(RichText::new("Mozilla Firefox").strong().color(Color32::WHITE), |ui| {
                            ui.checkbox(&mut self.prefs.firefox_cache, "Caché de Internet");
                            ui.checkbox(&mut self.prefs.firefox_cookies, "Cookie");
                            ui.checkbox(&mut self.prefs.firefox_history, "Historial");
                            ui.checkbox(&mut self.prefs.firefox_downloads, "Historial de Descargas");
                            ui.checkbox(&mut self.prefs.firefox_session, "Sesión");
                            ui.checkbox(&mut self.prefs.firefox_compact_db, "Base de Datos Compacta");
                        });

                        // Brave Browser
                        ui.collapsing(RichText::new("Brave Browser").strong().color(Color32::WHITE), |ui| {
                            ui.checkbox(&mut self.prefs.brave_cache, "Caché de Internet");
                            ui.checkbox(&mut self.prefs.brave_cookies, "Cookie");
                            ui.checkbox(&mut self.prefs.brave_history, "Historial");
                            ui.checkbox(&mut self.prefs.brave_downloads, "Historial de Descargas");
                            ui.checkbox(&mut self.prefs.brave_compact_db, "Base de Datos Compacta (VACUUM)");
                        });
                    });
            });

        ui.add_space(10.0);
        ui.label(
            RichText::new("Seleccione las cuentas de usuario para barrer sus rastros de privacidad:")
                .size(13.0)
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.add_space(4.0);

        // Cuadro de cuentas de usuario
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                ui.checkbox(
                    &mut self.prefs.privacy_clean_user_account,
                    &self.current_username,
                );
            });

        ui.add_space(8.0);
        ui.checkbox(
            &mut self.prefs.privacy_dod_wipe,
            "Habilitar Borrado Seguro de Archivo DOD 5220.22-M (Lento)",
        );

        ui.horizontal(|ui| {
            let mut keep = true;
            ui.checkbox(&mut keep, "Mantener el último historial del navegador web");
            ui.add_space(6.0);
            egui::ComboBox::from_id_salt("combo_privacy_duration")
                .selected_text(&self.prefs.privacy_keep_history_duration)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.prefs.privacy_keep_history_duration,
                        "1 semana".to_string(),
                        "1 semana",
                    );
                    ui.selectable_value(
                        &mut self.prefs.privacy_keep_history_duration,
                        "2 semanas".to_string(),
                        "2 semanas",
                    );
                    ui.selectable_value(
                        &mut self.prefs.privacy_keep_history_duration,
                        "1 mes".to_string(),
                        "1 mes",
                    );
                    ui.selectable_value(
                        &mut self.prefs.privacy_keep_history_duration,
                        "Todo".to_string(),
                        "Todo",
                    );
                });
        });
    }

    // =========================================================================
    // PANTALLA 4: Limpiador de archivos basura (Captura 4)
    // =========================================================================
    fn render_junk_screen(&mut self, ui: &mut Ui) {
        ui.label(
            RichText::new("Limpiador de archivos basura")
                .size(18.0)
                .strong()
                .color(Color32::WHITE),
        );
        ui.add_space(6.0);
        ui.label(
            RichText::new("Seleccione los objetos que desee limpiar:")
                .size(13.0)
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.add_space(6.0);

        // Cuadro con lista de 10 elementos de basura
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .id_salt("junk_items_scroll")
                    .max_height(210.0)
                    .show(ui, |ui| {
                        ui.checkbox(&mut self.prefs.junk_recycle_bin, "Papelera de Reciclaje de Windows");
                        ui.checkbox(&mut self.prefs.junk_windows_temp, "Archivos Temporales de Windows");
                        ui.checkbox(&mut self.prefs.junk_ie_edge_temp, "Archivos Temporales de IE");
                        ui.checkbox(&mut self.prefs.junk_directx_cache, "Caché DirectX Shader");
                        ui.checkbox(&mut self.prefs.junk_clipboard, "Portapapeles");
                        ui.checkbox(&mut self.prefs.junk_memory_dumps, "Volcados de Memoria");
                        ui.checkbox(&mut self.prefs.junk_chkdsk_fragments, "Fragmentos de Archivos Chkdsk");
                        ui.checkbox(&mut self.prefs.junk_windows_logs, "Archivos de registro de Windows");
                        ui.checkbox(&mut self.prefs.junk_error_reporting, "Informe de Errores de Windows");
                        ui.checkbox(&mut self.prefs.junk_dns_cache, "Caché DNS");
                    });
            });

        ui.add_space(10.0);
        ui.label(
            RichText::new("Seleccione las cuentas de usuario para limpiar sus archivos basura:")
                .size(13.0)
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.add_space(4.0);

        // Cuadro de usuario
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                ui.checkbox(
                    &mut self.prefs.junk_clean_user_account,
                    &self.current_username,
                );
            });

        ui.add_space(8.0);
        ui.checkbox(
            &mut self.prefs.junk_dod_wipe,
            "Borrado Seguro de Archivo DOD 5220.22-M (Lento)",
        );

        ui.horizontal(|ui| {
            let mut keep_temp = true;
            ui.checkbox(
                &mut keep_temp,
                "Mantener los últimos archivos temporales y dumpeos de Windows",
            );
            ui.add_space(6.0);
            egui::ComboBox::from_id_salt("combo_junk_temp_duration")
                .selected_text(&self.prefs.junk_keep_temp_duration)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.prefs.junk_keep_temp_duration,
                        "1 semana".to_string(),
                        "1 semana",
                    );
                    ui.selectable_value(
                        &mut self.prefs.junk_keep_temp_duration,
                        "2 semanas".to_string(),
                        "2 semanas",
                    );
                    ui.selectable_value(
                        &mut self.prefs.junk_keep_temp_duration,
                        "1 mes".to_string(),
                        "1 mes",
                    );
                });
        });

        ui.horizontal(|ui| {
            let mut keep_recycle = true;
            ui.checkbox(
                &mut keep_recycle,
                "Mantener los últimos archivos de la papelera de reciclaje",
            );
            ui.add_space(6.0);
            egui::ComboBox::from_id_salt("combo_junk_recycle_duration")
                .selected_text(&self.prefs.junk_keep_recycle_duration)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.prefs.junk_keep_recycle_duration,
                        "1 mes".to_string(),
                        "1 mes",
                    );
                    ui.selectable_value(
                        &mut self.prefs.junk_keep_recycle_duration,
                        "2 meses".to_string(),
                        "2 meses",
                    );
                    ui.selectable_value(
                        &mut self.prefs.junk_keep_recycle_duration,
                        "Nunca".to_string(),
                        "Nunca",
                    );
                });
        });
    }

    // =========================================================================
    // PANTALLA 5: Optimización del Sistema [PRO] (Captura 5)
    // =========================================================================
    fn render_system_opt_screen(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Optimización del Sistema")
                    .size(18.0)
                    .strong()
                    .color(Color32::WHITE),
            );
            RustCareTheme::draw_badge(
                ui,
                "PRO",
                Color32::WHITE,
                Color32::from_rgb(217, 119, 6),
            );
        });
        ui.add_space(10.0);

        ui.label(
            RichText::new("Optimiza completamente Windows para el mejor rendimiento del sistema y la más alta velocidad de Internet liberando todo el potencial de su sistema, basado en cómo usted configura su PC y su red. La Mejor Tecnología de Optimización puede convertir a su PC en una PC de negocios, una productiva estación de trabajo, un centro de entretenimiento, una máquina para juegos o una computadora científica.")
                .size(13.0)
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Optimización de PC:")
                    .size(13.0)
                    .strong()
                    .color(Color32::WHITE),
            );
            ui.add_space(6.0);
            egui::ComboBox::from_id_salt("combo_system_opt_profile")
                .selected_text(&self.prefs.system_opt_profile)
                .width(220.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.prefs.system_opt_profile,
                        "Básica".to_string(),
                        "Básica",
                    );
                    ui.selectable_value(
                        &mut self.prefs.system_opt_profile,
                        "Negocios".to_string(),
                        "Negocios",
                    );
                    ui.selectable_value(
                        &mut self.prefs.system_opt_profile,
                        "Juegos".to_string(),
                        "Juegos",
                    );
                    ui.selectable_value(
                        &mut self.prefs.system_opt_profile,
                        "Estación de Trabajo".to_string(),
                        "Estación de Trabajo",
                    );
                });
        });

        ui.add_space(6.0);
        let profile_desc = match self.prefs.system_opt_profile.as_str() {
            "Negocios" => "Optimiza para aplicaciones ofimáticas, conferencias y productividad multi-tarea.",
            "Juegos" => "Prioriza DirectX, minimiza latencia de hilos multimedia y suspende servicios no críticos.",
            "Estación de Trabajo" => "Maximiza rendimiento multi-núcleo de CPU, ancho de banda de memoria masiva y renderizado.",
            _ => "Optimiza para todo propósito con usos varios. Mejor compatibilidad pero más lenta.",
        };

        ui.label(
            RichText::new(profile_desc)
                .size(12.0)
                .color(RustCareTheme::TEXT_MUTED),
        );
    }

    // =========================================================================
    // PANTALLA 6: Acelerador de Internet [PRO] (Captura 6)
    // =========================================================================
    fn render_internet_booster_screen(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Acelerador de Internet")
                    .size(18.0)
                    .strong()
                    .color(Color32::WHITE),
            );
            RustCareTheme::draw_badge(
                ui,
                "PRO",
                Color32::WHITE,
                Color32::from_rgb(217, 119, 6),
            );
        });
        ui.add_space(10.0);

        ui.label(
            RichText::new("Acelera la velocidad de su conexión de red, configurando según su computadora se conecta a Internet. Y maximiza su velocidad de red con")
                .size(13.0)
                .color(RustCareTheme::TEXT_PRIMARY),
        );
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Modo Conexión de red:")
                    .size(13.0)
                    .strong()
                    .color(Color32::WHITE),
            );
            ui.add_space(6.0);
            egui::ComboBox::from_id_salt("combo_network_mode")
                .selected_text(&self.prefs.network_connection_mode)
                .width(220.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.prefs.network_connection_mode,
                        "Básica".to_string(),
                        "Básica",
                    );
                    ui.selectable_value(
                        &mut self.prefs.network_connection_mode,
                        "DSL".to_string(),
                        "DSL",
                    );
                    ui.selectable_value(
                        &mut self.prefs.network_connection_mode,
                        "Cable/Fibra".to_string(),
                        "Cable/Fibra",
                    );
                    ui.selectable_value(
                        &mut self.prefs.network_connection_mode,
                        "LAN".to_string(),
                        "LAN",
                    );
                });
        });

        ui.add_space(4.0);
        let net_desc = match self.prefs.network_connection_mode.as_str() {
            "DSL" => "Optimiza MTU a 1492 y ajusta tamaño de ventana TCP para enlaces telefónicos digitales.",
            "Cable/Fibra" => "Optimiza MTU a 1500, habilita ECN y TCP Window Scaling para Gigabit simétrico.",
            "LAN" => "Configura interfaz Ethernet de área local para baja latencia en transferencias de red interna.",
            _ => "No estoy seguro qué tipo de conexión a Internet tengo.",
        };

        ui.label(
            RichText::new(net_desc)
                .size(12.0)
                .color(RustCareTheme::TEXT_MUTED),
        );

        ui.add_space(14.0);
        ui.checkbox(
            &mut self.prefs.network_advanced_opt,
            "Habilitar Optimización de Red Avanzada",
        );
        ui.label(
            RichText::new("Ajusta la transmisión de paquetes de datos para obtener una red de velocidad máxima, optimizando parámetros TCP/IP, como TCP Chimney")
                .size(12.0)
                .color(RustCareTheme::TEXT_MUTED),
        );
    }

    // =========================================================================
    // Pantallas Adicionales del Árbol de Opciones
    // =========================================================================
    fn render_antispyware_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Anti-Spyware & Eliminador de Amenazas").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.spyware_realtime_shield, "Escudo de Protección en Tiempo Real (Spyware Shield)");
                ui.checkbox(&mut self.prefs.spyware_heuristic_scan, "Análisis heurístico de procesos activos y binarios sospechosos");
                ui.separator();
                ui.checkbox(&mut self.prefs.auto_update_definitions, "Actualizar automáticamente la base de firmas de amenazas");
            });
    }

    fn render_security_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Refuerzo de seguridad de Windows").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.security_reinforce_rdp, "Deshabilitar Escritorio Remoto (RDP) no solicitado");
                ui.checkbox(&mut self.prefs.security_reinforce_smb, "Bloquear protocolo vulnerable SMBv1 en la red local");
                ui.checkbox(&mut self.prefs.security_reinforce_uac, "Forzar validación estricta de Control de Cuentas de Usuario");
            });
    }

    fn render_vulnerability_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Soluciones de vulnerabilidades").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.vuln_auto_scan_winget, "Auditar parches de seguridad de aplicaciones instaladas con WinGet");
                ui.label(RichText::new("Detecta bibliotecas obsoletas y componentes desactualizados automáticamente.").small().color(RustCareTheme::TEXT_MUTED));
            });
    }

    fn render_disk_opt_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Optimización de disco").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.disk_opt_enable_trim, "Habilitar comando TRIM periódico para unidades SSD");
                ui.checkbox(&mut self.prefs.disk_opt_defrag_hdd, "Desfragmentación organizada para discos magnéticos (HDD)");
            });
    }

    fn render_auto_care_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Auto Cuidador (Mantenimiento Desatendido)").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.auto_care_idle_only, "Ejecutar mantenimiento automático únicamente cuando el equipo esté inactivo");
                ui.checkbox(&mut self.prefs.auto_care_battery_pause, "Pausar tareas automáticas si el portátil está funcionando con batería");
            });
    }

    fn render_auto_cleaner_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Auto Limpiador").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.auto_clean_on_browser_close, "Limpiar rastros de privacidad automáticamente al cerrar navegadores");
                ui.checkbox(&mut self.prefs.auto_clean_on_shutdown, "Vaciar archivos temporales al apagar el equipo");
            });
    }

    fn render_auto_updater_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Auto Actualizador").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.auto_update_definitions, "Actualizar bases de datos de software y firmas en segundo plano");
                ui.checkbox(&mut self.prefs.auto_update_app, "Notificar y actualizar RustCare a nuevas versiones estables");
            });
    }

    fn render_auto_ram_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Auto Limpiador de RAM").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.auto_ram_boost_enabled, "Habilitar vaciado inteligente de memoria de trabajo (EmptyWorkingSet)");
                if self.prefs.auto_ram_boost_enabled {
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label("Umbral de activación de RAM:");
                        ui.add(Slider::new(&mut self.prefs.auto_ram_threshold_percent, 50.0..=95.0).suffix("%"));
                    });
                }
            });
    }

    fn render_custom_cleaning_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Limpieza personalizada").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        ui.label(RichText::new("Ajusta los umbrales de advertencia de espacio residual:").color(RustCareTheme::TEXT_MUTED));
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Avisar cuando la basura supere:");
                    ui.add(Slider::new(&mut self.prefs.auto_clean_threshold_mb, 256..=8192).suffix(" MB"));
                });
            });
    }

    fn render_ignore_list_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Lista de ignorados (Exclusiones)").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(6.0);
        ui.label(RichText::new("Las rutas y carpetas añadidas aquí nunca serán modificadas ni eliminadas.").color(RustCareTheme::TEXT_MUTED));
        ui.add_space(8.0);

        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Ruta a excluir:");
                    ui.text_edit_singleline(&mut self.new_exclusion_path);
                    if ui.button("➕ Añadir").clicked() && !self.new_exclusion_path.trim().is_empty() {
                        let path = self.new_exclusion_path.trim().to_string();
                        if !self.prefs.excluded_paths.contains(&path) {
                            self.prefs.excluded_paths.push(path);
                            self.new_exclusion_path.clear();
                        }
                    }
                });

                ui.add_space(10.0);
                if self.prefs.excluded_paths.is_empty() {
                    ui.label(RichText::new("No hay exclusiones activas.").color(RustCareTheme::TEXT_MUTED));
                } else {
                    let mut to_remove = None;
                    for (i, p) in self.prefs.excluded_paths.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("• {}", p));
                            if ui.small_button("❌ Quitar").clicked() {
                                to_remove = Some(i);
                            }
                        });
                    }
                    if let Some(i) = to_remove {
                        self.prefs.excluded_paths.remove(i);
                    }
                }
            });
    }

    fn render_backups_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Registros y copias de seguridad").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.registry_auto_backup, "Crear copia de seguridad del registro (.reg) antes de cada reparación");
                ui.label(RichText::new("Las copias se almacenan de forma segura y pueden restaurarse en cualquier momento desde el Centro de Rescate.").small().color(RustCareTheme::SUCCESS_GREEN));
            });
    }

    fn render_notifications_screen(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Notificaciones del sistema").size(18.0).strong().color(Color32::WHITE));
        ui.add_space(10.0);
        egui::Frame::NONE
            .fill(RustCareTheme::CARD_BG)
            .corner_radius(CornerRadius::same(6))
            .stroke(Stroke::new(1.0_f32, RustCareTheme::CARD_BORDER))
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.checkbox(&mut self.prefs.notifications_enabled, "Habilitar notificaciones en la bandeja del sistema");
                if self.prefs.notifications_enabled {
                    ui.checkbox(&mut self.prefs.notify_on_updates, "Notificar actualizaciones de software pendientes");
                    ui.checkbox(&mut self.prefs.notify_on_drivers, "Notificar conflictos de controladores");
                    ui.checkbox(&mut self.prefs.notify_on_high_junk, "Notificar cuando haya más de 1 GB de archivos basura");
                    ui.add_space(8.0);
                    if ui.button("🔔 Enviar Notificación de Prueba").clicked() {
                        let _ = AutomationEngine::send_notification("RustCare PRO", "Las notificaciones están configuradas correctamente.");
                        self.status_message = Some("Notificación de prueba enviada al sistema.".into());
                    }
                }
            });
    }
}
