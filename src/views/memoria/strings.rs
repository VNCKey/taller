use crate::app::PortfolioState;
use eframe::egui;

/// Sección teórica completa sobre String y &str en Rust
pub fn mostrar_teoria_string_y_str(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.label(
        "String es el tipo de texto dinámico y modificable por excelencia en Rust. Se almacena como un vector de bytes UTF-8 en el Heap y gestiona su memoria automáticamente sin recolector de basura.",
    );
    ui.add_space(10.0);

    // Tabla 1: Anatomía de Memoria de String
    let mut table_mem = egui::Frame::new();
    table_mem.fill = egui::Color32::from_rgb(14, 18, 26);
    table_mem.inner_margin = egui::Margin::same(12);
    table_mem.corner_radius = egui::CornerRadius::same(8);
    table_mem.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_mem.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(
                    "Estructura Interna en Memoria (24 Bytes en Stack + Buffer en Heap)",
                )
                .strong()
                .size(14.0)
                .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(8.0);

            let btn_color = if state.show_railroad_modal == Some(7) {
                egui::Color32::from_rgb(255, 160, 50)
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            };
            if ui
                .add(
                    egui::Button::image(
                        egui::Image::from_bytes(
                            "bytes://view.svg",
                            include_bytes!("../../../diagramas/view.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(18.0, 18.0))
                        .tint(btn_color),
                    )
                    .frame(state.show_railroad_modal == Some(7)),
                )
                .on_hover_text("Ver diagrama visual de la arquitectura del Heap")
                .clicked()
            {
                state.show_railroad_modal = if state.show_railroad_modal == Some(7) {
                    None
                } else {
                    Some(7)
                };
            }
        });
        ui.add_space(8.0);

        egui::Grid::new("tabla_string_memoria_anatomia")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Campo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ubicación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito / Descripción")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("ptr")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Stack");
                ui.label("8 Bytes (64-bit)");
                ui.label("Puntero con la dirección de memoria exacta del buffer en el Heap.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("len")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Stack");
                ui.label("8 Bytes (usize)");
                ui.label("Longitud actual: cantidad de bytes UTF-8 válidos en uso.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("cap")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Stack");
                ui.label("8 Bytes (usize)");
                ui.label("Capacidad total: bytes reservados en Heap antes de requerir realloc.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Buffer UTF-8")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Heap");
                ui.label("Dinámico (cap bytes)");
                ui.label("Secuencia contigua de bytes donde residen las letras del texto.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Tabla 2: Métodos y Operaciones Esenciales de String
    let mut table_methods = egui::Frame::new();
    table_methods.fill = egui::Color32::from_rgb(14, 18, 26);
    table_methods.inner_margin = egui::Margin::same(12);
    table_methods.corner_radius = egui::CornerRadius::same(8);
    table_methods.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_methods.show(ui, |ui| {
        ui.label(
            egui::RichText::new("Métodos y Operaciones Esenciales de String")
                .strong()
                .size(14.0)
                .color(egui::Color32::from_rgb(255, 160, 50)),
        );
        ui.add_space(8.0);

        egui::Grid::new("tabla_string_metodos_operaciones")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Operación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Mutabilidad")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Descripción Técnica")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Resultado")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Creación
                ui.label("Crear vacío");
                ui.label(
                    egui::RichText::new("String::new()")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Inmutable / mut");
                ui.label("Crea un String vacío sin reservar Heap inicial.");
                ui.label("len: 0, cap: 0");
                ui.end_row();

                ui.label("Desde literal");
                ui.label(
                    egui::RichText::new("String::from(\"Hola\")")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Inmutable / mut");
                ui.label("Reserva memoria en Heap y copia el literal &str.");
                ui.label("\"Hola\"");
                ui.end_row();

                ui.label("to_string()");
                ui.label(
                    egui::RichText::new("\"Hola\".to_string()")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Inmutable / mut");
                ui.label("Convierte cualquier tipo con trait Display a String.");
                ui.label("\"Hola\"");
                ui.end_row();

                // Mutación
                ui.label("Añadir &str");
                ui.label(
                    egui::RichText::new("s.push_str(\" mundo\")")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Requiere mut");
                ui.label("Concatena un slice de texto al final del buffer.");
                ui.label("\"Hola mundo\"");
                ui.end_row();

                ui.label("Añadir char");
                ui.label(
                    egui::RichText::new("s.push('!')")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Requiere mut");
                ui.label("Añade un único carácter Unicode al final.");
                ui.label("\"Hola mundo!\"");
                ui.end_row();

                ui.label("Eliminar último");
                ui.label(
                    egui::RichText::new("s.pop()")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Requiere mut");
                ui.label("Extrae y devuelve el último carácter (Option<char>).");
                ui.label("Some('!')");
                ui.end_row();

                // Capacidad e Inspección
                ui.label("Longitud en bytes");
                ui.label(
                    egui::RichText::new("s.len()")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Solo Lectura");
                ui.label("Devuelve la cantidad de bytes que ocupa el texto.");
                ui.label("usize");
                ui.end_row();

                ui.label("Capacidad en Heap");
                ui.label(
                    egui::RichText::new("s.capacity()")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Solo Lectura");
                ui.label("Bytes totales asignados en el buffer del Heap.");
                ui.label("usize");
                ui.end_row();

                ui.label("Vaciar buffer");
                ui.label(
                    egui::RichText::new("s.clear()")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Requiere mut");
                ui.label("Elimina todo el contenido pero mantiene la capacidad en Heap.");
                ui.label("len: 0");
                ui.end_row();
            });
    });
}
