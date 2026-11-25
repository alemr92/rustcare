//! Punto de Entrada de RustCare GUI
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
pub mod cmdk;
pub mod icons;
pub mod theme;
pub mod toast;
mod views;
pub mod widgets;

use app::RustCareApp;
use eframe::egui;
use theme::RustCareTheme;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let force_live = args.iter().any(|a| a == "--live");
    let force_sandbox = args.iter().any(|a| a == "--sandbox");

    // En desarrollo (debug) o con flag --sandbox se usa Sandbox por defecto para seguridad total
    let is_sandbox = if force_live {
        false
    } else if force_sandbox {
        true
    } else {
        cfg!(debug_assertions)
    };

    let initial_section = if args.iter().any(|a| a == "--gallery") {
        app::NavSection::Gallery
    } else {
        args.iter().position(|a| a == "--view").and_then(|idx| args.get(idx + 1)).map(|s| match s.as_str() {
            "care" => app::NavSection::Care,
            "cleaner" => app::NavSection::Cleaner,
            "booster" => app::NavSection::Booster,
            "shield" => app::NavSection::Shield,
            "toolbox" => app::NavSection::Toolbox,
            "monitor" => app::NavSection::Monitor,
            "settings" => app::NavSection::Settings,
            "gallery" => app::NavSection::Gallery,
            _ => app::NavSection::Care,
        }).unwrap_or(app::NavSection::Care)
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1150.0, 720.0])
            .with_min_inner_size([950.0, 600.0])
            .with_decorations(false)
            .with_title("RustCare - Windows 10/11 Optimization Suite"),
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "RustCare",
        native_options,
        Box::new(move |cc| {
            RustCareTheme::apply(&cc.egui_ctx);
            let mut app = RustCareApp::new(cc, is_sandbox);
            app.current_section = initial_section;
            Ok(Box::new(app))
        }),
    )
}
