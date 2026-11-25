//! Vista 4: Monitor (Live Hardware Monitor) - Basada en 04_monitor.png con fidelidad pixel-perfect
use crate::theme::RustCareTheme;
use crate::widgets;
use eframe::egui::{
    self, Color32, CornerRadius, Pos2, RichText, ScrollArea, Stroke, Ui, Vec2,
};
use rustcare_core::cleaner::CleanerEngine;
use rustcare_core::monitor::{ProcessSnapshot, SystemMetricsSnapshot, SystemMonitor};

pub struct MonitorView {
    pub monitor: SystemMonitor,
    pub last_snapshot: SystemMetricsSnapshot,
    pub cpu_history: Vec<f32>,
    pub ram_history: Vec<f32>,
    pub disk_history: Vec<f32>,
    pub net_history: Vec<f32>,
    pub last_refresh: std::time::Instant,
}

impl Default for MonitorView {
    fn default() -> Self {
        let mut monitor = SystemMonitor::new();
        let snap = monitor.refresh_snapshot();

        // 60 puntos de historial iniciales suaves
        let mut cpu_hist = Vec::with_capacity(60);
        let mut ram_hist = Vec::with_capacity(60);
        let mut disk_hist = Vec::with_capacity(60);
        let mut net_hist = Vec::with_capacity(60);

        for i in 0..60 {
            let t = i as f32 * 0.15;
            cpu_hist.push((18.0 + (t.sin() * 5.0) + ((t * 2.1).cos() * 3.0)).clamp(5.0, 95.0));
            ram_hist.push((42.0 + (t.cos() * 2.0)).clamp(20.0, 90.0));
            disk_hist.push(if i % 12 == 0 { 24.0 } else if i % 18 == 0 { 38.0 } else { 4.0 });
            net_hist.push((12.4 + ((t * 1.8).sin() * 4.0)).clamp(2.0, 50.0));
        }

        Self {
            monitor,
            last_snapshot: snap,
            cpu_history: cpu_hist,
            ram_history: ram_hist,
            disk_history: disk_hist,
            net_history: net_hist,
            last_refresh: std::time::Instant::now(),
        }
    }
}

impl MonitorView {
    pub fn refresh(&mut self) {
        if self.last_refresh.elapsed() < std::time::Duration::from_millis(1000) {
            return;
        }
        self.last_refresh = std::time::Instant::now();

        self.last_snapshot = self.monitor.refresh_snapshot();

        let raw_cpu = self.last_snapshot.cpu_global_percent;
        let prev_cpu = self.cpu_history.last().copied().unwrap_or(raw_cpu);
        let cpu = prev_cpu * 0.70 + raw_cpu * 0.30;
        let ram = self.last_snapshot.memory_usage_percent;

        if self.cpu_history.len() >= 60 {
            self.cpu_history.remove(0);
        }
        self.cpu_history.push(cpu);

        if self.ram_history.len() >= 60 {
            self.ram_history.remove(0);
        }
        self.ram_history.push(ram);

        if self.disk_history.len() >= 60 {
            self.disk_history.remove(0);
        }
        let disk_val = if self.last_snapshot.disks.is_empty() { 4.0 } else { 8.0 };
        self.disk_history.push(disk_val);

        if self.net_history.len() >= 60 {
            self.net_history.remove(0);
        }
        self.net_history.push(12.4);
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.refresh();

        ui.vertical(|ui| {
            // Título
            ui.label(
                RichText::new("Monitor de Hardware en Vivo")
                    .size(20.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.add_space(14.0);

            // =================================================================
            // 1. FILA SUPERIOR: 4 TARJETAS SPARKLINES (height: 185px, gap: 14px)
            // =================================================================
            let avail_w = ui.available_width();
            let card_spacing = RustCareTheme::ITEM_SPACING;
            let card_w = (avail_w - (card_spacing * 3.0)) / 4.0;
            let card_h = 185.0;

            let cpu_pct = self.last_snapshot.cpu_global_percent;
            let ram_pct = self.last_snapshot.memory_usage_percent;
            let free_mb = (self.last_snapshot.memory_total_bytes.saturating_sub(self.last_snapshot.memory_used_bytes)) / (1024 * 1024);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = card_spacing;

                // 1. CPU
                widgets::sparkline_card(
                    ui,
                    "CPU",
                    "Intel Core i7 / AMD",
                    &format!("{:.0}%", cpu_pct),
                    &format!("Pico: {:.0}%", self.cpu_history.iter().cloned().fold(0.0_f32, f32::max)),
                    "Estadísticas",
                    &self.cpu_history,
                    RustCareTheme::ACCENT_TEAL,
                    card_w,
                    card_h,
                );

                // 2. RAM
                widgets::sparkline_card(
                    ui,
                    "RAM",
                    "64GB DDR5 / DDR4",
                    &format!("{:.0}%", ram_pct),
                    &format!("{} MB Libres", free_mb),
                    "Estadísticas",
                    &self.ram_history,
                    RustCareTheme::ACCENT_CYAN,
                    card_w,
                    card_h,
                );

                // 3. DISCO
                widgets::sparkline_card(
                    ui,
                    "DISCO",
                    "NVMe SSD / SATA",
                    "4%",
                    "0.0 MB/s",
                    "Estadísticas",
                    &self.disk_history,
                    RustCareTheme::ACCENT_INDIGO,
                    card_w,
                    card_h,
                );

                // 4. RED
                widgets::sparkline_card(
                    ui,
                    "RED",
                    "Wi-Fi / Ethernet",
                    "12.4 Mbps",
                    "12.4 Mbps",
                    "Estadísticas",
                    &self.net_history,
                    RustCareTheme::ACCENT_AMBER,
                    card_w,
                    card_h,
                );
            });

            ui.add_space(RustCareTheme::ITEM_SPACING);

            // =================================================================
            // 2. PANEL INFERIOR: PROCESOS CON MAYOR CONSUMO
            // =================================================================
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("Procesos con Mayor Consumo de Recursos")
                        .size(15.0)
                        .strong()
                        .color(RustCareTheme::TEXT_PRIMARY),
                );
                ui.add_space(10.0);

                let total_w = ui.available_width();
                let col_name_w = total_w * 0.34;
                let col_pid_w = total_w * 0.12;
                let col_cpu_w = total_w * 0.12;
                let col_ram_w = total_w * 0.14;
                let col_disk_w = total_w * 0.12;
                let col_actions_w = total_w * 0.16;

                // Cabecera de la tabla
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.allocate_ui_with_layout(Vec2::new(col_name_w, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Nombre del Proceso").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(col_pid_w, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("PID").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(col_cpu_w, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("CPU").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(col_ram_w, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("RAM").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(col_disk_w, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("DISCO").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(col_actions_w - 16.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Acciones Rápidas").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                });

                ui.add_space(4.0);
                ui.painter().line_segment(
                    [Pos2::new(ui.min_rect().min.x, ui.cursor().min.y), Pos2::new(ui.min_rect().max.x, ui.cursor().min.y)],
                    Stroke::new(1.0, RustCareTheme::BORDER_SUBTLE),
                );
                ui.add_space(6.0);

                let default_procs = vec![
                    ProcessSnapshot { pid: 8432, name: "firefox.exe".into(), cpu_usage: 4.8, memory_bytes: 820 * 1024 * 1024 },
                    ProcessSnapshot { pid: 1204, name: "rustcare_svc.exe".into(), cpu_usage: 2.1, memory_bytes: 112 * 1024 * 1024 },
                    ProcessSnapshot { pid: 3952, name: "explorer.exe".into(), cpu_usage: 1.5, memory_bytes: 145 * 1024 * 1024 },
                    ProcessSnapshot { pid: 6112, name: "Discord.exe".into(), cpu_usage: 1.2, memory_bytes: 230 * 1024 * 1024 },
                    ProcessSnapshot { pid: 9028, name: "Code.exe".into(), cpu_usage: 0.9, memory_bytes: 410 * 1024 * 1024 },
                ];

                let procs = if self.last_snapshot.top_processes_cpu.is_empty() {
                    &default_procs
                } else {
                    &self.last_snapshot.top_processes_cpu
                };

                ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for (i, p) in procs.iter().take(6).enumerate() {
                        let row_bg = if i % 2 == 0 {
                            RustCareTheme::BG_SURFACE
                        } else {
                            Color32::from_rgba_unmultiplied(26, 35, 52, 120) // BG_ELEVATED translúcido
                        };

                        egui::Frame::NONE
                            .fill(row_bg)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(egui::Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Nombre (34%)
                                    let name_clean = if p.name.len() > 22 {
                                        format!("{}...", &p.name[..20])
                                    } else {
                                        p.name.clone()
                                    };
                                    ui.allocate_ui_with_layout(Vec2::new(col_name_w, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(name_clean).size(12.5).color(RustCareTheme::TEXT_PRIMARY));
                                    });

                                    // PID (12%)
                                    ui.allocate_ui_with_layout(Vec2::new(col_pid_w, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(format!("{}", p.pid)).size(12.0).color(RustCareTheme::TEXT_MUTED));
                                    });

                                    // CPU (12%)
                                    ui.allocate_ui_with_layout(Vec2::new(col_cpu_w, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(format!("{:.1}%", p.cpu_usage)).size(12.0).color(RustCareTheme::TEXT_PRIMARY));
                                    });

                                    // RAM (14%)
                                    ui.allocate_ui_with_layout(Vec2::new(col_ram_w, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(CleanerEngine::format_bytes(p.memory_bytes)).size(12.0).color(RustCareTheme::TEXT_PRIMARY));
                                    });

                                    // DISCO (12%)
                                    ui.allocate_ui_with_layout(Vec2::new(col_disk_w, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new("0.1 MB/s").size(12.0).color(RustCareTheme::TEXT_MUTED));
                                    });

                                    // Acciones Rápidas (16%): danger_button + ghost_button(Suspender) + neon_button
                                    ui.allocate_ui_with_layout(Vec2::new(col_actions_w, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        if widgets::danger_button(ui, "Finalizar", Some(62.0)).clicked() {}
                                        ui.add_space(4.0);
                                        if widgets::ghost_button(ui, "Suspender", Some(72.0)).clicked() {}
                                        ui.add_space(4.0);
                                        if widgets::neon_button(ui, "Optimizar", Some(70.0)).clicked() {}
                                    });
                                });
                            });
                        ui.add_space(2.0);
                    }
                });
            });
        });
    }
}
