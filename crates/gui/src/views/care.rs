//! Vista 1: Cuidar (Care) - Transparencia total e inspiración AVG TuneUp + Spatial Obsidian 2027
use crate::app::NavSection;
use crate::icons::{draw_icon, AppIcon};
use crate::theme::RustCareTheme;
use crate::toast::ToastManager;
use crate::widgets;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind,
    Ui, Vec2,
};
use rustcare_core::automation::AutomationEngine;
use rustcare_core::backend::{SystemBackend, TurboBoostMode};
use rustcare_core::cleaner::{CleanerEngine, FullScanResult};
use rustcare_core::rescue::RescueCenter;
use rustcare_core::shield::ShieldScanResult;
use rustcare_core::startup::StartupEntry;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CareDetailDrawer {
    Space,
    SpeedUp,
    Problems,
    Health,
    Rescue,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CareScanOptions {
    pub clean_junk: bool,
    pub clean_browsers: bool,
    pub privacy_telemetry: bool,
    pub ram_boost: bool,
    pub malware_shield: bool,
}

impl Default for CareScanOptions {
    fn default() -> Self {
        Self {
            clean_junk: true,
            clean_browsers: true,
            privacy_telemetry: true,
            ram_boost: true,
            malware_shield: true,
        }
    }
}

pub struct CareView {
    pub is_scanning: bool,
    pub is_cleaning: bool,
    pub scan_completed: bool,
    pub clean_completed: bool,
    pub progress: f32,
    pub status_text: String,
    pub total_bytes_found: u64,
    pub total_items_found: usize,
    pub threats_found: usize,
    pub ram_free_pct: f32,
    pub options: CareScanOptions,
    pub health_score: u8,
    pub last_scan_result: Option<FullScanResult>,
    pub last_shield_result: Option<ShieldScanResult>,
    pub last_startup_entries: Vec<StartupEntry>,
    pub detail_drawer: Option<CareDetailDrawer>,
    pub last_scan_time: Option<Instant>,
    pub auto_care_enabled: bool,
}

impl Default for CareView {
    fn default() -> Self {
        Self {
            is_scanning: false,
            is_cleaning: false,
            scan_completed: false,
            clean_completed: false,
            progress: 0.0,
            status_text: "Listo para analizar".to_string(),
            total_bytes_found: 0,
            total_items_found: 0,
            threats_found: 0,
            ram_free_pct: 64.0,
            options: CareScanOptions::default(),
            health_score: 100,
            last_scan_result: None,
            last_shield_result: None,
            last_startup_entries: Vec::new(),
            detail_drawer: None,
            last_scan_time: None,
            auto_care_enabled: AutomationEngine::is_autostart_enabled(),
        }
    }
}

impl CareView {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        backend: &Arc<dyn SystemBackend>,
        toasts: &mut ToastManager,
        on_start_scan: impl FnOnce(CareScanOptions),
        on_start_clean: impl FnOnce(),
    ) -> Option<NavSection> {
        let mut nav_request: Option<NavSection> = None;
        let mut start_scan_clicked = false;
        let mut start_clean_clicked = false;

        ui.vertical_centered(|ui| {
            ui.add_space(6.0);

            // =================================================================
            // 1. CABECERA TRANSPARENTE ESTILO AVG TUNEUP (Estado + Re-escanear + AutoCare)
            // =================================================================
            ui.horizontal(|ui| {
                // Izquierda: Toggle Mantenimiento Automático
                ui.label(RichText::new("Mantenimiento Automático:").size(11.5).color(RustCareTheme::TEXT_MUTED));
                let prev_auto = self.auto_care_enabled;
                if widgets::pill_toggle(ui, &mut self.auto_care_enabled).changed() && self.auto_care_enabled != prev_auto {
                    if backend.is_sandbox() {
                        toasts.info(
                            "Auto-Care (Sandbox)",
                            if self.auto_care_enabled { "Mantenimiento automático activado" } else { "Mantenimiento automático desactivado" },
                        );
                    } else if AutomationEngine::set_autostart(self.auto_care_enabled).is_ok() {
                        toasts.success(
                            "Mantenimiento Automático",
                            if self.auto_care_enabled { "Activado al inicio de Windows" } else { "Desactivado correctamente" },
                        );
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !self.is_scanning && !self.is_cleaning {
                        if widgets::ghost_button(ui, "Re-escanear", Some(105.0)).clicked() {
                            start_scan_clicked = true;
                        }
                    }
                    if let Some(t) = self.last_scan_time {
                        let mins = t.elapsed().as_secs() / 60;
                        let ago = if mins == 0 {
                            "Último análisis: hace un momento".to_string()
                        } else {
                            format!("Último análisis: hace {} min", mins)
                        };
                        ui.label(RichText::new(ago).size(11.5).color(RustCareTheme::TEXT_MUTED));
                    }
                });
            });
            ui.add_space(4.0);

            if self.is_scanning {
                ui.label(
                    RichText::new(format!("Analizando tu PC · {:.0}%", self.progress * 100.0))
                        .size(15.0)
                        .strong()
                        .color(RustCareTheme::ACCENT_CYAN),
                );
            } else if self.is_cleaning {
                ui.label(
                    RichText::new("Optimizando y reparando el sistema...")
                        .size(15.0)
                        .strong()
                        .color(RustCareTheme::ACCENT_EMERALD),
                );
            } else if self.scan_completed {
                let score_col = if self.health_score >= 80 {
                    RustCareTheme::ACCENT_EMERALD
                } else if self.health_score >= 55 {
                    RustCareTheme::ACCENT_AMBER
                } else {
                    RustCareTheme::ACCENT_CORAL
                };
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - 260.0).max(0.0) * 0.5);
                    ui.label(RichText::new("Tu PC está al").size(15.0).color(RustCareTheme::TEXT_MUTED));
                    ui.label(RichText::new(format!("{}%", self.health_score)).size(17.0).strong().color(score_col));
                    ui.label(RichText::new("optimizado").size(15.0).color(RustCareTheme::TEXT_MUTED));
                });
            } else {
                ui.label(
                    RichText::new("Tu PC · Haz clic en Analizar para auditar espacio, arranque y seguridad")
                        .size(14.0)
                        .color(RustCareTheme::TEXT_MUTED),
                );
            }

            ui.add_space(10.0);

            // =================================================================
            // 2. BOTÓN CIRCULAR RADAR (260x260px) + SCI-FI HUD BACKGROUND
            // =================================================================

            let (title, subtitle, health_opt) = if self.is_scanning {
                (format!("{:.0}%", self.progress * 100.0), "Auditando...".to_string(), None)
            } else if self.is_cleaning {
                ("LIMPIANDO".to_string(), "Reparando sistema...".to_string(), None)
            } else if self.scan_completed && !self.clean_completed {
                ("REPARAR".to_string(), "Ver o limpiar".to_string(), Some(self.health_score))
            } else if self.clean_completed {
                ("ÓPTIMO".to_string(), "Sistema optimizado".to_string(), Some(self.health_score))
            } else {
                ("ANALIZAR".to_string(), String::new(), None)
            };

            // Overlay Sci-Fi HUD en el fondo del reactor
            let cursor_pos = ui.cursor().min;
            let hud_center = Pos2::new(cursor_pos.x + ui.available_width() / 2.0, cursor_pos.y + 130.0);
            Self::draw_scifi_hud_overlay(ui.painter(), hud_center);

            let radar_resp = widgets::care_radar_button(
                ui,
                &title,
                &subtitle,
                self.progress,
                self.is_scanning || self.is_cleaning,
                health_opt,
            );

            if radar_resp.clicked() {
                if self.scan_completed && !self.clean_completed {
                    start_clean_clicked = true;
                } else {
                    start_scan_clicked = true;
                }
            }

            // =================================================================
            // 2B. TIRA DE TRANSPARENCIA EN VIVO DURANTE EL ESCANEO
            // =================================================================
            if self.is_scanning || self.is_cleaning {
                ui.add_space(6.0);
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_max_width(680.0);
                    ui.horizontal(|ui| {
                        let stages = [
                            ("1. Basura y Cachés", 0.15_f32),
                            ("2. Programas Inicio", 0.45_f32),
                            ("3. Escudo Heurístico", 0.74_f32),
                            ("4. Memoria RAM", 0.94_f32),
                        ];
                        for (label, threshold) in stages {
                            let done = self.progress > threshold + 0.15;
                            let active = !done && self.progress >= threshold - 0.05;
                            let col = if done {
                                RustCareTheme::ACCENT_EMERALD
                            } else if active {
                                RustCareTheme::ACCENT_CYAN
                            } else {
                                RustCareTheme::TEXT_MUTED
                            };
                            let (dot_r, _) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                            let c = dot_r.center();
                            if done {
                                ui.painter().circle_filled(c, 5.5, col);
                                ui.painter().line_segment(
                                    [Pos2::new(c.x - 2.4, c.y + 0.2), Pos2::new(c.x - 0.6, c.y + 2.0)],
                                    Stroke::new(1.5_f32, Color32::WHITE),
                                );
                                ui.painter().line_segment(
                                    [Pos2::new(c.x - 0.6, c.y + 2.0), Pos2::new(c.x + 2.6, c.y - 1.8)],
                                    Stroke::new(1.5_f32, Color32::WHITE),
                                );
                            } else if active {
                                ui.painter().circle_filled(c, 4.5, col);
                            } else {
                                ui.painter().circle_stroke(c, 4.5, Stroke::new(1.2_f32, col));
                            }
                            ui.label(RichText::new(label).size(11.0).color(col));
                            ui.add_space(10.0);
                        }
                    });
                    ui.label(RichText::new(&self.status_text).size(11.5).color(RustCareTheme::TEXT_PRIMARY));
                });
                ui.add_space(8.0);
            } else {
                ui.add_space(16.0);
            }

            // =================================================================
            // 3. FILA INFERIOR: 4 BENTO GLASS CARDS 2027 (Icono 48px + Veredicto AVG)
            // =================================================================
            let avail_w = ui.available_width();
            let card_spacing = RustCareTheme::ITEM_SPACING;
            let card_w = (avail_w - (card_spacing * 3.0)) / 4.0;
            let card_h = 112.0;

            let enabled_startup = self.last_startup_entries.iter().filter(|e| e.is_enabled).count();

            let draw_bento_care_card = |ui: &mut Ui,
                                        title: &str,
                                        verdict: &str,
                                        sub: &str,
                                        icon: AppIcon,
                                        accent: Color32,
                                        is_ok: bool|
             -> egui::Response {
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(card_w, card_h), Sense::click());
                RustCareTheme::draw_glass_card(ui.painter(), rect, resp.hovered(), Some(accent));
                let p = ui.painter();

                // Icono volumétrico 48x48 a la izquierda (fidelidad 2027_care.png)
                let ic_box = Rect::from_min_size(Pos2::new(rect.min.x + 14.0, rect.center().y - 24.0), Vec2::splat(48.0));
                draw_icon(p, ic_box, icon, accent, 1.7);

                // Insignia de estado AVG vectorial (check o !) arriba a la derecha
                let badge_c = Pos2::new(rect.max.x - 16.0, rect.min.y + 16.0);
                let badge_col = if is_ok { RustCareTheme::ACCENT_EMERALD } else { accent };
                p.circle_filled(badge_c, 7.5, badge_col);
                if is_ok {
                    p.line_segment(
                        [Pos2::new(badge_c.x - 3.2, badge_c.y + 0.2), Pos2::new(badge_c.x - 0.8, badge_c.y + 2.6)],
                        Stroke::new(1.6_f32, Color32::WHITE),
                    );
                    p.line_segment(
                        [Pos2::new(badge_c.x - 0.8, badge_c.y + 2.6), Pos2::new(badge_c.x + 3.4, badge_c.y - 2.4)],
                        Stroke::new(1.6_f32, Color32::WHITE),
                    );
                } else {
                    p.text(badge_c, Align2::CENTER_CENTER, "!", FontId::proportional(10.0), Color32::WHITE);
                }

                // Título, Veredicto Grande y Subtexto a la derecha del icono
                let tx_x = rect.min.x + 72.0;
                p.text(Pos2::new(tx_x, rect.min.y + 18.0), Align2::LEFT_TOP, title, FontId::proportional(10.5), RustCareTheme::TEXT_MUTED);
                p.text(Pos2::new(tx_x, rect.min.y + 42.0), Align2::LEFT_TOP, verdict, FontId::proportional(18.0), Color32::WHITE);
                p.text(Pos2::new(tx_x, rect.max.y - 24.0), Align2::LEFT_TOP, sub, FontId::proportional(10.5), badge_col);

                resp
            };

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = card_spacing;

                // TARJETA 1: ARCHIVOS BASURA / LIBERAR ESPACIO
                let space_ok = !self.scan_completed || self.total_bytes_found == 0 || self.clean_completed;
                let c1_accent = if space_ok { Color32::from_rgb(0, 229, 168) } else { RustCareTheme::ACCENT_AMBER };
                let space_verdict = if self.scan_completed && self.total_bytes_found > 0 && !self.clean_completed {
                    CleanerEngine::format_bytes(self.total_bytes_found)
                } else if self.clean_completed {
                    format!("{} limpios", CleanerEngine::format_bytes(self.total_bytes_found))
                } else if self.scan_completed {
                    "0 GB".to_string()
                } else {
                    "Pendiente".to_string()
                };
                let space_sub = if self.scan_completed && !self.clean_completed {
                    format!("{} archivos · Ver →", self.total_items_found)
                } else if self.clean_completed {
                    "Espacio recuperado".to_string()
                } else {
                    "Haz clic para auditar".to_string()
                };
                if draw_bento_care_card(ui, "ARCHIVOS BASURA", &space_verdict, &space_sub, AppIcon::Trash, c1_accent, space_ok).clicked() {
                    if self.scan_completed {
                        self.detail_drawer = Some(CareDetailDrawer::Space);
                    } else {
                        start_scan_clicked = true;
                    }
                }

                // TARJETA 2: ESCUDO PRIVACIDAD
                let sec_ok = self.threats_found == 0;
                let c2_accent = if sec_ok { Color32::from_rgb(56, 189, 248) } else { RustCareTheme::ACCENT_CORAL };
                let sec_verdict = if sec_ok { "Seguro".to_string() } else { format!("{} alertas", self.threats_found) };
                let sec_sub = if let Some(sh) = &self.last_shield_result {
                    format!("{} proc · {} claves", sh.processes_scanned, sh.registry_keys_scanned)
                } else {
                    "Auditoría heurística".to_string()
                };
                if draw_bento_care_card(ui, "ESCUDO PRIVACIDAD", &sec_verdict, &sec_sub, AppIcon::Eye, c2_accent, sec_ok).clicked() {
                    self.detail_drawer = Some(CareDetailDrawer::Problems);
                }

                // TARJETA 3: RAM TURBO / ACELERAR PC
                let speed_ok = !self.scan_completed || enabled_startup <= 2;
                let c3_accent = Color32::from_rgb(45, 212, 191);
                let ram_verdict = format!("{:.0}% Libre", self.ram_free_pct);
                let ram_sub = if self.scan_completed {
                    format!("{} en inicio · Gestionar →", enabled_startup)
                } else {
                    "Inicio y Memoria".to_string()
                };
                if draw_bento_care_card(ui, "RAM TURBO", &ram_verdict, &ram_sub, AppIcon::RamChip, c3_accent, speed_ok).clicked() {
                    if self.last_startup_entries.is_empty() {
                        self.last_startup_entries = backend.list_startup_entries();
                    }
                    self.detail_drawer = Some(CareDetailDrawer::SpeedUp);
                }

                // TARJETA 4: SALUD DEL SISTEMA IA
                let health_ok = self.health_score >= 75;
                let c4_accent = if self.health_score >= 80 {
                    Color32::from_rgb(0, 229, 168)
                } else if self.health_score >= 55 {
                    RustCareTheme::ACCENT_AMBER
                } else {
                    RustCareTheme::ACCENT_CORAL
                };
                let health_verdict = if self.scan_completed {
                    format!("{}% Óptimo", self.health_score)
                } else {
                    "Excelente".to_string()
                };
                let health_sub = if self.scan_completed { "Ver cálculo →" } else { "Diagnóstico IA" };
                if draw_bento_care_card(ui, "SALUD DEL SISTEMA IA", &health_verdict, health_sub, AppIcon::Brain, c4_accent, health_ok).clicked() {
                    self.detail_drawer = Some(CareDetailDrawer::Health);
                }
            });

            // =================================================================
            // 4. BARRA INFERIOR ESTILO AVG TUNEUP (Perfiles Rápidos + Centro de Rescate)
            // =================================================================
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if widgets::ghost_button(ui, "Modo Juego", Some(110.0)).clicked() {
                    let res = backend.apply_turbo_boost(TurboBoostMode::Game);
                    toasts.success("Perfil Modo Juego activo", res.message);
                }
                if widgets::ghost_button(ui, "Modo Economía", Some(125.0)).clicked() {
                    let res = backend.apply_turbo_boost(TurboBoostMode::Economy);
                    toasts.info("Perfil Economía activo", res.message);
                }
                if widgets::ghost_button(ui, "Limpieza", Some(95.0)).clicked() {
                    nav_request = Some(NavSection::Cleaner);
                }
                if widgets::ghost_button(ui, "Todas las Funciones", Some(150.0)).clicked() {
                    nav_request = Some(NavSection::Toolbox);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if widgets::neon_button(ui, "Centro de Rescate", Some(155.0)).clicked() {
                        self.detail_drawer = Some(CareDetailDrawer::Rescue);
                    }
                });
            });

            if start_scan_clicked {
                self.is_scanning = true;
                self.scan_completed = false;
                self.clean_completed = false;
                self.progress = 0.05;
                self.status_text = "Iniciando auditoría transparente...".to_string();
                on_start_scan(self.options.clone());
            }

            if start_clean_clicked {
                self.is_cleaning = true;
                self.progress = 0.05;
                on_start_clean();
            }
        });

        // =====================================================================
        // 5. DRAWER DESLIZABLE TRANSPARENTE (ESTILO AVG TUNEUP + VAUL)
        // =====================================================================
        if let Some(drawer) = self.detail_drawer {
            let mut is_open = true;
            match drawer {
                CareDetailDrawer::Space => {
                    let scan_opt = self.last_scan_result.clone();
                    widgets::slide_over_drawer(
                        ui.ctx(),
                        &mut is_open,
                        "Desglose de Espacio Recuperable",
                        "Categorías exactas auditadas en tu sistema",
                        RustCareTheme::ACCENT_AMBER,
                        430.0,
                        |ui| {
                            if let Some(res) = scan_opt {
                                if widgets::neon_button(ui, "Abrir Limpieza Granular", Some(220.0)).clicked() {
                                    nav_request = Some(NavSection::Cleaner);
                                }
                                ui.add_space(10.0);
                                if res.targets.is_empty() {
                                    ui.label(RichText::new("No se encontraron archivos temporales.").color(RustCareTheme::TEXT_MUTED));
                                }
                                for t in &res.targets {
                                    if t.file_count == 0 && !t.skipped_unprivileged {
                                        continue;
                                    }
                                    RustCareTheme::card_frame().show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&t.target_name).strong().color(RustCareTheme::TEXT_PRIMARY));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(
                                                    RichText::new(CleanerEngine::format_bytes(t.total_bytes))
                                                        .strong()
                                                        .color(RustCareTheme::ACCENT_AMBER),
                                                );
                                            });
                                        });
                                        ui.label(
                                            RichText::new(format!("{} archivos · Grupo: {}", t.file_count, t.group_id))
                                                .size(11.0)
                                                .color(RustCareTheme::TEXT_MUTED),
                                        );
                                    });
                                    ui.add_space(6.0);
                                }
                            } else {
                                ui.label(RichText::new("Ejecuta un análisis primero para ver el desglose por categorías.").color(RustCareTheme::TEXT_MUTED));
                            }
                        },
                    );
                }
                CareDetailDrawer::SpeedUp => {
                    let mut entries = self.last_startup_entries.clone();
                    let mut toggled_idx: Option<(usize, String, bool)> = None;
                    widgets::slide_over_drawer(
                        ui.ctx(),
                        &mut is_open,
                        "Programas que ralentizan el arranque",
                        "Activa o desactiva elementos de inicio sin perder datos",
                        RustCareTheme::ACCENT_CYAN,
                        460.0,
                        |ui| {
                            if widgets::neon_button(ui, "Abrir Acelerador Completo", Some(220.0)).clicked() {
                                nav_request = Some(NavSection::Booster);
                            }
                            ui.add_space(10.0);
                            if entries.is_empty() {
                                ui.label(RichText::new("No se detectaron entradas de inicio.").color(RustCareTheme::TEXT_MUTED));
                            }
                            for (idx, entry) in entries.iter_mut().enumerate() {
                                let is_high = entry.command.to_lowercase().contains("update")
                                    || entry.command.to_lowercase().contains("helper")
                                    || entry.is_enabled;
                                let (imp_label, imp_frac, imp_col) = if entry.is_enabled && is_high {
                                    ("Medio", 0.55_f32, RustCareTheme::ACCENT_AMBER)
                                } else {
                                    ("Bajo", 0.22_f32, RustCareTheme::ACCENT_EMERALD)
                                };

                                RustCareTheme::card_frame().show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label(RichText::new(&entry.name).strong().color(RustCareTheme::TEXT_PRIMARY));
                                            ui.horizontal(|ui| {
                                                // Barra de impacto estilo AVG TuneUp (capturas 2 y 5)
                                                let (bar_r, _) = ui.allocate_exact_size(Vec2::new(90.0, 6.0), Sense::hover());
                                                let bp = ui.painter();
                                                bp.rect_filled(bar_r, CornerRadius::same(3), RustCareTheme::BG_ELEVATED);
                                                bp.rect_filled(
                                                    Rect::from_min_size(bar_r.min, Vec2::new(bar_r.width() * imp_frac, 6.0)),
                                                    CornerRadius::same(3),
                                                    imp_col,
                                                );
                                                ui.label(RichText::new(imp_label).size(10.5).color(imp_col));
                                            });
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            let prev = entry.is_enabled;
                                            if widgets::pill_toggle(ui, &mut entry.is_enabled).changed() && entry.is_enabled != prev {
                                                toggled_idx = Some((idx, entry.id.clone(), entry.is_enabled));
                                            }
                                        });
                                    });
                                });
                                ui.add_space(6.0);
                            }
                        },
                    );
                    if let Some((idx, id, enable)) = toggled_idx {
                        if backend.toggle_startup_entry(&id, enable).is_ok() {
                            if let Some(item) = self.last_startup_entries.get_mut(idx) {
                                item.is_enabled = enable;
                            }
                        }
                    }
                }
                CareDetailDrawer::Problems => {
                    let shield_opt = self.last_shield_result.clone();
                    widgets::slide_over_drawer(
                        ui.ctx(),
                        &mut is_open,
                        "Auditoría de Seguridad y Problemas",
                        "Resultados reales de memoria y registro de Windows",
                        if self.threats_found == 0 { RustCareTheme::ACCENT_EMERALD } else { RustCareTheme::ACCENT_CORAL },
                        430.0,
                        |ui| {
                            if widgets::neon_button(ui, "Abrir RustCare Shield", Some(220.0)).clicked() {
                                nav_request = Some(NavSection::Shield);
                            }
                            ui.add_space(10.0);
                            if let Some(sh) = shield_opt {
                                RustCareTheme::card_frame().show(ui, |ui| {
                                    ui.label(RichText::new(format!("• Procesos analizados: {}", sh.processes_scanned)).color(RustCareTheme::TEXT_PRIMARY));
                                    ui.label(RichText::new(format!("• Claves de registro auditadas: {}", sh.registry_keys_scanned)).color(RustCareTheme::TEXT_PRIMARY));
                                    ui.label(RichText::new("• Vectores comprobados: binarios en %TEMP%, suplantación svchost.exe, IFEO y AppInit_DLLs").size(11.5).color(RustCareTheme::TEXT_MUTED));
                                });
                                ui.add_space(10.0);
                                if sh.threats.is_empty() {
                                    ui.label(RichText::new("✓ Sin amenazas ni persistencias maliciosas detectadas.").color(RustCareTheme::ACCENT_EMERALD));
                                } else {
                                    for th in &sh.threats {
                                        RustCareTheme::card_frame().show(ui, |ui| {
                                            ui.label(RichText::new(&th.title).strong().color(RustCareTheme::ACCENT_CORAL));
                                            ui.label(RichText::new(&th.description).size(11.5).color(RustCareTheme::TEXT_MUTED));
                                        });
                                        ui.add_space(6.0);
                                    }
                                }
                            } else {
                                ui.label(RichText::new("Ejecuta un análisis para ver los procesos y claves de registro auditadas.").color(RustCareTheme::TEXT_MUTED));
                            }
                        },
                    );
                }
                CareDetailDrawer::Health => {
                    let enabled_cnt = self.last_startup_entries.iter().filter(|e| e.is_enabled).count();
                    let bytes = self.total_bytes_found;
                    let threats = self.threats_found;
                    let ram_free = self.ram_free_pct;
                    let score = self.health_score;
                    widgets::slide_over_drawer(
                        ui.ctx(),
                        &mut is_open,
                        "Cálculo Transparente de Salud",
                        "Cómo se calcula tu porcentaje de optimización",
                        RustCareTheme::ACCENT_CYAN,
                        420.0,
                        |ui| {
                            RustCareTheme::card_frame().show(ui, |ui| {
                                ui.label(RichText::new(format!("Puntuación actual: {} / 100", score)).size(16.0).strong().color(RustCareTheme::ACCENT_CYAN));
                                ui.add_space(8.0);
                                ui.label(RichText::new(format!("• Basura acumulada: {}", CleanerEngine::format_bytes(bytes))).color(RustCareTheme::TEXT_PRIMARY));
                                ui.label(RichText::new(format!("• Programas activos en inicio: {}", enabled_cnt)).color(RustCareTheme::TEXT_PRIMARY));
                                ui.label(RichText::new(format!("• Amenazas de seguridad: {}", threats)).color(RustCareTheme::TEXT_PRIMARY));
                                ui.label(RichText::new(format!("• Memoria RAM libre: {:.0}%", ram_free)).color(RustCareTheme::TEXT_PRIMARY));
                            });
                        },
                    );
                }
                CareDetailDrawer::Rescue => {
                    let backups = RescueCenter::list_backups();
                    widgets::slide_over_drawer(
                        ui.ctx(),
                        &mut is_open,
                        "Centro de Rescate (Deshacer Cambios)",
                        "Copias de seguridad .reg y puntos de restauración",
                        RustCareTheme::ACCENT_EMERALD,
                        430.0,
                        |ui| {
                            if widgets::neon_button(ui, "Abrir Centro de Rescate Completo", Some(250.0)).clicked() {
                                nav_request = Some(NavSection::Toolbox);
                            }
                            ui.add_space(10.0);
                            if backups.is_empty() {
                                ui.label(RichText::new("No hay copias .reg guardadas todavía. Se crean automáticamente antes de modificar el registro.").color(RustCareTheme::TEXT_MUTED));
                            } else {
                                for b in &backups {
                                    RustCareTheme::card_frame().show(ui, |ui| {
                                        ui.label(RichText::new(&b.file_name).strong().color(RustCareTheme::TEXT_PRIMARY));
                                        ui.label(
                                            RichText::new(format!("{} · {}", b.created_at, CleanerEngine::format_bytes(b.size_bytes)))
                                                .size(11.0)
                                                .color(RustCareTheme::TEXT_MUTED),
                                        );
                                    });
                                    ui.add_space(6.0);
                                }
                            }
                        },
                    );
                }
            }
            if !is_open || nav_request.is_some() {
                self.detail_drawer = None;
            }
        }

        nav_request
    }

    fn draw_scifi_hud_overlay(painter: &eframe::egui::Painter, center: Pos2) {
        let hud_cyan = Color32::from_rgba_unmultiplied(56, 189, 248, 55);
        let hud_cyan_bright = Color32::from_rgba_unmultiplied(56, 189, 248, 120);
        let text_dim = Color32::from_rgba_unmultiplied(160, 185, 215, 115);

        let sx = 225.0_f32;
        let sy = 135.0_f32;
        let blen = 22.0_f32;

        let tl = Pos2::new(center.x - sx, center.y - sy);
        painter.line_segment([tl, Pos2::new(tl.x + blen, tl.y)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([tl, Pos2::new(tl.x, tl.y + blen)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([Pos2::new(tl.x - 4.0, tl.y), Pos2::new(tl.x + 4.0, tl.y)], Stroke::new(1.0_f32, hud_cyan_bright));
        painter.line_segment([Pos2::new(tl.x, tl.y - 4.0), Pos2::new(tl.x, tl.y + 4.0)], Stroke::new(1.0_f32, hud_cyan_bright));

        let tr = Pos2::new(center.x + sx, center.y - sy);
        painter.line_segment([tr, Pos2::new(tr.x - blen, tr.y)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([tr, Pos2::new(tr.x, tr.y + blen)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([Pos2::new(tr.x - 4.0, tr.y), Pos2::new(tr.x + 4.0, tr.y)], Stroke::new(1.0_f32, hud_cyan_bright));
        painter.line_segment([Pos2::new(tr.x, tr.y - 4.0), Pos2::new(tr.x, tr.y + 4.0)], Stroke::new(1.0_f32, hud_cyan_bright));

        let bl = Pos2::new(center.x - sx, center.y + sy);
        painter.line_segment([bl, Pos2::new(bl.x + blen, bl.y)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([bl, Pos2::new(bl.x, bl.y - blen)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([Pos2::new(bl.x - 4.0, bl.y), Pos2::new(bl.x + 4.0, bl.y)], Stroke::new(1.0_f32, hud_cyan_bright));
        painter.line_segment([Pos2::new(bl.x, bl.y - 4.0), Pos2::new(bl.x, bl.y + 4.0)], Stroke::new(1.0_f32, hud_cyan_bright));

        let br = Pos2::new(center.x + sx, center.y + sy);
        painter.line_segment([br, Pos2::new(br.x - blen, br.y)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([br, Pos2::new(br.x, br.y - blen)], Stroke::new(1.0_f32, hud_cyan));
        painter.line_segment([Pos2::new(br.x - 4.0, br.y), Pos2::new(br.x + 4.0, br.y)], Stroke::new(1.0_f32, hud_cyan_bright));
        painter.line_segment([Pos2::new(br.x, br.y - 4.0), Pos2::new(br.x, br.y + 4.0)], Stroke::new(1.0_f32, hud_cyan_bright));

        painter.circle_stroke(center, 168.0_f32, Stroke::new(0.8_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 22)));

        let scale_lx = center.x - 172.0_f32;
        let scale_rx = center.x + 172.0_f32;
        for i in -4..=4 {
            let ny = center.y + (i as f32) * 16.0_f32;
            let nw = if i == 0 { 10.0_f32 } else if i % 2 == 0 { 6.0_f32 } else { 3.5_f32 };
            painter.line_segment([Pos2::new(scale_lx, ny), Pos2::new(scale_lx + nw, ny)], Stroke::new(0.9_f32, hud_cyan));
            painter.line_segment([Pos2::new(scale_rx - nw, ny), Pos2::new(scale_rx, ny)], Stroke::new(0.9_f32, hud_cyan));
        }

        let font_nano = FontId::monospace(8.5);
        painter.text(Pos2::new(center.x - 198.0, center.y - 88.0), Align2::LEFT_CENTER, "33.6000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x - 222.0, center.y - 8.0), Align2::LEFT_CENTER, "360000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x - 204.0, center.y + 68.0), Align2::LEFT_CENTER, "0.025", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x - 198.0, center.y + 82.0), Align2::LEFT_CENTER, "-491500", font_nano.clone(), text_dim);

        let box_l = Rect::from_min_size(Pos2::new(center.x - 224.0, center.y + 20.0), Vec2::new(26.0, 18.0));
        painter.rect_stroke(box_l, CornerRadius::ZERO, Stroke::new(0.8_f32, hud_cyan), StrokeKind::Inside);
        for b in 0..3 {
            let bar_y = box_l.min.y + 3.0 + (b as f32) * 4.5;
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(box_l.min.x + 3.0, bar_y), Vec2::new(14.0 - (b as f32) * 3.0, 2.5)),
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(0, 229, 168, 140),
            );
        }

        painter.text(Pos2::new(center.x + 168.0, center.y - 96.0), Align2::LEFT_CENTER, "3.60000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 178.0, center.y - 84.0), Align2::LEFT_CENTER, "70000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 188.0, center.y - 72.0), Align2::LEFT_CENTER, "15000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 218.0, center.y - 24.0), Align2::LEFT_CENTER, "+30000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 226.0, center.y + 18.0), Align2::LEFT_CENTER, "52000", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 178.0, center.y + 76.0), Align2::LEFT_CENTER, "0.020", font_nano.clone(), text_dim);
        painter.text(Pos2::new(center.x + 178.0, center.y + 88.0), Align2::LEFT_CENTER, "0.300", font_nano, text_dim);

        for a_deg in (15..=45).step_by(5) {
            let rad = (a_deg as f32).to_radians();
            let p_arc = Pos2::new(center.x + 154.0 * rad.cos(), center.y + 154.0 * rad.sin());
            painter.circle_filled(p_arc, 1.2_f32, Color32::from_rgba_unmultiplied(0, 229, 168, 110));
        }
    }
}
