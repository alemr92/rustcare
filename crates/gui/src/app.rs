//! Orquestador de Interfaz Gráfica de RustCare 2026
use crate::theme::RustCareTheme;
use crate::views::{BoosterView, CareView, CleanerView, MonitorView, SettingsView, ShieldView, ToolboxView};
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Rect, RichText, Stroke, StrokeKind, Ui, Vec2};
use rustcare_core::config::ConfigManager;
use rustcare_core::cleaner::{CleanerEngine, FullCleanResult, FullScanResult};
use rustcare_core::rules::RulesEngine;
use rustcare_core::security::is_elevated;
use rustcare_core::shield::{ShieldEngine, ShieldScanResult};
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
    current_section: NavSection,
    is_admin: bool,

    // Vistas
    care_view: CareView,
    cleaner_view: CleanerView,
    booster_view: BoosterView,
    shield_view: ShieldView,
    toolbox_view: ToolboxView,
    monitor_view: MonitorView,
    settings_view: SettingsView,

    // Canales asíncronos para evitar congelamientos de UI
    sender: Sender<AppEvent>,
    receiver: Receiver<AppEvent>,
}

impl RustCareApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let cleaner_config = RulesEngine::load_cleaner_rules()
            .unwrap_or_else(|_| rustcare_core::rules::CleanerConfig { groups: Vec::new() });

        Self {
            current_section: NavSection::Care,
            is_admin: is_elevated(),
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

        // Barra Superior de Estado y Privilegios 2026
        egui::TopBottomPanel::top("top_bar")
            .exact_height(54.0)
            .show_separator_line(true)
            .frame(egui::Frame::NONE.fill(RustCareTheme::TOPBAR_BG).inner_margin(egui::Margin::symmetric(12, 8)))
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    // Logotipo Futurista
                    ui.label(
                        RichText::new("⚡ RUSTCARE")
                            .size(21.0)
                            .strong()
                            .color(RustCareTheme::ACCENT_CYAN),
                    );
                    ui.label(
                        RichText::new("PRO 2026")
                            .size(11.0)
                            .strong()
                            .color(RustCareTheme::ACCENT_PURPLE),
                    );
                    ui.label(
                        RichText::new("• Cross-Platform Suite")
                            .small()
                            .color(RustCareTheme::TEXT_DIM),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
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
                        if self.is_admin {
                            RustCareTheme::draw_badge(
                                ui,
                                "🛡️ Modo Administrador Activo",
                                RustCareTheme::SUCCESS_GREEN,
                                Color32::from_rgba_premultiplied(16, 185, 129, 30),
                            );
                        } else {
                            RustCareTheme::draw_badge(
                                ui,
                                "🛡️ Modo Usuario Seguro (Zero-Admin)",
                                RustCareTheme::ACCENT_CYAN,
                                Color32::from_rgba_premultiplied(0, 229, 255, 25),
                            );
                        }
                    });
                });
            });

        // Barra Lateral de Navegación 2026
        egui::SidePanel::left("sidebar")
            .exact_width(225.0)
            .show(ctx, |ui| {
                ui.add_space(15.0);

                fn nav_item(ui: &mut Ui, label: &str, active: bool) -> bool {
                    let desired_size = Vec2::new(205.0, 44.0);
                    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

                    let bg_color = if active {
                        Color32::from_rgb(28, 34, 52)
                    } else if response.hovered() {
                        Color32::from_rgb(22, 26, 38)
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
                            RustCareTheme::ACCENT_CYAN,
                        );
                        ui.painter().rect_stroke(
                            rect,
                            CornerRadius::same(8),
                            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0, 229, 255, 50)),
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
                            ui.label(RichText::new("⚡ 100% Rust Nativo").small().strong().color(RustCareTheme::ACCENT_CYAN));
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

                    self.care_view.render(
                        ui,
                        move |_options| {
                            thread::spawn(move || {
                                let _ = tx.send(AppEvent::CareScanProgress(0.25, "Analizando según tus opciones configuradas...".into()));
                                let prefs = ConfigManager::load();
                                let mut cleaner_cfg = RulesEngine::load_cleaner_rules().unwrap();
                                cleaner_cfg.apply_preferences(&prefs);
                                let scan_res = CleanerEngine::scan_all(&cleaner_cfg);

                                let _ = tx.send(AppEvent::CareScanProgress(0.65, "Inspeccionando persistencias y procesos en memoria...".into()));
                                let shield_res = ShieldEngine::scan_all();

                                let _ = tx.send(AppEvent::CareScanProgress(0.95, "Consolidando métricas del sistema...".into()));
                                thread::sleep(Duration::from_millis(250));

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
                                let scan_res = CleanerEngine::scan_all(&cleaner_cfg);
                                let clean_res = CleanerEngine::clean_all(&scan_res);
                                let _ = rustcare_core::booster::BoosterEngine::clean_ram();

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
                    let scan_cache = self.cleaner_view.scan_result.clone();

                    self.cleaner_view.render(
                        ui,
                        move |_selected_targets| {
                            thread::spawn(move || {
                                let prefs = ConfigManager::load();
                                let mut config = RulesEngine::load_cleaner_rules().unwrap();
                                config.apply_preferences(&prefs);
                                let res = CleanerEngine::scan_all(&config);
                                let _ = tx.send(AppEvent::CleanerScanFinished(res));
                            });
                        },
                        move || {
                            if let Some(scan) = scan_cache {
                                thread::spawn(move || {
                                    let res = CleanerEngine::clean_all(&scan);
                                    let _ = clean_tx.send(AppEvent::CleanerCleanFinished(res));
                                });
                            }
                        },
                    );
                }
                NavSection::Booster => {
                    self.booster_view.render(ui);
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
