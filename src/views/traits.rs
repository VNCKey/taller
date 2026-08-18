use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;

pub fn mostrar_tutorial_traits(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("🧬 Lección 11: Traits, Genéricos & Dispatch")
                .size(30.0)
                .strong()
                .color(egui::Color32::from_rgb(100, 220, 200)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("Polimorfismo en Rust: Static Monomorphization vs Dynamic Vtables")
                .size(15.0)
                .italics(),
        );
    });

    ui.add_space(25.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.traits_code,
        Arc::clone(&state.traits_output),
        "▶ Ejecutar Traits & Genéricos",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

