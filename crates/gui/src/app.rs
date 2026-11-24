//! Orquestador de Interfaz Gráfica de RustCare 2026 - Determinista Pixel-Perfect
use crate::icons::{draw_icon, AppIcon};
use crate::theme::RustCareTheme;
use crate::views::{BoosterView, CareView, CleanerView, MonitorView, SettingsView, ShieldView, ToolboxView};
use crate::widgets;
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Stroke, StrokeKind, Ui, Vec2};
use rustcare_core::backend::{LiveWindowsBackend, SandboxBackend, SystemBackend};
use rustcare_core::cleaner::{FullCleanResult, FullScanResult};
use rustcare_core::config::ConfigManager;
use rustcare_core::rules::RulesEngine;
use rustcare_core::security::is_elevated;
use rustcare_core::shield::{ShieldEngine, ShieldScanResult};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavSection {
    Care,
    Booster,
    Cleaner,
    Shield,
    Toolbox,
    Monitor,
    Settings,
}

pub enum AppEvent {
    CareScanProgress(f32, String),
    CareScanFinished {
        bytes_found: u64,
        files_found: usize,
        threats_found: usize,
    },
    CareCleanFinished {
        bytes_freed: u64,
        files_deleted: usize,
    },
    CleanerScanFinished(FullScanResult),
    CleanerCleanFinished(FullCleanResult),
    ShieldScanFinished(ShieldScanResult),
}

pub struct RustCareApp {
    pub current_section: NavSection,
    #[allow(dead_code)]
    pub is_admin: bool,
    pub backend: Arc<dyn SystemBackend>,

    // Vistas
    pub care_view: CareView,
    pub cleaner_view: CleanerView,
    pub booster_view: BoosterView,
    pub shield_view: ShieldView,
    pub toolbox_view: ToolboxView,
    pub monitor_view: MonitorView,
    pub settings_view: SettingsView,

    // Canales asíncronos para evitar congelamientos de UI
    sender: Sender<AppEvent>,
    receiver: Receiver<AppEvent>,
}

impl RustCareApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, is_sandbox: bool) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let cleaner_config = RulesEngine::load_cleaner_rules()
            .unwrap_or_else(|_| rustcare_core::rules::CleanerConfig { groups: Vec::new() });

        let backend: Arc<dyn SystemBackend> = if is_sandbox {
            Arc::new(SandboxBackend::new())
        } else {
            Arc::new(LiveWindowsBackend::default())
        };

        Self {
            current_section: NavSection::Care,
            is_admin: is_elevated(),
            backend,
            care_view: CareView::default(),
            cleaner_view: CleanerView::new(cleaner_config),
            booster_view: BoosterView::default(),
            shield_view: ShieldView::default(),
            toolbox_view: ToolboxView::default(),
            monitor_view: MonitorView::default(),
            settings_view: SettingsView::default(),
            sender,
            receiver,
        }
    }

    fn handle_events(&mut self) {
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                AppEvent::CareScanProgress(p, status) => {
                    self.care_view.progress = p;
                    self.care_view.status_text = status;
                }
                AppEvent::CareScanFinished {
                    bytes_found,
                    files_found,
                    threats_found,
                } => {
                    self.care_view.is_scanning = false;
                    self.care_view.scan_completed = true;
                    self.care_view.total_bytes_found = bytes_found;
                    self.care_view.total_items_found = files_found;
                    self.care_view.threats_found = threats_found;
                    self.care_view.health_score = 42; // Crítica
                    self.care_view.progress = 1.0;
                }
                AppEvent::CareCleanFinished {
                    bytes_freed,
                    files_deleted,
                } => {
                    self.care_view.is_cleaning = false;
                    self.care_view.clean_completed = true;
                    self.care_view.total_bytes_found = bytes_freed;
                    self.care_view.total_items_found = files_deleted;
                    self.care_view.health_score = 98; // Óptima
                    self.care_view.progress = 1.0;
                }
                AppEvent::CleanerScanFinished(res) => {
                    self.cleaner_view.is_scanning = false;
                    self.cleaner_view.scan_result = Some(res);
                }
                AppEvent::CleanerCleanFinished(res) => {
                    self.cleaner_view.is_cleaning = false;
                    self.cleaner_view.clean_result = Some(res);
                    if let Some(scan) = &mut self.cleaner_view.scan_result {
                        scan.total_bytes = 0;
                        scan.total_files = 0;
                    }
                }
                AppEvent::ShieldScanFinished(res) => {
                    self.shield_view.is_scanning = false;
                    self.shield_view.scan_result = Some(res);
                }
            }
        }
    }
}

impl eframe::App for RustCareApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_events();

        // =====================================================================
        // 1. TOPBAR EXACTA (height: 48.0px, fondo #090C10, borde 1.0px #232D3F)
        // =====================================================================
        egui::TopBottomPanel::top("top_bar")
            .exact_height(48.0)
            .show_separator_line(false)
            .frame(
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_VOID)
                    .stroke(Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE))
                    .inner_margin(egui::Margin::symmetric(16, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    // Logo vectorial LogoShield (24x24) + RUSTCARE PRO
                    let (logo_rect, _) = ui.allocate_exact_size(Vec2::new(24.0, 24.0), egui::Sense::hover());
                    draw_icon(ui.painter(), logo_rect, AppIcon::LogoShield, RustCareTheme::ACCENT_CYAN, 1.8);

                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("RUSTCARE PRO")
                            .size(17.5)
                            .strong()
                            .color(Color32::WHITE),
                    );

                    // Lado derecho de la Topbar (Controles de ventana estilo 2027: — □ ✕)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Botón Cerrar (✕)
                        let (close_rect, close_resp) = ui.allocate_exact_size(Vec2::new(32.0, 28.0), egui::Sense::click());
                        let p = ui.painter();
                        if close_resp.hovered() {
                            p.rect_filled(close_rect, CornerRadius::same(6), Color32::from_rgb(220, 38, 38));
                        }
                        let cl_c = close_rect.center();
                        let cl_col = if close_resp.hovered() { Color32::WHITE } else { RustCareTheme::TEXT_MUTED };
                        p.line_segment([Pos2::new(cl_c.x - 4.5, cl_c.y - 4.5), Pos2::new(cl_c.x + 4.5, cl_c.y + 4.5)], Stroke::new(1.4, cl_col));
                        p.line_segment([Pos2::new(cl_c.x + 4.5, cl_c.y - 4.5), Pos2::new(cl_c.x - 4.5, cl_c.y + 4.5)], Stroke::new(1.4, cl_col));
                        if close_resp.clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        // Botón Maximizar (□)
                        let (max_rect, max_resp) = ui.allocate_exact_size(Vec2::new(32.0, 28.0), egui::Sense::click());
                        let p = ui.painter();
                        if max_resp.hovered() {
                            p.rect_filled(max_rect, CornerRadius::same(6), RustCareTheme::BG_ELEVATED);
                        }
                        let mx_c = max_rect.center();
                        let mx_col = if max_resp.hovered() { Color32::WHITE } else { RustCareTheme::TEXT_MUTED };
                        p.rect_stroke(Rect::from_center_size(mx_c, Vec2::splat(9.0)), CornerRadius::ZERO, Stroke::new(1.2, mx_col), StrokeKind::Inside);
                        if max_resp.clicked() {
                            // Alternar maximizar
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(true));
                        }

                        // Botón Minimizar (—)
                        let (min_rect, min_resp) = ui.allocate_exact_size(Vec2::new(32.0, 28.0), egui::Sense::click());
                        let p = ui.painter();
                        if min_resp.hovered() {
                            p.rect_filled(min_rect, CornerRadius::same(6), RustCareTheme::BG_ELEVATED);
                        }
                        let mn_c = min_rect.center();
                        let mn_col = if min_resp.hovered() { Color32::WHITE } else { RustCareTheme::TEXT_MUTED };
                        p.line_segment([Pos2::new(mn_c.x - 4.5, mn_c.y), Pos2::new(mn_c.x + 4.5, mn_c.y)], Stroke::new(1.4, mn_col));
                        if min_resp.clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }

                        ui.add_space(8.0);

                        // Píldora discreta MODO SANDBOX / WINDOWS REAL
                        let is_sb = self.backend.is_sandbox();
                        let (pill_border, pill_bg, pill_text, pill_w) = if is_sb {
                            (RustCareTheme::ACCENT_AMBER, Color32::from_rgba_unmultiplied(245, 158, 11, 20), "MODO SANDBOX", 125.0)
                        } else {
                            (RustCareTheme::ACCENT_CORAL, Color32::from_rgba_unmultiplied(244, 63, 94, 20), "MODO REAL", 105.0)
                        };

                        let pill_size = Vec2::new(pill_w, 24.0);
                        let (pill_rect, pill_resp) = ui.allocate_exact_size(pill_size, egui::Sense::click());
                        let p = ui.painter();
                        p.rect_filled(pill_rect, CornerRadius::same(12), pill_bg);
                        p.rect_stroke(pill_rect, CornerRadius::same(12), Stroke::new(1.0_f32, pill_border), StrokeKind::Inside);
                        p.text(
                            pill_rect.center(),
                            Align2::CENTER_CENTER,
                            pill_text,
                            FontId::proportional(11.0),
                            pill_border,
                        );

                        if pill_resp.clicked() {
                            if is_sb {
                                self.backend = Arc::new(LiveWindowsBackend::default());
                            } else {
                                self.backend = Arc::new(SandboxBackend::new());
                            }
                            self.booster_view.startup_entries = self.backend.list_startup_entries();
                        }
                    });
                });
            });

        // =====================================================================
        // 2. SIDEBAR IZQUIERDA EXACTA (width: 210.0px, fondo #0D1118, borde #232D3F)
        // =====================================================================
        egui::SidePanel::left("sidebar")
            .exact_width(210.0)
            .show_separator_line(false)
            .frame(
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SIDEBAR)
                    .stroke(Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE)),
            )
            .show(ctx, |ui| {
                ui.add_space(10.0);

                // Función auxiliar de ítem de navegación con placa de icono tridimensional y distinción cromática
                let nav_item = |ui: &mut Ui, label: &str, active: bool, icon: AppIcon, accent: Color32| -> bool {
                    let desired_size = Vec2::new(210.0, 44.0);
                    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
                    let p = ui.painter();

                    let (bg_col, text_col, icon_stroke, badge_fill, badge_stroke) = if active {
                        (
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 22),
                            RustCareTheme::TEXT_PRIMARY,
                            accent,
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 38),
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 110),
                        )
                    } else if response.hovered() {
                        (
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 14),
                            RustCareTheme::TEXT_PRIMARY,
                            accent,
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 24),
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 60),
                        )
                    } else {
                        (
                            Color32::TRANSPARENT,
                            RustCareTheme::TEXT_MUTED,
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 180),
                            Color32::from_rgba_unmultiplied(26, 35, 52, 140),
                            RustCareTheme::BORDER_SUBTLE,
                        )
                    };

                    p.rect_filled(rect, CornerRadius::same(8), bg_col);

                    // Si está activo: contenedor con borde sutil y barra vertical neón brillante en borde derecho
                    if active {
                        p.rect_stroke(
                            rect,
                            CornerRadius::same(8),
                            Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 90)),
                            StrokeKind::Inside,
                        );
                        let bar_rect = Rect::from_min_size(
                            Pos2::new(rect.max.x - 3.5, rect.min.y + 6.0),
                            Vec2::new(3.5, rect.height() - 12.0),
                        );
                        // Halo difuso de la barra
                        p.rect_filled(
                            Rect::from_center_size(bar_rect.center(), Vec2::new(7.0, bar_rect.height() + 4.0)),
                            CornerRadius::same(3),
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 65),
                        );
                        p.rect_filled(bar_rect, CornerRadius::same(2), accent);
                    }

                    // Placa elevada del icono (26x26px) con borde e iluminación
                    let ic_badge = Rect::from_center_size(Pos2::new(rect.min.x + 24.0, rect.center().y), Vec2::splat(26.0));
                    p.rect_filled(ic_badge, CornerRadius::same(7), badge_fill);
                    p.rect_stroke(ic_badge, CornerRadius::same(7), Stroke::new(1.0_f32, badge_stroke), StrokeKind::Inside);
                    draw_icon(p, ic_badge, icon, icon_stroke, 1.5);

                    // Texto
                    p.text(
                        Pos2::new(rect.min.x + 46.0, rect.center().y),
                        Align2::LEFT_CENTER,
                        label,
                        FontId::proportional(13.5),
                        text_col,
                    );

                    response.clicked()
                };

                let cyan = Color32::from_rgb(0, 229, 168);
                if nav_item(ui, "Inicio", self.current_section == NavSection::Care, AppIcon::Home, cyan) {
                    self.current_section = NavSection::Care;
                }
                if nav_item(ui, "Limpieza", self.current_section == NavSection::Cleaner, AppIcon::Broom, Color32::from_rgb(56, 189, 248)) {
                    self.current_section = NavSection::Cleaner;
                }
                if nav_item(ui, "Seguridad", self.current_section == NavSection::Shield, AppIcon::ShieldCheck, Color32::from_rgb(16, 185, 129)) {
                    self.current_section = NavSection::Shield;
                }
                if nav_item(ui, "Rendimiento", self.current_section == NavSection::Booster, AppIcon::Gauge, Color32::from_rgb(245, 158, 11)) {
                    self.current_section = NavSection::Booster;
                }
                if nav_item(ui, "Herramientas", self.current_section == NavSection::Toolbox, AppIcon::Tools, Color32::from_rgb(129, 140, 248)) {
                    self.current_section = NavSection::Toolbox;
                }
                if nav_item(ui, "Monitor", self.current_section == NavSection::Monitor, AppIcon::Pulse, Color32::from_rgb(244, 63, 94)) {
                    self.current_section = NavSection::Monitor;
                }

                ui.add_space(8.0);
                // Separador 1px #232D3F
                let (sep_rect, _) = ui.allocate_exact_size(Vec2::new(190.0, 1.0), egui::Sense::hover());
                ui.painter().line_segment(
                    [Pos2::new(sep_rect.min.x + 10.0, sep_rect.min.y), Pos2::new(sep_rect.max.x + 10.0, sep_rect.min.y)],
                    Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE),
                );
                ui.add_space(8.0);

                if nav_item(ui, "Ajustes", self.current_section == NavSection::Settings, AppIcon::Gear, Color32::from_rgb(148, 163, 184)) {
                    self.current_section = NavSection::Settings;
                }

                // =====================================================================
                // MINI-PANEL ANCLADO ABAJO (CPU/RAM en vivo + Botón Liberar RAM)
                // =====================================================================
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(12.0);

                    egui::Frame::NONE
                        .fill(RustCareTheme::BG_SURFACE)
                        .corner_radius(CornerRadius::same(RustCareTheme::RADIUS_CARD as u8))
                        .stroke(Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE))
                        .inner_margin(egui::Margin::same(10))
                        .show(ui, |ui| {
                            ui.set_width(182.0);
                            let metrics = self.backend.get_metrics();
                            let cpu_pct = metrics.cpu_global_percent.clamp(0.0, 100.0);
                            let ram_pct = metrics.memory_usage_percent.clamp(0.0, 100.0);

                            // CPU Bar
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("CPU").size(11.0).color(RustCareTheme::TEXT_MUTED));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(format!("{:.0}%", cpu_pct)).size(11.0).strong().color(RustCareTheme::ACCENT_TEAL));
                                });
                            });
                            let (cpu_bar, _) = ui.allocate_exact_size(Vec2::new(162.0, 4.0), egui::Sense::hover());
                            ui.painter().rect_filled(cpu_bar, CornerRadius::same(2), RustCareTheme::BG_ELEVATED);
                            let fill_w = (cpu_bar.width() * (cpu_pct / 100.0)).max(2.0);
                            ui.painter().rect_filled(
                                Rect::from_min_size(cpu_bar.min, Vec2::new(fill_w, 4.0)),
                                CornerRadius::same(2),
                                RustCareTheme::ACCENT_TEAL,
                            );

                            ui.add_space(6.0);

                            // RAM Bar
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("RAM").size(11.0).color(RustCareTheme::TEXT_MUTED));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(format!("{:.0}%", ram_pct)).size(11.0).strong().color(RustCareTheme::ACCENT_CYAN));
                                });
                            });
                            let (ram_bar, _) = ui.allocate_exact_size(Vec2::new(162.0, 4.0), egui::Sense::hover());
                            ui.painter().rect_filled(ram_bar, CornerRadius::same(2), RustCareTheme::BG_ELEVATED);
                            let fill_ram_w = (ram_bar.width() * (ram_pct / 100.0)).max(2.0);
                            ui.painter().rect_filled(
                                Rect::from_min_size(ram_bar.min, Vec2::new(fill_ram_w, 4.0)),
                                CornerRadius::same(2),
                                RustCareTheme::ACCENT_CYAN,
                            );

                            ui.add_space(8.0);

                            // Botón ancho completo Liberar RAM con neon_button
                            if widgets::neon_button(ui, "Liberar RAM", Some(162.0)).clicked() {
                                let _ = self.backend.clean_ram();
                            }
                        });
                });
            });

        // =====================================================================
        // 3. CONTENIDO PRINCIPAL CENTRAL
        // =====================================================================
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(RustCareTheme::BG_VOID).inner_margin(egui::Margin::same(18)))
            .show(ctx, |ui| {
                let time = ui.input(|i| i.time) as f32;
                RustCareTheme::draw_ambient_mesh_glow(ui.painter(), ui.max_rect(), time);

                match self.current_section {
                    NavSection::Care => {
                        let tx = self.sender.clone();
                        let clean_tx = self.sender.clone();
                        let backend = self.backend.clone();
                        let clean_backend = self.backend.clone();

                        self.care_view.render(
                            ui,
                            move |_options| {
                                thread::spawn(move || {
                                    let _ = tx.send(AppEvent::CareScanProgress(0.18, "Iniciando escáner de alta velocidad...".into()));
                                    let prefs = ConfigManager::load();
                                    let mut cleaner_cfg = RulesEngine::load_cleaner_rules().unwrap();
                                    cleaner_cfg.apply_preferences(&prefs);

                                    let _ = tx.send(AppEvent::CareScanProgress(0.55, "Analizando archivos temporales, logs y cachés...".into()));
                                    let scan_res = backend.scan_junk(&cleaner_cfg);

                                    let _ = tx.send(AppEvent::CareScanProgress(0.82, "Inspeccionando persistencias y memoria...".into()));
                                    let shield_res = ShieldEngine::scan_all();

                                    let _ = tx.send(AppEvent::CareScanProgress(0.98, "Consolidando métricas del sistema...".into()));
                                    thread::sleep(Duration::from_millis(150));

                                    let _ = tx.send(AppEvent::CareScanFinished {
                                        bytes_found: scan_res.total_bytes,
                                        files_found: scan_res.total_files,
                                        threats_found: shield_res.threats.len(),
                                    });
                                });
                            },
                            move || {
                                thread::spawn(move || {
                                    let prefs = ConfigManager::load();
                                    let mut cleaner_cfg = RulesEngine::load_cleaner_rules().unwrap();
                                    cleaner_cfg.apply_preferences(&prefs);
                                    let scan_res = clean_backend.scan_junk(&cleaner_cfg);
                                    let clean_res = clean_backend.clean_junk(&scan_res);
                                    let _ = clean_backend.clean_ram();

                                    let _ = clean_tx.send(AppEvent::CareCleanFinished {
                                        bytes_freed: clean_res.total_bytes_freed,
                                        files_deleted: clean_res.total_files_deleted,
                                    });
                                });
                            },
                        );
                    }
                    NavSection::Booster => {
                        self.booster_view.render(ui, &self.backend);
                    }
                    NavSection::Cleaner => {
                        let tx = self.sender.clone();
                        let clean_tx = self.sender.clone();
                        let backend = self.backend.clone();
                        let clean_backend = self.backend.clone();
                        let scan_cache = self.cleaner_view.scan_result.clone();

                        self.cleaner_view.render(
                            ui,
                            move |_selected_targets| {
                                thread::spawn(move || {
                                    let prefs = ConfigManager::load();
                                    let mut config = RulesEngine::load_cleaner_rules().unwrap();
                                    config.apply_preferences(&prefs);
                                    let res = backend.scan_junk(&config);
                                    let _ = tx.send(AppEvent::CleanerScanFinished(res));
                                });
                            },
                            move || {
                                if let Some(scan) = scan_cache {
                                    thread::spawn(move || {
                                        let res = clean_backend.clean_junk(&scan);
                                        let _ = clean_tx.send(AppEvent::CleanerCleanFinished(res));
                                    });
                                }
                            },
                        );
                    }
                    NavSection::Shield => {
                        let tx = self.sender.clone();
                        self.shield_view.render(ui, move || {
                            thread::spawn(move || {
                                let res = ShieldEngine::scan_all();
                                let _ = tx.send(AppEvent::ShieldScanFinished(res));
                            });
                        });
                    }
                    NavSection::Toolbox => {
                        self.toolbox_view.render(ui);
                    }
                    NavSection::Monitor => {
                        self.monitor_view.render(ui);
                        ctx.request_repaint_after(Duration::from_millis(1500));
                    }
                    NavSection::Settings => {
                        self.settings_view.render(ui);
                    }
                }
            });

        if self.care_view.is_scanning
            || self.care_view.is_cleaning
            || self.cleaner_view.is_scanning
            || self.shield_view.is_scanning
        {
            ctx.request_repaint_after(Duration::from_millis(16));
        }
    }
}
