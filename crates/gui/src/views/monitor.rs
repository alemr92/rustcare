//! Vista de Monitor del Sistema en Tiempo Real
use crate::theme::RustCareTheme;
use eframe::egui::{self, ProgressBar, RichText, ScrollArea, Ui};
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
            ui.heading(RichText::new("📊 Monitor del Sistema en Tiempo Real").size(22.0).color(RustCareTheme::TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄 Actualizar").clicked() {
                    self.refresh();
                }
            });
        });

        ui.label(RichText::new(format!("Sistema: {} {} | Tiempo de actividad: {} horas", self.last_snapshot.os_name, self.last_snapshot.os_version, self.last_snapshot.uptime_seconds / 3600)).color(RustCareTheme::TEXT_MUTED));
        ui.separator();
        ui.add_space(10.0);

        ScrollArea::vertical().show(ui, |ui| {
            // Fila 1: CPU y RAM en tarjetas gemelas
            ui.horizontal(|ui| {
                // Tarjeta CPU
                egui::Frame::NONE
                    .fill(RustCareTheme::CARD_BG)
                    .corner_radius(8)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width() / 2.0 - 10.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new("⚡ Uso de Procesador (CPU)").strong().size(16.0).color(RustCareTheme::ACCENT_CYAN));
                            ui.add_space(5.0);
                            let cpu_val = self.last_snapshot.cpu_global_percent / 100.0;
                            ui.add(ProgressBar::new(cpu_val).text(format!("{:.1}% Global", self.last_snapshot.cpu_global_percent)));
                            ui.add_space(5.0);
                            ui.label(RichText::new(format!("Núcleos lógicos: {}", self.last_snapshot.cpu_count)).small().color(RustCareTheme::TEXT_MUTED));
                        });
                    });

                // Tarjeta RAM
                egui::Frame::NONE
                    .fill(RustCareTheme::CARD_BG)
                    .corner_radius(8)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new("🧠 Memoria RAM").strong().size(16.0).color(RustCareTheme::ACCENT_CYAN));
                            ui.add_space(5.0);
                            let ram_ratio = self.last_snapshot.memory_usage_percent / 100.0;
                            ui.add(ProgressBar::new(ram_ratio).text(format!("{:.1}% ({})", self.last_snapshot.memory_usage_percent, CleanerEngine::format_bytes(self.last_snapshot.memory_used_bytes))));
                            ui.add_space(5.0);
                            ui.label(RichText::new(format!("Total: {} | Disponible: {}", CleanerEngine::format_bytes(self.last_snapshot.memory_total_bytes), CleanerEngine::format_bytes(self.last_snapshot.memory_free_bytes))).small().color(RustCareTheme::TEXT_MUTED));
                        });
                    });
            });

            ui.add_space(15.0);

            // Almacenamiento / Discos
            ui.label(RichText::new("💾 Almacenamiento y Unidades de Disco").strong().size(16.0).color(RustCareTheme::TEXT_PRIMARY));
            ui.add_space(6.0);

            egui::Frame::NONE
                .fill(RustCareTheme::CARD_BG)
                .corner_radius(8)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    for disk in &self.last_snapshot.disks {
                        ui.horizontal(|ui| {
                            ui.label(format!("Disco {} ({})", disk.name, disk.mount_point));
                            let used = disk.total_space.saturating_sub(disk.available_space);
                            let ratio = if disk.total_space > 0 { used as f32 / disk.total_space as f32 } else { 0.0 };
                            ui.add(ProgressBar::new(ratio).text(format!("{} / {}", CleanerEngine::format_bytes(used), CleanerEngine::format_bytes(disk.total_space))));
                        });
                        ui.separator();
                    }
                });

            ui.add_space(15.0);

            // Top Procesos
            ui.label(RichText::new("🔥 Procesos con Mayor Consumo").strong().size(16.0).color(RustCareTheme::TEXT_PRIMARY));
            ui.add_space(6.0);

            egui::Grid::new("top_processes_grid")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    egui::Frame::NONE
                        .fill(RustCareTheme::CARD_BG)
                        .corner_radius(8)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Top Consumo de CPU").strong().color(RustCareTheme::WARNING_AMBER));
                            ui.separator();
                            for p in &self.last_snapshot.top_processes_cpu {
                                ui.horizontal(|ui| {
                                    ui.label(&p.name);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(format!("{:.1}%", p.cpu_usage));
                                    });
                                });
                            }
                        });

                    egui::Frame::NONE
                        .fill(RustCareTheme::CARD_BG)
                        .corner_radius(8)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Top Consumo de Memoria").strong().color(RustCareTheme::ACCENT_CYAN));
                            ui.separator();
                            for p in &self.last_snapshot.top_processes_memory {
                                ui.horizontal(|ui| {
                                    ui.label(&p.name);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(CleanerEngine::format_bytes(p.memory_bytes));
                                    });
                                });
                            }
                        });
                });
        });
    }
}
