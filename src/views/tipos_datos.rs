use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::control_flujo::card_frame_tutorial;

pub fn mostrar_categoria_enteros(ui: &mut egui::Ui) {
    ui.label("En Rust, los enteros se dividen según si admiten números negativos (signed `i`) o solo positivos y cero (unsigned `u`).");
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_enteros")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Familia")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Bits")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Rango Mínimo .. Máximo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                let datos_enteros = [
                    ("i8", "Con signo", "8", "-128 .. 127", "let x: i8 = -50;"),
                    (
                        "i16",
                        "Con signo",
                        "16",
                        "-32,768 .. 32,767",
                        "let x: i16 = -1500;",
                    ),
                    (
                        "i32",
                        "Con signo",
                        "32",
                        "-2,147,483,648 .. 2,147,483,647",
                        "let x: i32 = -25000; (Por defecto)",
                    ),
                    (
                        "i64",
                        "Con signo",
                        "64",
                        "-9.22×10¹⁸ .. 9.22×10¹⁸",
                        "let x: i64 = -9_000_000_000;",
                    ),
                    (
                        "i128",
                        "Con signo",
                        "128",
                        "-1.70×10³⁸ .. 1.70×10³⁸",
                        "let x: i128 = -100_000_000;",
                    ),
                    (
                        "isize",
                        "Según arquitectura",
                        "32 u 64",
                        "Depende del procesador",
                        "let x: isize = -100;",
                    ),
                    ("u8", "Sin signo", "8", "0 .. 255", "let x: u8 = 255;"),
                    (
                        "u16",
                        "Sin signo",
                        "16",
                        "0 .. 65,535",
                        "let x: u16 = 65535;",
                    ),
                    (
                        "u32",
                        "Sin signo",
                        "32",
                        "0 .. 4,294,967,295",
                        "let x: u32 = 100_000;",
                    ),
                    (
                        "u64",
                        "Sin signo",
                        "64",
                        "0 .. 1.84×10¹⁹",
                        "let x: u64 = 5_000_000;",
                    ),
                    (
                        "u128",
                        "Sin signo",
                        "128",
                        "0 .. 3.40×10³⁸",
                        "let x: u128 = 100_000_000;",
                    ),
                    (
                        "usize",
                        "Según arquitectura",
                        "32 u 64",
                        "0 .. Max Memoria CPU",
                        "let x: usize = 10; (Por defecto arreglo)",
                    ),
                ];

                for (tipo, fam, bits, rango, ej_codigo) in datos_enteros {
                    ui.label(
                        egui::RichText::new(tipo)
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label(
                        egui::RichText::new(fam).color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(bits).color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(rango)
                            .monospace()
                            .color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(ej_codigo)
                            .monospace()
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                    ui.end_row();
                }
            });
    });
}


pub fn mostrar_categoria_flotantes(ui: &mut egui::Ui) {
    ui.label("Los tipos flotantes representan números con coma o fracción decimal en el estándar IEEE-754.");
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_flotantes")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Precisión")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
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
                    egui::RichText::new("f32")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Precisión Simple (~6-9 dígitos)");
                ui.label("32 bits (4 bytes)");
                ui.label(
                    egui::RichText::new("let pi: f32 = 3.14159;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Ideal para gráficos 3D, física de juegos y ahorro de memoria.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("f64")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Precisión Doble (~15-17 dígitos)");
                ui.label("64 bits (8 bytes)");
                ui.label(
                    egui::RichText::new("let pi: f64 = 3.141592653589793;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Por defecto en Rust para decimales. Alta precisión científica.");
                ui.end_row();
            });
    });
}


pub fn mostrar_categoria_booleanos(ui: &mut egui::Ui) {
    ui.label("El tipo booleano representa una verdad lógica simple. En Rust solo existen dos valores posibles: true y false.");
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_booleanos")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Valores Posibles")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("bool")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("true  |  false")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label("1 byte");
                ui.label(
                    egui::RichText::new(
                        "let es_activo: bool = true;\nlet mut error: bool = false;",
                    )
                    .monospace()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();
            });
    });
}


pub fn mostrar_categoria_caracteres(ui: &mut egui::Ui) {
    ui.label("En Rust, un 'char' es un valor escalar Unicode de 4 bytes (32 bits), lo que significa que soporta mucho más que texto ASCII.");
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_caracteres")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Características")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("char")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Comillas simples ''");
                ui.label("4 bytes (32 bits)");
                ui.label(
                    egui::RichText::new(
                        "let letra: char = 'A';\nlet minuscula: char = 'z';\nlet simbolo: char = '@';\nlet letra_n: char = 'ñ';",
                    )
                    .monospace()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Soporta ASCII, acentos, Emojis y caracteres de todo el mundo.");
                ui.end_row();
            });
    });
}


pub fn mostrar_macro_println(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.heading("🧩 ¿Por qué `println!` termina con `!`?");
    ui.label("El signo `!` indica que estás invocando una macro. Una macro recibe tokens y genera código durante la compilación.");
    ui.add_space(8.0);
    ui.columns(2, |columns| {
        columns[0].group(|ui| {
            ui.label(egui::RichText::new("Lo que escribes").strong());
            ui.code("println!(\"Hola, Ferris!\");");
            ui.label("La macro valida el formato y construye los argumentos de impresión.");
        });
        columns[1].group(|ui| {
            ui.label(egui::RichText::new("Modelo mental").strong());
            ui.code("tokens → expansión → código compilable");
            ui.label("Las macros son más potentes que una simple sustitución de texto.");
        });
    });
    ui.add_space(10.0);
    if ui.button("🔬 Expandir println! con cargo expand").clicked() {
        state.show_macro_expansion = true;
    }

    let mut abierto = state.show_macro_expansion;
    egui::Window::new("Expansión didáctica de println!")
        .open(&mut abierto)
        .collapsible(false)
        .default_width(620.0)
        .show(ui.ctx(), |ui| {
            ui.label("Representación simplificada para entender la idea:");
            ui.code("std::io::_print(format_args!(\"Hola, Ferris!\\n\"));");
            ui.add_space(8.0);
            ui.label("La expansión exacta depende de la versión del compilador y puede usar detalles internos.");
            ui.separator();
            ui.code("cargo install cargo-expand");
            ui.code("cargo expand");
        });
    state.show_macro_expansion = abierto;
}


pub fn mostrar_tutorial_tipos_datos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Tipos compuestos")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        for (i, label) in [(0, "Array [T; N]"), (1, "Slice &[T]"), (2, "Tupla")] {
            let activo = state.compuestos_tab == i;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.compuestos_tab = i;
            }
            ui.add_space(4.0);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.compuestos_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Comparar").strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.compuestos_tab = 3;
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
    ui.add_space(12.0);

    match state.compuestos_tab {
        0 => mostrar_compuesto_array(ui, state, naranja, cyan, texto),
        1 => mostrar_compuesto_slice(ui, state, naranja, cyan, texto),
        2 => mostrar_compuesto_tupla(ui, state, naranja, cyan, texto),
        _ => mostrar_compuesto_comparar(ui, naranja, cyan, texto),
    }
}


fn mostrar_compuesto_array(
    ui: &mut egui::Ui,
    state: &mut PortfolioState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un array `[T; N]` guarda N valores del mismo tipo, contiguos, con tamaño fijo \
             conocido en compilación. Suele vivir en el stack.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_array_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Pieza").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Nota").strong().color(egui::Color32::WHITE));
                ui.end_row();
                ui.label(egui::RichText::new("Tipo").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("[i32; 5]").monospace().color(cyan));
                ui.label("T y N fijos; N es parte del tipo.");
                ui.end_row();
                ui.label(egui::RichText::new("Acceso").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("arr[i]").monospace().color(cyan));
                ui.label("Fuera de rango → panic en runtime.");
                ui.end_row();
                ui.label(egui::RichText::new("len").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("arr.len()").monospace().color(cyan));
                ui.label("Siempre N; no crece como un Vec.");
                ui.end_row();
            });
    });

    ui.add_space(12.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Tipo T:").strong().color(texto));
        ui.selectable_value(&mut state.arr_elem_type, 0, "i8");
        ui.selectable_value(&mut state.arr_elem_type, 1, "i32");
        ui.selectable_value(&mut state.arr_elem_type, 2, "f64");
        ui.selectable_value(&mut state.arr_elem_type, 3, "bool");
        ui.selectable_value(&mut state.arr_elem_type, 4, "char");
        ui.add_space(12.0);
        ui.label(egui::RichText::new("N:").strong().color(texto));
        ui.add(egui::Slider::new(&mut state.arr_len, 1..=8).text("elems"));
    });

    let mut custom_items: Vec<String> = Vec::new();
    if let Some(pos_eq) = state.arr_code.find("= [") {
        let rest = &state.arr_code[pos_eq + 3..];
        if let Some(pos_end) = rest.find(']') {
            custom_items = rest[..pos_end]
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !custom_items.is_empty() {
                state.arr_len = custom_items.len().clamp(1, 8);
            }
        }
    }
    if state.arr_code.contains("i8") {
        state.arr_elem_type = 0;
    } else if state.arr_code.contains("f64") {
        state.arr_elem_type = 2;
    } else if state.arr_code.contains("bool") {
        state.arr_elem_type = 3;
    } else if state.arr_code.contains("char") {
        state.arr_elem_type = 4;
    } else if state.arr_code.contains("i32") || state.arr_code.contains("u32") {
        state.arr_elem_type = 1;
    }

    let (type_str, elem_size, default_samples) = match state.arr_elem_type {
        0 => ("i8", 1, vec!["-12", "45", "127", "-8", "0", "99", "-50", "12"]),
        1 => (
            "i32",
            4,
            vec!["100", "-500", "2048", "42", "0", "999", "-123", "8888"],
        ),
        2 => (
            "f64",
            8,
            vec!["3.14", "9.81", "-0.5", "2.71", "100.0", "0.001", "-45.2", "1.61"],
        ),
        3 => (
            "bool",
            1,
            vec!["true", "false", "true", "true", "false", "false", "true", "false"],
        ),
        _ => (
            "char",
            4,
            vec!["'R'", "'u'", "'s'", "'t'", "'🦀'", "'⚡'", "'🔥'", "'A'"],
        ),
    };
    let samples: Vec<&str> = if !custom_items.is_empty() {
        custom_items.iter().map(|s| s.as_str()).collect()
    } else {
        default_samples
    };
    let total_stack_bytes = state.arr_len * elem_size;

    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(format!(
            "Firma: [{type_str}; {}]  ·  Stack ≈ {total_stack_bytes} bytes",
            state.arr_len
        ))
        .monospace()
        .color(cyan),
    );

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 120.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let cell_w = (rect.width() - 40.0) / state.arr_len as f32;
    let start_x = rect.left() + 20.0;
    let y = rect.center().y;
    for i in 0..state.arr_len {
        let box_x = start_x + (i as f32 * cell_w) + cell_w / 2.0;
        let box_rect =
            egui::Rect::from_center_size(egui::pos2(box_x, y), egui::vec2(cell_w - 6.0, 48.0));
        let is_active = i == state.arr_active_idx;
        let fill = if is_active {
            egui::Color32::from_rgb(48, 36, 22)
        } else {
            egui::Color32::from_rgb(28, 36, 52)
        };
        let stroke_c = if is_active { naranja } else { cyan };
        painter.rect(
            box_rect,
            5.0,
            fill,
            egui::Stroke::new(1.5, stroke_c),
            egui::StrokeKind::Middle,
        );
        painter.text(
            egui::pos2(box_rect.center().x, box_rect.top() + 6.0),
            egui::Align2::CENTER_TOP,
            format!("[{i}]"),
            egui::FontId::proportional(11.0),
            egui::Color32::LIGHT_GRAY,
        );
        painter.text(
            box_rect.center() + egui::vec2(0.0, 4.0),
            egui::Align2::CENTER_CENTER,
            samples[i % samples.len()],
            egui::FontId::monospace(12.0),
            egui::Color32::WHITE,
        );
    }

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("Índice arr[i]:");
        ui.add(egui::Slider::new(&mut state.arr_active_idx, 0..=state.arr_len.max(1)).text("i"));
        if ui.button("arr.len()").clicked() {
            state.arr_action_msg = format!("arr.len() = {}", state.arr_len);
        }
        if ui.button("size_of").clicked() {
            state.arr_action_msg = format!("≈ {total_stack_bytes} bytes en stack");
        }
    });
    if state.arr_active_idx >= state.arr_len {
        ui.label(
            egui::RichText::new(format!(
                "PANIC: índice {} fuera de rango (len {})",
                state.arr_active_idx, state.arr_len
            ))
            .color(egui::Color32::from_rgb(255, 120, 120)),
        );
    } else {
        ui.label(
            egui::RichText::new(format!(
                "arr[{}] = {}",
                state.arr_active_idx,
                samples[state.arr_active_idx % samples.len()]
            ))
            .color(egui::Color32::from_rgb(120, 220, 140)),
        );
    }
    if !state.arr_action_msg.is_empty() {
        ui.label(
            egui::RichText::new(&state.arr_action_msg)
                .italics()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
    }

    ui.add_space(12.0);
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.arr_code,
        Arc::clone(&state.arr_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}


fn mostrar_compuesto_slice(
    ui: &mut egui::Ui,
    state: &mut PortfolioState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un slice `&[T]` es una vista (préstamo) sobre una secuencia contigua: \
             fat pointer = puntero + longitud. No es dueño de los datos (Ownership).",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_slice_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Pieza").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Nota").strong().color(egui::Color32::WHITE));
                ui.end_row();
                ui.label(egui::RichText::new("Tipo").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("&[i32]").monospace().color(cyan));
                ui.label("Referencia; el array/String sigue siendo dueño.");
                ui.end_row();
                ui.label(egui::RichText::new("Rango").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("&arr[1..4]").monospace().color(cyan));
                ui.label("Inicio inclusivo, fin exclusivo.");
                ui.end_row();
                ui.label(egui::RichText::new("&str").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("&str ≈ &[u8] UTF-8").monospace().color(cyan));
                ui.label("El slice de texto que ya viste en Strings.");
                ui.end_row();
            });
    });

    let slice_max = 6;
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Rango:");
        ui.add(egui::Slider::new(&mut state.slice_start, 0..=slice_max - 1).text("start"));
        ui.add(egui::Slider::new(&mut state.slice_end, 1..=slice_max).text("end"));
    });
    if state.slice_start >= state.slice_end {
        state.slice_end = state.slice_start + 1;
    }
    let slice_len = state.slice_end - state.slice_start;

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 130.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let cell_w = 72.0;
    let start_x = rect.left() + 36.0;
    let y = rect.center().y + 4.0;
    let vals = ["10", "20", "30", "40", "50", "60"];
    for i in 0..slice_max {
        let box_x = start_x + (i as f32 * cell_w) + cell_w / 2.0;
        let box_rect =
            egui::Rect::from_center_size(egui::pos2(box_x, y), egui::vec2(cell_w - 8.0, 44.0));
        let in_slice = i >= state.slice_start && i < state.slice_end;
        painter.rect(
            box_rect,
            4.0,
            if in_slice {
                egui::Color32::from_rgb(28, 48, 72)
            } else {
                egui::Color32::from_rgb(24, 28, 36)
            },
            egui::Stroke::new(1.2, if in_slice { cyan } else { egui::Color32::from_rgb(60, 70, 85) }),
            egui::StrokeKind::Middle,
        );
        painter.text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            vals[i],
            egui::FontId::monospace(13.0),
            egui::Color32::WHITE,
        );
    }
    let slice_min_x = start_x + (state.slice_start as f32 * cell_w);
    let slice_max_x = start_x + (state.slice_end as f32 * cell_w);
    let slice_rect = egui::Rect::from_min_max(
        egui::pos2(slice_min_x + 2.0, y - 30.0),
        egui::pos2(slice_max_x - 2.0, y + 30.0),
    );
    painter.rect_stroke(
        slice_rect,
        6.0,
        egui::Stroke::new(2.0, naranja),
        egui::StrokeKind::Middle,
    );
    painter.text(
        egui::pos2(slice_rect.center().x, slice_rect.top() - 4.0),
        egui::Align2::CENTER_BOTTOM,
        format!(
            "&arr[{}..{}]  len={slice_len}",
            state.slice_start, state.slice_end
        ),
        egui::FontId::proportional(12.0),
        naranja,
    );

    ui.add_space(12.0);
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.slice_code,
        Arc::clone(&state.slice_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}


fn mostrar_compuesto_tupla(
    ui: &mut egui::Ui,
    state: &mut PortfolioState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Una tupla agrupa valores de tipos distintos, sin nombres de campo. \
             Acceso por `.0`, `.1`… o desestructuración. Puente natural hacia `struct`.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_tupla_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Pieza").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Nota").strong().color(egui::Color32::WHITE));
                ui.end_row();
                ui.label(egui::RichText::new("Tipo").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("(i32, bool, f64)").monospace().color(cyan));
                ui.label("Heterogénea; el orden define el tipo.");
                ui.end_row();
                ui.label(egui::RichText::new("Campo").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("t.0  t.1").monospace().color(cyan));
                ui.label("Índices fijos desde cero.");
                ui.end_row();
                ui.label(egui::RichText::new("Destruct").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("let (a, b, c) = t;").monospace().color(cyan));
                ui.label("Muy usado al devolver varios valores desde fn.");
                ui.end_row();
            });
    });

    ui.add_space(10.0);
    let row = |ui: &mut egui::Ui, label: &str, slot: &mut usize| {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.selectable_value(slot, 0, "i32");
            ui.selectable_value(slot, 1, "bool");
            ui.selectable_value(slot, 2, "f64");
            ui.selectable_value(slot, 3, "char");
        });
    };
    row(ui, "Campo .0:", &mut state.tup_t0);
    row(ui, "Campo .1:", &mut state.tup_t1);
    row(ui, "Campo .2:", &mut state.tup_t2);

    let info = |id: usize| match id {
        0 => ("i32", "100", egui::Color32::from_rgb(60, 140, 240)),
        1 => ("bool", "true", egui::Color32::from_rgb(40, 180, 100)),
        2 => ("f64", "3.14", egui::Color32::from_rgb(240, 140, 40)),
        _ => ("char", "'R'", egui::Color32::from_rgb(180, 120, 240)),
    };
    let (n0, v0, c0) = info(state.tup_t0);
    let (n1, v1, c1) = info(state.tup_t1);
    let (n2, v2, c2) = info(state.tup_t2);

    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(format!("Firma: ({n0}, {n1}, {n2})"))
            .monospace()
            .color(cyan),
    );

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 110.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let y = rect.center().y;
    for (i, (name, val, col)) in [(n0, v0, c0), (n1, v1, c1), (n2, v2, c2)]
        .into_iter()
        .enumerate()
    {
        let x = rect.left() + 90.0 + i as f32 * 180.0;
        let r = egui::Rect::from_center_size(egui::pos2(x, y), egui::vec2(150.0, 52.0));
        painter.rect(
            r,
            6.0,
            egui::Color32::from_rgb(22, 28, 40),
            egui::Stroke::new(2.0, col),
            egui::StrokeKind::Middle,
        );
        painter.text(
            r.center(),
            egui::Align2::CENTER_CENTER,
            format!(".{i}: {val} ({name})"),
            egui::FontId::monospace(12.0),
            egui::Color32::WHITE,
        );
    }

    ui.add_space(12.0);
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.tup_code,
        Arc::clone(&state.tup_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}


fn mostrar_compuesto_comparar(
    ui: &mut egui::Ui,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Elige la forma según homogeneidad, tamaño fijo y si necesitas nombres de campo. \
             Después, `struct` pondrá nombres; `Vec` hará crecer lo homogéneo.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_compuestos_vs")
            .striped(true)
            .spacing([14.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Tipo").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Homogéneo").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Tamaño").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Dueño").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Siguiente paso").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("[T; N]").monospace().strong().color(naranja));
                ui.label("Sí");
                ui.label("Fijo N");
                ui.label("El array");
                ui.label(egui::RichText::new("Vec<T>").monospace().color(cyan));
                ui.end_row();

                ui.label(egui::RichText::new("&[T]").monospace().strong().color(naranja));
                ui.label("Sí");
                ui.label("Dinámico (vista)");
                ui.label("No (préstamo)");
                ui.label(egui::RichText::new("&str / APIs").monospace().color(cyan));
                ui.end_row();

                ui.label(egui::RichText::new("(A,B,…)").monospace().strong().color(naranja));
                ui.label("No");
                ui.label("Fijo #campos");
                ui.label("La tupla");
                ui.label(egui::RichText::new("struct").monospace().color(cyan));
                ui.end_row();
            });
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(
            "Cuando la tupla se vuelve confusa (¿qué era .2?) → sesión Structs & impl.",
        )
        .italics()
        .color(egui::Color32::from_rgb(140, 150, 165)),
    );
}

