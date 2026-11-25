//! Paleta de Comandos Global Ctrl+K (inspirada en dip/cmdk)
use crate::app::NavSection;
use crate::theme::RustCareTheme;
use crate::views::toolbox::ToolboxTab;
use eframe::egui::{
    self, Align2, Area, Color32, CornerRadius, FontId, Id, Key, Order, Pos2, RichText,
    ScrollArea, Sense, Stroke, StrokeKind, Vec2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmdkAction {
    Navigate(NavSection),
    OpenTool(ToolboxTab),
    CleanRam,
    ToggleSandbox,
}

#[derive(Debug, Clone)]
struct CmdkEntry {
    label: &'static str,
    category: &'static str,
    keywords: &'static str,
    action: CmdkAction,
}

pub struct CommandPalette {
    pub is_open: bool,
    pub query: String,
    pub selected_idx: usize,
    focus_requested: bool,
    entries: Vec<CmdkEntry>,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self {
            is_open: false,
            query: String::new(),
            selected_idx: 0,
            focus_requested: false,
            entries: vec![
                // Acciones Rápidas
                CmdkEntry { label: "Liberar Memoria RAM ahora", category: "Acción Rápida", keywords: "ram memoria limpiar booster", action: CmdkAction::CleanRam },
                CmdkEntry { label: "Alternar Modo Sandbox / Windows Real", category: "Acción Rápida", keywords: "sandbox real modo seguro", action: CmdkAction::ToggleSandbox },
                // Vistas Principales
                CmdkEntry { label: "Ir a Inicio (Análisis Inteligente)", category: "Navegación", keywords: "inicio care analizar salud", action: CmdkAction::Navigate(NavSection::Care) },
                CmdkEntry { label: "Ir a Limpieza & Privacidad", category: "Navegación", keywords: "limpieza basura navegador privacidad", action: CmdkAction::Navigate(NavSection::Cleaner) },
                CmdkEntry { label: "Ir a Seguridad (RustCare Shield)", category: "Navegación", keywords: "seguridad malware shield escudo", action: CmdkAction::Navigate(NavSection::Shield) },
                CmdkEntry { label: "Ir a Rendimiento (Turbo Boost)", category: "Navegación", keywords: "rendimiento turbo juego arranque", action: CmdkAction::Navigate(NavSection::Booster) },
                CmdkEntry { label: "Ir a Herramientas (Bento PRO)", category: "Navegación", keywords: "herramientas toolbox bento", action: CmdkAction::Navigate(NavSection::Toolbox) },
                CmdkEntry { label: "Ir a Monitor de Hardware en Vivo", category: "Navegación", keywords: "monitor cpu ram disco procesos", action: CmdkAction::Navigate(NavSection::Monitor) },
                CmdkEntry { label: "Ir a Ajustes y Preferencias", category: "Navegación", keywords: "ajustes configuracion preferencias", action: CmdkAction::Navigate(NavSection::Settings) },
                CmdkEntry { label: "Abrir Galería de Componentes (Storybook)", category: "Desarrollo UI", keywords: "galeria storybook componentes ui widgets", action: CmdkAction::Navigate(NavSection::Gallery) },
                // Herramientas Especializadas
                CmdkEntry { label: "Analizador de Disco WizTree (Treemap)", category: "Herramienta", keywords: "disco wiztree treemap espacio", action: CmdkAction::OpenTool(ToolboxTab::DiskAnalyzer) },
                CmdkEntry { label: "Desinstalador Profundo de Software", category: "Herramienta", keywords: "desinstalador apps programas residuos", action: CmdkAction::OpenTool(ToolboxTab::Uninstaller) },
                CmdkEntry { label: "Driver Booster PRO (Controladores)", category: "Herramienta", keywords: "drivers controladores hardware", action: CmdkAction::OpenTool(ToolboxTab::DriverBooster) },
                CmdkEntry { label: "Actualizador de Software (WinGet)", category: "Herramienta", keywords: "winget actualizar software", action: CmdkAction::OpenTool(ToolboxTab::Updater) },
                CmdkEntry { label: "Buscador de Archivos Duplicados", category: "Herramienta", keywords: "duplicados blake3 espacio", action: CmdkAction::OpenTool(ToolboxTab::Duplicates) },
                CmdkEntry { label: "Filtro de Archivos Gigantes", category: "Herramienta", keywords: "archivos grandes gigantes", action: CmdkAction::OpenTool(ToolboxTab::LargeFiles) },
                CmdkEntry { label: "Destructor Seguro de Archivos (DoD Wipe)", category: "Herramienta", keywords: "destructor shredder borrar seguro dod", action: CmdkAction::OpenTool(ToolboxTab::FileShredder) },
                CmdkEntry { label: "Acelerador de Red TCP/IP", category: "Herramienta", keywords: "red internet tcp dns ping", action: CmdkAction::OpenTool(ToolboxTab::InternetBooster) },
                CmdkEntry { label: "Centro de Rescate & Backups", category: "Herramienta", keywords: "rescate backup restauracion", action: CmdkAction::OpenTool(ToolboxTab::RescueCenter) },
                CmdkEntry { label: "Limpieza Profunda WinSxS (DISM)", category: "Herramienta", keywords: "dism winsxs windows", action: CmdkAction::OpenTool(ToolboxTab::DismWinSxs) },
                CmdkEntry { label: "Compactador SQLite de Navegadores", category: "Herramienta", keywords: "navegadores sqlite vacuum chrome edge", action: CmdkAction::OpenTool(ToolboxTab::BrowserVacuum) },
                CmdkEntry { label: "Desfragmentador & TRIM SSD", category: "Herramienta", keywords: "trim ssd disco optimizar", action: CmdkAction::OpenTool(ToolboxTab::DiskOptimizer) },
                CmdkEntry { label: "Limpiador de Carpetas Vacías", category: "Herramienta", keywords: "carpetas vacias", action: CmdkAction::OpenTool(ToolboxTab::EmptyFolders) },
                CmdkEntry { label: "Reparador de Accesos Directos Rotos", category: "Herramienta", keywords: "accesos directos lnk rotos", action: CmdkAction::OpenTool(ToolboxTab::Shortcuts) },
                CmdkEntry { label: "Gestor de Menú Contextual", category: "Herramienta", keywords: "menu contextual clic derecho", action: CmdkAction::OpenTool(ToolboxTab::ContextMenu) },
                CmdkEntry { label: "Auditor de Tareas Programadas", category: "Herramienta", keywords: "tareas programadas scheduler", action: CmdkAction::OpenTool(ToolboxTab::ScheduledTasks) },
                CmdkEntry { label: "Planes de Energía", category: "Herramienta", keywords: "energia bateria rendimiento", action: CmdkAction::OpenTool(ToolboxTab::PowerPlan) },
                CmdkEntry { label: "Gestor de Hibernación (hiberfil.sys)", category: "Herramienta", keywords: "hibernacion hiberfil", action: CmdkAction::OpenTool(ToolboxTab::Hibernation) },
                CmdkEntry { label: "Reparador de Iconos y Miniaturas", category: "Herramienta", keywords: "iconos miniaturas cache ui", action: CmdkAction::OpenTool(ToolboxTab::UiRepair) },
                CmdkEntry { label: "Diagnóstico de Conflictos de Hardware", category: "Herramienta", keywords: "hardware conflictos dispositivos", action: CmdkAction::OpenTool(ToolboxTab::HardwareTroubleshooter) },
                CmdkEntry { label: "Ficha Técnica de Hardware y Sistema", category: "Herramienta", keywords: "sistema info hardware ficha", action: CmdkAction::OpenTool(ToolboxTab::SystemInfo) },
            ],
        }
    }
}

impl CommandPalette {
    pub fn open(&mut self) {
        self.is_open = true;
        self.query.clear();
        self.selected_idx = 0;
        self.focus_requested = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.query.clear();
    }

    pub fn render(&mut self, ctx: &egui::Context) -> Option<CmdkAction> {
        // Atajo global Ctrl+K
        let ctrl_k = ctx.input(|i| (i.modifiers.ctrl || i.modifiers.command) && i.key_pressed(Key::K));
        if ctrl_k {
            if self.is_open {
                self.close();
            } else {
                self.open();
            }
        }

        if !self.is_open {
            return None;
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.close();
            return None;
        }

        let q = self.query.trim().to_lowercase();
        let filtered_indices: Vec<usize> = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                q.is_empty()
                    || e.label.to_lowercase().contains(&q)
                    || e.category.to_lowercase().contains(&q)
                    || e.keywords.to_lowercase().contains(&q)
            })
            .map(|(i, _)| i)
            .collect();

        if self.selected_idx >= filtered_indices.len() && !filtered_indices.is_empty() {
            self.selected_idx = filtered_indices.len() - 1;
        }

        if ctx.input(|i| i.key_pressed(Key::ArrowDown)) && !filtered_indices.is_empty() {
            self.selected_idx = (self.selected_idx + 1) % filtered_indices.len();
        }
        if ctx.input(|i| i.key_pressed(Key::ArrowUp)) && !filtered_indices.is_empty() {
            self.selected_idx = if self.selected_idx == 0 {
                filtered_indices.len() - 1
            } else {
                self.selected_idx - 1
            };
        }

        let mut triggered: Option<CmdkAction> = None;
        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            if let Some(&entry_idx) = filtered_indices.get(self.selected_idx) {
                triggered = Some(self.entries[entry_idx].action);
            }
        }

        let mut should_close = false;
        let screen = ctx.screen_rect();
        // Backdrop atenuado
        Area::new(Id::new("cmdk_backdrop"))
            .order(Order::Middle)
            .fixed_pos(screen.min)
            .show(ctx, |ui| {
                let (bg_rect, bg_resp) = ui.allocate_exact_size(screen.size(), Sense::click());
                ui.painter().rect_filled(
                    bg_rect,
                    CornerRadius::ZERO,
                    Color32::from_rgba_unmultiplied(6, 9, 15, 165),
                );
                if bg_resp.clicked() {
                    should_close = true;
                }
            });

        let modal_w = 540.0_f32.min(screen.width() - 40.0);
        let modal_h = 380.0_f32.min(screen.height() - 80.0);
        let modal_pos = Pos2::new(
            screen.center().x - modal_w * 0.5,
            screen.min.y + 88.0,
        );

        Area::new(Id::new("cmdk_modal"))
            .order(Order::Foreground)
            .fixed_pos(modal_pos)
            .show(ctx, |ui| {
                let frame = egui::Frame::NONE
                    .fill(Color32::from_rgba_unmultiplied(15, 22, 34, 245))
                    .corner_radius(CornerRadius::same(14))
                    .stroke(Stroke::new(1.2_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 120)))
                    .inner_margin(egui::Margin::same(14));

                frame.show(ui, |ui| {
                    ui.set_width(modal_w - 28.0);
                    ui.set_max_height(modal_h);

                    // Buscador superior
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(">").size(16.0).strong().color(RustCareTheme::ACCENT_CYAN));
                        let edit = ui.add(
                            egui::TextEdit::singleline(&mut self.query)
                                .hint_text("Buscar herramienta, vista o acción (ej. disco, ram, winget)...")
                                .desired_width(ui.available_width() - 55.0)
                                .frame(false),
                        );
                        if self.focus_requested {
                            edit.request_focus();
                            self.focus_requested = false;
                        }
                        if edit.changed() {
                            self.selected_idx = 0;
                        }
                        ui.label(RichText::new("ESC").size(10.0).color(RustCareTheme::TEXT_MUTED));
                    });

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);

                    // Lista de resultados
                    ScrollArea::vertical().max_height(modal_h - 90.0).show(ui, |ui| {
                        if filtered_indices.is_empty() {
                            ui.add_space(24.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new("No se encontraron comandos coincidentes")
                                        .color(RustCareTheme::TEXT_MUTED),
                                );
                            });
                            return;
                        }

                        for (idx, &entry_idx) in filtered_indices.iter().enumerate() {
                            let entry = &self.entries[entry_idx];
                            let is_sel = idx == self.selected_idx;
                            let row_w = ui.available_width();
                            let (row_rect, row_resp) =
                                ui.allocate_exact_size(Vec2::new(row_w, 34.0), Sense::click());
                            let p = ui.painter();

                            if is_sel || row_resp.hovered() {
                                p.rect_filled(
                                    row_rect,
                                    CornerRadius::same(8),
                                    Color32::from_rgba_unmultiplied(56, 189, 248, if is_sel { 32 } else { 18 }),
                                );
                                if is_sel {
                                    p.rect_stroke(
                                        row_rect,
                                        CornerRadius::same(8),
                                        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 95)),
                                        StrokeKind::Inside,
                                    );
                                }
                            }

                            p.text(
                                Pos2::new(row_rect.min.x + 12.0, row_rect.center().y),
                                Align2::LEFT_CENTER,
                                entry.label,
                                FontId::proportional(13.0),
                                if is_sel { Color32::WHITE } else { RustCareTheme::TEXT_PRIMARY },
                            );

                            p.text(
                                Pos2::new(row_rect.max.x - 12.0, row_rect.center().y),
                                Align2::RIGHT_CENTER,
                                entry.category,
                                FontId::proportional(10.5),
                                if is_sel { RustCareTheme::ACCENT_CYAN } else { RustCareTheme::TEXT_MUTED },
                            );

                            if row_resp.clicked() {
                                triggered = Some(entry.action);
                            }
                        }
                    });

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("↑↓ Navegar  ·  Enter Ejecutar  ·  Esc Cerrar")
                                .size(10.5)
                                .color(RustCareTheme::TEXT_MUTED),
                        );
                    });
                });
            });

        if triggered.is_some() || should_close {
            self.close();
        }
        triggered
    }
}
