use eframe::egui;
use crate::app::PortfolioState;
use crate::views::conceptos::mostrar_componente_terminal_3_modos;

pub fn mostrar_modal_terminal(ctx: &egui::Context, state: &mut PortfolioState) {
    let mut abierto = state.show_terminal_modal;
    if !abierto {
        return;
    }

    egui::Window::new("Terminal Linux")
        .open(&mut abierto)
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -15.0))
        .default_size([760.0, 320.0])
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            mostrar_componente_terminal_3_modos(ui, "cargo run", state);
        });

    state.show_terminal_modal = abierto;
}
