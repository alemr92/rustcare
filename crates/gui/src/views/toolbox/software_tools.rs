use super::*;

impl ToolboxView {
    pub(super) fn render_uninstaller(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Buscar aplicación:");
            ui.text_edit_singleline(&mut self.app_search);
            if ghost_button(ui, "Refrescar", Some(100.0)).clicked() {
                self.apps = UninstallerEngine::list_installed_apps();
            }
        });
        ui.add_space(8.0);

        if !self.residuals.is_empty() {
            ui.label(RichText::new(format!("Carpetas residuales encontradas para '{}':", self.residual_app_name)).strong().color(RustCareTheme::WARNING_AMBER));
            for r in &self.residuals {
                ui.label(format!("- {} ({})", r.path.display(), CleanerEngine::format_bytes(r.size_bytes)));
            }
            ui.add_space(8.0);
        }

        let filtered_apps: Vec<&InstalledApp> = self
            .apps
            .iter()
            .filter(|a| self.app_search.is_empty() || a.name.to_lowercase().contains(&self.app_search.to_lowercase()))
            .collect();

        ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::BG_SURFACE)
                .corner_radius(8)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    for app in filtered_apps {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&app.name).strong());
                                ui.label(RichText::new(format!("{} | {}", app.version, app.publisher)).small().color(RustCareTheme::TEXT_MUTED));
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if danger_button(ui, "Desinstalar", Some(90.0)).clicked() {
                                    let _ = UninstallerEngine::run_uninstaller(app);
                                }
                                if ghost_button(ui, "Rastrear Residuos", Some(140.0)).clicked() {
                                    self.residuals = UninstallerEngine::scan_residuals(&app.name);
                                    self.residual_app_name = app.name.clone();
                                }
                            });
                        });
                        ui.separator();
                    }
                });
        });
    }

    pub(super) fn render_drivers(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Driver Booster PRO - Auditoría & Respaldo de Controladores").size(18.0).color(RustCareTheme::ACCENT_CYAN).strong());
        ui.label(RichText::new("Inspección a bajo nivel de controladores de hardware (Kernel Class GUIDs) con exportación oficial vía pnputil.").color(RustCareTheme::TEXT_MUTED));
        ui.add_space(10.0);

        // Banner de Métricas KPI Estilo IObit Driver Booster
        ui.horizontal(|ui| {
            let total_drivers = self.drivers.len();
            let gpu_drivers = self.drivers.iter().filter(|d| d.class_name.to_lowercase().contains("pantalla") || d.class_name.to_lowercase().contains("display") || d.class_name.to_lowercase().contains("gpu")).count();
            let net_drivers = self.drivers.iter().filter(|d| d.class_name.to_lowercase().contains("net") || d.class_name.to_lowercase().contains("red") || d.class_name.to_lowercase().contains("wifi")).count();
            let audio_drivers = self.drivers.iter().filter(|d| d.class_name.to_lowercase().contains("audio") || d.class_name.to_lowercase().contains("sound") || d.class_name.to_lowercase().contains("media")).count();

            fn draw_kpi(ui: &mut Ui, title: &str, count: usize, color: Color32) {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(170.0, 56.0), Sense::hover());
                ui.painter().rect_filled(rect, CornerRadius::same(8), RustCareTheme::BG_SURFACE);
                ui.painter().rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE), StrokeKind::Inside);
                ui.painter().text(rect.min + Vec2::new(12.0, 8.0), Align2::LEFT_TOP, title, FontId::proportional(11.0), RustCareTheme::TEXT_MUTED);
                ui.painter().text(rect.min + Vec2::new(12.0, 24.0), Align2::LEFT_TOP, count.to_string(), FontId::proportional(20.0), color);
            }

            draw_kpi(ui, "Total Controladores", total_drivers, RustCareTheme::ACCENT_CYAN);
            ui.add_space(8.0);
            draw_kpi(ui, "Gráficos / GPU", gpu_drivers, RustCareTheme::ACCENT_INDIGO);
            ui.add_space(8.0);
            draw_kpi(ui, "Adaptadores de Red", net_drivers, RustCareTheme::ACCENT_TEAL);
            ui.add_space(8.0);
            draw_kpi(ui, "Audio & Multimedia", audio_drivers, RustCareTheme::SUCCESS_GREEN);
        });
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.label("Filtrar controlador:");
            ui.text_edit_singleline(&mut self.driver_search);
            if ghost_button(ui, "Reanalizar Hardware", Some(160.0)).clicked() {
                self.drivers = DriverManager::scan_installed_drivers();
            }
            if neon_button(ui, "Crear Copia de Respaldo Completa", Some(230.0)).clicked() {
                match DriverManager::backup_drivers() {
                    Ok(res) => {
                        self.driver_backup_msg = Some(format!(
                            "Copia de seguridad completada. {} controladores respaldados en: {}",
                            res.drivers_backed_up,
                            res.backup_path.display()
                        ));
                    }
                    Err(e) => {
                        self.driver_backup_msg = Some(format!("Error al crear respaldo: {}", e));
                    }
                }
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.driver_backup_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    RustCareTheme::draw_badge(
                        ui,
                        msg,
                        RustCareTheme::SUCCESS_GREEN,
                        Color32::from_rgba_unmultiplied(16, 185, 129, 25),
                    );
                });
            });
            ui.add_space(6.0);
        }

        ui.label(RichText::new(format!("Se auditaron {} controladores de hardware instalados:", self.drivers.len())).strong().color(RustCareTheme::ACCENT_CYAN));
        ui.add_space(6.0);

        let filtered: Vec<&DeviceDriverInfo> = self
            .drivers
            .iter()
            .filter(|d| {
                self.driver_search.is_empty()
                    || d.name.to_lowercase().contains(&self.driver_search.to_lowercase())
                    || d.class_name.to_lowercase().contains(&self.driver_search.to_lowercase())
            })
            .collect();

        ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::BG_SURFACE)
                .corner_radius(8)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    for drv in filtered {
                        ui.horizontal(|ui| {
                            let (badge_text, badge_color) = if drv.class_name.to_lowercase().contains("pantalla") || drv.class_name.to_lowercase().contains("display") || drv.class_name.to_lowercase().contains("gpu") {
                                ("GPU", RustCareTheme::ACCENT_INDIGO)
                            } else if drv.class_name.to_lowercase().contains("net") || drv.class_name.to_lowercase().contains("red") || drv.class_name.to_lowercase().contains("wifi") {
                                ("Red", RustCareTheme::ACCENT_TEAL)
                            } else if drv.class_name.to_lowercase().contains("audio") || drv.class_name.to_lowercase().contains("sound") || drv.class_name.to_lowercase().contains("media") {
                                ("Audio", RustCareTheme::SUCCESS_GREEN)
                            } else {
                                ("Sistema", RustCareTheme::TEXT_DIM)
                            };

                            RustCareTheme::draw_badge(ui, badge_text, badge_color, Color32::from_rgba_unmultiplied(badge_color.r(), badge_color.g(), badge_color.b(), 25));
                            ui.add_space(4.0);

                            ui.vertical(|ui| {
                                ui.label(RichText::new(&drv.name).strong());
                                ui.label(RichText::new(format!("Clase: {} | Fabricante: {}", drv.class_name, drv.provider)).small().color(RustCareTheme::TEXT_MUTED));
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("v{} ({})", drv.version, drv.date)).small().color(RustCareTheme::SUCCESS_GREEN));
                            });
                        });
                        ui.separator();
                    }
                });
        });
    }
}

