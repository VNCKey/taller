mod app;
mod components;
mod execution;
mod routes;
mod views;

use app::PortfolioState;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_title("FerrisKey 🦀🔑 - El Ecosistema Interactivo de Rust"),
        ..Default::default()
    };

    eframe::run_native(
        "FerrisKey 🦀🔑 - El Ecosistema Interactivo de Rust (Luis Alexander / Alekay)",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(PortfolioState::default()))
        }),
    )
}
