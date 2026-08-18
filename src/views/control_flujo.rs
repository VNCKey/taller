use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::comenzando::mostrar_selector_proyectos_estandar;

pub fn mostrar_tutorial_control_flujo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.heading(
        egui::RichText::new("Control de Flujo")
            .size(22.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(10.0);

    // Selector de pestañas interactivas de Control de Flujo
    ui.horizontal(|ui| {
        if ui
            .selectable_label(
                state.controlflujo_tab == 0,
                egui::RichText::new("Condicionales").strong(),
            )
            .clicked()
        {
            state.controlflujo_tab = 0;
        }
        if ui
            .selectable_label(
                state.controlflujo_tab == 1,
                egui::RichText::new("Bucles").strong(),
            )
            .clicked()
        {
            state.controlflujo_tab = 1;
        }
        if ui
            .selectable_label(
                state.controlflujo_tab == 2,
                egui::RichText::new("Match").strong(),
            )
            .clicked()
        {
            state.controlflujo_tab = 2;
        }
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(15.0);

    match state.controlflujo_tab {
        0 => {
            ui.label(
                "En Rust, 'if' no es solo una declaración de control, sino una expresión que devuelve un valor. Esto permite asignar el resultado de una condición directamente a una variable.",
            );
            ui.add_space(10.0);

            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_if_else_rust")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Estructura").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("if / else").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("if c { a } else { b }").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Evalúa una condición booleana; ambos bloques deben retornar el mismo tipo.");
                        ui.end_row();

                        ui.label(egui::RichText::new("else if").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("if c1 { } else if c2 { }").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Permite encadenar múltiples evaluaciones de forma secuencial.");
                        ui.end_row();
                    });
            });
        }
        1 => {
            ui.label(
                "Rust ofrece 3 construcciones para bucles: 'loop' para repetición infinita o con retorno de valor, 'while' para ejecución condicional y 'for' para iterar de forma segura sobre rangos y colecciones.",
            );
            ui.add_space(10.0);

            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_bucles_rust")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Bucle")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Ejemplo de Sintaxis")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Descripción")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("loop")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label(
                            egui::RichText::new("loop { break valor; }")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label(
                            "Bucle infinito. Permite retornar un valor mediante 'break valor;'.",
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("while")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label(
                            egui::RichText::new("while condicion { ... }")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label("Se ejecuta repetidamente mientras la condición sea 'true'.");
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("for")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label(
                            egui::RichText::new("for i in 1..=5 { ... }")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label("Itera sobre un rango o colección de elementos de forma segura.");
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.label(
                "La sentencia 'match' permite comparar un valor contra una serie de patrones y ejecutar código basado en el primer patrón que coincida. El compilador de Rust exige exhaustividad total.",
            );
            ui.add_space(10.0);

            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_match_rust")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Patrón").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Valor Literal").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("1 => println!(\"Uno\"),").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Coincidencia exacta con un valor explícito.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Rangos").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("1..=5 => println!(\"1 a 5\"),").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Coincidencia con cualquier número dentro del rango.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Comodín _").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("_ => println!(\"Cualquier otro\"),").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Captura cualquier caso no especificado previamente (obligatorio para cumplir exhaustividad).");
                        ui.end_row();
                    });
            });
        }
    }

    ui.add_space(15.0);

    mostrar_selector_proyectos_estandar(
        ui,
        &mut state.selected_project,
        &mut state.term_cwd,
        "combo_proyectos_control_flujo",
        &mut state.controlflujo_code,
    );

    ui.add_space(10.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.controlflujo_code,
        Arc::clone(&state.controlflujo_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}


pub fn card_frame_tutorial() -> egui::Frame {
    let mut f = egui::Frame::new();
    f.fill = egui::Color32::from_rgb(14, 18, 26);
    f.inner_margin = egui::Margin::same(12);
    f.corner_radius = egui::CornerRadius::same(8);
    f.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));
    f
}

