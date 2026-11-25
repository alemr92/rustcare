use super::*;

impl ToolboxView {
    pub(super) fn render_duplicates(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Ruta a escanear:");
            ui.text_edit_singleline(&mut self.duplicate_path);
            if neon_button(ui, "Buscar Duplicados", Some(160.0)).clicked() {
                self.duplicate_result = Some(DuplicateFinder::scan_directory(&self.duplicate_path));
            }
        });
        ui.add_space(10.0);

        if let Some(res) = &self.duplicate_result {
            ui.label(
                RichText::new(format!(
                    "Archivos duplicados encontrados: {} | Espacio desperdiciado: {}",
                    res.total_duplicate_files,
                    CleanerEngine::format_bytes(res.total_wasted_bytes)
                ))
                .strong()
                .color(RustCareTheme::WARNING_AMBER),
            );
            ui.add_space(8.0);

            ScrollArea::vertical().show(ui, |ui| {
                for group in &res.groups {
                    egui::Frame::NONE
                        .fill(RustCareTheme::BG_SURFACE)
                        .corner_radius(8)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new(format!("Tamaño: {} | Hash: {}", CleanerEngine::format_bytes(group.file_size), &group.blake3_hash[..12])).strong().color(RustCareTheme::ACCENT_CYAN));
                            for file in &group.files {
                                ui.label(format!("- {}", file.display()));
                            }
                        });
                    ui.add_space(6.0);
                }
            });
        }
    }

    /// Color característico según el tipo de archivo (Estilo WizTree / WinDirStat)
    fn get_file_color(ext: &str) -> Color32 {
        match ext {
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "m4v" => Color32::from_rgb(168, 85, 247), // Púrpura (Videos)
            "zip" | "rar" | "7z" | "tar" | "gz" | "iso" | "vhd" | "img" => Color32::from_rgb(239, 68, 68), // Rojo (Comprimidos/ISOs)
            "exe" | "msi" | "dll" | "sys" => Color32::from_rgb(34, 197, 94), // Verde (Ejecutables/Sistema)
            "pdf" | "docx" | "xlsx" | "pptx" | "txt" | "db" | "sqlite" => Color32::from_rgb(59, 130, 246), // Azul (Documentos/DB)
            "png" | "jpg" | "jpeg" | "webp" | "gif" | "psd" => Color32::from_rgb(245, 158, 11), // Ámbar (Imágenes)
            "rs" | "cpp" | "c" | "js" | "ts" | "py" | "json" | "toml" => Color32::from_rgb(6, 182, 212), // Cian (Código)
            _ => Color32::from_rgb(100, 116, 139), // Gris pizarra (Otros)
        }
    }

    /// Renderiza recursivamente el Treemap proporcional de bloques estilo WizTree
    pub(super) fn render_treemap_recursive(
        painter: &egui::Painter,
        rect: Rect,
        items: &[TreemapItem],
        total_size: u64,
        depth: usize,
        hover_pos: Option<Pos2>,
        hovered_item: &mut Option<TreemapItem>,
    ) {
        if items.is_empty() || total_size == 0 || rect.width() < 2.0 || rect.height() < 2.0 {
            return;
        }

        if items.len() == 1 {
            let item = &items[0];
            let color = Self::get_file_color(&item.extension);
            let is_hovered = hover_pos.map(|p| rect.contains(p)).unwrap_or(false);

            if is_hovered {
                *hovered_item = Some(item.clone());
            }

            let stroke = if is_hovered {
                Stroke::new(2.0_f32, Color32::WHITE)
            } else {
                Stroke::new(1.0_f32, Color32::from_black_alpha(150))
            };

            painter.rect_filled(rect, CornerRadius::same(2), color);
            painter.rect_stroke(rect, CornerRadius::same(2), stroke, StrokeKind::Inside);

            if rect.width() > 65.0 && rect.height() > 22.0 {
                let name_preview = if item.name.len() > 16 {
                    format!("{}...", &item.name[..13])
                } else {
                    item.name.clone()
                };
                painter.text(
                    rect.min + Vec2::new(4.0, 3.0),
                    Align2::LEFT_TOP,
                    name_preview,
                    FontId::proportional(11.0),
                    Color32::WHITE,
                );
            }
            return;
        }

        // Bipartición proporcional
        let half = items.len() / 2;
        let left_items = &items[..half];
        let right_items = &items[half..];

        let left_size: u64 = left_items.iter().map(|i| i.size_bytes).sum();
        let right_size: u64 = right_items.iter().map(|i| i.size_bytes).sum();
        let total = left_size + right_size;

        if total == 0 {
            return;
        }

        let ratio = left_size as f32 / total as f32;

        let (left_rect, right_rect) = if depth % 2 == 0 {
            // División vertical
            let split_x = rect.min.x + rect.width() * ratio;
            (
                Rect::from_min_max(rect.min, Pos2::new(split_x, rect.max.y)),
                Rect::from_min_max(Pos2::new(split_x, rect.min.y), rect.max),
            )
        } else {
            // División horizontal
            let split_y = rect.min.y + rect.height() * ratio;
            (
                Rect::from_min_max(rect.min, Pos2::new(rect.max.x, split_y)),
                Rect::from_min_max(Pos2::new(rect.min.x, split_y), rect.max),
            )
        };

        Self::render_treemap_recursive(painter, left_rect, left_items, left_size, depth + 1, hover_pos, hovered_item);
        Self::render_treemap_recursive(painter, right_rect, right_items, right_size, depth + 1, hover_pos, hovered_item);
    }

    pub(super) fn render_disk_analyzer(&mut self, ui: &mut Ui) {
        ui.label(
            RichText::new("Analizador de Espacio en Disco WizTree (Treemap a 60 FPS)")
                .size(18.0)
                .strong()
                .color(RustCareTheme::ACCENT_CYAN),
        );
        ui.label(
            RichText::new("Selecciona cualquier unidad de disco detectada o especifica una carpeta para visualizar qué archivos ocupan más espacio.")
                .color(RustCareTheme::TEXT_MUTED),
        );
        ui.add_space(10.0);

        // Selector visual de unidades detectadas (C:, D:, E:, etc.)
        ui.horizontal(|ui| {
            ui.label(RichText::new("UNIDADES DETECTADAS EN TU EQUIPO:").small().strong().color(RustCareTheme::TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ghost_button(ui, "Actualizar Unidades", Some(150.0)).clicked() {
                    self.detected_drives = DiskAnalyzer::list_drives();
                }
            });
        });
        ui.add_space(6.0);

        if self.detected_drives.is_empty() {
            self.detected_drives = DiskAnalyzer::list_drives();
        }

        ui.horizontal_wrapped(|ui| {
            for drive in &self.detected_drives {
                let is_selected = self.disk_path.to_uppercase().starts_with(&drive.letter.to_uppercase());
                let border_color = if is_selected {
                    RustCareTheme::ACCENT_CYAN
                } else {
                    RustCareTheme::BORDER_SUBTLE
                };

                let bg_color = if is_selected {
                    Color32::from_rgb(26, 36, 56)
                } else {
                    RustCareTheme::BG_SURFACE
                };

                egui::Frame::NONE
                    .fill(bg_color)
                    .corner_radius(CornerRadius::same(8))
                    .stroke(Stroke::new(if is_selected { 1.5_f32 } else { 1.0_f32 }, border_color))
                    .inner_margin(Margin::same(10))
                    .show(ui, |ui| {
                        ui.set_width(215.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let (ic_rect, _) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::hover());
                                crate::icons::draw_icon(ui.painter(), ic_rect, AppIcon::Pulse, RustCareTheme::ACCENT_CYAN, 1.5);
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(&drive.name).strong().color(Color32::WHITE));
                                    ui.label(RichText::new(format!("Formato {}", drive.fs_type)).small().color(RustCareTheme::TEXT_DIM));
                                });
                            });

                            if drive.total_bytes > 0 {
                                let used = drive.total_bytes.saturating_sub(drive.available_bytes);
                                let ratio = (used as f32 / drive.total_bytes as f32).clamp(0.0, 1.0);
                                ui.add_space(4.0);
                                let free_str = CleanerEngine::format_bytes(drive.available_bytes);
                                let total_str = CleanerEngine::format_bytes(drive.total_bytes);
                                ui.label(RichText::new(format!("{} libres / {}", free_str, total_str)).small().color(RustCareTheme::TEXT_MUTED));

                                let bar_color = if ratio > 0.90 {
                                    RustCareTheme::ACCENT_CORAL
                                } else if ratio > 0.80 {
                                    RustCareTheme::ACCENT_AMBER
                                } else {
                                    RustCareTheme::ACCENT_TEAL
                                };
                                crate::widgets::styled_progress_bar(
                                    ui,
                                    ratio,
                                    bar_color,
                                    Some(&format!("{:.0}% ocupado", ratio * 100.0)),
                                );
                            }

                            ui.add_space(6.0);
                            let btn_label = if is_selected && self.disk_result.is_some() { "Reanalizar Unidad" } else { "Analizar Unidad" };
                            if neon_button(ui, btn_label, Some(195.0)).clicked() {
                                self.disk_path = drive.letter.clone();
                                self.disk_result = Some(DiskAnalyzer::analyze_directory(&self.disk_path));
                            }
                        });
                    });
            }
        });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Ruta actual o carpeta personalizada:").strong().color(Color32::WHITE));
            ui.text_edit_singleline(&mut self.disk_path);
            if neon_button(ui, "Analizar Ruta", Some(130.0)).clicked() {
                self.disk_result = Some(DiskAnalyzer::analyze_directory(&self.disk_path));
            }
        });
        ui.add_space(10.0);

        if let Some(res) = &self.disk_result {
            ui.label(
                RichText::new(format!(
                    "Tamaño analizado en '{}': {}",
                    res.root_path.display(),
                    CleanerEngine::format_bytes(res.total_size_bytes)
                ))
                .size(15.0)
                .strong()
                .color(RustCareTheme::ACCENT_CYAN),
            );
            ui.add_space(6.0);

            // DIAGRAMA TREEMAP VISUAL ESTILO WIZTREE
            ui.label(RichText::new("Mapa Visual de Distribución de Disco (WizTree Treemap):").strong());
            ui.add_space(4.0);

            let canvas_width = ui.available_width().max(400.0);
            let canvas_height = 320.0;
            let (response, painter) = ui.allocate_painter(Vec2::new(canvas_width, canvas_height), Sense::hover());
            let bounding_rect = response.rect;

            let mut hovered_item: Option<TreemapItem> = None;
            let treemap_total: u64 = res.treemap_items.iter().map(|i| i.size_bytes).sum();

            if !res.treemap_items.is_empty() && treemap_total > 0 {
                Self::render_treemap_recursive(
                    &painter,
                    bounding_rect,
                    &res.treemap_items,
                    treemap_total,
                    0,
                    response.hover_pos(),
                    &mut hovered_item,
                );
            } else {
                painter.rect_filled(bounding_rect, CornerRadius::same(6), RustCareTheme::BG_SURFACE);
                painter.text(
                    bounding_rect.center(),
                    Align2::CENTER_CENTER,
                    "No hay archivos grandes suficientes para diagramar el treemap",
                    FontId::proportional(14.0),
                    RustCareTheme::TEXT_MUTED,
                );
            }

            // Tooltip flotante interactivo al pasar el cursor sobre un bloque
            if let Some(item) = hovered_item {
                response.show_tooltip_text(format!(
                    "{}\nTamaño: {}\nRuta: {}",
                    item.name,
                    CleanerEngine::format_bytes(item.size_bytes),
                    item.path.display()
                ));
            }

            // Leyenda de colores estilo WizTree
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("[Vídeos]").color(Color32::from_rgb(168, 85, 247)).small());
                ui.label(RichText::new("[Comprimidos/ISO]").color(Color32::from_rgb(239, 68, 68)).small());
                ui.label(RichText::new("[Ejecutables/DLL]").color(Color32::from_rgb(34, 197, 94)).small());
                ui.label(RichText::new("[Documentos/DB]").color(Color32::from_rgb(59, 130, 246)).small());
                ui.label(RichText::new("[Imágenes]").color(Color32::from_rgb(245, 158, 11)).small());
                ui.label(RichText::new("[Código Dev]").color(Color32::from_rgb(6, 182, 212)).small());
                ui.label(RichText::new("[Otros]").color(Color32::from_rgb(100, 116, 139)).small());
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);

            // Tabla jerárquica con porcentajes
            ui.label(RichText::new("Explorador de Carpetas Más Pesadas:").strong());
            ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(8)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        for item in &res.items {
                            let ratio = if res.total_size_bytes > 0 {
                                item.size_bytes as f32 / res.total_size_bytes as f32
                            } else {
                                0.0
                            };

                            ui.horizontal(|ui| {
                                let icon = if item.is_directory { "[Dir]" } else { "[Doc]" };
                                ui.label(format!("{} {}", icon, item.name));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(CleanerEngine::format_bytes(item.size_bytes)).strong());
                                });
                            });
                            crate::widgets::styled_progress_bar(
                                ui,
                                ratio,
                                RustCareTheme::ACCENT_CYAN,
                                Some(&format!("{:.1}%", ratio * 100.0)),
                            );
                            ui.separator();
                        }
                    });
            });
        }
    }

    pub(super) fn render_large_files(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Ruta:");
            ui.text_edit_singleline(&mut self.large_files_path);
            ui.label("Tamaño Mínimo (MB):");
            ui.add(egui::DragValue::new(&mut self.large_files_min_mb).speed(10).range(10..=10000));
            if neon_button(ui, "Escanear Archivos Gigantes", Some(200.0)).clicked() {
                self.large_files = LargeFilesFinder::scan_directory(&self.large_files_path, self.large_files_min_mb * 1024 * 1024);
            }
        });
        ui.add_space(8.0);

        if !self.large_files.is_empty() {
            ui.label(RichText::new(format!("Se encontraron {} archivos superiores a {} MB:", self.large_files.len(), self.large_files_min_mb)).strong().color(RustCareTheme::WARNING_AMBER));
            ui.add_space(6.0);

            ScrollArea::vertical().show(ui, |ui| {
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(8)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        for item in &self.large_files.clone() {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(&item.name).strong());
                                    ui.label(RichText::new(format!("Categoría: {} | Ruta: {}", item.category.display_name(), item.path.display())).small().color(RustCareTheme::TEXT_MUTED));
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(CleanerEngine::format_bytes(item.size_bytes)).strong().color(RustCareTheme::ACCENT_CYAN));
                                });
                            });
                            ui.separator();
                        }
                    });
            });
        }
    }

    pub(super) fn render_empty_folders(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Ruta a explorar:");
            ui.text_edit_singleline(&mut self.empty_folders_path);
            if neon_button(ui, "Buscar Carpetas Vacías", Some(180.0)).clicked() {
                self.empty_folders = EmptyFoldersFinder::scan_directory(&self.empty_folders_path);
            }
            if !self.empty_folders.is_empty() {
                if danger_button(ui, "Eliminar Todas", Some(130.0)).clicked() {
                    for f in &self.empty_folders {
                        let _ = EmptyFoldersFinder::delete_folder(f);
                    }
                    self.empty_folders.clear();
                }
            }
        });
        ui.add_space(8.0);

        if !self.empty_folders.is_empty() {
            ui.label(RichText::new(format!("Se encontraron {} carpetas vacías:", self.empty_folders.len())).strong().color(RustCareTheme::WARNING_AMBER));
            ScrollArea::vertical().show(ui, |ui| {
                egui::Frame::NONE
                    .fill(RustCareTheme::BG_SURFACE)
                    .corner_radius(8)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        for item in &self.empty_folders {
                            ui.label(format!("- {}", item.path.display()));
                        }
                    });
            });
        }
    }

    pub(super) fn render_shortcuts(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if neon_button(ui, "Escanear Accesos Directos Rotos", Some(240.0)).clicked() {
                self.broken_shortcuts = ShortcutCleaner::scan_broken_shortcuts();
                self.shortcuts_scanned = true;
            }
            if !self.broken_shortcuts.is_empty() {
                if danger_button(ui, "Eliminar Todos los Rotos", Some(180.0)).clicked() {
                    for item in &self.broken_shortcuts {
                        let _ = ShortcutCleaner::remove_broken_shortcut(item);
                    }
                    self.broken_shortcuts.clear();
                }
            }
        });
        ui.add_space(8.0);

        if self.shortcuts_scanned {
            if self.broken_shortcuts.is_empty() {
                ui.label(RichText::new("No se detectaron accesos directos rotos en el Escritorio o Menú Inicio.").color(RustCareTheme::SUCCESS_GREEN));
            } else {
                ui.label(RichText::new(format!("Se encontraron {} accesos directos que apuntan a archivos inexistentes:", self.broken_shortcuts.len())).strong().color(RustCareTheme::WARNING_AMBER));
                ui.add_space(6.0);

                ScrollArea::vertical().show(ui, |ui| {
                    egui::Frame::NONE
                        .fill(RustCareTheme::BG_SURFACE)
                        .corner_radius(8)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            for item in &self.broken_shortcuts {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(&item.file_name).strong());
                                        ui.label(RichText::new(format!("Destino no encontrado: {}", item.target_path)).small().color(RustCareTheme::ACCENT_CORAL));
                                    });
                                });
                                ui.separator();
                            }
                        });
                });
            }
        }
    }
}

