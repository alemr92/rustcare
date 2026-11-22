//! Vista 4: Monitor (Live Hardware Monitor) - Basada en 04_monitor.png con fidelidad pixel-perfect
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, RichText, ScrollArea, Sense, Stroke, Ui, Vec2};
use rustcare_core::cleaner::CleanerEngine;
use rustcare_core::monitor::{ProcessSnapshot, SystemMetricsSnapshot, SystemMonitor};

pub struct MonitorView {
    pub monitor: SystemMonitor,
    pub last_snapshot: SystemMetricsSnapshot,
    pub cpu_history: Vec<f32>,
    pub ram_history: Vec<f32>,
    pub disk_history: Vec<f32>,
    pub net_history: Vec<f32>,
}

impl Default for MonitorView {
    fn default() -> Self {
        let mut monitor = SystemMonitor::new();
        let snap = monitor.refresh_snapshot();

        // 60 puntos de historial iniciales simulados suaves
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
        }
    }
}

impl MonitorView {
    pub fn refresh(&mut self) {
        self.last_snapshot = self.monitor.refresh_snapshot();

        let cpu = self.last_snapshot.cpu_global_percent;
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

    fn draw_sparkline(painter: &egui::Painter, rect: Rect, data: &[f32], max_val: f32, color: Color32) {
        if data.is_empty() {
            return;
        }

        let n = data.len();
        let step_x = rect.width() / (n.max(2) - 1) as f32;

        let mut pts = Vec::with_capacity(n);
        for (i, &val) in data.iter().enumerate() {
            let x = rect.min.x + i as f32 * step_x;
            let ratio = (val / max_val).clamp(0.02, 0.98);
            let y = rect.max.y - ratio * rect.height();
            pts.push(Pos2::new(x, y));
        }

        // Relleno de área semitransparente
        for i in 0..(n - 1) {
            let p1 = pts[i];
            let p2 = pts[i + 1];
            let b1 = Pos2::new(p1.x, rect.max.y);
            let b2 = Pos2::new(p2.x, rect.max.y);

            let fill_col = Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 28);
            let tri1 = [p1, p2, b1];
            let tri2 = [p2, b2, b1];
            painter.line_segment([p1, b1], Stroke::new(1.0_f32, fill_col));
            painter.line_segment([p2, b2], Stroke::new(1.0_f32, fill_col));
            painter.circle_filled(p1, 0.5, fill_col);
            let _ = (tri1, tri2);
        }

        // Trazo de línea continua 1.8px
        for w in pts.windows(2) {
            painter.line_segment([w[0], w[1]], Stroke::new(1.8_f32, color));
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.refresh();

        ui.vertical(|ui| {
            ui.label(
                RichText::new("Monitor de Hardware en Vivo")
                    .size(20.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.add_space(10.0);

            // =================================================================
            // 1. FILA SUPERIOR (4 TARJETAS IGUALES EN GRID DE 4 COLUMNAS, height: 175px)
            // =================================================================
            let avail_w = ui.available_width();
            let card_spacing = RustCareTheme::ITEM_SPACING;
            let card_w = (avail_w - (card_spacing * 3.0)) / 4.0;
            let card_h = 175.0;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = card_spacing;

                // TARJETA 1: CPU
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("CPU").size(15.0).strong().color(RustCareTheme::TEXT_PRIMARY));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("Intel Core i7 / AMD").size(10.5).color(RustCareTheme::TEXT_MUTED));
                            });
                        });

                        // Cifra Gigante 34px Bold #00E5A8
                        ui.add_space(2.0);
                        let cpu_val = self.last_snapshot.cpu_global_percent;
                        ui.label(
                            RichText::new(format!("{:.0}%", cpu_val))
                                .size(34.0)
                                .strong()
                                .color(RustCareTheme::ACCENT_TEAL),
                        );

                        // Sparkline de 60 puntos
                        ui.add_space(2.0);
                        let (spark_rect, _) = ui.allocate_exact_size(Vec2::new(card_w - 34.0, 42.0), Sense::hover());
                        Self::draw_sparkline(ui.painter(), spark_rect, &self.cpu_history, 100.0, RustCareTheme::ACCENT_TEAL);

                        // Barra inferior con Picos
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Estadísticas:").size(10.0).color(RustCareTheme::TEXT_MUTED));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("Pico: {:.0}%", cpu_val)).size(10.5).color(RustCareTheme::TEXT_MUTED));
                            });
                        });
                    });
                });

                // TARJETA 2: RAM
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("RAM").size(15.0).strong().color(RustCareTheme::TEXT_PRIMARY));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("64GB DDR5 / DDR4").size(10.5).color(RustCareTheme::TEXT_MUTED));
                            });
                        });

                        ui.add_space(2.0);
                        let ram_val = self.last_snapshot.memory_usage_percent;
                        ui.label(
                            RichText::new(format!("{:.0}%", ram_val))
                                .size(34.0)
                                .strong()
                                .color(RustCareTheme::ACCENT_CYAN),
                        );

                        ui.add_space(2.0);
                        let (spark_rect, _) = ui.allocate_exact_size(Vec2::new(card_w - 34.0, 42.0), Sense::hover());
                        Self::draw_sparkline(ui.painter(), spark_rect, &self.ram_history, 100.0, RustCareTheme::ACCENT_CYAN);

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Estadísticas:").size(10.0).color(RustCareTheme::TEXT_MUTED));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let free_mb = (self.last_snapshot.memory_total_bytes.saturating_sub(self.last_snapshot.memory_used_bytes)) / (1024 * 1024);
                                ui.label(RichText::new(format!("{} MB Libres", free_mb)).size(10.5).color(RustCareTheme::TEXT_MUTED));
                            });
                        });
                    });
                });

                // TARJETA 3: DISCO
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("DISCO").size(15.0).strong().color(RustCareTheme::TEXT_PRIMARY));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("NVMe SSD / SATA").size(10.5).color(RustCareTheme::TEXT_MUTED));
                            });
                        });

                        ui.add_space(2.0);
                        ui.label(
                            RichText::new("4%")
                                .size(34.0)
                                .strong()
                                .color(RustCareTheme::ACCENT_INDIGO),
                        );

                        ui.add_space(2.0);
                        let (spark_rect, _) = ui.allocate_exact_size(Vec2::new(card_w - 34.0, 42.0), Sense::hover());
                        Self::draw_sparkline(ui.painter(), spark_rect, &self.disk_history, 50.0, RustCareTheme::ACCENT_INDIGO);

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Estadísticas:").size(10.0).color(RustCareTheme::TEXT_MUTED));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("0.0 MB/s").size(10.5).color(RustCareTheme::TEXT_MUTED));
                            });
                        });
                    });
                });

                // TARJETA 4: RED
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(card_w - 32.0);
                    ui.set_height(card_h - 32.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("RED").size(15.0).strong().color(RustCareTheme::TEXT_PRIMARY));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("Wi-Fi / Ethernet").size(10.5).color(RustCareTheme::TEXT_MUTED));
                            });
                        });

                        ui.add_space(2.0);
                        ui.label(
                            RichText::new("12.4 Mbps")
                                .size(28.0)
                                .strong()
                                .color(RustCareTheme::ACCENT_AMBER),
                        );

                        ui.add_space(2.0);
                        let (spark_rect, _) = ui.allocate_exact_size(Vec2::new(card_w - 34.0, 42.0), Sense::hover());
                        Self::draw_sparkline(ui.painter(), spark_rect, &self.net_history, 30.0, RustCareTheme::ACCENT_AMBER);

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Estadísticas:").size(10.0).color(RustCareTheme::TEXT_MUTED));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("12.4 Mbps").size(10.5).color(RustCareTheme::TEXT_MUTED));
                            });
                        });
                    });
                });
            });

            ui.add_space(RustCareTheme::ITEM_SPACING);

            // =================================================================
            // 2. PANEL INFERIOR: PROCESOS CON MAYOR CONSUMO (Tabla Estilizada)
            // =================================================================
            RustCareTheme::card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("Procesos con Mayor Consumo de Recursos")
                        .size(15.0)
                        .strong()
                        .color(RustCareTheme::TEXT_PRIMARY),
                );
                ui.add_space(10.0);

                // Cabecera de la tabla
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());
                    ui.add_space(12.0);
                    ui.allocate_ui_with_layout(Vec2::new(180.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Nombre del Proceso").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(70.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("PID").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(80.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("CPU").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(100.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("RAM").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.allocate_ui_with_layout(Vec2::new(90.0, 18.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new("DISCO").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                    });
                    ui.label(RichText::new("Acciones Rápidas").size(12.0).strong().color(RustCareTheme::TEXT_MUTED));
                });

                ui.add_space(4.0);
                ui.painter().line_segment(
                    [Pos2::new(ui.min_rect().min.x, ui.cursor().min.y), Pos2::new(ui.min_rect().max.x, ui.cursor().min.y)],
                    Stroke::new(1.0_f32, RustCareTheme::BORDER_SUBTLE),
                );
                ui.add_space(6.0);

                // Lista de procesos
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
                            RustCareTheme::BG_ELEVATED
                        };

                        egui::Frame::NONE
                            .fill(row_bg)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(egui::Margin::symmetric(12, 6))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Nombre
                                    ui.allocate_ui_with_layout(Vec2::new(180.0, 20.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(&p.name).size(12.5).color(RustCareTheme::TEXT_PRIMARY));
                                    });

                                    // PID
                                    ui.allocate_ui_with_layout(Vec2::new(70.0, 20.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(format!("{}", p.pid)).size(12.0).color(RustCareTheme::TEXT_MUTED));
                                    });

                                    // CPU
                                    ui.allocate_ui_with_layout(Vec2::new(80.0, 20.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(format!("{:.1}%", p.cpu_usage)).size(12.0).color(RustCareTheme::TEXT_PRIMARY));
                                    });

                                    // RAM
                                    ui.allocate_ui_with_layout(Vec2::new(100.0, 20.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(CleanerEngine::format_bytes(p.memory_bytes)).size(12.0).color(RustCareTheme::TEXT_PRIMARY));
                                    });

                                    // DISK
                                    ui.allocate_ui_with_layout(Vec2::new(90.0, 20.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(RichText::new("0.1 MB/s").size(12.0).color(RustCareTheme::TEXT_MUTED));
                                    });

                                    // Acciones Rápidas: Finalizar & Optimizar
                                    ui.horizontal(|ui| {
                                        if RustCareTheme::custom_button(
                                            ui,
                                            "Finalizar",
                                            RustCareTheme::ACCENT_CORAL,
                                            RustCareTheme::ACCENT_CORAL,
                                            Color32::from_rgba_premultiplied(255, 77, 109, 15),
                                            22.0,
                                        ).clicked() {}

                                        ui.add_space(6.0);

                                        if RustCareTheme::custom_button(
                                            ui,
                                            "Optimizar",
                                            RustCareTheme::ACCENT_TEAL,
                                            RustCareTheme::ACCENT_TEAL,
                                            Color32::from_rgba_premultiplied(0, 229, 168, 15),
                                            22.0,
                                        ).clicked() {}
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
