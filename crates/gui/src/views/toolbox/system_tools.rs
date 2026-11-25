use super::*;

impl ToolboxView {
    pub(super) fn render_shredder(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Destrucción Segura de Archivos (Sobrescritura irreversible con ceros antes de borrar):").strong());
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Ruta del archivo a destruir:");
            ui.text_edit_singleline(&mut self.shred_target_path);
            if danger_button(ui, "Destruir Archivo Irreversiblemente", Some(260.0)).clicked() {
                if std::path::Path::new(&self.shred_target_path).exists() {
                    match FileShredder::shred_file(&self.shred_target_path) {
                        Ok(_) => self.shred_status_msg = Some("Archivo destruido con éxito. Imposible de recuperar.".to_string()),
                        Err(e) => self.shred_status_msg = Some(format!("Error: {}", e)),
                    }
                } else {
                    self.shred_status_msg = Some("El archivo no existe.".to_string());
                }
            }
        });

        if let Some(msg) = &self.shred_status_msg {
            ui.add_space(6.0);
            ui.label(RichText::new(msg).color(RustCareTheme::WARNING_AMBER));
        }
    }

    pub(super) fn render_context_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Gestión de Elementos de Clic Derecho (Shell Extensions):").strong());
            if ghost_button(ui, "Refrescar", Some(100.0)).clicked() {
                self.context_handlers = ContextMenuManager::list_handlers();
            }
        });
        ui.add_space(8.0);

        ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::BG_SURFACE)
                .corner_radius(8)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    for item in &self.context_handlers.clone() {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&item.name).strong());
                                ui.label(RichText::new(&item.key_path).small().color(RustCareTheme::TEXT_MUTED));
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if item.is_user_specific {
                                    let btn_txt = if item.is_enabled { "Deshabilitar" } else { "Habilitar" };
                                    if ghost_button(ui, btn_txt, Some(100.0)).clicked() {
                                        let _ = ContextMenuManager::toggle_handler(item);
                                        self.context_handlers = ContextMenuManager::list_handlers();
                                    }
                                } else {
                                    ui.label(RichText::new("[Global]").small().color(RustCareTheme::TEXT_MUTED));
                                }
                            });
                        });
                        ui.separator();
                    }
                });
        });
    }

    pub(super) fn render_updater(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if neon_button(ui, "Comprobar Actualizaciones (WinGet)", Some(260.0)).clicked() {
                if let Ok(updates) = UpdaterEngine::check_updates() {
                    self.updates = updates;
                    self.winget_status_msg = Some(format!("Se encontraron {} actualizaciones.", self.updates.len()));
                } else {
                    self.winget_status_msg = Some("WinGet no está disponible o devolvió un error.".to_string());
                }
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.winget_status_msg {
            ui.label(RichText::new(msg).small().color(RustCareTheme::ACCENT_CYAN));
            ui.add_space(6.0);
        }

        ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::BG_SURFACE)
                .corner_radius(8)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    if self.updates.is_empty() {
                        ui.label("No hay actualizaciones pendientes o no se ha realizado la comprobación.");
                        return;
                    }

                    for update in &self.updates {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&update.name).strong());
                                ui.label(RichText::new(format!("Actual: {} -> Nueva: {}", update.installed_version, update.available_version)).small().color(RustCareTheme::SUCCESS_GREEN));
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if neon_button(ui, "Actualizar", Some(100.0)).clicked() {
                                    let _ = UpdaterEngine::update_package(&update.id);
                                }
                            });
                        });
                        ui.separator();
                    }
                });
        });
    }

    pub(super) fn render_rescue(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Centro de Rescate y Restauración de Copias de Seguridad:").strong());
            if ghost_button(ui, "Actualizar Lista", Some(130.0)).clicked() {
                self.backups = RescueCenter::list_backups();
            }
            if neon_button(ui, "Crear Punto de Restauración", Some(220.0)).clicked() {
                match RescueCenter::create_restore_point("Punto de restauración manual de RustCare") {
                    Ok(info) => {
                        self.rescue_status_msg = Some(format!(
                            "Punto de restauración '{}' creado con éxito. ({})",
                            info.description,
                            if info.is_system_vss { "VSS Windows Activado" } else { "Modo Seguro Usuario" }
                        ));
                        self.backups = RescueCenter::list_backups();
                    }
                    Err(e) => {
                        self.rescue_status_msg = Some(format!("Error: {}", e));
                    }
                }
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.rescue_status_msg {
            ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            ui.add_space(6.0);
        }

        ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::BG_SURFACE)
                .corner_radius(8)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    if self.backups.is_empty() {
                        ui.label("No hay copias de seguridad de registro almacenadas.");
                        return;
                    }

                    for backup in &self.backups {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&backup.file_name).strong());
                                ui.label(RichText::new(format!("Tamaño: {} | {}", CleanerEngine::format_bytes(backup.size_bytes), backup.created_at)).small().color(RustCareTheme::TEXT_MUTED));
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if danger_button(ui, "Restaurar Backup", Some(140.0)).clicked() {
                                    match RescueCenter::restore_backup(backup) {
                                        Ok(true) => self.rescue_status_msg = Some(format!("Copia {} restaurada con éxito.", backup.file_name)),
                                        _ => self.rescue_status_msg = Some("Error al importar el archivo .reg".to_string()),
                                    }
                                }
                            });
                        });
                        ui.separator();
                    }
                });
        });
    }

    pub(super) fn render_network_booster(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Acelerador de Red e Internet TCP/IP").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Optimiza la latencia de ping, throughput de descarga y parámetros del kernel de red (Window Auto-Tuning, ECN, RSS).");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if neon_button(ui, "Aplicar Optimización de Red", Some(210.0)).clicked() {
                match NetworkBooster::apply_optimizations() {
                    Ok(res) => {
                        self.network_msg = Some(format!(
                            "Red optimizada con éxito. Respaldo guardado en: {}",
                            res.backup_file.display()
                        ));
                    }
                    Err(e) => {
                        self.network_msg = Some(format!("Resultado: {}", e));
                    }
                }
            }

            if ghost_button(ui, "Restaurar Valores Predeterminados de Red", Some(270.0)).clicked() {
                match NetworkBooster::restore_defaults() {
                    Ok(_) => self.network_msg = Some("Se restauraron los valores estándar del kernel TCP/IP de Windows.".to_string()),
                    Err(e) => self.network_msg = Some(format!("Error: {}", e)),
                }
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.network_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
        }
    }

    pub(super) fn render_disk_optimizer(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Desfragmentador y TRIM de Discos SSD/NVMe").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Optimiza celdas flash enviando comandos TRIM periódicos y audita el estado del sistema de archivos.");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if neon_button(ui, "Ejecutar TRIM en Unidad C:", Some(210.0)).clicked() {
                let res = DiskOptimizer::trim_drive("C");
                self.disk_trim_msg = Some(format!(
                    "TRIM ejecutado en {}: {}",
                    res.drive_letter,
                    res.output_message
                ));
            }

            if ghost_button(ui, "Chequeo de Integridad del Disco C:", Some(250.0)).clicked() {
                let res = DiskOptimizer::check_filesystem_integrity("C");
                self.disk_trim_msg = Some(format!(
                    "Estado del Sistema de Archivos {}: {}",
                    res.drive_letter,
                    if !res.is_dirty { "Limpio, sin sectores marcados como corruptos." } else { "Marcado con anomalías (se sugiere chkdsk)." }
                ));
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.disk_trim_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
        }
    }

    pub(super) fn render_browser_vacuum(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Compactador SQLite de Navegadores (Browser VACUUM)").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Elimina fragmentación en bases de datos de Chrome, Edge, Brave y Firefox sin borrar historial ni contraseñas.");
        ui.add_space(8.0);

        if neon_button(ui, "Compactar y Desfragmentar Todas las Bases de Datos", Some(360.0)).clicked() {
            self.vacuum_results = BrowserVacuum::compact_all_browsers();
            let total_freed: u64 = self.vacuum_results.iter().map(|r| r.bytes_freed).sum();
            self.vacuum_msg = Some(format!(
                "Compactadas {} bases de datos. Espacio recuperado: {}",
                self.vacuum_results.len(),
                CleanerEngine::format_bytes(total_freed)
            ));
        }
        ui.add_space(8.0);

        if let Some(msg) = &self.vacuum_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
            ui.add_space(6.0);
        }

        if !self.vacuum_results.is_empty() {
            ScrollArea::vertical().show(ui, |ui| {
                RustCareTheme::card_frame().show(ui, |ui| {
                    for res in &self.vacuum_results {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&res.browser_name).strong());
                            ui.label(format!("(Reducido de {} a {})", CleanerEngine::format_bytes(res.original_size_bytes), CleanerEngine::format_bytes(res.compacted_size_bytes)));
                        });
                        ui.separator();
                    }
                });
            });
        }
    }

    pub(super) fn render_ui_repair(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Reparador de Caché de Iconos de Windows").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Repara iconos en blanco, corrupciones visuales en el escritorio y miniaturas reiniciando el explorador de forma limpia.");
        ui.add_space(8.0);

        if neon_button(ui, "Reconstruir Caché de Iconos y Miniaturas", Some(300.0)).clicked() {
            match UiRepair::rebuild_icon_cache() {
                Ok(res) => {
                    self.ui_repair_msg = Some(format!(
                        "Caché de iconos reconstruido con éxito ({} archivos recreados). {}",
                        res.files_purged, res.message
                    ));
                }
                Err(e) => self.ui_repair_msg = Some(format!("Error: {}", e)),
            }
        }
        ui.add_space(8.0);

        if let Some(msg) = &self.ui_repair_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
        }
    }

    pub(super) fn render_system_info(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Ficha de Especificaciones de Hardware y SO").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Resumen completo de CPU, Memoria RAM, Placas Gráficas, Discos y versión de Windows.");
        ui.add_space(8.0);

        if self.system_report.is_none() {
            self.system_report = Some(SystemInfoHub::get_report());
        }

        ui.horizontal(|ui| {
            if ghost_button(ui, "Actualizar Datos", Some(140.0)).clicked() {
                self.system_report = Some(SystemInfoHub::get_report());
            }

            if ghost_button(ui, "Copiar Reporte Completo", Some(180.0)).clicked() {
                let txt = SystemInfoHub::export_text_report();
                ui.ctx().copy_text(txt);
            }
        });
        ui.add_space(8.0);

        if let Some(rep) = &self.system_report {
            ScrollArea::vertical().show(ui, |ui| {
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.label(RichText::new(format!("Sistema Operativo: {} ({})", rep.os_name, rep.os_version)).strong());
                    ui.label(RichText::new(format!("Equipo / Host: {}", rep.host_name)).color(RustCareTheme::TEXT_MUTED));
                    ui.separator();
                    ui.label(RichText::new(format!("Procesador (CPU): {}", rep.cpu_brand)).strong());
                    ui.label(format!("Núcleos lógicos: {}", rep.cpu_cores));
                    ui.separator();
                    ui.label(RichText::new(format!("Memoria RAM Total: {} (Disponible: {})", rep.memory_total, rep.memory_free)).strong());
                    ui.separator();
                    ui.label(RichText::new("Unidades de Almacenamiento:").strong());
                    for d in &rep.storage_units {
                        ui.label(format!("- {}", d));
                    }
                    if !rep.gpus.is_empty() {
                        ui.separator();
                        ui.label(RichText::new("Controladores Gráficos (GPU):").strong());
                        for g in &rep.gpus {
                            ui.label(format!("- {}", g));
                        }
                    }
                });
            });
        }
    }

    pub(super) fn render_power_plan(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Selector de Planes de Energía").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Cambia el esquema de energía de Windows para ahorro de batería o máxima velocidad sin limitaciones.");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if ghost_button(ui, "Refrescar Planes", Some(140.0)).clicked() {
                self.power_schemes = PowerPlanOptimizer::list_schemes();
            }

            if neon_button(ui, "Desbloquear Máximo Rendimiento", Some(250.0)).clicked() {
                match PowerPlanOptimizer::unlock_ultimate_performance() {
                    Ok(guid) => {
                        self.power_msg = Some(format!("Plan Máximo Rendimiento habilitado. (GUID: {})", guid));
                        self.power_schemes = PowerPlanOptimizer::list_schemes();
                    }
                    Err(e) => self.power_msg = Some(format!("Resultado: {}", e)),
                }
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.power_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
            ui.add_space(6.0);
        }

        let mut activate_guid_and_name = None;

        ScrollArea::vertical().show(ui, |ui| {
            RustCareTheme::card_frame().show(ui, |ui| {
                for scheme in &self.power_schemes {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let label = if scheme.is_active {
                                RichText::new(format!("[Activo] {}", scheme.name)).strong().color(RustCareTheme::ACCENT_CYAN)
                            } else {
                                RichText::new(&scheme.name).strong()
                            };
                            ui.label(label);
                            ui.label(RichText::new(&scheme.guid).small().color(RustCareTheme::TEXT_MUTED));
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if !scheme.is_active && neon_button(ui, "Activar", Some(90.0)).clicked() {
                                activate_guid_and_name = Some((scheme.guid.clone(), scheme.name.clone()));
                            }
                        });
                    });
                    ui.separator();
                }
            });
        });

        if let Some((guid, name)) = activate_guid_and_name {
            match PowerPlanOptimizer::set_active_scheme(&guid) {
                Ok(_) => {
                    self.power_msg = Some(format!("Plan '{}' activado con éxito.", name));
                    self.power_schemes = PowerPlanOptimizer::list_schemes();
                }
                Err(e) => self.power_msg = Some(format!("Error: {}", e)),
            }
        }
    }

    pub(super) fn render_dism_winsxs(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Limpieza Profunda del Almacén de Componentes (WinSxS / DISM)").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Invoca la herramienta oficial de mantenimiento de Windows (Dism.exe) para purgar copias de seguridad de actualizaciones antiguas. Puede liberar entre 5 GB y 20 GB de disco.");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if neon_button(ui, "Ejecutar StartComponentCleanup", Some(240.0)).clicked() {
                let res = DismEngine::start_component_cleanup(false);
                self.dism_msg = Some(res.output_message);
            }

            if danger_button(ui, "Limpieza Definitiva con /ResetBase", Some(260.0)).clicked() {
                let res = DismEngine::start_component_cleanup(true);
                self.dism_msg = Some(res.output_message);
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.dism_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
        }
    }

    pub(super) fn render_hibernation(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Gestor del Archivo de Hibernación (hiberfil.sys)").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Controla el archivo de hibernación de Windows. Si no usas hibernación en tu PC, desactivarlo libera inmediatamente tantos gigabytes en el SSD como gigas de memoria RAM tengas.");
        ui.add_space(8.0);

        if self.hibernation_status.is_none() {
            self.hibernation_status = Some(HibernationManager::get_status());
        }

        if let Some(stat) = &self.hibernation_status {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("Estado de Hibernación: {}", if stat.is_enabled { "Activada" } else { "Desactivada" })).strong());
                    ui.label(format!("| Ahorro potencial: {}", stat.potential_savings));
                });
            });
            ui.add_space(8.0);
        }

        ui.horizontal(|ui| {
            if danger_button(ui, "Desactivar Hibernación (Liberar SSD)", Some(260.0)).clicked() {
                match HibernationManager::disable_hibernation() {
                    Ok(true) => {
                        self.hibernation_msg = Some("Hibernación desactivada. Archivo hiberfil.sys eliminado del SSD.".into());
                        self.hibernation_status = Some(HibernationManager::get_status());
                    }
                    _ => self.hibernation_msg = Some("Error al desactivar (se requieren permisos de Administrador).".into()),
                }
            }

            if ghost_button(ui, "Modo Reducido (Fast Startup con 50% de tamaño)", Some(330.0)).clicked() {
                match HibernationManager::set_reduced_hibernation() {
                    Ok(true) => {
                        self.hibernation_msg = Some("Modo de hibernación reducida configurado con éxito.".into());
                        self.hibernation_status = Some(HibernationManager::get_status());
                    }
                    _ => self.hibernation_msg = Some("Error al configurar modo reducido.".into()),
                }
            }

            if neon_button(ui, "Reactivar Hibernación", Some(180.0)).clicked() {
                match HibernationManager::enable_hibernation() {
                    Ok(true) => {
                        self.hibernation_msg = Some("Hibernación activada.".into());
                        self.hibernation_status = Some(HibernationManager::get_status());
                    }
                    _ => self.hibernation_msg = Some("Error al activar hibernación.".into()),
                }
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.hibernation_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
        }
    }

    pub(super) fn render_hardware_troubleshooter(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Diagnóstico de Hardware y Conflictos de Dispositivos").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Audita el Administrador de Dispositivos y detecta controladores con fallos (Código 10, 43, 28) con opción de reinicio del dispositivo PnP.");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Filtrar dispositivos:");
            ui.text_edit_singleline(&mut self.problem_search);
            if neon_button(ui, "Escanear Dispositivos con Error", Some(230.0)).clicked() {
                self.problem_devices = HardwareTroubleshooter::scan_problem_devices();
                self.hardware_msg = Some(format!(
                    "Escaneo finalizado: {} dispositivos con advertencias encontrados.",
                    self.problem_devices.len()
                ));
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.hardware_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
            ui.add_space(6.0);
        }

        let filtered: Vec<&ProblemDevice> = self
            .problem_devices
            .iter()
            .filter(|d| self.problem_search.is_empty() || d.description.to_lowercase().contains(&self.problem_search.to_lowercase()) || d.instance_id.to_lowercase().contains(&self.problem_search.to_lowercase()))
            .collect();

        ScrollArea::vertical().show(ui, |ui| {
            RustCareTheme::card_frame().show(ui, |ui| {
                if filtered.is_empty() {
                    ui.label("No se detectaron dispositivos de hardware con código de error.");
                    return;
                }

                for dev in filtered {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(&dev.description).strong().color(RustCareTheme::WARNING_AMBER));
                            ui.label(RichText::new(format!("ID: {} | Código: {} ({})", dev.instance_id, dev.problem_code, dev.status)).small().color(RustCareTheme::TEXT_MUTED));
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ghost_button(ui, "Reiniciar Dispositivo", Some(160.0)).clicked() {
                                match HardwareTroubleshooter::restart_device(&dev.instance_id) {
                                    Ok(true) => self.hardware_msg = Some(format!("Dispositivo '{}' reiniciado con éxito.", dev.description)),
                                    _ => self.hardware_msg = Some("No se pudo reiniciar el dispositivo (requiere permisos de Administrador).".to_string()),
                                }
                            }
                        });
                    });
                    ui.separator();
                }
            });
        });
    }

    pub(super) fn render_scheduled_tasks(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("Auditor de Tareas Programadas Huérfanas de Windows").size(18.0).color(RustCareTheme::ACCENT_CYAN));
        ui.label("Inspecciona el Programador de Tareas (Task Scheduler) e identifica tareas residuales dejadas por programas desinstalados cuyos ejecutables ya no existen.");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Buscar tarea:");
            ui.text_edit_singleline(&mut self.tasks_search);
            if ghost_button(ui, "Analizar Tareas", Some(140.0)).clicked() {
                self.scheduled_tasks = ScheduledTaskManager::list_tasks();
                let orphan_count = self.scheduled_tasks.iter().filter(|t| t.is_orphaned).count();
                self.tasks_msg = Some(format!(
                    "Análisis completado: {} tareas auditadas, {} huérfanas encontradas.",
                    self.scheduled_tasks.len(),
                    orphan_count
                ));
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.tasks_msg {
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            });
            ui.add_space(6.0);
        }

        let filtered: Vec<&ScheduledTaskItem> = self
            .scheduled_tasks
            .iter()
            .filter(|t| self.tasks_search.is_empty() || t.task_name.to_lowercase().contains(&self.tasks_search.to_lowercase()))
            .collect();

        let mut delete_task_name = None;

        ScrollArea::vertical().show(ui, |ui| {
            RustCareTheme::card_frame().show(ui, |ui| {
                if filtered.is_empty() {
                    ui.label("Presiona 'Analizar Tareas' para auditar las tareas programadas.");
                    return;
                }

                for task in filtered {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let title = if task.is_orphaned {
                                RichText::new(format!("[Huérfana] {}", task.task_name)).strong().color(RustCareTheme::WARNING_AMBER)
                            } else {
                                RichText::new(&task.task_name).strong()
                            };
                            ui.label(title);
                            ui.label(RichText::new(format!("Comando: {} | Estado: {}", task.action_exec, task.status)).small().color(RustCareTheme::TEXT_MUTED));
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if task.is_orphaned && danger_button(ui, "Eliminar", Some(90.0)).clicked() {
                                delete_task_name = Some(task.task_name.clone());
                            }
                        });
                    });
                    ui.separator();
                }
            });
        });

        if let Some(name) = delete_task_name {
            match ScheduledTaskManager::delete_task(&name) {
                Ok(true) => {
                    self.tasks_msg = Some(format!("Tarea '{}' eliminada con éxito.", name));
                    self.scheduled_tasks = ScheduledTaskManager::list_tasks();
                }
                _ => self.tasks_msg = Some("Error al eliminar la tarea (requiere permisos de Administrador).".into()),
            }
        }
    }
}

