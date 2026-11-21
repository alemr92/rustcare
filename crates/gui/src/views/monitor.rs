//! Vista de Monitor del Sistema en Tiempo Real - 2026 Edition
use crate::theme::RustCareTheme;
use eframe::egui::{self, Color32, CornerRadius, ProgressBar, RichText, ScrollArea, Ui};
use rustcare_core::cleaner::CleanerEngine;
use rustcare_core::monitor::{SystemMetricsSnapshot, SystemMonitor};

pub struct MonitorView {
    pub monitor: SystemMonitor,
    pub last_snapshot: SystemMetricsSnapshot,
}

impl Default for MonitorView {
    fn default() -> Self {
        let mut monitor = SystemMonitor::new();
        let snap = monitor.refresh_snapshot();
        Self {
            monitor,
            last_snapshot: snap,
        }
    }
}

impl MonitorView {
    pub fn refresh(&mut self) {
        self.last_snapshot = self.monitor.refresh_snapshot();
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new("📊 Monitor del Sistema en Tiempo Real")
                    .size(22.0)
                    .strong()
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new("🔄 Actualizar")
                            .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    self.refresh();
                }
            });
        });

        ui.label(
            RichText::new(format!(
                "SO: {} {} • Uptime: {} horas • Hilos: {}",
                self.last_snapshot.os_name,
                self.last_snapshot.os_version,
                self.last_snapshot.uptime_seconds / 3600,
                self.last_snapshot.cpu_count
            ))
            .color(RustCareTheme::TEXT_MUTED),
        );
        ui.separator();
        ui.add_space(10.0);

        ScrollArea::vertical().show(ui, |ui| {
            // Fila 1: CPU y RAM en tarjetas gemelas de cristal
            let half_width = (ui.available_width() - 16.0) / 2.0;

            ui.horizontal(|ui| {
                // Tarjeta CPU
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(half_width);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("⚡ Uso de Procesador (CPU)")
                                    .strong()
                                    .size(16.0)
                                    .color(RustCareTheme::ACCENT_CYAN),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                RustCareTheme::draw_badge(
                                    ui,
                                    &format!("{:.1}%", self.last_snapshot.cpu_global_percent),
                                    RustCareTheme::ACCENT_CYAN,
                                    Color32::from_rgba_premultiplied(0, 229, 255, 25),
                                );
                            });
                        });
                        ui.add_space(8.0);

                        let cpu_val = (self.last_snapshot.cpu_global_percent / 100.0).clamp(0.0, 1.0);
                        ui.add(
                            ProgressBar::new(cpu_val)
                                .text(format!("{:.1}% Global", self.last_snapshot.cpu_global_percent))
                                .corner_radius(CornerRadius::same(6)),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(format!("Núcleos lógicos detectados: {}", self.last_snapshot.cpu_count))
                                .small()
                                .color(RustCareTheme::TEXT_DIM),
                        );
                    });
                });

                // Tarjeta RAM
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(half_width);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("🧠 Memoria RAM")
                                    .strong()
                                    .size(16.0)
                                    .color(RustCareTheme::ACCENT_CYAN),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                RustCareTheme::draw_badge(
                                    ui,
                                    &format!("{:.1}%", self.last_snapshot.memory_usage_percent),
                                    RustCareTheme::SUCCESS_GREEN,
                                    Color32::from_rgba_premultiplied(16, 185, 129, 25),
                                );
                            });
                        });
                        ui.add_space(8.0);

                        let ram_ratio = (self.last_snapshot.memory_usage_percent / 100.0).clamp(0.0, 1.0);
                        ui.add(
                            ProgressBar::new(ram_ratio)
                                .text(format!(
                                    "{} / {}",
                                    CleanerEngine::format_bytes(self.last_snapshot.memory_used_bytes),
                                    CleanerEngine::format_bytes(self.last_snapshot.memory_total_bytes)
                                ))
                                .corner_radius(CornerRadius::same(6)),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(format!(
                                "Disponible: {}",
                                CleanerEngine::format_bytes(self.last_snapshot.memory_free_bytes)
                            ))
                            .small()
                            .color(RustCareTheme::TEXT_DIM),
                        );
                    });
                });
            });

            ui.add_space(16.0);

            // Almacenamiento / Unidades de Disco
            ui.label(
                RichText::new("💾 Almacenamiento y Unidades Físicas")
                    .strong()
                    .size(16.0)
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.add_space(6.0);

            RustCareTheme::card_frame().show(ui, |ui| {
                if self.last_snapshot.disks.is_empty() {
                    ui.label("No se detectaron unidades de disco accesibles.");
                } else {
                    for disk in &self.last_snapshot.disks {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("Unidad {} ({})", disk.name, disk.mount_point))
                                    .strong(),
                            );
                            let used = disk.total_space.saturating_sub(disk.available_space);
                            let ratio = if disk.total_space > 0 {
                                (used as f32 / disk.total_space as f32).clamp(0.0, 1.0)
                            } else {
                                0.0
                            };
                            let disk_color = if ratio > 0.9 {
                                RustCareTheme::DANGER_RED
                            } else if ratio > 0.75 {
                                RustCareTheme::WARNING_AMBER
                            } else {
                                RustCareTheme::ACCENT_BLUE
                            };
                            ui.add(
                                ProgressBar::new(ratio)
                                    .text(format!(
                                        "{:.1}% • {} / {}",
                                        ratio * 100.0,
                                        CleanerEngine::format_bytes(used),
                                        CleanerEngine::format_bytes(disk.total_space)
                                    ))
                                    .fill(disk_color)
                                    .corner_radius(CornerRadius::same(6)),
                            );
                        });
                        ui.separator();
                    }
                }
            });

            ui.add_space(16.0);

            // Top Procesos en Memoria y CPU
            ui.label(
                RichText::new("🔥 Procesos con Mayor Carga de Recursos")
                    .strong()
                    .size(16.0)
                    .color(RustCareTheme::TEXT_PRIMARY),
            );
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                // Top CPU
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(half_width);
                    ui.label(
                        RichText::new("⚡ Top Consumo de CPU")
                            .strong()
                            .color(RustCareTheme::WARNING_AMBER),
                    );
                    ui.separator();
                    for p in &self.last_snapshot.top_processes_cpu {
                        ui.horizontal(|ui| {
                            RustCareTheme::draw_badge(
                                ui,
                                &format!("{:.1}%", p.cpu_usage),
                                RustCareTheme::WARNING_AMBER,
                                Color32::from_rgba_premultiplied(245, 158, 11, 20),
                            );
                            ui.label(
                                RichText::new(&p.name)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("PID: {}", p.pid))
                                        .small()
                                        .color(RustCareTheme::TEXT_DIM),
                                );
                            });
                        });
                    }
                });

                // Top RAM
                RustCareTheme::card_frame().show(ui, |ui| {
                    ui.set_width(half_width);
                    ui.label(
                        RichText::new("🧠 Top Consumo de Memoria")
                            .strong()
                            .color(RustCareTheme::ACCENT_CYAN),
                    );
                    ui.separator();
                    for p in &self.last_snapshot.top_processes_memory {
                        ui.horizontal(|ui| {
                            RustCareTheme::draw_badge(
                                ui,
                                &CleanerEngine::format_bytes(p.memory_bytes),
                                RustCareTheme::ACCENT_CYAN,
                                Color32::from_rgba_premultiplied(0, 229, 255, 20),
                            );
                            ui.label(
                                RichText::new(&p.name)
                                    .strong()
                                    .color(RustCareTheme::TEXT_PRIMARY),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("PID: {}", p.pid))
                                        .small()
                                        .color(RustCareTheme::TEXT_DIM),
                                );
                            });
                        });
                    }
                });
            });
        });
    }
}
