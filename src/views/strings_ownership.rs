use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::comenzando::mostrar_selector_proyectos_estandar;
use crate::views::control_flujo::card_frame_tutorial;

pub fn mostrar_tutorial_strings_ownership(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Strings & Ownership")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Tabs al estilo Comenzando / Funciones
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Conceptos:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );

        let tabs = [
            (0, "String vs &str"),
            (1, "Ownership"),
            (2, "Borrowing"),
        ];
        for (indice, label) in tabs {
            let activo = state.strings_ownership_tab == indice;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.strings_ownership_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.strings_ownership_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Stack / Heap").strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.strings_ownership_tab = 3;
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

    match state.strings_ownership_tab {
        0 => {
            ui.label(
                egui::RichText::new(
                    "En Rust hay dos caras del texto: el dueño que puede crecer (`String`) y la \
                     vista de solo lectura (`&str`). Entenderlas es el puente hacia Ownership.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_string_vs_str")
                    .striped(true)
                    .spacing([18.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Tipo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("¿Dónde vive?").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Cuándo usarlo").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("&str").monospace().strong().color(naranja));
                        ui.label("Binario y/o préstamo (vista)");
                        ui.label(egui::RichText::new("let s: &str = \"Hola\";").monospace().color(cyan));
                        ui.label("Leer o pasar texto sin regalar el dueño.");
                        ui.end_row();

                        ui.label(egui::RichText::new("String").monospace().strong().color(naranja));
                        ui.label("Heap (propietario)");
                        ui.label(egui::RichText::new("String::from(\"Hola\")").monospace().color(cyan));
                        ui.label("Crear, modificar, crecer (`push_str`).");
                        ui.end_row();

                        ui.label(egui::RichText::new("&String → &str").monospace().strong().color(naranja));
                        ui.label("Deref coercion");
                        ui.label(egui::RichText::new("let v: &str = &mi_string;").monospace().color(cyan));
                        ui.label("Casi siempre las APIs piden `&str`, no `&String`.");
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(
                    "Regla práctica: dueño y mutable → `String`. Solo mirar o firmar funciones → `&str`.",
                )
                .italics()
                .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        }
        1 => {
            ui.label(
                egui::RichText::new(
                    "Ownership es el modelo de memoria de Rust: cada valor tiene un dueño; al moverse, \
                     el dueño anterior deja de valer; al salir del scope, se libera (`drop`). \
                     `String` en el heap lo hace visible (a diferencia de un `i32` que se copia).",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_ownership_reglas")
                    .striped(true)
                    .spacing([18.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Regla").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("En código").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Significado").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("1. Un dueño").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let s1 = String::from(\"a\");").monospace().color(cyan));
                        ui.label("Cada valor tiene una variable propietaria.");
                        ui.end_row();

                        ui.label(egui::RichText::new("2. Move").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let s2 = s1; // s1 inválido").monospace().color(cyan));
                        ui.label("Solo un dueño a la vez; asignar mueve (no copia el heap).");
                        ui.end_row();

                        ui.label(egui::RichText::new("3. Drop").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("} // fin de scope").monospace().color(cyan));
                        ui.label("Al salir el dueño, Rust libera la memoria automáticamente.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Copy").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let b = a; // i32 sí copia").monospace().color(cyan));
                        ui.label("Tipos chicos en stack (`i32`, `bool`…) se copian; no se invalidan.");
                        ui.end_row();
                    });
            });
        }
        2 => {
            ui.label(
                egui::RichText::new(
                    "Borrowing = prestar sin regalar el dueño. `&T` (lectura, muchas a la vez) o \
                     `&mut T` (escritura, solo una). Nunca ambas al mismo tiempo sobre el mismo dato.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_borrowing")
                    .striped(true)
                    .spacing([18.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Préstamo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Regla").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Inmutable").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let r = &s;").monospace().color(cyan));
                        ui.label("Muchas `&T` simultáneas; no puedes mutar por ellas.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Mutable").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let r = &mut s;").monospace().color(cyan));
                        ui.label("Solo una `&mut T`; exclusivo mientras vive el préstamo.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Exclusividad").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("& y &mut a la vez → error").monospace().color(cyan));
                        ui.label("El borrow checker evita data races en compilación.");
                        ui.end_row();

                        ui.label(egui::RichText::new("API típica").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("fn f(s: &str)").monospace().color(cyan));
                        ui.label("Pides vista; el caller sigue siendo dueño del `String`.");
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.label(
                egui::RichText::new(
                    "Visualiza el heap con el texto real y el stack con el dueño (puntero, len, cap). \
                     El MOVE cambia quién apunta; el BORROW añade una referencia sin quitar el dueño.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Paso:").strong().color(gris_tab));
                ui.add_space(6.0);
                for (i, label) in [
                    (0, "1. s1 = String"),
                    (1, "2. MOVE s2 = s1"),
                    (2, "3. BORROW &s2"),
                ] {
                    let activo = state.ownership_step == i;
                    let color = if activo { naranja } else { gris_tab };
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(label).strong().color(color))
                                .frame(activo),
                        )
                        .clicked()
                    {
                        state.ownership_step = i;
                    }
                    ui.add_space(4.0);
                }
            });

            ui.add_space(12.0);
            mostrar_simulador_ownership_memoria(ui, state.ownership_step);
        }
    }

    // Editor en tabs de concepto; en visual también útil probar el código
    ui.add_space(15.0);
    if state.strings_ownership_tab < 3 {
        mostrar_selector_proyectos_estandar(
            ui,
            &mut state.selected_project,
            &mut state.term_cwd,
            "combo_proyectos_strings_ownership",
            &mut state.ownership_code,
        );
        ui.add_space(10.0);
    }
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.ownership_code,
        Arc::clone(&state.ownership_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

/// Simulador stack/heap para MOVE y BORROW (tema del curso).

pub fn mostrar_simulador_ownership_memoria(ui: &mut egui::Ui, step: usize) {
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let verde = egui::Color32::from_rgb(80, 200, 120);
    let rojo = egui::Color32::from_rgb(220, 100, 100);
    let morado = egui::Color32::from_rgb(180, 140, 255);
    let canvas_bg = egui::Color32::from_rgb(14, 18, 26);
    let border = egui::Color32::from_rgb(45, 60, 90);

    let height = 220.0;
    let width = ui.available_width().min(780.0);
    let (_, rect) = ui.allocate_space(egui::vec2(width, height));
    let painter = ui.painter_at(rect);

    painter.rect(
        rect,
        8.0,
        canvas_bg,
        egui::Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );

    let stack_rect = egui::Rect::from_min_size(
        egui::pos2(rect.left() + 24.0, rect.top() + 28.0),
        egui::vec2(240.0, 168.0),
    );
    let heap_rect = egui::Rect::from_min_size(
        egui::pos2(rect.left() + 360.0, rect.top() + 28.0),
        egui::vec2(240.0, 168.0),
    );

    painter.rect(
        stack_rect,
        6.0,
        egui::Color32::from_rgb(22, 28, 40),
        egui::Stroke::new(1.5, cyan),
        egui::StrokeKind::Middle,
    );
    painter.text(
        stack_rect.left_top() + egui::vec2(12.0, 10.0),
        egui::Align2::LEFT_TOP,
        "STACK",
        egui::FontId::proportional(13.0),
        cyan,
    );

    painter.rect(
        heap_rect,
        6.0,
        egui::Color32::from_rgb(22, 36, 30),
        egui::Stroke::new(1.5, verde),
        egui::StrokeKind::Middle,
    );
    painter.text(
        heap_rect.left_top() + egui::vec2(12.0, 10.0),
        egui::Align2::LEFT_TOP,
        "HEAP  \"Hola\"",
        egui::FontId::proportional(13.0),
        verde,
    );

    let heap_data = heap_rect.center() + egui::vec2(0.0, 12.0);
    painter.circle_filled(heap_data, 28.0, egui::Color32::from_rgb(36, 90, 60));
    painter.circle_stroke(heap_data, 28.0, egui::Stroke::new(2.0, verde));
    painter.text(
        heap_data,
        egui::Align2::CENTER_CENTER,
        "\"Hola\"",
        egui::FontId::monospace(13.0),
        egui::Color32::WHITE,
    );

    let slot = |y: f32| egui::pos2(stack_rect.center().x, stack_rect.top() + y);

    match step {
        0 => {
            let s1 = slot(70.0);
            painter.rect(
                egui::Rect::from_center_size(s1, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(28, 48, 72),
                egui::Stroke::new(1.5, cyan),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s1,
                egui::Align2::CENTER_CENTER,
                "s1  dueño activo",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s1 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(28.0, 0.0)],
                egui::Stroke::new(2.0, cyan),
            );
        }
        1 => {
            let s1 = slot(55.0);
            let s2 = slot(115.0);
            painter.rect(
                egui::Rect::from_center_size(s1, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(48, 28, 28),
                egui::Stroke::new(1.5, rojo),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s1,
                egui::Align2::CENTER_CENTER,
                "s1  MOVED (inválido)",
                egui::FontId::proportional(12.0),
                rojo,
            );
            painter.rect(
                egui::Rect::from_center_size(s2, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(28, 48, 36),
                egui::Stroke::new(1.5, verde),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s2,
                egui::Align2::CENTER_CENTER,
                "s2  nuevo dueño",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s2 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(28.0, 0.0)],
                egui::Stroke::new(2.0, verde),
            );
        }
        _ => {
            let s2 = slot(55.0);
            let s3 = slot(115.0);
            painter.rect(
                egui::Rect::from_center_size(s2, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(28, 48, 36),
                egui::Stroke::new(1.5, verde),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s2,
                egui::Align2::CENTER_CENTER,
                "s2  dueño",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.rect(
                egui::Rect::from_center_size(s3, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(40, 32, 56),
                egui::Stroke::new(1.5, morado),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s3,
                egui::Align2::CENTER_CENTER,
                "s3 = &s2  préstamo",
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s2 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(28.0, 0.0)],
                egui::Stroke::new(2.0, verde),
            );
            painter.line_segment(
                [s3 + egui::vec2(0.0, -17.0), s2 + egui::vec2(0.0, 17.0)],
                egui::Stroke::new(2.0, morado),
            );
        }
    }

    let caption = match step {
        0 => "s1 en el stack apunta al buffer \"Hola\" en el heap.",
        1 => "MOVE: el dueño pasa a s2; usar s1 sería error de compilación.",
        _ => "BORROW: s3 presta a s2; el dueño sigue siendo s2.",
    };
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(caption)
            .small()
            .color(egui::Color32::from_rgb(140, 150, 165)),
    );
}


pub fn mostrar_tutorial_memoria(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.heading(
        egui::RichText::new("Gestión de Memoria: Stack vs Heap")
            .size(28.0)
            .strong(),
    );
    ui.add_space(10.0);
    ui.label("Presiona 'Ejecutar Siguiente Línea' para ver cómo el compilador asigna la memoria.");
    ui.add_space(20.0);

    ui.columns(2, |columns| {
        // --- COLUMNA 1: EDITOR DE CÓDIGO ---
        columns[0].group(|ui| {
            ui.heading("📝 Editor de Código");
            ui.add_space(15.0);

            let code = [
                "fn main() {",
                "    let a: i32 = 42;",
                "    let s = String::from(\"Hola\");",
                "} // Fin del Scope",
            ];

            for (i, line) in code.iter().enumerate() {
                let is_current = i == state.tutorial_step;
                let color = if is_current {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::LIGHT_GRAY
                };
                ui.label(
                    egui::RichText::new(*line)
                        .color(color)
                        .monospace()
                        .size(18.0),
                );
            }

            ui.add_space(30.0);
            if ui
                .button(egui::RichText::new("▶ Ejecutar Siguiente Línea").size(16.0))
                .clicked()
            {
                state.tutorial_step = (state.tutorial_step + 1) % 4;
            }
        });

        // --- COLUMNA 2: VISUALIZACIÓN DE MEMORIA (epaint) ---
        let (response, painter) = columns[1].allocate_painter(
            egui::vec2(columns[1].available_width(), 450.0),
            egui::Sense::hover(),
        );

        let rect = response.rect;
        let stack_rect =
            egui::Rect::from_min_size(rect.min + egui::vec2(10.0, 40.0), egui::vec2(160.0, 350.0));
        let heap_rect =
            egui::Rect::from_min_size(rect.min + egui::vec2(200.0, 40.0), egui::vec2(220.0, 350.0));

        painter.rect(
            stack_rect,
            5.0,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 250)),
            egui::StrokeKind::Middle,
        );
        painter.text(
            stack_rect.center_top() - egui::vec2(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "STACK",
            egui::FontId::proportional(18.0),
            egui::Color32::LIGHT_BLUE,
        );

        painter.rect(
            heap_rect,
            5.0,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 250, 100)),
            egui::StrokeKind::Middle,
        );
        painter.text(
            heap_rect.center_top() - egui::vec2(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "HEAP",
            egui::FontId::proportional(18.0),
            egui::Color32::LIGHT_GREEN,
        );

        let float_y = (state.tutorial_time * 3.0).sin() as f32 * 5.0;

        if state.tutorial_step >= 1 && state.tutorial_step < 3 {
            let var_a_rect = egui::Rect::from_min_size(
                stack_rect.min + egui::vec2(10.0, 290.0),
                egui::vec2(140.0, 40.0),
            );
            painter.rect(
                var_a_rect,
                4.0,
                egui::Color32::from_rgb(60, 60, 180),
                egui::Stroke::NONE,
                egui::StrokeKind::Middle,
            );
            painter.text(
                var_a_rect.center(),
                egui::Align2::CENTER_CENTER,
                "a: i32 = 42",
                egui::FontId::monospace(16.0),
                egui::Color32::WHITE,
            );
        }

        if state.tutorial_step >= 2 && state.tutorial_step < 3 {
            let var_s_rect = egui::Rect::from_min_size(
                stack_rect.min + egui::vec2(10.0, 200.0),
                egui::vec2(140.0, 70.0),
            );
            painter.rect(
                var_s_rect,
                4.0,
                egui::Color32::from_rgb(200, 150, 50),
                egui::Stroke::NONE,
                egui::StrokeKind::Middle,
            );
            painter.text(
                var_s_rect.center(),
                egui::Align2::CENTER_CENTER,
                "s (String)\nptr: 0x...",
                egui::FontId::monospace(14.0),
                egui::Color32::BLACK,
            );

            let heap_data_rect = egui::Rect::from_min_size(
                heap_rect.min + egui::vec2(30.0, 150.0 + float_y),
                egui::vec2(160.0, 50.0),
            );
            painter.rect(
                heap_data_rect,
                8.0,
                egui::Color32::from_rgb(50, 200, 50),
                egui::Stroke::NONE,
                egui::StrokeKind::Middle,
            );
            painter.text(
                heap_data_rect.center(),
                egui::Align2::CENTER_CENTER,
                "['H','o','l','a']",
                egui::FontId::monospace(16.0),
                egui::Color32::BLACK,
            );

            let start = var_s_rect.right_center();
            let end = heap_data_rect.left_center();
            let control1 = start + egui::vec2(50.0, 0.0);
            let control2 = end - egui::vec2(50.0, 0.0);

            painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
                [start, control1, control2, end],
                false,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(3.0, egui::Color32::YELLOW),
            ));
            painter.circle_filled(end, 6.0, egui::Color32::YELLOW);
        }
    });
}

