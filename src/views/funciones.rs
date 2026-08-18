use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::comenzando::mostrar_selector_proyectos_estandar;

pub fn mostrar_tutorial_funciones(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Funciones & Closures")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Tabs: mismo patrón que Comenzando / Control de Flujo
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );

        let tabs_izq = [(0, "fn y parámetros"), (1, "Retorno"), (2, "Closures")];
        for (indice, label) in tabs_izq {
            let activo = state.funciones_tab == indice;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.funciones_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.funciones_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("Call Stack")
                            .strong()
                            .color(color),
                    )
                    .frame(activo),
                )
                .clicked()
            {
                state.funciones_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Visual:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.funciones_tab {
        0 => {
            ui.label(
                egui::RichText::new(
                    "Una función empaqueta lógica reutilizable. En Rust la firma declara tipos de \
                     parámetros y (si devuelve algo) el tipo de retorno. El cuerpo es un bloque `{}`.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_fn_params")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Pieza").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Declaración").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("fn nombre(args) { ... }").monospace().color(cyan));
                        ui.label("Define la función. Los tipos de cada parámetro son obligatorios.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Parámetro").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("x: i32").monospace().color(cyan));
                        ui.label("Nombre + tipo. Se pasan por valor salvo que uses referencias.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Referencia").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("val: &mut i32").monospace().color(cyan));
                        ui.label("Presta el valor sin moverlo; &mut permite modificarlo.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Llamada").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("calcular_suma(a, b)").monospace().color(cyan));
                        ui.label("Ejecuta la función y (si hay retorno) produce un valor.");
                        ui.end_row();
                    });
            });
        }
        1 => {
            ui.label(
                egui::RichText::new(
                    "En Rust casi todo es una expresión. La última línea de un bloque sin `;` es el \
                     valor que devuelve. `return` existe, pero el estilo idiomático es el retorno implícito.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_fn_retorno")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Forma").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Notas").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Firma -> T").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("fn f(x: i32) -> i32").monospace().color(cyan));
                        ui.label("Obligatorio si la función devuelve un valor distinto de ().");
                        ui.end_row();

                        ui.label(egui::RichText::new("Implícito").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("x + y   // sin ';'").monospace().color(cyan));
                        ui.label("Última expresión del bloque = valor de retorno.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Con ;").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("x + y;  // devuelve ()").monospace().color(cyan));
                        ui.label("El `;` convierte la expresión en declaración → no hay valor.");
                        ui.end_row();

                        ui.label(egui::RichText::new("return").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("return x + y;").monospace().color(cyan));
                        ui.label("Salida temprana; útil en ramas, no obligatorio al final.");
                        ui.end_row();
                    });
            });
        }
        2 => {
            ui.label(
                egui::RichText::new(
                    "Un closure es una función anónima que puede capturar variables del entorno. \
                     Se escribe con `|params| cuerpo` y es la base de iteradores (`.map`, `.filter`).",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_closures")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Idea").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Básico").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("|x| x * 2").monospace().color(cyan));
                        ui.label("Un parámetro; el tipo suele inferirse del uso.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Tipado").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("|x: i32| -> i32 { x + 1 }").monospace().color(cyan));
                        ui.label("Puedes anotar tipos y usar bloque `{}` si hay varias líneas.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Captura").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let f = |x| x * factor;").monospace().color(cyan));
                        ui.label("`factor` vive fuera: el closure la toma prestada o la mueve.");
                        ui.end_row();

                        ui.label(egui::RichText::new("fn vs closure").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("fn f(x: i32) vs |x|").monospace().color(cyan));
                        ui.label("`fn` no captura el entorno; el closure sí (Fn / FnMut / FnOnce).");
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.label(
                egui::RichText::new(
                    "Cada llamada apila un frame (variables locales + punto de retorno). Al terminar, \
                     el frame se desapila y el valor vuelve al llamador — aquí con retorno implícito.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Paso:").strong().color(gris_tab));
                ui.add_space(6.0);
                for (i, label) in [
                    (0, "1. main()"),
                    (1, "2. Apilar suma"),
                    (2, "3. Retorno 40"),
                ] {
                    let activo = state.funciones_step == i;
                    let color = if activo { naranja } else { gris_tab };
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(label).strong().color(color))
                                .frame(activo),
                        )
                        .clicked()
                    {
                        state.funciones_step = i;
                    }
                    ui.add_space(4.0);
                }
            });

            ui.add_space(12.0);
            mostrar_simulador_call_stack(ui, state.funciones_step);
        }
    }

    // Editor en todas las pestañas de práctica (0–2); también útil tras ver el stack
    if state.funciones_tab < 3 {
        ui.add_space(15.0);
        mostrar_selector_proyectos_estandar(
            ui,
            &mut state.selected_project,
            &mut state.term_cwd,
            "combo_proyectos_funciones",
            &mut state.funciones_code,
        );
        ui.add_space(10.0);
        let theme = &state.theme_set.themes["base16-ocean.dark"];
        mostrar_editor_interactivo(
            ui,
            &mut state.funciones_code,
            Arc::clone(&state.funciones_output),
            "",
            ejecutar_codigo_rust,
            &state.syntax_set,
            theme,
        );
    } else {
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(
                "Prueba el mismo flujo en el editor: pestaña «fn y parámetros» o «Retorno».",
            )
            .small()
            .italics()
            .color(egui::Color32::from_rgb(140, 150, 165)),
        );
    }
}

/// Simulador visual de call stack (tema naranja/cyan del curso).

pub fn mostrar_simulador_call_stack(ui: &mut egui::Ui, step: usize) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let frame_bg = egui::Color32::from_rgb(30, 40, 60);
    let canvas_bg = egui::Color32::from_rgb(14, 18, 26);
    let border = egui::Color32::from_rgb(45, 60, 90);

    let height = 168.0;
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), height));
    let painter = ui.painter_at(rect);

    painter.rect(
        rect,
        8.0,
        canvas_bg,
        egui::Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );

    let main_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 150.0, rect.center().y),
        egui::vec2(230.0, 108.0),
    );
    painter.rect(
        main_rect,
        6.0,
        frame_bg,
        egui::Stroke::new(1.5, cyan),
        egui::StrokeKind::Middle,
    );
    painter.text(
        main_rect.left_top() + egui::vec2(12.0, 10.0),
        egui::Align2::LEFT_TOP,
        "Frame: main()",
        egui::FontId::proportional(13.0),
        cyan,
    );
    painter.text(
        main_rect.center() + egui::vec2(0.0, 8.0),
        egui::Align2::CENTER_CENTER,
        "let a = 15;\nlet b = 25;",
        egui::FontId::monospace(13.0),
        egui::Color32::WHITE,
    );

    if step >= 1 {
        let sub_rect = egui::Rect::from_center_size(
            egui::pos2(rect.left() + 470.0, rect.center().y),
            egui::vec2(250.0, 108.0),
        );
        let (fill, stroke_c, title, body) = if step == 1 {
            (
                egui::Color32::from_rgb(48, 36, 22),
                naranja,
                "Frame: calcular_suma",
                "x = 15, y = 25\nx + y   // sin ';'",
            )
        } else {
            (
                egui::Color32::from_rgb(22, 42, 32),
                egui::Color32::from_rgb(80, 200, 120),
                "Retorno → main",
                "valor = 40\nframe desapilado",
            )
        };
        painter.rect(
            sub_rect,
            6.0,
            fill,
            egui::Stroke::new(2.0, stroke_c),
            egui::StrokeKind::Middle,
        );
        painter.text(
            sub_rect.left_top() + egui::vec2(12.0, 10.0),
            egui::Align2::LEFT_TOP,
            title,
            egui::FontId::proportional(13.0),
            stroke_c,
        );
        painter.text(
            sub_rect.center() + egui::vec2(0.0, 8.0),
            egui::Align2::CENTER_CENTER,
            body,
            egui::FontId::monospace(13.0),
            egui::Color32::WHITE,
        );

        let a = main_rect.right_center();
        let b = sub_rect.left_center();
        painter.line_segment([a, b], egui::Stroke::new(2.0, naranja));
        // Flechita simple hacia el frame hijo / retorno
        let tip = if step == 1 { b } else { a };
        let dir = if step == 1 { -1.0 } else { 1.0 };
        painter.line_segment(
            [
                tip,
                egui::pos2(tip.x + 8.0 * dir, tip.y - 5.0),
            ],
            egui::Stroke::new(2.0, naranja),
        );
        painter.line_segment(
            [
                tip,
                egui::pos2(tip.x + 8.0 * dir, tip.y + 5.0),
            ],
            egui::Stroke::new(2.0, naranja),
        );
    }

    let caption = match step {
        0 => "Solo main está en la pila.",
        1 => "Se apila calcular_suma; main espera el retorno.",
        _ => "Se desapila el frame hijo; main recibe 40.",
    };
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(caption)
            .small()
            .color(egui::Color32::from_rgb(140, 150, 165)),
    );
}

