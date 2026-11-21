//! Orquestador de Interfaz Gráfica de RustCare
use crate::theme::RustCareTheme;
use crate::views::{BoosterView, CareView, CleanerView, MonitorView, ShieldView, ToolboxView};
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{self, Color32, RichText, Ui};
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

    // Canales asíncronos para evitar congelamientos de UI
    sender: Sender<AppEvent>,
    receiver: Receiver<AppEvent>,
}

impl RustCareApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let cleaner_config = RulesEngine::load_cleaner_rules().unwrap_or_else(|_| rustcare_core::rules::CleanerConfig { groups: Vec::new() });

        Self {
            current_section: NavSection::Care,
            is_admin: is_elevated(),
            care_view: CareView::default(),
            cleaner_view: CleanerView::new(cleaner_config),
            booster_view: BoosterView::default(),
            shield_view: ShieldView::default(),
            toolbox_view: ToolboxView::default(),
            monitor_view: MonitorView::default(),
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
                AppEvent::CareCleanFinished { bytes_freed, files_deleted } => {
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
                    // Actualizar el scan_result restando lo limpiado
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

        // Barra Superior de Estado y Privilegios
        egui::TopBottomPanel::top("top_bar")
            .exact_height(50.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚡ RUSTCARE").size(20.0).strong().color(RustCareTheme::ACCENT_CYAN));
                    ui.label(RichText::new("v0.1.0 • Windows 10/11 Suite").small().color(RustCareTheme::TEXT_MUTED));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_admin {
                            ui.label(
                                RichText::new("🛡️ Modo Administrador Activo")
                                    .strong()
                                    .color(RustCareTheme::SUCCESS_GREEN),
                            )
                            .on_hover_text("Tiene permisos completos para optimizar componentes globales del sistema.");
                        } else {
                            ui.label(
                                RichText::new("👤 Modo Usuario Estándar (Seguro)")
                                    .strong()
                                    .color(RustCareTheme::ACCENT_BLUE),
                            )
                            .on_hover_text("RustCare opera con seguridad sin privilegios de administrador. Las carpetas y servicios del sistema que requieran admin se omiten automáticamente.");
                        }
                    });
                });
            });

        // Barra Lateral de Navegación
        egui::SidePanel::left("sidebar")
            .exact_width(210.0)
            .show(ctx, |ui| {
                ui.add_space(15.0);

                fn nav_button(ui: &mut Ui, label: &str, active: bool) -> bool {
                    let text = if active {
                        RichText::new(label).strong().size(15.0).color(Color32::WHITE)
                    } else {
                        RichText::new(label).size(15.0).color(RustCareTheme::TEXT_PRIMARY)
                    };

                    let btn = egui::Button::new(text).fill(if active {
                        RustCareTheme::ACCENT_BLUE
                    } else {
                        RustCareTheme::SIDEBAR_BG
                    });

                    ui.add_sized([190.0, 42.0], btn).clicked()
                }

                if nav_button(ui, "🚀 Análisis 1-Clic", self.current_section == NavSection::Care) {
                    self.current_section = NavSection::Care;
                }
                ui.add_space(6.0);

                if nav_button(ui, "🧹 Limpieza & Datos", self.current_section == NavSection::Cleaner) {
                    self.current_section = NavSection::Cleaner;
                }
                ui.add_space(6.0);

                if nav_button(ui, "⚡ Aceleración (Booster)", self.current_section == NavSection::Booster) {
                    self.current_section = NavSection::Booster;
                }
                ui.add_space(6.0);

                if nav_button(ui, "🛡️ Seguridad (Shield)", self.current_section == NavSection::Shield) {
                    self.current_section = NavSection::Shield;
                }
                ui.add_space(6.0);

                if nav_button(ui, "🧰 Herramientas", self.current_section == NavSection::Toolbox) {
                    self.current_section = NavSection::Toolbox;
                }
                ui.add_space(6.0);

                if nav_button(ui, "📊 Monitor del Sistema", self.current_section == NavSection::Monitor) {
                    self.current_section = NavSection::Monitor;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.label(RichText::new("100% Rust Nativo • Seguro").small().color(RustCareTheme::TEXT_MUTED));
                });
            });

        // Contenido Principal Central
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            match self.current_section {
                NavSection::Care => {
                    let tx = self.sender.clone();
                    let clean_tx = self.sender.clone();

                    self.care_view.render(
                        ui,
                        move |_options| {
                            thread::spawn(move || {
                                let _ = tx.send(AppEvent::CareScanProgress(0.2, "Analizando temporales y basura...".into()));
                                let cleaner_cfg = RulesEngine::load_cleaner_rules().unwrap();
                                let scan_res = CleanerEngine::scan_all(&cleaner_cfg);

                                let _ = tx.send(AppEvent::CareScanProgress(0.6, "Auditando memoria y procesos sospechosos...".into()));
                                let shield_res = ShieldEngine::scan_all();

                                let _ = tx.send(AppEvent::CareScanProgress(0.9, "Consolidando métricas...".into()));
                                thread::sleep(Duration::from_millis(300));

                                let _ = tx.send(AppEvent::CareScanFinished {
                                    bytes_found: scan_res.total_bytes,
                                    files_found: scan_res.total_files,
                                    threats_found: shield_res.threats.len(),
                                });
                            });
                        },
                        move || {
                            thread::spawn(move || {
                                let cleaner_cfg = RulesEngine::load_cleaner_rules().unwrap();
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
                                let config = RulesEngine::load_cleaner_rules().unwrap();
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
            }
        });

        if self.care_view.is_scanning || self.care_view.is_cleaning || self.cleaner_view.is_scanning || self.shield_view.is_scanning {
            ctx.request_repaint();
        }
    }
}
