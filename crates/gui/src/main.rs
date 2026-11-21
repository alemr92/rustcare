//! Punto de Entrada de RustCare GUI
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod theme;
mod views;

use app::RustCareApp;
use eframe::egui;
use theme::RustCareTheme;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1150.0, 720.0])
            .with_min_inner_size([950.0, 600.0])
            .with_title("RustCare - Windows 10/11 Optimization Suite"),
        ..Default::default()
    };

    eframe::run_native(
        "RustCare",
        native_options,
        Box::new(|cc| {
            RustCareTheme::apply(&cc.egui_ctx);
            Ok(Box::new(RustCareApp::new(cc)))
        }),
    )
}
