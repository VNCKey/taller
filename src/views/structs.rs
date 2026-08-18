use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::comenzando::mostrar_selector_proyectos_estandar;
use crate::views::control_flujo::card_frame_tutorial;

pub fn mostrar_tutorial_structs(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Structs & impl")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Conceptos:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        for (i, label) in [
            (0, "struct"),
            (1, "impl / métodos"),
            (2, "Asociadas"),
        ] {
            let activo = state.structs_tab == i;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.structs_tab = i;
            }
            ui.add_space(4.0);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.structs_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("vs tupla")
                            .strong()
                            .color(color),
                    )
                    .frame(activo),
                )
                .clicked()
            {
                state.structs_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Comparar:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.structs_tab {
        0 => {
            ui.label(
                egui::RichText::new(
                    "Un `struct` agrupa datos relacionados con campos nombrados. \
                     Es la forma idiomática de modelar entidades (usuario, servidor, punto…).",
                )
                .color(texto),
            );
            ui.add_space(10.0);
            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_struct_formas")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Forma").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Uso").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Nombrado").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("struct User { id: u64, name: String }")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Campos con nombre; lo más habitual.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Tupla struct").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("struct Color(u8, u8, u8);")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Como tupla, pero con tipo propio.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Unit").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("struct Marcador;")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Sin datos; útil como marca de tipo.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Instancia").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("User { id: 1, name: s }")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Hay que nombrar todos los campos (o ..base).");
                        ui.end_row();
                    });
            });
        }
        1 => {
            ui.label(
                egui::RichText::new(
                    "El bloque `impl` asocia funciones al tipo. Los métodos reciben `self`, \
                     `&self` o `&mut self` (Ownership + borrowing aplicados a tus datos).",
                )
                .color(texto),
            );
            ui.add_space(10.0);
            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_impl_metodos")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Receptor").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Firma").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Significado").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("&self").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("fn len(&self) -> usize")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Solo lee; no toma el dueño.");
                        ui.end_row();

                        ui.label(egui::RichText::new("&mut self").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("fn iniciar(&mut self)")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Modifica el struct en el sitio.");
                        ui.end_row();

                        ui.label(egui::RichText::new("self").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("fn into_parts(self)")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Consume el valor (move).");
                        ui.end_row();

                        ui.label(egui::RichText::new("Llamada").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("obj.metodo()")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Azúcar de Tipo::metodo(&obj) / similar.");
                        ui.end_row();
                    });
            });
        }
        2 => {
            ui.label(
                egui::RichText::new(
                    "Las funciones asociadas no llevan `self`: viven en el tipo \
                     (`ServidorWeb::new`). `Self` es alias del tipo del `impl`.",
                )
                .color(texto),
            );
            ui.add_space(10.0);
            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_asociadas")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Idea").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Nota").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Constructor").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("fn new(...) -> Self")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Convención; no es palabra clave.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Self").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("Self { campo: v }")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Igual que escribir el nombre del struct.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Ruta").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("Tipo::new(args)")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Sin instancia previa.");
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.label(
                egui::RichText::new(
                    "La tupla es anónima y posicional; el struct da nombres y puede llevar `impl`. \
                     Cuando `.2` ya no se entiende solo, es hora de campos nombrados.",
                )
                .color(texto),
            );
            ui.add_space(10.0);
            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_struct_vs_tupla")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Tupla").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("struct").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Campos").strong().color(naranja));
                        ui.label(egui::RichText::new(".0 .1").monospace().color(cyan));
                        ui.label(egui::RichText::new(".nombre").monospace().color(cyan));
                        ui.end_row();

                        ui.label(egui::RichText::new("Métodos").strong().color(naranja));
                        ui.label("No (de serie)");
                        ui.label(egui::RichText::new("impl").monospace().color(cyan));
                        ui.end_row();

                        ui.label(egui::RichText::new("Legibilidad").strong().color(naranja));
                        ui.label("Pocas piezas");
                        ui.label("Datos de dominio");
                        ui.end_row();

                        ui.label(egui::RichText::new("Ejemplo").strong().color(naranja));
                        ui.label(
                            egui::RichText::new("(u16, String)")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label(
                            egui::RichText::new("ServidorWeb { puerto, host }")
                                .monospace()
                                .color(cyan),
                        );
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
        "combo_proyectos_structs",
        &mut state.structs_code,
    );
    ui.add_space(10.0);
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.structs_code,
        Arc::clone(&state.structs_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}


