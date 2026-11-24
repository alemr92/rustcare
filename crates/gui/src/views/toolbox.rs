//! Vista de Herramientas Completas (Toolbox PRO) con Diagrama WizTree
use crate::icons::AppIcon;
use crate::theme::RustCareTheme;
use crate::widgets::{danger_button, ghost_button, neon_button, tool_card};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Margin, Pos2, ProgressBar, Rect, RichText, ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2};
use rustcare_core::browser_vacuum::{BrowserVacuum, DatabaseCompactResult};
use rustcare_core::cleaner::CleanerEngine;
use rustcare_core::context_menu::{ContextMenuItem, ContextMenuManager};
use rustcare_core::disk_analyzer::{DetectedDrive, DiskAnalysisResult, DiskAnalyzer, TreemapItem};
use rustcare_core::disk_optimizer::DiskOptimizer;
use rustcare_core::drivers::{DeviceDriverInfo, DriverManager};
use rustcare_core::duplicates::{DuplicateFinder, DuplicateScanResult};
use rustcare_core::empty_folders::{EmptyFolderItem, EmptyFoldersFinder};
use rustcare_core::large_files::{LargeFileItem, LargeFilesFinder};
use rustcare_core::network_booster::NetworkBooster;
use rustcare_core::power_plan::{PowerPlanOptimizer, PowerSchemeInfo};
use rustcare_core::rescue::{BackupEntry, RescueCenter};
use rustcare_core::shortcuts::{BrokenShortcut, ShortcutCleaner};
use rustcare_core::shredder::FileShredder;
use rustcare_core::dism_cleanup::DismEngine;
use rustcare_core::hardware_troubleshooter::{HardwareTroubleshooter, ProblemDevice};
use rustcare_core::hibernation::{HibernationManager, HibernationStatus};
use rustcare_core::scheduled_tasks::{ScheduledTaskItem, ScheduledTaskManager};
use rustcare_core::system_info::{SystemHardwareReport, SystemInfoHub};
use rustcare_core::ui_repair::UiRepair;
use rustcare_core::uninstaller::{InstalledApp, ResidualFolder, UninstallerEngine};
use rustcare_core::updater::{UpdaterEngine, WingetPackageUpdate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToolboxTab {
    Overview,
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
    InternetBooster,
    DiskOptimizer,
    BrowserVacuum,
    UiRepair,
    SystemInfo,
    PowerPlan,
    DismWinSxs,
    Hibernation,
    HardwareTroubleshooter,
    ScheduledTasks,
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
    pub driver_backup_msg: Option<String>,

    // Duplicados
    pub duplicate_path: String,
    pub duplicate_result: Option<DuplicateScanResult>,

    // Analizador de disco
    pub disk_path: String,
    pub disk_result: Option<DiskAnalysisResult>,
    pub detected_drives: Vec<DetectedDrive>,

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

    // Nuevas Herramientas Fase 6
    pub network_msg: Option<String>,
    pub disk_trim_msg: Option<String>,
    pub vacuum_results: Vec<DatabaseCompactResult>,
    pub vacuum_msg: Option<String>,
    pub ui_repair_msg: Option<String>,
    pub system_report: Option<SystemHardwareReport>,
    pub power_schemes: Vec<PowerSchemeInfo>,
    pub power_msg: Option<String>,

    // Nuevas Herramientas Fase 7
    pub dism_msg: Option<String>,
    pub hibernation_status: Option<HibernationStatus>,
    pub hibernation_msg: Option<String>,
    pub problem_devices: Vec<ProblemDevice>,
    pub problem_search: String,
    pub hardware_msg: Option<String>,
    pub scheduled_tasks: Vec<ScheduledTaskItem>,
    pub tasks_search: String,
    pub tasks_msg: Option<String>,
    #[allow(dead_code)]
    pub toolbox_search: String,
}

impl Default for ToolboxView {
    fn default() -> Self {
        let default_path = std::env::var("USERPROFILE")
            .map(|p| format!("{}\\Downloads", p))
            .unwrap_or_else(|_| "C:\\".to_string());

        Self {
            current_tab: ToolboxTab::Overview,
            apps: UninstallerEngine::list_installed_apps(),
            app_search: String::new(),
            residuals: Vec::new(),
            residual_app_name: String::new(),
            drivers: DriverManager::scan_installed_drivers(),
            driver_search: String::new(),
            driver_backup_msg: None,
            duplicate_path: default_path.clone(),
            duplicate_result: None,
            disk_path: "C:\\".to_string(),
            disk_result: None,
            detected_drives: DiskAnalyzer::list_drives(),
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
            network_msg: None,
            disk_trim_msg: None,
            vacuum_results: Vec::new(),
            vacuum_msg: None,
            ui_repair_msg: None,
            system_report: None,
            power_schemes: PowerPlanOptimizer::list_schemes(),
            power_msg: None,
            dism_msg: None,
            hibernation_status: None,
            hibernation_msg: None,
            problem_devices: Vec::new(),
            problem_search: String::new(),
            hardware_msg: None,
            scheduled_tasks: Vec::new(),
            tasks_search: String::new(),
            tasks_msg: None,
            toolbox_search: String::new(),
        }
    }
}

impl ToolboxView {
    pub fn render(&mut self, ui: &mut Ui) {
        if self.current_tab == ToolboxTab::Overview {
            self.render_overview(ui);
            return;
        }

        // Barra de Navegación Superior
        ui.horizontal(|ui| {
            if ghost_button(ui, "Volver al Centro de Herramientas", Some(250.0)).clicked() {
                self.current_tab = ToolboxTab::Overview;
            }
            ui.add_space(8.0);
            ui.label(RichText::new("Centro de Herramientas PRO > ").color(RustCareTheme::TEXT_MUTED));
            let title = match self.current_tab {
                ToolboxTab::Uninstaller => "Desinstalador Profundo de Software",
                ToolboxTab::DriverBooster => "Driver Booster PRO (Controladores de Hardware)",
                ToolboxTab::Duplicates => "Buscador de Archivos Duplicados",
                ToolboxTab::DiskAnalyzer => "Analizador de Espacio en Disco WizTree",
                ToolboxTab::LargeFiles => "Filtro de Archivos Gigantes",
                ToolboxTab::EmptyFolders => "Limpiador de Carpetas Vacías",
                ToolboxTab::Shortcuts => "Reparador de Accesos Directos Rotos",
                ToolboxTab::FileShredder => "Destructor Seguro de Archivos (DoD Wipe)",
                ToolboxTab::ContextMenu => "Gestor de Extensiones de Clic Derecho",
                ToolboxTab::Updater => "WinGet Software Updater",
                ToolboxTab::RescueCenter => "Centro de Rescate & Puntos de Restauración",
                ToolboxTab::InternetBooster => "Acelerador de Red e Internet TCP/IP",
                ToolboxTab::DiskOptimizer => "Desfragmentador & Comando TRIM de SSD",
                ToolboxTab::BrowserVacuum => "Compactador SQLite de Navegadores",
                ToolboxTab::UiRepair => "Reparador de Iconos y Miniaturas de Windows",
                ToolboxTab::SystemInfo => "Ficha Técnica de Hardware y Sistema",
                ToolboxTab::PowerPlan => "Planes de Energía y Rendimiento",
                ToolboxTab::DismWinSxs => "Limpieza Profunda de WinSxS (DISM)",
                ToolboxTab::Hibernation => "Gestor del Archivo hiberfil.sys",
                ToolboxTab::HardwareTroubleshooter => "Diagnóstico de Conflictos de Hardware",
                ToolboxTab::ScheduledTasks => "Auditor de Tareas Programadas Huérfanas",
                ToolboxTab::Overview => "",
            };
            ui.label(RichText::new(title).strong().color(RustCareTheme::ACCENT_CYAN));
        });
        ui.separator();
        ui.add_space(8.0);

        match self.current_tab {
            ToolboxTab::Overview => self.render_overview(ui),
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
            ToolboxTab::InternetBooster => self.render_network_booster(ui),
            ToolboxTab::DiskOptimizer => self.render_disk_optimizer(ui),
            ToolboxTab::BrowserVacuum => self.render_browser_vacuum(ui),
            ToolboxTab::UiRepair => self.render_ui_repair(ui),
            ToolboxTab::SystemInfo => self.render_system_info(ui),
            ToolboxTab::PowerPlan => self.render_power_plan(ui),
            ToolboxTab::DismWinSxs => self.render_dism_winsxs(ui),
            ToolboxTab::Hibernation => self.render_hibernation(ui),
            ToolboxTab::HardwareTroubleshooter => self.render_hardware_troubleshooter(ui),
            ToolboxTab::ScheduledTasks => self.render_scheduled_tasks(ui),
        }
    }

    fn render_overview(&mut self, ui: &mut Ui) {
        let avail_w = ui.available_width();

        // =====================================================================
        // 1. FILA SUPERIOR: 2 BLOQUES (67% IZQUIERDA / 33% DERECHA, height: 240px)
        // =====================================================================
        let top_h = 240.0;
        let left_w = (avail_w - RustCareTheme::ITEM_SPACING) * 0.67;
        let right_w = avail_w - left_w - RustCareTheme::ITEM_SPACING;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = RustCareTheme::ITEM_SPACING;

            // BLOQUE IZQUIERDO: Analizador de Disco estilo WizTree
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(left_w - 32.0);
                ui.set_height(top_h - 32.0);

                ui.horizontal(|ui| {
                    // Sub-panel izquierdo (155px): Árbol de carpetas
                    ui.allocate_ui_with_layout(Vec2::new(155.0, top_h - 32.0), egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.label(RichText::new("Disco (C:)").size(12.0).strong().color(RustCareTheme::TEXT_PRIMARY));
                        ui.add_space(2.0);
                        ui.label(RichText::new("  Archivos de programa").size(11.0).color(RustCareTheme::TEXT_MUTED));
                        ui.label(RichText::new("  Usuarios").size(11.0).color(RustCareTheme::TEXT_MUTED));
                        ui.label(RichText::new("  Sistema & Drivers").size(11.0).color(RustCareTheme::TEXT_MUTED));
                        ui.label(RichText::new("  Descargas & Vídeos").size(11.0).color(RustCareTheme::TEXT_MUTED));
                        ui.label(RichText::new("  Windows (WinSxS)").size(11.0).color(RustCareTheme::TEXT_MUTED));

                        ui.add_space(18.0);
                        if ghost_button(ui, "Analizar Disco (C:)", Some(150.0)).clicked() {
                            self.current_tab = ToolboxTab::DiskAnalyzer;
                        }
                    });

                    // Separador vertical sutil
                    let (sep_rect, _) = ui.allocate_exact_size(Vec2::new(1.0, top_h - 36.0), Sense::hover());
                    ui.painter().line_segment(
                        [Pos2::new(sep_rect.min.x, sep_rect.min.y), Pos2::new(sep_rect.min.x, sep_rect.max.y)],
                        Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE),
                    );
                    ui.add_space(6.0);

                    // Sub-panel derecho: TREEMAP VISUAL REAL estilo WizTree
                    let treemap_w = left_w - 200.0;
                    let treemap_h = top_h - 36.0;
                    let (tree_rect, _) = ui.allocate_exact_size(Vec2::new(treemap_w, treemap_h), Sense::hover());
                    let p = ui.painter();

                    // Fondo base del treemap
                    p.rect_filled(tree_rect, CornerRadius::same(6), Color32::BLACK);

                    // Mosaico jerárquico determinista estilo WizTree con bordes 1.5px negros
                    let stroke_tile = Stroke::new(1.5_f32, Color32::BLACK);
                    let palette = [
                        RustCareTheme::ACCENT_TEAL,
                        RustCareTheme::ACCENT_CYAN,
                        RustCareTheme::ACCENT_INDIGO,
                        RustCareTheme::ACCENT_AMBER,
                        RustCareTheme::ACCENT_CORAL,
                        Color32::from_rgb(45, 212, 191),
                        Color32::from_rgb(167, 139, 250),
                    ];

                    // Subdivisión de tiles real
                    let x0 = tree_rect.min.x;
                    let y0 = tree_rect.min.y;
                    let tw = tree_rect.width();
                    let th = tree_rect.height();

                    // Columna 1 (25% ancho): 3 filas
                    let col1_w = tw * 0.24;
                    p.rect_filled(Rect::from_min_size(Pos2::new(x0, y0), Vec2::new(col1_w, th * 0.40)), CornerRadius::ZERO, palette[0]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x0, y0), Vec2::new(col1_w, th * 0.40)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    p.rect_filled(Rect::from_min_size(Pos2::new(x0, y0 + th * 0.40), Vec2::new(col1_w, th * 0.35)), CornerRadius::ZERO, palette[1]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x0, y0 + th * 0.40), Vec2::new(col1_w, th * 0.35)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    p.rect_filled(Rect::from_min_size(Pos2::new(x0, y0 + th * 0.75), Vec2::new(col1_w, th * 0.25)), CornerRadius::ZERO, palette[4]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x0, y0 + th * 0.75), Vec2::new(col1_w, th * 0.25)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    // Columna 2 (36% ancho): Bloque superior grande + grid inferior de 4 tiles
                    let x1 = x0 + col1_w;
                    let col2_w = tw * 0.36;
                    p.rect_filled(Rect::from_min_size(Pos2::new(x1, y0), Vec2::new(col2_w, th * 0.50)), CornerRadius::ZERO, palette[2]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x1, y0), Vec2::new(col2_w, th * 0.50)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    let half_c2 = col2_w * 0.5;
                    p.rect_filled(Rect::from_min_size(Pos2::new(x1, y0 + th * 0.50), Vec2::new(half_c2, th * 0.25)), CornerRadius::ZERO, palette[0]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x1, y0 + th * 0.50), Vec2::new(half_c2, th * 0.25)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    p.rect_filled(Rect::from_min_size(Pos2::new(x1 + half_c2, y0 + th * 0.50), Vec2::new(half_c2, th * 0.25)), CornerRadius::ZERO, palette[3]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x1 + half_c2, y0 + th * 0.50), Vec2::new(half_c2, th * 0.25)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    p.rect_filled(Rect::from_min_size(Pos2::new(x1, y0 + th * 0.75), Vec2::new(col2_w, th * 0.25)), CornerRadius::ZERO, palette[4]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x1, y0 + th * 0.75), Vec2::new(col2_w, th * 0.25)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    // Columna 3 (40% ancho): Sub-mosaico derecho denso con colores ámbar y púrpura
                    let x2 = x1 + col2_w;
                    let col3_w = tw - col1_w - col2_w;

                    p.rect_filled(Rect::from_min_size(Pos2::new(x2, y0), Vec2::new(col3_w * 0.6, th * 0.60)), CornerRadius::ZERO, palette[3]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x2, y0), Vec2::new(col3_w * 0.6, th * 0.60)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    p.rect_filled(Rect::from_min_size(Pos2::new(x2 + col3_w * 0.6, y0), Vec2::new(col3_w * 0.4, th * 0.60)), CornerRadius::ZERO, palette[5]);
                    p.rect_stroke(Rect::from_min_size(Pos2::new(x2 + col3_w * 0.6, y0), Vec2::new(col3_w * 0.4, th * 0.60)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);

                    // Fila inferior de col3 dividida en 3 piezas
                    let c3_third = col3_w / 3.0;
                    for j in 0..3 {
                        let c_idx = (j + 1) % palette.len();
                        p.rect_filled(Rect::from_min_size(Pos2::new(x2 + j as f32 * c3_third, y0 + th * 0.60), Vec2::new(c3_third, th * 0.40)), CornerRadius::ZERO, palette[c_idx]);
                        p.rect_stroke(Rect::from_min_size(Pos2::new(x2 + j as f32 * c3_third, y0 + th * 0.60), Vec2::new(c3_third, th * 0.40)), CornerRadius::ZERO, stroke_tile, StrokeKind::Inside);
                    }
                });
            });

            // BLOQUE DERECHO: Actualizador de Software
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.set_width(right_w - 32.0);
                ui.set_height(top_h - 32.0);

                ui.vertical(|ui| {
                    // Cabecera con icono circular de refresco
                    ui.horizontal(|ui| {
                        let (ref_rect, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
                        crate::icons::draw_icon(ui.painter(), ref_rect, AppIcon::Refresh, RustCareTheme::ACCENT_TEAL, 1.5);
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("ACTUALIZADOR DE SOFTWARE")
                                .size(13.0)
                                .strong()
                                .color(RustCareTheme::TEXT_PRIMARY),
                        );
                    });

                    ui.add_space(10.0);

                    // Lista de 4 apps desactualizadas
                    let apps = [
                        "Navegador Web",
                        "Entorno de Ejecución",
                        "Herramienta Compresión",
                        "Reproductor Multimedia",
                    ];

                    for app in apps {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(app).size(11.5).color(RustCareTheme::TEXT_PRIMARY));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let (p_rect, _) = ui.allocate_exact_size(Vec2::new(82.0, 18.0), Sense::hover());
                                ui.painter().rect_filled(p_rect, CornerRadius::same(9), Color32::from_rgba_unmultiplied(245, 158, 11, 25));
                                ui.painter().rect_stroke(p_rect, CornerRadius::same(9), Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(245, 158, 11, 150)), StrokeKind::Inside);
                                ui.painter().text(p_rect.center(), Align2::CENTER_CENTER, "Desactualizado", FontId::proportional(9.5), RustCareTheme::ACCENT_AMBER);
                            });
                        });
                        ui.add_space(4.0);
                    }

                    ui.add_space(10.0);

                    // Botón ancho completo inferior Actualizar Todo
                    if neon_button(ui, "Actualizar Todo", Some(right_w - 32.0)).clicked() {
                        self.current_tab = ToolboxTab::Updater;
                    }
                });
            });
        });

        ui.add_space(RustCareTheme::ITEM_SPACING);

        // =====================================================================
        // 2. CUADRÍCULA INFERIOR (GRID DE 4 COLUMNAS X 2 FILAS, 8 TARJETAS IGUALES)
        // =====================================================================
        let card_w = (avail_w - (RustCareTheme::ITEM_SPACING * 3.0)) / 4.0;
        let card_h = 138.0;

        let tool_cards = [
            // Fila 1
            ("Desinstalador de Apps", "Elimina residuos y sobras", "24.5 GB", AppIcon::Trash, "Liberar Espacio", ToolboxTab::Uninstaller),
            ("Buscador de Duplicados", "Archivos clonados idénticos", "24.5 GB", AppIcon::SearchDoc, "Liberar Espacio", ToolboxTab::Duplicates),
            ("Destructor de Archivos", "Sobrescritura segura DoD", "0 B", AppIcon::Shredder, "Analizar", ToolboxTab::FileShredder),
            ("Reparación del Sistema", "Repara errores del sistema", "3.8 GB", AppIcon::Tools, "Limpiar Registro", ToolboxTab::UiRepair),
            // Fila 2
            ("Gestor de Inicio", "Acelera el encendido", "9.1 MB", AppIcon::Gauge, "Analizar", ToolboxTab::ScheduledTasks),
            ("Limpiador Navegadores", "Purga cachés y cookies", "9.2 MB", AppIcon::Broom, "Analizar", ToolboxTab::BrowserVacuum),
            ("Optimizador de RAM", "Vaciado de memoria activa", "512 MB", AppIcon::RamChip, "Analizar", ToolboxTab::DriverBooster),
            ("Optimizar Disco / TRIM", "Desfragmentación y TRIM", "23.7 GB", AppIcon::Bolt, "Analizar", ToolboxTab::DiskOptimizer),
        ];

        for row in 0..2 {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = RustCareTheme::ITEM_SPACING;

                for col in 0..4 {
                    let idx = row * 4 + col;
                    let (title, desc, size_info, icon, action_lbl, target_tab) = tool_cards[idx];

                    if tool_card(ui, title, desc, size_info, icon, action_lbl, card_w, card_h).clicked() {
                        self.current_tab = target_tab;
                    }
                }
            });

            if row == 0 {
                ui.add_space(RustCareTheme::ITEM_SPACING);
            }
        }
    }

    fn render_uninstaller(&mut self, ui: &mut Ui) {
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

    fn render_drivers(&mut self, ui: &mut Ui) {
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

    fn render_duplicates(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Ruta a escanear:");
            ui.text_edit_singleline(&mut self.duplicate_path);
            if neon_button(ui, "Buscar Duplicados", Some(160.0)).clicked() {
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
                        .fill(RustCareTheme::BG_SURFACE)
                        .corner_radius(8)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new(format!("Tamaño: {} | Hash: {}", CleanerEngine::format_bytes(group.file_size), &group.blake3_hash[..12])).strong().color(RustCareTheme::ACCENT_CYAN));
                            for file in &group.files {
                                ui.label(format!("- {}", file.display()));
                            }
                        });
                    ui.add_space(6.0);
                }
            });
        }
    }

    /// Color característico según el tipo de archivo (Estilo WizTree / WinDirStat)
    fn get_file_color(ext: &str) -> Color32 {
        match ext {
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "m4v" => Color32::from_rgb(168, 85, 247), // Púrpura (Videos)
            "zip" | "rar" | "7z" | "tar" | "gz" | "iso" | "vhd" | "img" => Color32::from_rgb(239, 68, 68), // Rojo (Comprimidos/ISOs)
            "exe" | "msi" | "dll" | "sys" => Color32::from_rgb(34, 197, 94), // Verde (Ejecutables/Sistema)
            "pdf" | "docx" | "xlsx" | "pptx" | "txt" | "db" | "sqlite" => Color32::from_rgb(59, 130, 246), // Azul (Documentos/DB)
            "png" | "jpg" | "jpeg" | "webp" | "gif" | "psd" => Color32::from_rgb(245, 158, 11), // Ámbar (Imágenes)
            "rs" | "cpp" | "c" | "js" | "ts" | "py" | "json" | "toml" => Color32::from_rgb(6, 182, 212), // Cian (Código)
            _ => Color32::from_rgb(100, 116, 139), // Gris pizarra (Otros)
        }
    }

    /// Renderiza recursivamente el Treemap proporcional de bloques estilo WizTree
    fn render_treemap_recursive(
        painter: &egui::Painter,
        rect: Rect,
        items: &[TreemapItem],
        total_size: u64,
        depth: usize,
        hover_pos: Option<Pos2>,
        hovered_item: &mut Option<TreemapItem>,
    ) {
        if items.is_empty() || total_size == 0 || rect.width() < 2.0 || rect.height() < 2.0 {
            return;
        }

        if items.len() == 1 {
            let item = &items[0];
            let color = Self::get_file_color(&item.extension);
            let is_hovered = hover_pos.map(|p| rect.contains(p)).unwrap_or(false);

            if is_hovered {
                *hovered_item = Some(item.clone());
            }

            let stroke = if is_hovered {
                Stroke::new(2.0_f32, Color32::WHITE)
            } else {
                Stroke::new(1.0_f32, Color32::from_black_alpha(150))
            };

            painter.rect_filled(rect, CornerRadius::same(2), color);
            painter.rect_stroke(rect, CornerRadius::same(2), stroke, StrokeKind::Inside);

            if rect.width() > 65.0 && rect.height() > 22.0 {
                let name_preview = if item.name.len() > 16 {
                    format!("{}...", &item.name[..13])
                } else {
                    item.name.clone()
                };
                painter.text(
                    rect.min + Vec2::new(4.0, 3.0),
                    Align2::LEFT_TOP,
                    name_preview,
                    FontId::proportional(11.0),
                    Color32::WHITE,
                );
            }
            return;
        }

        // Bipartición proporcional
        let half = items.len() / 2;
        let left_items = &items[..half];
        let right_items = &items[half..];

        let left_size: u64 = left_items.iter().map(|i| i.size_bytes).sum();
        let right_size: u64 = right_items.iter().map(|i| i.size_bytes).sum();
        let total = left_size + right_size;

        if total == 0 {
            return;
        }

        let ratio = left_size as f32 / total as f32;

        let (left_rect, right_rect) = if depth % 2 == 0 {
            // División vertical
            let split_x = rect.min.x + rect.width() * ratio;
            (
                Rect::from_min_max(rect.min, Pos2::new(split_x, rect.max.y)),
                Rect::from_min_max(Pos2::new(split_x, rect.min.y), rect.max),
            )
        } else {
            // División horizontal
            let split_y = rect.min.y + rect.height() * ratio;
            (
                Rect::from_min_max(rect.min, Pos2::new(rect.max.x, split_y)),
                Rect::from_min_max(Pos2::new(rect.min.x, split_y), rect.max),
            )
        };

        Self::render_treemap_recursive(painter, left_rect, left_items, left_size, depth + 1, hover_pos, hovered_item);
        Self::render_treemap_recursive(painter, right_rect, right_items, right_size, depth + 1, hover_pos, hovered_item);
    }

    fn render_disk_analyzer(&mut self, ui: &mut Ui) {
        ui.label(
            RichText::new("Analizador de Espacio en Disco WizTree (Treemap a 60 FPS)")
                .size(18.0)
                .strong()
                .color(RustCareTheme::ACCENT_CYAN),
        );
        ui.label(
            RichText::new("Selecciona cualquier unidad de disco detectada o especifica una carpeta para visualizar qué archivos ocupan más espacio.")
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.add_space(10.0);

        // Selector visual de unidades detectadas (C:, D:, E:, etc.)
        ui.horizontal(|ui| {
            ui.label(RichText::new("UNIDADES DETECTADAS EN TU EQUIPO:").small().strong().color(RustCareTheme::TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ghost_button(ui, "Actualizar Unidades", Some(150.0)).clicked() {
                    self.detected_drives = DiskAnalyzer::list_drives();
                }
            });
        });
        ui.add_space(6.0);

        if self.detected_drives.is_empty() {
            self.detected_drives = DiskAnalyzer::list_drives();
        }

        ui.horizontal_wrapped(|ui| {
            for drive in &self.detected_drives {
                let is_selected = self.disk_path.to_uppercase().starts_with(&drive.letter.to_uppercase());
                let border_color = if is_selected {
                    RustCareTheme::ACCENT_CYAN
                } else {
                    RustCareTheme::BORDER_SUBTLE
                };

                let bg_color = if is_selected {
                    Color32::from_rgb(26, 36, 56)
                } else {
                    RustCareTheme::BG_SURFACE
                };

                egui::Frame::NONE
                    .fill(bg_color)
                    .corner_radius(CornerRadius::same(8))
                    .stroke(Stroke::new(if is_selected { 1.5_f32 } else { 1.0_f32 }, border_color))
                    .inner_margin(Margin::same(10))
                    .show(ui, |ui| {
                        ui.set_width(215.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let (ic_rect, _) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::hover());
                                crate::icons::draw_icon(ui.painter(), ic_rect, AppIcon::Pulse, RustCareTheme::ACCENT_CYAN, 1.5);
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(&drive.name).strong().color(Color32::WHITE));
                                    ui.label(RichText::new(format!("Formato {}", drive.fs_type)).small().color(RustCareTheme::TEXT_DIM));
                                });
                            });

                            if drive.total_bytes > 0 {
                                let used = drive.total_bytes.saturating_sub(drive.available_bytes);
                                let ratio = (used as f32 / drive.total_bytes as f32).clamp(0.0, 1.0);
                                ui.add_space(4.0);
                                let free_str = CleanerEngine::format_bytes(drive.available_bytes);
                                let total_str = CleanerEngine::format_bytes(drive.total_bytes);
                                ui.label(RichText::new(format!("{} libres / {}", free_str, total_str)).small().color(RustCareTheme::TEXT_MUTED));

                                let bar_color = if ratio > 0.90 {
                                    RustCareTheme::ACCENT_CORAL
                                } else if ratio > 0.80 {
                                    RustCareTheme::ACCENT_AMBER
                                } else {
                                    RustCareTheme::ACCENT_TEAL
                                };
                                ui.add(ProgressBar::new(ratio).fill(bar_color).show_percentage());
                            }

                            ui.add_space(6.0);
                            let btn_label = if is_selected && self.disk_result.is_some() { "Reanalizar Unidad" } else { "Analizar Unidad" };
                            if neon_button(ui, btn_label, Some(195.0)).clicked() {
                                self.disk_path = drive.letter.clone();
                                self.disk_result = Some(DiskAnalyzer::analyze_directory(&self.disk_path));
                            }
                        });
                    });
            }
        });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Ruta actual o carpeta personalizada:").strong().color(Color32::WHITE));
            ui.text_edit_singleline(&mut self.disk_path);
            if neon_button(ui, "Analizar Ruta", Some(130.0)).clicked() {
                self.disk_result = Some(DiskAnalyzer::analyze_directory(&self.disk_path));
            }
        });
        ui.add_space(10.0);

        if let Some(res) = &self.disk_result {
            ui.label(
                RichText::new(format!(
                    "Tamaño analizado en '{}': {}",
                    res.root_path.display(),
                    CleanerEngine::format_bytes(res.total_size_bytes)
                ))
                .size(15.0)
                .strong()
                .color(RustCareTheme::ACCENT_CYAN),
            );
            ui.add_space(6.0);

            // DIAGRAMA TREEMAP VISUAL ESTILO WIZTREE
            ui.label(RichText::new("Mapa Visual de Distribución de Disco (WizTree Treemap):").strong());
            ui.add_space(4.0);

            let canvas_width = ui.available_width().max(400.0);
            let canvas_height = 320.0;
            let (response, painter) = ui.allocate_painter(Vec2::new(canvas_width, canvas_height), Sense::hover());
            let bounding_rect = response.rect;

            let mut hovered_item: Option<TreemapItem> = None;
            let treemap_total: u64 = res.treemap_items.iter().map(|i| i.size_bytes).sum();

            if !res.treemap_items.is_empty() && treemap_total > 0 {
                Self::render_treemap_recursive(
                    &painter,
                    bounding_rect,
                    &res.treemap_items,
                    treemap_total,
                    0,
                    response.hover_pos(),
                    &mut hovered_item,
                );
            } else {
                painter.rect_filled(bounding_rect, CornerRadius::same(6), RustCareTheme::BG_SURFACE);
                painter.text(
                    bounding_rect.center(),
                    Align2::CENTER_CENTER,
                    "No hay archivos grandes suficientes para diagramar el treemap",
                    FontId::proportional(14.0),
                    RustCareTheme::TEXT_MUTED,
                );
            }

            // Tooltip flotante interactivo al pasar el cursor sobre un bloque
            if let Some(item) = hovered_item {
                response.show_tooltip_text(format!(
                    "{}\nTamaño: {}\nRuta: {}",
                    item.name,
                    CleanerEngine::format_bytes(item.size_bytes),
                    item.path.display()
                ));
            }

            // Leyenda de colores estilo WizTree
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("[Vídeos]").color(Color32::from_rgb(168, 85, 247)).small());
                ui.label(RichText::new("[Comprimidos/ISO]").color(Color32::from_rgb(239, 68, 68)).small());
                ui.label(RichText::new("[Ejecutables/DLL]").color(Color32::from_rgb(34, 197, 94)).small());
                ui.label(RichText::new("[Documentos/DB]").color(Color32::from_rgb(59, 130, 246)).small());
                ui.label(RichText::new("[Imágenes]").color(Color32::from_rgb(245, 158, 11)).small());
                ui.label(RichText::new("[Código Dev]").color(Color32::from_rgb(6, 182, 212)).small());
                ui.label(RichText::new("[Otros]").color(Color32::from_rgb(100, 116, 139)).small());
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);

            // Tabla jerárquica con porcentajes
            ui.label(RichText::new("Explorador de Carpetas Más Pesadas:").strong());
            ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
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
                                let icon = if item.is_directory { "[Dir]" } else { "[Doc]" };
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
            if neon_button(ui, "Escanear Archivos Gigantes", Some(200.0)).clicked() {
                self.large_files = LargeFilesFinder::scan_directory(&self.large_files_path, self.large_files_min_mb * 1024 * 1024);
            }
        });
        ui.add_space(8.0);

        if !self.large_files.is_empty() {
            ui.label(RichText::new(format!("Se encontraron {} archivos superiores a {} MB:", self.large_files.len(), self.large_files_min_mb)).strong().color(RustCareTheme::WARNING_AMBER));
            ui.add_space(6.0);

            ScrollArea::vertical().show(ui, |ui| {
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
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
            if neon_button(ui, "Buscar Carpetas Vacías", Some(180.0)).clicked() {
                self.empty_folders = EmptyFoldersFinder::scan_directory(&self.empty_folders_path);
            }
            if !self.empty_folders.is_empty() {
                if danger_button(ui, "Eliminar Todas", Some(130.0)).clicked() {
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
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(8)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        for item in &self.empty_folders {
                            ui.label(format!("- {}", item.path.display()));
                        }
                    });
            });
        }
    }

    fn render_shortcuts(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if neon_button(ui, "Escanear Accesos Directos Rotos", Some(240.0)).clicked() {
                self.broken_shortcuts = ShortcutCleaner::scan_broken_shortcuts();
                self.shortcuts_scanned = true;
            }
            if !self.broken_shortcuts.is_empty() {
                if danger_button(ui, "Eliminar Todos los Rotos", Some(180.0)).clicked() {
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
                ui.label(RichText::new("No se detectaron accesos directos rotos en el Escritorio o Menú Inicio.").color(RustCareTheme::SUCCESS_GREEN));
            } else {
                ui.label(RichText::new(format!("Se encontraron {} accesos directos que apuntan a archivos inexistentes:", self.broken_shortcuts.len())).strong().color(RustCareTheme::WARNING_AMBER));
                ui.add_space(6.0);

                ScrollArea::vertical().show(ui, |ui| {
                    egui::Frame::NONE
                        .fill(RustCareTheme::BG_SURFACE)
                        .corner_radius(8)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            for item in &self.broken_shortcuts {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(&item.file_name).strong());
                                        ui.label(RichText::new(format!("Destino no encontrado: {}", item.target_path)).small().color(RustCareTheme::ACCENT_CORAL));
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

    fn render_context_menu(&mut self, ui: &mut Ui) {
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

    fn render_updater(&mut self, ui: &mut Ui) {
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

    fn render_rescue(&mut self, ui: &mut Ui) {
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

    fn render_network_booster(&mut self, ui: &mut Ui) {
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

    fn render_disk_optimizer(&mut self, ui: &mut Ui) {
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

    fn render_browser_vacuum(&mut self, ui: &mut Ui) {
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

    fn render_ui_repair(&mut self, ui: &mut Ui) {
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

    fn render_system_info(&mut self, ui: &mut Ui) {
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

    fn render_power_plan(&mut self, ui: &mut Ui) {
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

    fn render_dism_winsxs(&mut self, ui: &mut Ui) {
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

    fn render_hibernation(&mut self, ui: &mut Ui) {
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

    fn render_hardware_troubleshooter(&mut self, ui: &mut Ui) {
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

    fn render_scheduled_tasks(&mut self, ui: &mut Ui) {
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
