pub mod entorno;
pub mod anatomy;
pub mod cargo_workflow;
pub mod estructura_tiempos;
pub mod foundations;
pub mod pipeline;
pub mod project_anatomy;

use crate::app::PortfolioState;
use eframe::egui;

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let mut is_expanded = state.mostrar_nav_superior;

    let color_header = egui::Color32::from_rgb(13, 15, 19);

    egui::Panel::top("nav_top_global")
        .frame(egui::Frame::default().fill(color_header).inner_margin(4.0))
        .resizable(false)
        .show_collapsible(ui, &mut is_expanded, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space(5.0);
                
                // --- LADO IZQUIERDO: Título y Teoría ---
                ui.label(
                    egui::RichText::new("Rust Foundations")
                        .size(16.0)
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                
                ui.separator();

                let img_book = egui::Image::new(egui::include_image!("../../../assets/icons/book-line.svg")).fit_to_exact_size(egui::Vec2::new(24.0, 24.0));
                ui.add(img_book);
                let tabs_teoria = [
                    (0, "Rust Ecosystem"),
                    (3, "Build & Execution"),
                    (2, "Cargo Workflow"),
                    (4, "Project Anatomy"),
                ];
                for (indice, texto) in tabs_teoria {
                    let es_activo = state.pilares_step == indice;
                    if ui.selectable_label(es_activo, texto).clicked() {
                        state.pilares_step = indice;
                        state.anim_trigger = ui.input(|i| i.time); // Reiniciar animaciones al cambiar de pestaña
                    }
                }

                // --- LADO DERECHO: Práctica ---
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(5.0);
                    
                    let tabs_practica = [
                        (1, "Code Lab"),
                    ];
                    // Iteramos al revés para que se dibujen correctamente de derecha a izquierda
                    for (indice, texto) in tabs_practica.iter().rev() {
                        let es_activo = state.pilares_step == *indice;
                        if ui.selectable_label(es_activo, *texto).clicked() {
                            state.pilares_step = *indice;
                        }
                    }

                    let img_code = egui::Image::new(egui::include_image!("../../../assets/icons/monitor-code-line.svg")).fit_to_exact_size(egui::Vec2::new(24.0, 24.0));
                    ui.add(img_code);

                    ui.separator();
                });
            });
            ui.add_space(6.0);
        });

    state.mostrar_nav_superior = is_expanded;
}

pub fn mostrar_tutorial_cargo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    // El encabezado y tabs se movieron al panel derecho global

    match state.pilares_step {
        0 => foundations::mostrar_ecosystem(ui, state),
        1 => anatomy::mostrar_anatomia_cargo(ui, state),
        2 => cargo_workflow::mostrar_cargo_workflow(ui, state),
        3 => estructura_tiempos::mostrar_build_execution(ui, state),
        4 => project_anatomy::mostrar_project_anatomy(ui, state),
        _ => foundations::mostrar_ecosystem(ui, state),
    }
}

fn mostrar_tab(
    ui: &mut egui::Ui,
    state: &mut PortfolioState,
    indice: usize,
    texto: &str,
    naranja: egui::Color32,
    gris_tab: egui::Color32,
) {
    let es_activo = state.pilares_step == indice;
    let color = if es_activo { naranja } else { gris_tab };

    if ui
        .add(egui::Button::new(egui::RichText::new(texto).strong().color(color)).frame(es_activo))
        .clicked()
    {
        state.pilares_step = indice;
    }
    ui.add_space(4.0);
}
