use eframe::egui;
use crate::app::PortfolioState;

pub fn mostrar_modal_comparacion_compiladores(ctx: &egui::Context, state: &mut PortfolioState) {
    if !state.show_rustc_compilador_modal {
        return;
    }

    let mut open = true;
    egui::Window::new("Pipeline de Compilación de rustc")
        .open(&mut open)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(true)
        .default_size([980.0, 680.0])
        .collapsible(false)
        .show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.add_space(4.0);

                // Diagrama SVG con texto nativo estándar compatible con egui
                let img = egui::Image::from_bytes(
                    "bytes://pipe.svg",
                    include_bytes!("../../../diagramas/pipe.svg"),
                )
                .fit_to_original_size(1.0);

                ui.add(img);
                ui.add_space(4.0);
            });
        });

    if !open {
        state.show_rustc_compilador_modal = false;
    }
}
