//! Orquestador de Interfaz Gráfica de RustCare 2026
use crate::theme::RustCareTheme;
use crate::views::{BoosterView, CareView, CleanerView, MonitorView, SettingsView, ShieldView, ToolboxView};
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Rect, RichText, Stroke, StrokeKind, Ui, Vec2};
use rustcare_core::backend::{LiveWindowsBackend, SandboxBackend, SystemBackend};
use rustcare_core::cleaner::{CleanerEngine, FullCleanResult, FullScanResult};
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
    Cleaner,
    Booster,
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
                    self.care_view.health_score = 42; // Estado crítico inicial tras escaneo
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
                    self.care_view.health_score = 98; // Óptimo
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

        // Barra Superior de Estado, Privilegios y Selector de Sandbox 2026
        egui::TopBottomPanel::top("top_bar")
            .exact_height(54.0)
            .show_separator_line(true)
            .frame(egui::Frame::NONE.fill(RustCareTheme::TOPBAR_BG).inner_margin(egui::Margin::symmetric(14, 8)))
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    // Logotipo Cyber-Dark
                    ui.label(
                        RichText::new("⚡ RUSTCARE")
                            .size(20.0)
                            .strong()
                            .color(RustCareTheme::ACCENT_CYAN),
                    );
                    ui.label(
                        RichText::new("ASC PRO 2026")
                            .size(11.0)
                            .strong()
                            .color(RustCareTheme::ACCENT_PURPLE),
                    );

                    ui.add_space(20.0);

                    // ==========================================
                    // Quick Performance Mini-Monitor en TopBar
                    // ==========================================
                    let metrics = self.backend.get_metrics();
                    let cpu_pct = metrics.cpu_global_percent;
                    let ram_pct = metrics.memory_usage_percent;

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("CPU:").small().color(RustCareTheme::TEXT_MUTED));
                        let cpu_col = if cpu_pct > 80.0 { RustCareTheme::DANGER_RED } else if cpu_pct > 50.0 { RustCareTheme::WARNING_AMBER } else { RustCareTheme::NEON_TEAL };
                        ui.label(RichText::new(format!("{:.0}%", cpu_pct)).small().strong().color(cpu_col));

                        ui.add_space(10.0);
                        ui.label(RichText::new("RAM:").small().color(RustCareTheme::TEXT_MUTED));
                        let ram_col = if ram_pct > 80.0 { RustCareTheme::DANGER_RED } else if ram_pct > 50.0 { RustCareTheme::WARNING_AMBER } else { RustCareTheme::NEON_TEAL };
                        ui.label(RichText::new(format!("{:.0}%", ram_pct)).small().strong().color(ram_col));
                    });

                    // Lado Derecho: Toggle Sandbox, Admin badge y Botón Opciones
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(6.0);
                        if ui
                            .button(
                                RichText::new("⚙️ Opciones")
                                    .size(13.0)
                                    .strong()
                                    .color(RustCareTheme::ACCENT_CYAN),
                            )
                            .clicked()
                        {
                            self.current_section = NavSection::Settings;
                        }

                        ui.add_space(8.0);

                        // ==========================================
                        // SELECTOR INTERACTIVO MODO SANDBOX / WINDOWS REAL
                        // ==========================================
                        let is_sb = self.backend.is_sandbox();
                        let (badge_text, badge_color, badge_bg) = if is_sb {
                            (
                                "🧪 MODO SANDBOX (Aislado Seguro)",
                                RustCareTheme::NEON_TEAL,
                                Color32::from_rgba_premultiplied(0, 229, 168, 30),
                            )
                        } else {
                            (
                                "🔴 MODO WINDOWS REAL",
                                RustCareTheme::DANGER_RED,
                                Color32::from_rgba_premultiplied(255, 77, 109, 30),
                            )
                        };

                        let btn_resp = ui.add(
                            egui::Button::new(
                                RichText::new(badge_text)
                                    .size(12.0)
                                    .strong()
                                    .color(badge_color),
                            )
                            .fill(badge_bg)
                            .stroke(Stroke::new(1.0, badge_color))
                            .corner_radius(CornerRadius::same(6)),
                        );

                        if btn_resp.clicked() {
                            if is_sb {
                                self.backend = Arc::new(LiveWindowsBackend::default());
                            } else {
                                self.backend = Arc::new(SandboxBackend::new());
                            }
                            self.booster_view.startup_entries = self.backend.list_startup_entries();
                        }

                        btn_resp.on_hover_text(if is_sb {
                            "Entorno de pruebas 100% aislado en ./target/sandbox_env/. Clic para pasar a Windows Real."
                        } else {
                            "Operando directamente sobre Windows real. Clic para pasar a Sandbox aislado de pruebas."
                        });

                        ui.add_space(8.0);
                        if self.is_admin {
                            RustCareTheme::draw_badge(
                                ui,
                                "🛡️ Admin",
                                RustCareTheme::SUCCESS_GREEN,
                                Color32::from_rgba_premultiplied(16, 185, 129, 25),
                            );
                        }
                    });
                });
            });

        // Barra Lateral de Navegación 2026
        egui::SidePanel::left("sidebar")
            .exact_width(225.0)
            .show(ctx, |ui| {
                ui.add_space(14.0);

                fn nav_item(ui: &mut Ui, label: &str, active: bool) -> bool {
                    let desired_size = Vec2::new(205.0, 44.0);
                    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

                    let bg_color = if active {
                        Color32::from_rgb(28, 36, 52)
                    } else if response.hovered() {
                        Color32::from_rgb(22, 28, 40)
                    } else {
                        Color32::TRANSPARENT
                    };

                    ui.painter().rect_filled(rect, CornerRadius::same(8), bg_color);

                    // Indicador de acento izquierdo para el elemento activo
                    if active {
                        let indicator_rect = Rect::from_min_size(
                            rect.min,
                            Vec2::new(4.0, rect.height()),
                        );
                        ui.painter().rect_filled(
                            indicator_rect,
                            CornerRadius::same(2),
                            RustCareTheme::NEON_TEAL,
                        );
                        ui.painter().rect_stroke(
                            rect,
                            CornerRadius::same(8),
                            Stroke::new(1.0, Color32::from_rgba_premultiplied(0, 229, 168, 50)),
                            StrokeKind::Inside,
                        );
                    }

                    let text_color = if active {
                        Color32::WHITE
                    } else if response.hovered() {
                        RustCareTheme::TEXT_PRIMARY
                    } else {
                        RustCareTheme::TEXT_MUTED
                    };

                    let font_id = FontId::proportional(15.0);
                    let text_pos = egui::pos2(rect.min.x + 14.0, rect.center().y);
                    ui.painter().text(
                        text_pos,
                        Align2::LEFT_CENTER,
                        label,
                        font_id,
                        text_color,
                    );

                    response.clicked()
                }

                if nav_item(ui, "🚀  Análisis 1-Clic", self.current_section == NavSection::Care) {
                    self.current_section = NavSection::Care;
                }
                ui.add_space(4.0);

                if nav_item(ui, "🧹  Limpieza & Datos", self.current_section == NavSection::Cleaner) {
                    self.current_section = NavSection::Cleaner;
                }
                ui.add_space(4.0);

                if nav_item(ui, "⚡  Aceleración (Booster)", self.current_section == NavSection::Booster) {
                    self.current_section = NavSection::Booster;
                }
                ui.add_space(4.0);

                if nav_item(ui, "🛡️  Seguridad (Shield)", self.current_section == NavSection::Shield) {
                    self.current_section = NavSection::Shield;
                }
                ui.add_space(4.0);

                if nav_item(ui, "🧰  Herramientas PRO", self.current_section == NavSection::Toolbox) {
                    self.current_section = NavSection::Toolbox;
                }
                ui.add_space(4.0);

                if nav_item(ui, "📊  Monitor de Sistema", self.current_section == NavSection::Monitor) {
                    self.current_section = NavSection::Monitor;
                }
                ui.add_space(4.0);

                if nav_item(ui, "⚙️  Ajustes & AutoCare", self.current_section == NavSection::Settings) {
                    self.current_section = NavSection::Settings;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(14.0);
                    RustCareTheme::card_frame().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("⚡ 100% Rust Nativo").small().strong().color(RustCareTheme::NEON_TEAL));
                            ui.label(RichText::new("Sin telemetría • Zero-Admin").small().color(RustCareTheme::TEXT_DIM));
                        });
                    });
                });
            });

        // Contenido Principal Central
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
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
                                let _ = tx.send(AppEvent::CareScanProgress(0.20, "Iniciando escáner en entorno...".into()));
                                let prefs = ConfigManager::load();
                                let mut cleaner_cfg = RulesEngine::load_cleaner_rules().unwrap();
                                cleaner_cfg.apply_preferences(&prefs);

                                let _ = tx.send(AppEvent::CareScanProgress(0.50, "Analizando archivos temporales, logs y cachés...".into()));
                                let scan_res = backend.scan_junk(&cleaner_cfg);

                                let _ = tx.send(AppEvent::CareScanProgress(0.80, "Inspeccionando persistencias y memoria...".into()));
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
                NavSection::Booster => {
                    self.booster_view.render(ui, &self.backend);
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
            ctx.request_repaint();
        }
    }
}
