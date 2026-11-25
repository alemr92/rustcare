//! Vista de Herramientas Completas (Toolbox PRO) con Diagrama WizTree
use crate::icons::AppIcon;
use crate::theme::RustCareTheme;
use crate::widgets::{danger_button, ghost_button, neon_button};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2};
use rustcare_core::backend::SystemBackend;
use rustcare_core::browser_vacuum::{BrowserVacuum, DatabaseCompactResult};
use rustcare_core::monitor::SystemMetricsSnapshot;
use std::sync::Arc;
use std::time::Instant;
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
use rustcare_core::sleep_mode::{SleepEngine, SleepProgramItem};
use rustcare_core::dism_cleanup::DismEngine;
use rustcare_core::hardware_troubleshooter::{HardwareTroubleshooter, ProblemDevice};
use rustcare_core::hibernation::{HibernationManager, HibernationStatus};
use rustcare_core::scheduled_tasks::{ScheduledTaskItem, ScheduledTaskManager};
use rustcare_core::system_info::{SystemHardwareReport, SystemInfoHub};
use rustcare_core::ui_repair::UiRepair;
use rustcare_core::uninstaller::{InstalledApp, ResidualFolder, UninstallerEngine};
use rustcare_core::updater::{UpdaterEngine, WingetPackageUpdate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerKind {
    Apps,
    Drivers,
}

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
    SleepMode,
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
    pub ai_opt_boost: bool,
    pub ai_opt_procs: bool,
    pub shred_dod_toggle: bool,
    pub drivers_auto_toggle: bool,
    // Telemetría en vivo para Bento Sensors
    pub cpu_history: Vec<f32>,
    pub gpu_history: Vec<f32>,
    pub ram_history: Vec<f32>,
    pub ssd_history: Vec<f32>,
    pub last_metrics_snapshot: Option<SystemMetricsSnapshot>,
    pub last_metrics_sample: Instant,
    pub drawer_open: Option<DrawerKind>,
    // Modo Reposo y Simulador de Desinstalación (AVG TuneUp)
    pub sleep_programs: Vec<SleepProgramItem>,
    pub sleep_msg: Option<String>,
    pub sim_uninstalled: std::collections::HashSet<String>,
    pub only_high_impact: bool,
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
            ai_opt_boost: true,
            ai_opt_procs: true,
            shred_dod_toggle: true,
            drivers_auto_toggle: true,
            cpu_history: vec![16.0, 21.0, 18.0, 26.0, 20.0, 24.0, 19.0, 22.0],
            gpu_history: vec![22.0, 28.0, 24.0, 32.0, 26.0, 30.0, 25.0, 27.0],
            ram_history: vec![58.0, 62.0, 60.0, 65.0, 63.0, 67.0, 66.0, 68.0],
            ssd_history: vec![45.0, 52.0, 48.0, 56.0, 50.0, 54.0, 51.0, 53.0],
            last_metrics_snapshot: None,
            last_metrics_sample: Instant::now() - std::time::Duration::from_secs(5),
            drawer_open: None,
            sleep_programs: Vec::new(),
            sleep_msg: None,
            sim_uninstalled: std::collections::HashSet::new(),
            only_high_impact: false,
        }
    }
}

impl ToolboxView {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        backend: &Arc<dyn SystemBackend>,
    ) -> Option<crate::app::NavSection> {
        let mut nav_request: Option<crate::app::NavSection> = None;

        // Actualizar telemetría real cada 900ms sin saturar la UI
        if self.last_metrics_snapshot.is_none()
            || self.last_metrics_sample.elapsed() >= std::time::Duration::from_millis(900)
        {
            let snap = backend.get_metrics();
            let prev_cpu = self.cpu_history.last().copied().unwrap_or(snap.cpu_global_percent);
            let smooth_cpu = (prev_cpu * 0.65 + snap.cpu_global_percent * 0.35).clamp(1.0, 100.0);
            if self.cpu_history.len() >= 16 {
                self.cpu_history.remove(0);
            }
            self.cpu_history.push(smooth_cpu);

            let ram_pct = snap.memory_usage_percent.clamp(1.0, 100.0);
            if self.ram_history.len() >= 16 {
                self.ram_history.remove(0);
            }
            self.ram_history.push(ram_pct);

            // Uso de disco principal en % si está disponible
            let disk_pct = snap
                .disks
                .first()
                .map(|d| {
                    if d.total_space > 0 {
                        let used = d.total_space.saturating_sub(d.available_space);
                        (used as f32 / d.total_space as f32) * 100.0
                    } else {
                        12.0
                    }
                })
                .unwrap_or(12.0);
            if self.ssd_history.len() >= 16 {
                self.ssd_history.remove(0);
            }
            self.ssd_history.push(disk_pct.clamp(1.0, 100.0));

            self.last_metrics_snapshot = Some(snap);
            self.last_metrics_sample = Instant::now();
        }

        // Barra de Navegación Superior estilo 2027 (interactiva)
        if self.current_tab == ToolboxTab::Overview {
            ui.horizontal(|ui| {
                let nav_pill = |ui: &mut Ui, text: &str, is_active: bool| -> bool {
                    let padding = Vec2::new(14.0, 6.0);
                    let font_id = FontId::proportional(12.5);
                    let galley = ui.painter().layout_no_wrap(
                        text.to_string(),
                        font_id,
                        if is_active {
                            RustCareTheme::TEXT_PRIMARY
                        } else {
                            RustCareTheme::TEXT_MUTED
                        },
                    );
                    let desired_size = galley.size() + padding * 2.0;
                    let (rect, resp) = ui.allocate_exact_size(desired_size, Sense::click());
                    let p = ui.painter();
                    if is_active {
                        p.rect_filled(
                            rect,
                            CornerRadius::same(8),
                            Color32::from_rgba_unmultiplied(56, 189, 248, 22),
                        );
                        p.rect_stroke(
                            rect,
                            CornerRadius::same(8),
                            Stroke::new(1.0_f32, Color32::from_rgb(56, 189, 248)),
                            StrokeKind::Inside,
                        );
                    } else if resp.hovered() {
                        p.rect_filled(rect, CornerRadius::same(8), RustCareTheme::BG_HOVER);
                    }
                    p.text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        text,
                        FontId::proportional(12.5),
                        if is_active {
                            RustCareTheme::TEXT_PRIMARY
                        } else {
                            RustCareTheme::TEXT_MUTED
                        },
                    );
                    resp.clicked()
                };

                if nav_pill(ui, "Inicio", false) {
                    nav_request = Some(crate::app::NavSection::Care);
                }
                let _ = nav_pill(ui, "Herramientas", true);
                if nav_pill(ui, "Monitoreo", false) {
                    nav_request = Some(crate::app::NavSection::Monitor);
                }
                if nav_pill(ui, "Rendimiento", false) {
                    nav_request = Some(crate::app::NavSection::Booster);
                }
                if nav_pill(ui, "Limpieza", false) {
                    nav_request = Some(crate::app::NavSection::Cleaner);
                }
            });
            ui.add_space(6.0);
        } else {
            ui.horizontal(|ui| {
                if ghost_button(ui, "← Volver al Centro de Herramientas", Some(260.0)).clicked() {
                    self.current_tab = ToolboxTab::Overview;
                }
                ui.add_space(8.0);
                ui.label(RichText::new("Herramientas > ").color(RustCareTheme::TEXT_MUTED));
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
                    ToolboxTab::SleepMode => "Modo Reposo de Programas (Sleep Mode)",
                    ToolboxTab::Overview => "",
                };
                ui.label(RichText::new(title).strong().color(RustCareTheme::ACCENT_CYAN));
            });
            ui.add_space(6.0);
        }

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
            ToolboxTab::SleepMode => self.render_sleep_mode(ui, backend),
        }

        // Slide-Over Drawer (estilo Vaul) para vista rápida sin perder el Bento Grid
        if let Some(kind) = self.drawer_open {
            let mut is_open = true;
            let mut jump_to_tab: Option<ToolboxTab> = None;

            match kind {
                DrawerKind::Apps => {
                    let apps_snapshot = self.apps.clone();
                    crate::widgets::slide_over_drawer(
                        ui.ctx(),
                        &mut is_open,
                        "Desinstalador Avanzado (Vista Rápida)",
                        &format!("{} aplicaciones detectadas en el sistema", apps_snapshot.len()),
                        RustCareTheme::ACCENT_CYAN,
                        400.0,
                        |ui| {
                            if neon_button(ui, "Abrir Desinstalador Completo", Some(260.0)).clicked() {
                                jump_to_tab = Some(ToolboxTab::Uninstaller);
                            }
                            ui.add_space(12.0);
                            for app in apps_snapshot.iter().take(18) {
                                RustCareTheme::card_frame().show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label(RichText::new(&app.name).strong().color(RustCareTheme::TEXT_PRIMARY));
                                            ui.label(
                                                RichText::new(format!("{} · {}", app.publisher, app.version))
                                                    .size(11.0)
                                                    .color(RustCareTheme::TEXT_MUTED),
                                            );
                                        });
                                    });
                                });
                                ui.add_space(6.0);
                            }
                        },
                    );
                }
                DrawerKind::Drivers => {
                    let drivers_snapshot = self.drivers.clone();
                    crate::widgets::slide_over_drawer(
                        ui.ctx(),
                        &mut is_open,
                        "Protección de Drivers (Vista Rápida)",
                        &format!("{} controladores auditados", drivers_snapshot.len()),
                        Color32::from_rgb(37, 99, 235),
                        400.0,
                        |ui| {
                            if neon_button(ui, "Abrir Gestor Completo de Drivers", Some(260.0)).clicked() {
                                jump_to_tab = Some(ToolboxTab::DriverBooster);
                            }
                            ui.add_space(12.0);
                            for drv in drivers_snapshot.iter().take(18) {
                                RustCareTheme::card_frame().show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(&drv.name).strong().color(RustCareTheme::TEXT_PRIMARY));
                                        ui.label(
                                            RichText::new(format!("{} · v{}", drv.provider, drv.version))
                                                .size(11.0)
                                                .color(RustCareTheme::TEXT_MUTED),
                                        );
                                    });
                                });
                                ui.add_space(6.0);
                            }
                        },
                    );
                }
            }

            if let Some(tab) = jump_to_tab {
                self.current_tab = tab;
                self.drawer_open = None;
            } else if !is_open {
                self.drawer_open = None;
            }
        }

        nav_request
    }

    fn render_overview(&mut self, ui: &mut Ui) {
        let avail_w = ui.available_width();
        let spacing = RustCareTheme::ITEM_SPACING;

        // =====================================================================
        // 1. TIRA SUPERIOR DE 4 SENSORES LÍQUIDOS EN VIVO (CPU, GPU, RAM, NVMe)
        // =====================================================================
        let sensor_w = (avail_w - (spacing * 3.0)) / 4.0;
        let sensor_h = 114.0;

        let cpu_val = self.cpu_history.last().copied().unwrap_or(18.0);
        let cpu_peak = self.cpu_history.iter().copied().fold(0.0_f32, f32::max);
        let ram_val = self.ram_history.last().copied().unwrap_or(42.0);
        let ssd_val = self.ssd_history.last().copied().unwrap_or(4.0);

        let (ram_gb_used, ram_gb_total, cpu_cores) = if let Some(snap) = &self.last_metrics_snapshot {
            let used = snap.memory_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            let total = snap.memory_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            (used, total, snap.cpu_count)
        } else {
            (13.4, 32.0, 8)
        };

        let gpu_val = self.gpu_history.last().copied().unwrap_or(24.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = spacing;

            // SENSOR 1: CPU en vivo suavizada
            crate::widgets::sparkline_card(
                ui,
                "CPU",
                &format!("{} núcleos · Vivo", cpu_cores),
                &format!("{:.0}%", cpu_val),
                &format!("Pico: {:.0}%", cpu_peak),
                &format!("{:.0}%", cpu_val),
                &self.cpu_history,
                RustCareTheme::ACCENT_CYAN,
                sensor_w,
                sensor_h,
            );

            // SENSOR 2: GPU
            crate::widgets::sparkline_card(
                ui,
                "GPU",
                "Motor Gráfico",
                &format!("{:.0}%", gpu_val),
                "Activo",
                "",
                &self.gpu_history,
                Color32::from_rgb(45, 212, 191),
                sensor_w,
                sensor_h,
            );

            // SENSOR 3: RAM en vivo
            crate::widgets::sparkline_card(
                ui,
                "RAM",
                &format!("{:.0} GB Total", ram_gb_total),
                &format!("{:.0}% - {:.1} GB", ram_val, ram_gb_used),
                "Capacidad",
                &format!("Carga {:.0}%", ram_val),
                &self.ram_history,
                RustCareTheme::ACCENT_INDIGO,
                sensor_w,
                sensor_h,
            );

            // SENSOR 4: NVMe SSD en vivo
            crate::widgets::sparkline_card(
                ui,
                "NVMe SSD",
                "Uso Unidad C:",
                &format!("{:.0}%", ssd_val),
                "Estado SSD",
                "Salud Óptima",
                &self.ssd_history,
                Color32::from_rgb(37, 99, 235),
                sensor_w,
                sensor_h,
            );
        });

        ui.add_space(spacing);

        // =====================================================================
        // 2. BENTO GRID ASIMÉTRICO (WizTree Treemap + Bento Cards)
        // =====================================================================
        let bento_h = 330.0;
        let left_w = (avail_w - spacing) * 0.33;
        let right_w = avail_w - left_w - spacing;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = spacing;

            // -----------------------------------------------------------------
            // COLUMNA IZQUIERDA: WizTree Disk Treemap (Alto)
            // -----------------------------------------------------------------
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(left_w, bento_h), Sense::click());
            RustCareTheme::draw_glass_card(ui.painter(), rect, resp.hovered(), Some(RustCareTheme::ACCENT_CYAN));

            // Cabecera de la tarjeta WizTree
            let p = ui.painter();
            let ic_rect = Rect::from_min_size(Pos2::new(rect.min.x + 14.0, rect.min.y + 12.0), Vec2::splat(22.0));
            crate::icons::draw_icon(p, ic_rect, AppIcon::Folder, RustCareTheme::ACCENT_AMBER, 1.5);

            p.text(
                Pos2::new(rect.min.x + 42.0, rect.min.y + 16.0),
                Align2::LEFT_CENTER,
                "WizTree Disk Treemap",
                FontId::proportional(13.0),
                RustCareTheme::TEXT_PRIMARY,
            );
            p.text(
                Pos2::new(rect.min.x + 42.0, rect.min.y + 30.0),
                Align2::LEFT_CENTER,
                "Mapa de Disco",
                FontId::proportional(10.5),
                RustCareTheme::TEXT_MUTED,
            );

            // Botón Scan en la esquina superior derecha
            let scan_rect = Rect::from_min_size(Pos2::new(rect.max.x - 64.0, rect.min.y + 14.0), Vec2::new(50.0, 22.0));
            let scan_resp = ui.interact(scan_rect, ui.id().with("btn_wiztree_scan"), Sense::click());
            p.rect_filled(scan_rect, CornerRadius::same(5), if scan_resp.hovered() { RustCareTheme::BG_HOVER } else { RustCareTheme::BG_ELEVATED });
            p.rect_stroke(scan_rect, CornerRadius::same(5), Stroke::new(1.0, RustCareTheme::BORDER_SUBTLE), StrokeKind::Inside);
            p.text(scan_rect.center(), Align2::CENTER_CENTER, "Escanear", FontId::proportional(11.0), RustCareTheme::TEXT_PRIMARY);

            if scan_resp.clicked() || resp.clicked() {
                self.current_tab = ToolboxTab::DiskAnalyzer;
            }

            // Canvas Treemap densamente poblado
            let tree_rect = Rect::from_min_max(
                Pos2::new(rect.min.x + 12.0, rect.min.y + 44.0),
                Pos2::new(rect.max.x - 12.0, rect.max.y - 12.0),
            );
            p.rect_filled(tree_rect, CornerRadius::same(6), Color32::BLACK);

            let stroke_t = Stroke::new(1.5, Color32::from_rgb(18, 24, 36));
            let t_x0 = tree_rect.min.x;
            let t_y0 = tree_rect.min.y;
            let tw = tree_rect.width();
            let th = tree_rect.height();

            // Bloques proporcionales densos estilo WizTree (subdivisión anidada 2027_tools.png)
            let draw_sub_grid = |p: &egui::Painter, r: Rect, cols: usize, rows: usize, palette: &[Color32]| {
                let cw = r.width() / cols as f32;
                let ch = r.height() / rows as f32;
                for ry in 0..rows {
                    for rx in 0..cols {
                        let sub_r = Rect::from_min_size(
                            Pos2::new(r.min.x + rx as f32 * cw, r.min.y + ry as f32 * ch),
                            Vec2::new(cw, ch),
                        );
                        let col = palette[(rx + ry * 2) % palette.len()];
                        p.rect_filled(sub_r, CornerRadius::ZERO, col);
                        p.rect_stroke(sub_r, CornerRadius::ZERO, stroke_t, StrokeKind::Inside);
                    }
                }
            };

            let c_cyan = Color32::from_rgb(0, 229, 168);
            let c_teal = Color32::from_rgb(45, 212, 191);
            let c_sky = Color32::from_rgb(14, 165, 233);
            let c_blue = Color32::from_rgb(37, 99, 235);
            let c_indigo = Color32::from_rgb(99, 102, 241);
            let c_purple = Color32::from_rgb(168, 85, 247);
            let c_pink = Color32::from_rgb(236, 72, 153);
            let c_darkblue = Color32::from_rgb(30, 58, 138);

            let col1_w = tw * 0.44;
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x0, t_y0), Vec2::new(col1_w, th * 0.48)), 3, 3, &[c_cyan, c_teal, c_sky]);
            let half_c1 = col1_w * 0.5;
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x0, t_y0 + th * 0.48), Vec2::new(half_c1, th * 0.30)), 2, 2, &[c_teal, c_sky, c_cyan]);
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x0 + half_c1, t_y0 + th * 0.48), Vec2::new(half_c1, th * 0.30)), 2, 3, &[c_blue, c_indigo, c_darkblue]);
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x0, t_y0 + th * 0.78), Vec2::new(col1_w, th * 0.22)), 4, 2, &[c_indigo, c_purple, c_blue]);

            let t_x1 = t_x0 + col1_w;
            let col2_w = tw - col1_w;
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x1, t_y0), Vec2::new(col2_w * 0.65, th * 0.38)), 3, 2, &[c_blue, c_darkblue, c_indigo]);
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x1 + col2_w * 0.65, t_y0), Vec2::new(col2_w * 0.35, th * 0.38)), 2, 3, &[c_pink, c_purple, c_indigo]);
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x1, t_y0 + th * 0.38), Vec2::new(col2_w, th * 0.32)), 4, 2, &[c_darkblue, c_blue, c_sky]);
            let half_c2 = col2_w * 0.5;
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x1, t_y0 + th * 0.70), Vec2::new(half_c2, th * 0.30)), 3, 2, &[c_purple, c_indigo, c_pink]);
            draw_sub_grid(p, Rect::from_min_size(Pos2::new(t_x1 + half_c2, t_y0 + th * 0.70), Vec2::new(half_c2, th * 0.30)), 3, 2, &[c_pink, c_purple, c_blue]);

            // -----------------------------------------------------------------
            // COLUMNA DERECHA: Bento Grid Superior + Inferior
            // -----------------------------------------------------------------
            ui.vertical(|ui| {
                ui.set_width(right_w);

                // --- FILA DERECHA SUPERIOR (2 Tarjetas) ---
                let row_top_h = 162.0;
                let half_right_w = (right_w - spacing) * 0.5;

                let mut card2_rect_saved = Rect::ZERO;
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;

                    // TARJETA: Desinstalador Avanzado
                    let (card1_r, card1_resp) = ui.allocate_exact_size(Vec2::new(half_right_w, row_top_h), Sense::click());
                    RustCareTheme::draw_glass_card(ui.painter(), card1_r, card1_resp.hovered(), Some(RustCareTheme::ACCENT_CYAN));
                    let p1 = ui.painter();

                    // Título
                    p1.text(
                        Pos2::new(card1_r.min.x + 16.0, card1_r.min.y + 16.0),
                        Align2::LEFT_TOP,
                        "Desinstalador Avanzado",
                        FontId::proportional(14.0),
                        RustCareTheme::TEXT_PRIMARY,
                    );

                    // Gran Icono Neón
                    let ic_r1 = Rect::from_min_size(Pos2::new(card1_r.min.x + 18.0, card1_r.min.y + 44.0), Vec2::splat(58.0));
                    p1.rect_filled(ic_r1, CornerRadius::same(12), Color32::from_rgba_unmultiplied(0, 229, 168, 20));
                    p1.rect_stroke(ic_r1, CornerRadius::same(12), Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 229, 168, 80)), StrokeKind::Inside);
                    crate::icons::draw_icon(p1, ic_r1, AppIcon::Trash, RustCareTheme::ACCENT_CYAN, 1.8);

                    // Estadísticas a la derecha del icono
                    let tx_x1 = card1_r.min.x + 88.0;
                    p1.text(
                        Pos2::new(tx_x1, card1_r.min.y + 46.0),
                        Align2::LEFT_TOP,
                        &format!("{} apps encontradas", self.apps.len()),
                        FontId::proportional(14.5),
                        RustCareTheme::TEXT_PRIMARY,
                    );
                    p1.text(
                        Pos2::new(tx_x1, card1_r.min.y + 68.0),
                        Align2::LEFT_TOP,
                        "Estado de datos: Optimizado",
                        FontId::proportional(11.0),
                        RustCareTheme::TEXT_MUTED,
                    );
                    p1.text(
                        Pos2::new(tx_x1, card1_r.min.y + 86.0),
                        Align2::LEFT_TOP,
                        "Limpieza profunda de sobras",
                        FontId::proportional(11.0),
                        RustCareTheme::ACCENT_CYAN,
                    );

                    if card1_resp.clicked() {
                        self.drawer_open = Some(DrawerKind::Apps);
                    }

                    // TARJETA: Optimización IA de Inicio
                    let (card2_r, card2_resp) = ui.allocate_exact_size(Vec2::new(half_right_w, row_top_h), Sense::hover());
                    card2_rect_saved = card2_r;
                    RustCareTheme::draw_glass_card(ui.painter(), card2_r, card2_resp.hovered(), Some(RustCareTheme::ACCENT_INDIGO));

                    // Título
                    ui.painter().text(
                        Pos2::new(card2_r.min.x + 16.0, card2_r.min.y + 16.0),
                        Align2::LEFT_TOP,
                        "Optimización IA de Inicio",
                        FontId::proportional(14.0),
                        RustCareTheme::TEXT_PRIMARY,
                    );

                    // Gran Icono IA Cerebro Neón
                    let ic_r2 = Rect::from_min_size(Pos2::new(card2_r.min.x + 18.0, card2_r.min.y + 44.0), Vec2::splat(58.0));
                    ui.painter().rect_filled(ic_r2, CornerRadius::same(12), Color32::from_rgba_unmultiplied(56, 189, 248, 20));
                    ui.painter().rect_stroke(ic_r2, CornerRadius::same(12), Stroke::new(1.0, Color32::from_rgba_unmultiplied(56, 189, 248, 80)), StrokeKind::Inside);
                    crate::icons::draw_icon(ui.painter(), ic_r2, AppIcon::Brain, RustCareTheme::ACCENT_INDIGO, 1.8);

                    // Toggles a la derecha del icono (posicionados exactamente dentro de card2_r tras cerrar el horizontal)
                });

                let tog_x = card2_rect_saved.min.x + 86.0;
                let tog_r_w = (card2_rect_saved.max.x - tog_x - 14.0).max(80.0);
                let tog_rect = Rect::from_min_size(
                    Pos2::new(tog_x, card2_rect_saved.min.y + 44.0),
                    Vec2::new(tog_r_w, 94.0),
                );
                let mut toggle_ui = ui.new_child(egui::UiBuilder::new().max_rect(tog_rect));
                toggle_ui.set_width(tog_r_w);
                toggle_ui.horizontal(|ui| {
                    ui.label(RichText::new("32ms mejora").size(11.0).color(RustCareTheme::TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        crate::widgets::pill_toggle(ui, &mut self.ai_opt_boost);
                    });
                });
                toggle_ui.add_space(4.0);
                toggle_ui.horizontal(|ui| {
                    ui.label(RichText::new("Auto procesos").size(11.0).color(RustCareTheme::TEXT_MUTED));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        crate::widgets::pill_toggle(ui, &mut self.ai_opt_procs);
                    });
                });
                toggle_ui.add_space(4.0);
                toggle_ui.horizontal(|ui| {
                    ui.label(RichText::new("Inicio rápido").size(11.0).color(RustCareTheme::TEXT_MUTED));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut fast_boot = true;
                        crate::widgets::pill_toggle(ui, &mut fast_boot);
                    });
                });

                ui.add_space(spacing);

                // --- FILA DERECHA INFERIOR (3 Tarjetas) ---
                let row_bot_h = 152.0;
                let third_right_w = (right_w - (spacing * 2.0)) / 3.0;

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;

                    // TARJETA INFERIOR 1: Impulsor de Red
                    let (bot1_r, bot1_resp) = ui.allocate_exact_size(Vec2::new(third_right_w, row_bot_h), Sense::click());
                    RustCareTheme::draw_glass_card(ui.painter(), bot1_r, bot1_resp.hovered(), Some(Color32::from_rgb(45, 212, 191)));
                    let p_b1 = ui.painter();

                    p_b1.text(
                        Pos2::new(bot1_r.min.x + 14.0, bot1_r.min.y + 14.0),
                        Align2::LEFT_TOP,
                        "Impulsor de Red",
                        FontId::proportional(13.0),
                        RustCareTheme::TEXT_PRIMARY,
                    );

                    let ic_b1 = Rect::from_min_size(Pos2::new(bot1_r.min.x + 14.0, bot1_r.min.y + 40.0), Vec2::splat(40.0));
                    p_b1.rect_filled(ic_b1, CornerRadius::same(10), Color32::from_rgba_unmultiplied(45, 212, 191, 20));
                    p_b1.rect_stroke(ic_b1, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(45, 212, 191, 80)), StrokeKind::Inside);
                    crate::icons::draw_icon(p_b1, ic_b1, AppIcon::Gauge, Color32::from_rgb(45, 212, 191), 1.6);

                    // Mini sparkline de latencia junto al icono (estilo 2027_tools.png)
                    let net_box = Rect::from_min_max(
                        Pos2::new(bot1_r.min.x + 64.0, bot1_r.min.y + 42.0),
                        Pos2::new(bot1_r.max.x - 14.0, bot1_r.min.y + 78.0),
                    );
                    p_b1.rect_filled(net_box, CornerRadius::same(4), Color32::from_rgba_unmultiplied(11, 15, 23, 180));
                    p_b1.rect_stroke(net_box, CornerRadius::same(4), Stroke::new(0.8_f32, RustCareTheme::BORDER_SUBTLE), StrokeKind::Inside);
                    let net_pts = [0.65_f32, 0.55, 0.72, 0.42, 0.60, 0.50, 0.58, 0.48];
                    let n_step = net_box.width() / (net_pts.len() - 1) as f32;
                    for idx in 0..(net_pts.len() - 1) {
                        let p0 = Pos2::new(net_box.min.x + idx as f32 * n_step, net_box.min.y + net_pts[idx] * net_box.height());
                        let p1 = Pos2::new(net_box.min.x + (idx + 1) as f32 * n_step, net_box.min.y + net_pts[idx + 1] * net_box.height());
                        p_b1.line_segment([p0, p1], Stroke::new(1.5_f32, Color32::from_rgb(45, 212, 191)));
                    }

                    p_b1.text(
                        Pos2::new(bot1_r.min.x + 14.0, bot1_r.max.y - 28.0),
                        Align2::LEFT_CENTER,
                        "12ms Latencia",
                        FontId::proportional(12.0),
                        RustCareTheme::ACCENT_CYAN,
                    );
                    p_b1.text(
                        Pos2::new(bot1_r.min.x + 14.0, bot1_r.max.y - 14.0),
                        Align2::LEFT_CENTER,
                        "TCP/MTU Optimizado",
                        FontId::proportional(10.5),
                        RustCareTheme::TEXT_MUTED,
                    );

                    if bot1_resp.clicked() {
                        self.current_tab = ToolboxTab::InternetBooster;
                    }

                    // TARJETA INFERIOR 2: Destructor de Archivos
                    let (bot2_r, bot2_resp) = ui.allocate_exact_size(Vec2::new(third_right_w, row_bot_h), Sense::hover());
                    RustCareTheme::draw_glass_card(ui.painter(), bot2_r, bot2_resp.hovered(), Some(RustCareTheme::ACCENT_CYAN));

                    ui.painter().text(
                        Pos2::new(bot2_r.min.x + 14.0, bot2_r.min.y + 14.0),
                        Align2::LEFT_TOP,
                        "Destructor de Archivos",
                        FontId::proportional(13.0),
                        RustCareTheme::TEXT_PRIMARY,
                    );

                    let ic_b2 = Rect::from_min_size(Pos2::new(bot2_r.min.x + 14.0, bot2_r.min.y + 40.0), Vec2::splat(40.0));
                    ui.painter().rect_filled(ic_b2, CornerRadius::same(10), Color32::from_rgba_unmultiplied(56, 189, 248, 20));
                    ui.painter().rect_stroke(ic_b2, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(56, 189, 248, 80)), StrokeKind::Inside);
                    crate::icons::draw_icon(ui.painter(), ic_b2, AppIcon::Shredder, RustCareTheme::ACCENT_CYAN, 1.6);

                    let mut tog_shred = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(bot2_r.min.x + 60.0, bot2_r.min.y + 40.0), Vec2::new(third_right_w - 74.0, 90.0))));
                    tog_shred.horizontal(|ui| {
                        ui.label(RichText::new("DoD 5220").size(10.5).color(RustCareTheme::TEXT_MUTED));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            crate::widgets::pill_toggle(ui, &mut self.shred_dod_toggle);
                        });
                    });

                    let shred_btn_r = Rect::from_min_size(Pos2::new(bot2_r.min.x + 14.0, bot2_r.max.y - 30.0), Vec2::new(third_right_w - 28.0, 22.0));
                    let shred_btn = ui.interact(shred_btn_r, ui.id().with("btn_shred_open"), Sense::click());
                    ui.painter().rect_filled(shred_btn_r, CornerRadius::same(5), if shred_btn.hovered() { RustCareTheme::BG_HOVER } else { RustCareTheme::BG_ELEVATED });
                    ui.painter().rect_stroke(shred_btn_r, CornerRadius::same(5), Stroke::new(1.0, RustCareTheme::BORDER_SUBTLE), StrokeKind::Inside);
                    ui.painter().text(shred_btn_r.center(), Align2::CENTER_CENTER, "Abrir Destructor", FontId::proportional(11.0), RustCareTheme::TEXT_PRIMARY);

                    if shred_btn.clicked() {
                        self.current_tab = ToolboxTab::FileShredder;
                    }

                    // TARJETA INFERIOR 3: Protección de Drivers
                    let (bot3_r, bot3_resp) = ui.allocate_exact_size(Vec2::new(third_right_w, row_bot_h), Sense::hover());
                    RustCareTheme::draw_glass_card(ui.painter(), bot3_r, bot3_resp.hovered(), Some(Color32::from_rgb(37, 99, 235)));

                    ui.painter().text(
                        Pos2::new(bot3_r.min.x + 14.0, bot3_r.min.y + 14.0),
                        Align2::LEFT_TOP,
                        "Protección Drivers",
                        FontId::proportional(13.0),
                        RustCareTheme::TEXT_PRIMARY,
                    );

                    let ic_b3 = Rect::from_min_size(Pos2::new(bot3_r.min.x + 14.0, bot3_r.min.y + 40.0), Vec2::splat(40.0));
                    ui.painter().rect_filled(ic_b3, CornerRadius::same(10), Color32::from_rgba_unmultiplied(37, 99, 235, 20));
                    ui.painter().rect_stroke(ic_b3, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgba_unmultiplied(37, 99, 235, 80)), StrokeKind::Inside);
                    crate::icons::draw_icon(ui.painter(), ic_b3, AppIcon::ShieldCheck, Color32::from_rgb(37, 99, 235), 1.6);

                    let mut tog_drv = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(bot3_r.min.x + 60.0, bot3_r.min.y + 40.0), Vec2::new(third_right_w - 74.0, 90.0))));
                    tog_drv.horizontal(|ui| {
                        ui.label(RichText::new("Auto-act.").size(10.5).color(RustCareTheme::TEXT_MUTED));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            crate::widgets::pill_toggle(ui, &mut self.drivers_auto_toggle);
                        });
                    });

                    let drv_btn_r = Rect::from_min_size(Pos2::new(bot3_r.min.x + 14.0, bot3_r.max.y - 30.0), Vec2::new(third_right_w - 28.0, 22.0));
                    let drv_btn = ui.interact(drv_btn_r, ui.id().with("btn_drv_open"), Sense::click());
                    ui.painter().rect_filled(drv_btn_r, CornerRadius::same(5), if drv_btn.hovered() { RustCareTheme::BG_HOVER } else { RustCareTheme::BG_ELEVATED });
                    ui.painter().rect_stroke(drv_btn_r, CornerRadius::same(5), Stroke::new(1.0, RustCareTheme::BORDER_SUBTLE), StrokeKind::Inside);
                    ui.painter().text(drv_btn_r.center(), Align2::CENTER_CENTER, "Ver Drivers", FontId::proportional(11.0), RustCareTheme::TEXT_PRIMARY);

                    if drv_btn.clicked() {
                        self.drawer_open = Some(DrawerKind::Drivers);
                    }
                });
            });
        });

        ui.add_space(spacing);

        // =====================================================================
        // 3. BARRA DE ACCESO RÁPIDO A HERRAMIENTAS ESPECIALIZADAS
        // =====================================================================
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Herramientas Avanzadas:").size(11.5).strong().color(RustCareTheme::TEXT_MUTED));
            ui.add_space(4.0);

            let extra_tools = [
                ("Modo Reposo", ToolboxTab::SleepMode),
                ("Actualizador WinGet", ToolboxTab::Updater),
                ("Limpieza WinSxS DISM", ToolboxTab::DismWinSxs),
                ("Compactar SQLite", ToolboxTab::BrowserVacuum),
                ("Centro de Rescate", ToolboxTab::RescueCenter),
                ("Duplicados", ToolboxTab::Duplicates),
                ("Archivos Grandes", ToolboxTab::LargeFiles),
                ("Reparar Iconos", ToolboxTab::UiRepair),
                ("Planes de Energía", ToolboxTab::PowerPlan),
                ("TRIM SSD", ToolboxTab::DiskOptimizer),
            ];

            for (name, tab) in extra_tools {
                if ghost_button(ui, name, None).clicked() {
                    self.current_tab = tab;
                }
            }
        });
    }
}

mod disk_tools;
mod software_tools;
mod system_tools;


