pub mod bucles;
pub mod condicionales;
pub mod info;
pub mod match_expr;

use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::conceptos::mostrar_selector_proyectos_estandar_con_archivos;
use eframe::egui;

pub fn mostrar_tutorial_control_flujo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Control de Flujo")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Barra de navegación de pestañas de Control de Flujo
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        let tabs_practica = [(0, "Condicionales"), (1, "Bucles"), (2, "Match")];
        for (indice, label) in tabs_practica {
            let activo = state.controlflujo_tab == indice;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.controlflujo_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.controlflujo_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Info").strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.controlflujo_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Teórico:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(10.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if state.controlflujo_tab < 3 {
                let code_target = if state.selected_project.is_some() {
                    &mut state.shared_project_code
                } else {
                    &mut state.controlflujo_code
                };

                mostrar_selector_proyectos_estandar_con_archivos(
                    ui,
                    &mut state.selected_project,
                    &mut state.selected_file,
                    &mut state.term_cwd,
                    "combo_proyectos_control_flujo",
                    code_target,
                );

                ui.add_space(10.0);

                let syntax_set = state.syntax_set.clone();
                let theme = state.theme_set.themes["base16-ocean.dark"].clone();
                let (code_ref, output_arc) = state.obtener_editor_activo_mut();
                mostrar_editor_interactivo(
                    ui,
                    code_ref,
                    output_arc,
                    "",
                    ejecutar_codigo_rust,
                    &syntax_set,
                    &theme,
                );

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(12.0);
            }

            match state.controlflujo_tab {
                0 => condicionales::mostrar_tab_condicionales(ui, state),
                1 => bucles::mostrar_tab_bucles(ui, state),
                2 => match_expr::mostrar_tab_match(ui, state),
                _ => info::mostrar_control_flujo_info(ui, state, naranja, cyan, texto),
            }
        });
}

pub fn card_frame_tutorial() -> egui::Frame {
    let mut f = egui::Frame::new();
    f.fill = egui::Color32::from_rgb(14, 18, 26);
    f.inner_margin = egui::Margin::same(12);
    f.corner_radius = egui::CornerRadius::same(8);
    f.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));
    f
}
