//! Vista de Herramientas Completas (Toolbox PRO)
use crate::theme::RustCareTheme;
use eframe::egui::{self, ProgressBar, RichText, ScrollArea, Ui};
use rustcare_core::cleaner::CleanerEngine;
use rustcare_core::context_menu::{ContextMenuItem, ContextMenuManager};
use rustcare_core::disk_analyzer::{DiskAnalysisResult, DiskAnalyzer};
use rustcare_core::drivers::{DeviceDriverInfo, DriverManager};
use rustcare_core::duplicates::{DuplicateFinder, DuplicateScanResult};
use rustcare_core::empty_folders::{EmptyFolderItem, EmptyFoldersFinder};
use rustcare_core::large_files::{LargeFileItem, LargeFilesFinder};
use rustcare_core::rescue::{BackupEntry, RescueCenter};
use rustcare_core::shortcuts::{BrokenShortcut, ShortcutCleaner};
use rustcare_core::shredder::FileShredder;
use rustcare_core::uninstaller::{InstalledApp, ResidualFolder, UninstallerEngine};
use rustcare_core::updater::{UpdaterEngine, WingetPackageUpdate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolboxTab {
    Uninstaller,
    DriverBooster,
    Duplicates,
    DiskAnalyzer,
    LargeFiles,
    EmptyFolders,
    Shortcuts,
    FileShredder,
    ContextMenu,
    Updater,
    RescueCenter,
}

pub struct ToolboxView {
    pub current_tab: ToolboxTab,

    // Desinstalador
    pub apps: Vec<InstalledApp>,
    pub app_search: String,
    pub residuals: Vec<ResidualFolder>,
    pub residual_app_name: String,

    // Drivers
    pub drivers: Vec<DeviceDriverInfo>,
    pub driver_search: String,

    // Duplicados
    pub duplicate_path: String,
    pub duplicate_result: Option<DuplicateScanResult>,

    // Analizador de disco
    pub disk_path: String,
    pub disk_result: Option<DiskAnalysisResult>,

    // Archivos Grandes
    pub large_files_path: String,
    pub large_files_min_mb: u64,
    pub large_files: Vec<LargeFileItem>,

    // Carpetas Vacías
    pub empty_folders_path: String,
    pub empty_folders: Vec<EmptyFolderItem>,

    // Accesos Directos
    pub broken_shortcuts: Vec<BrokenShortcut>,
    pub shortcuts_scanned: bool,

    // Destructor de Archivos
    pub shred_target_path: String,
    pub shred_status_msg: Option<String>,

    // Menú Contextual
    pub context_handlers: Vec<ContextMenuItem>,

    // Actualizador WinGet
    pub updates: Vec<WingetPackageUpdate>,
    pub winget_status_msg: Option<String>,

    // Centro de Rescate
    pub backups: Vec<BackupEntry>,
    pub rescue_status_msg: Option<String>,
}

impl Default for ToolboxView {
    fn default() -> Self {
        let default_path = std::env::var("USERPROFILE")
            .map(|p| format!("{}\\Downloads", p))
            .unwrap_or_else(|_| "C:\\".to_string());

        Self {
            current_tab: ToolboxTab::Uninstaller,
            apps: UninstallerEngine::list_installed_apps(),
            app_search: String::new(),
            residuals: Vec::new(),
            residual_app_name: String::new(),
            drivers: DriverManager::scan_installed_drivers(),
            driver_search: String::new(),
            duplicate_path: default_path.clone(),
            duplicate_result: None,
            disk_path: "C:\\".to_string(),
            disk_result: None,
            large_files_path: default_path.clone(),
            large_files_min_mb: 50,
            large_files: Vec::new(),
            empty_folders_path: default_path,
            empty_folders: Vec::new(),
            broken_shortcuts: Vec::new(),
            shortcuts_scanned: false,
            shred_target_path: String::new(),
            shred_status_msg: None,
            context_handlers: ContextMenuManager::list_handlers(),
            updates: Vec::new(),
            winget_status_msg: None,
            backups: RescueCenter::list_backups(),
            rescue_status_msg: None,
        }
    }
}

impl ToolboxView {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.heading(RichText::new("🧰 Caja de Herramientas de Mantenimiento PRO").size(22.0).color(RustCareTheme::TEXT_PRIMARY));
        ui.label(RichText::new("Conjunto de utilidades de ingeniería nativa: drivers, desinstalador, duplicados, archivos gigantes, destructor y rescate.").color(RustCareTheme::TEXT_MUTED));
        ui.add_space(8.0);

        // Barra de pestañas horizontales con scroll si es necesario
        ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                let tabs = [
                    (ToolboxTab::Uninstaller, "📦 Desinstalador"),
                    (ToolboxTab::DriverBooster, "🚗 Driver Booster"),
                    (ToolboxTab::Duplicates, "👥 Duplicados"),
                    (ToolboxTab::DiskAnalyzer, "📊 Analizador Disco"),
                    (ToolboxTab::LargeFiles, "🐘 Archivos Gigantes"),
                    (ToolboxTab::EmptyFolders, "📂 Carpetas Vacías"),
                    (ToolboxTab::Shortcuts, "🔗 Accesos Directos"),
                    (ToolboxTab::FileShredder, "🛡️ Destructor Seguro"),
                    (ToolboxTab::ContextMenu, "🖱️ Menú Contextual"),
                    (ToolboxTab::Updater, "🔄 WinGet Updater"),
                    (ToolboxTab::RescueCenter, "🛟 Centro de Rescate"),
                ];

                for (tab, label) in tabs {
                    if ui.selectable_label(self.current_tab == tab, label).clicked() {
                        self.current_tab = tab;
                    }
                }
            });
        });
        ui.separator();
        ui.add_space(8.0);

        match self.current_tab {
            ToolboxTab::Uninstaller => self.render_uninstaller(ui),
            ToolboxTab::DriverBooster => self.render_drivers(ui),
            ToolboxTab::Duplicates => self.render_duplicates(ui),
            ToolboxTab::DiskAnalyzer => self.render_disk_analyzer(ui),
            ToolboxTab::LargeFiles => self.render_large_files(ui),
            ToolboxTab::EmptyFolders => self.render_empty_folders(ui),
            ToolboxTab::Shortcuts => self.render_shortcuts(ui),
            ToolboxTab::FileShredder => self.render_shredder(ui),
            ToolboxTab::ContextMenu => self.render_context_menu(ui),
            ToolboxTab::Updater => self.render_updater(ui),
            ToolboxTab::RescueCenter => self.render_rescue(ui),
        }
    }

    fn render_uninstaller(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Buscar aplicación:");
            ui.text_edit_singleline(&mut self.app_search);
            if ui.button("🔄 Refrescar").clicked() {
                self.apps = UninstallerEngine::list_installed_apps();
            }
        });
        ui.add_space(8.0);

        if !self.residuals.is_empty() {
            ui.label(RichText::new(format!("Carpetas residuales encontradas para '{}':", self.residual_app_name)).strong().color(RustCareTheme::WARNING_AMBER));
            for r in &self.residuals {
                ui.label(format!("• {} ({})", r.path.display(), CleanerEngine::format_bytes(r.size_bytes)));
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
                .fill(RustCareTheme::CARD_BG)
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
                                if ui.button("Desinstalar").clicked() {
                                    let _ = UninstallerEngine::run_uninstaller(app);
                                }
                                if ui.button("🔍 Rastrear Residuos").clicked() {
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

    fn render_drivers(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Filtrar controlador:");
            ui.text_edit_singleline(&mut self.driver_search);
            if ui.button("🔄 Reanalizar Hardware").clicked() {
                self.drivers = DriverManager::scan_installed_drivers();
            }
        });
        ui.add_space(8.0);

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
                .fill(RustCareTheme::CARD_BG)
                .corner_radius(8)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    for drv in filtered {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&drv.name).strong());
                                ui.label(RichText::new(format!("Tipo: {} | Fabricante: {}", drv.class_name, drv.provider)).small().color(RustCareTheme::TEXT_MUTED));
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

    fn render_duplicates(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Ruta a escanear:");
            ui.text_edit_singleline(&mut self.duplicate_path);
            if ui.button("🔍 Buscar Duplicados").clicked() {
                self.duplicate_result = Some(DuplicateFinder::scan_directory(&self.duplicate_path));
            }
        });
        ui.add_space(10.0);

        if let Some(res) = &self.duplicate_result {
            ui.label(
                RichText::new(format!(
                    "Archivos duplicados encontrados: {} | Espacio desperdiciado: {}",
                    res.total_duplicate_files,
                    CleanerEngine::format_bytes(res.total_wasted_bytes)
                ))
                .strong()
                .color(RustCareTheme::WARNING_AMBER),
            );
            ui.add_space(8.0);

            ScrollArea::vertical().show(ui, |ui| {
                for group in &res.groups {
                    egui::Frame::NONE
                        .fill(RustCareTheme::CARD_BG)
                        .corner_radius(8)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new(format!("Tamaño: {} | Hash: {}", CleanerEngine::format_bytes(group.file_size), &group.blake3_hash[..12])).strong().color(RustCareTheme::ACCENT_CYAN));
                            for file in &group.files {
                                ui.label(format!("• {}", file.display()));
                            }
                        });
                    ui.add_space(6.0);
                }
            });
        }
    }

    fn render_disk_analyzer(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Directorio raíz:");
            ui.text_edit_singleline(&mut self.disk_path);
            if ui.button("📊 Analizar Estilo WizTree").clicked() {
                self.disk_result = Some(DiskAnalyzer::analyze_directory(&self.disk_path));
            }
        });
        ui.add_space(10.0);

        if let Some(res) = &self.disk_result {
            ui.label(RichText::new(format!("Tamaño total en '{}': {}", res.root_path.display(), CleanerEngine::format_bytes(res.total_size_bytes))).strong().color(RustCareTheme::ACCENT_CYAN));
            ui.add_space(8.0);

            ScrollArea::vertical().show(ui, |ui| {
                egui::Frame::NONE
                    .fill(RustCareTheme::CARD_BG)
                    .corner_radius(8)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        for item in &res.items {
                            let ratio = if res.total_size_bytes > 0 {
                                item.size_bytes as f32 / res.total_size_bytes as f32
                            } else {
                                0.0
                            };

                            ui.horizontal(|ui| {
                                let icon = if item.is_directory { "📁" } else { "📄" };
                                ui.label(format!("{} {}", icon, item.name));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(CleanerEngine::format_bytes(item.size_bytes)).strong());
                                });
                            });
                            ui.add(ProgressBar::new(ratio).text(format!("{:.1}%", ratio * 100.0)));
                            ui.separator();
                        }
                    });
            });
        }
    }

    fn render_large_files(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Ruta:");
            ui.text_edit_singleline(&mut self.large_files_path);
            ui.label("Tamaño Mínimo (MB):");
            ui.add(egui::DragValue::new(&mut self.large_files_min_mb).speed(10).range(10..=10000));
            if ui.button("🔍 Escanear Archivos Gigantes").clicked() {
                self.large_files = LargeFilesFinder::scan_directory(&self.large_files_path, self.large_files_min_mb * 1024 * 1024);
            }
        });
        ui.add_space(8.0);

        if !self.large_files.is_empty() {
            ui.label(RichText::new(format!("Se encontraron {} archivos superiores a {} MB:", self.large_files.len(), self.large_files_min_mb)).strong().color(RustCareTheme::WARNING_AMBER));
            ui.add_space(6.0);

            ScrollArea::vertical().show(ui, |ui| {
                egui::Frame::NONE
                    .fill(RustCareTheme::CARD_BG)
                    .corner_radius(8)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        for item in &self.large_files.clone() {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(&item.name).strong());
                                    ui.label(RichText::new(format!("Categoría: {} | Ruta: {}", item.category.display_name(), item.path.display())).small().color(RustCareTheme::TEXT_MUTED));
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(CleanerEngine::format_bytes(item.size_bytes)).strong().color(RustCareTheme::ACCENT_CYAN));
                                });
                            });
                            ui.separator();
                        }
                    });
            });
        }
    }

    fn render_empty_folders(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Ruta a explorar:");
            ui.text_edit_singleline(&mut self.empty_folders_path);
            if ui.button("🔍 Buscar Carpetas Vacías").clicked() {
                self.empty_folders = EmptyFoldersFinder::scan_directory(&self.empty_folders_path);
            }
            if !self.empty_folders.is_empty() {
                if ui.button("🗑️ Eliminar Todas").clicked() {
                    for f in &self.empty_folders {
                        let _ = EmptyFoldersFinder::delete_folder(f);
                    }
                    self.empty_folders.clear();
                }
            }
        });
        ui.add_space(8.0);

        if !self.empty_folders.is_empty() {
            ui.label(RichText::new(format!("Se encontraron {} carpetas vacías:", self.empty_folders.len())).strong().color(RustCareTheme::WARNING_AMBER));
            ScrollArea::vertical().show(ui, |ui| {
                egui::Frame::NONE
                    .fill(RustCareTheme::CARD_BG)
                    .corner_radius(8)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        for item in &self.empty_folders {
                            ui.label(format!("📁 {}", item.path.display()));
                        }
                    });
            });
        }
    }

    fn render_shortcuts(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("🔍 Escanear Accesos Directos Rotos").clicked() {
                self.broken_shortcuts = ShortcutCleaner::scan_broken_shortcuts();
                self.shortcuts_scanned = true;
            }
            if !self.broken_shortcuts.is_empty() {
                if ui.button("🗑️ Eliminar Todos los Rotos").clicked() {
                    for item in &self.broken_shortcuts {
                        let _ = ShortcutCleaner::remove_broken_shortcut(item);
                    }
                    self.broken_shortcuts.clear();
                }
            }
        });
        ui.add_space(8.0);

        if self.shortcuts_scanned {
            if self.broken_shortcuts.is_empty() {
                ui.label(RichText::new("✅ No se detectaron accesos directos rotos en el Escritorio o Menú Inicio.").color(RustCareTheme::SUCCESS_GREEN));
            } else {
                ui.label(RichText::new(format!("Se encontraron {} accesos directos que apuntan a archivos inexistentes:", self.broken_shortcuts.len())).strong().color(RustCareTheme::WARNING_AMBER));
                ui.add_space(6.0);

                ScrollArea::vertical().show(ui, |ui| {
                    egui::Frame::NONE
                        .fill(RustCareTheme::CARD_BG)
                        .corner_radius(8)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            for item in &self.broken_shortcuts {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(&item.file_name).strong());
                                        ui.label(RichText::new(format!("Destino no encontrado: {}", item.target_path)).small().color(RustCareTheme::DANGER_RED));
                                    });
                                });
                                ui.separator();
                            }
                        });
                });
            }
        }
    }

    fn render_shredder(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Destrucción Segura de Archivos (Sobrescritura irreversible con ceros antes de borrar):").strong());
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Ruta del archivo a destruir:");
            ui.text_edit_singleline(&mut self.shred_target_path);
            if ui.button("🔥 Destruir Archivo Irreversiblemente").clicked() {
                if std::path::Path::new(&self.shred_target_path).exists() {
                    match FileShredder::shred_file(&self.shred_target_path) {
                        Ok(_) => self.shred_status_msg = Some("¡Archivo destruido con éxito! Imposible de recuperar.".to_string()),
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

    fn render_context_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Gestión de Elementos de Clic Derecho (Shell Extensions):").strong());
            if ui.button("🔄 Refrescar").clicked() {
                self.context_handlers = ContextMenuManager::list_handlers();
            }
        });
        ui.add_space(8.0);

        ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::CARD_BG)
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
                                    if ui.button(btn_txt).clicked() {
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

    fn render_updater(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("🔍 Comprobar Actualizaciones (WinGet)").clicked() {
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
                .fill(RustCareTheme::CARD_BG)
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
                                ui.label(RichText::new(format!("Actual: {} ➔ Nueva: {}", update.installed_version, update.available_version)).small().color(RustCareTheme::SUCCESS_GREEN));
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Actualizar").clicked() {
                                    let _ = UpdaterEngine::update_package(&update.id);
                                }
                            });
                        });
                        ui.separator();
                    }
                });
        });
    }

    fn render_rescue(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Centro de Rescate y Restauración de Copias de Seguridad:").strong());
            if ui.button("🔄 Actualizar Lista").clicked() {
                self.backups = RescueCenter::list_backups();
            }
        });
        ui.add_space(8.0);

        if let Some(msg) = &self.rescue_status_msg {
            ui.label(RichText::new(msg).color(RustCareTheme::SUCCESS_GREEN));
            ui.add_space(6.0);
        }

        ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::NONE
                .fill(RustCareTheme::CARD_BG)
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
                                if ui.button("🛟 Restaurar Backup").clicked() {
                                    match RescueCenter::restore_backup(backup) {
                                        Ok(true) => self.rescue_status_msg = Some(format!("¡Copia {} restaurada con éxito!", backup.file_name)),
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
}
