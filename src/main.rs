mod app;
mod components;
mod execution;
mod routes;
mod views;

use app::PortfolioState;
use eframe::egui;

fn main() -> eframe::Result {
    // 1. Inicializar logger para mostrar información, advertencias y errores en la terminal
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("🦀 Iniciando FerrisKey Desktop...");

    // 2. Configurar captura de panics amigable en la terminal
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        eprintln!("\n========================================================");
        eprintln!("🚨 [ERROR CRÍTICO / PANIC EN FERRISKEY]");
        eprintln!("{panic_info}");
        eprintln!("Tip: Ejecuta con RUST_BACKTRACE=1 para ver la traza completa.");
        eprintln!("========================================================\n");
        default_hook(panic_info);
    }));

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
            log::info!("✅ Loaders de imágenes y SVGs cargados exitosamente.");
            Ok(Box::new(PortfolioState::default()))
        }),
    )
}
