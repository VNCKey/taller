pub mod entorno;
pub mod estructura_tiempos;
pub mod pipeline;

use crate::app::PortfolioState;
use eframe::egui;

pub fn mostrar_tutorial_cargo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Pilares de Rust")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });

    ui.add_space(15.0);

    // Barra de navegación de pestañas de Pilares
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        let tabs = [
            (0, "Entorno de Trabajo"),
            (1, "Bases"),
        ];
        for (indice, texto) in tabs {
            let es_activo = state.pilares_step == indice;
            let text_color = if es_activo { naranja } else { gris_tab };

            let btn_text = egui::RichText::new(texto).strong().color(text_color);
            if ui
                .add(egui::Button::new(btn_text).frame(es_activo))
                .clicked()
            {
                state.pilares_step = indice;
            }
            ui.add_space(6.0);
        }
    });
    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.pilares_step {
        0 => entorno::mostrar_pilares_entorno_trabajo(ui, state),
        _ => estructura_tiempos::mostrar_pilares_conceptos(ui, state),
    }
}
