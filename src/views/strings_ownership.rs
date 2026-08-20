use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::comenzando::mostrar_selector_proyectos_estandar;

pub fn mostrar_tutorial_strings_ownership(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Memoria & Ownership")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Barra de navegación con el mismo patrón unificado que Comenzando
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );

        let tabs_practica = [
            (0, "Stack vs Heap"),
            (1, "Ownership & Move"),
            (2, "Copy vs Clone"),
            (3, "Borrowing (& y &mut)"),
            (4, "String vs &str"),
        ];
        for (indice, texto) in tabs_practica {
            let es_activo = state.strings_ownership_tab == indice;
            let color = if es_activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(texto).strong().color(color))
                        .frame(es_activo),
                )
                .clicked()
            {
                state.strings_ownership_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let es_activo = state.strings_ownership_tab == 5;
            let color = if es_activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("Simulador Memoria")
                            .strong()
                            .color(color),
                    )
                    .frame(es_activo),
                )
                .clicked()
            {
                state.strings_ownership_tab = 5;
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
    ui.add_space(10.0);

    // El selector de proyectos y el editor de código interactivo en primera posición para las pestañas prácticas
    if state.strings_ownership_tab < 5 {
        mostrar_selector_proyectos_estandar(
            ui,
            &mut state.selected_project,
            &mut state.term_cwd,
            "combo_proyectos_strings_ownership",
            &mut state.ownership_code,
        );

        ui.add_space(10.0);

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

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(12.0);
    }

    match state.strings_ownership_tab {
        0 => {
            ui.label(
                "Rust gestiona la memoria dividiendo las asignaciones entre la Pila (Stack) y el Montículo (Heap). Comprender cómo interactúan ambas regiones es el fundamento del modelo de Ownership sin Garbage Collector.",
            );
            ui.add_space(10.0);

            // Tabla Comparativa: Stack vs Heap
            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_stack_vs_heap")
                    .striped(true)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Aspecto").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Stack (Pila)").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Heap (Montículo)").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Diferencia Técnica").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        // Fila 1: Tamaño
                        ui.label(egui::RichText::new("Tamaño de Datos").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Fijo y conocido en compilación");
                        ui.label("Dinámico o variable en ejecución");
                        ui.label("Stack no puede almacenar datos de longitud desconocida.");
                        ui.end_row();

                        // Fila 2: Velocidad
                        ui.label(egui::RichText::new("Velocidad de Acceso").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Ultra rápida (Costo cero de asignación)");
                        ui.label("Más lenta (Búsqueda de dirección + puntero)");
                        ui.label("Stack solo mueve el puntero de pila (Stack Pointer).");
                        ui.end_row();

                        // Fila 3: Organización
                        ui.label(egui::RichText::new("Estructura de Memoria").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("LIFO (Last In, First Out)");
                        ui.label("Bloques dispersos asignados por el OS");
                        ui.label("Heap requiere punteros en Stack que guarden su dirección.");
                        ui.end_row();

                        // Fila 4: Liberación
                        ui.label(egui::RichText::new("Limpieza").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Automática al salir del scope");
                        ui.label("Controlada por el dueño del puntero");
                        ui.label("Rust ejecuta drop() cuando el dueño en Stack sale de scope.");
                        ui.end_row();
                    });
            });

            ui.add_space(14.0);

            // Dos Columnas: Stack vs Heap
            ui.columns(2, |cols| {
                // Columna Izquierda: Stack
                let mut stack_frame = egui::Frame::new();
                stack_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                stack_frame.inner_margin = egui::Margin::same(12);
                stack_frame.corner_radius = egui::CornerRadius::same(8);
                stack_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                stack_frame.show(&mut cols[0], |ui| {
                    ui.label(
                        egui::RichText::new("La Pila (Stack)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Almacena variables locales cuyo tamaño exacto en bytes se conoce al compilar:",
                    );
                    ui.add_space(4.0);
                    ui.label("• Enteros, booleanos, flotantes, caracteres y arrays fijos.");
                    ui.label("• Punteros a datos dinámicos (dirección, longitud y capacidad).");
                    ui.label("• Asignar y desasignar solo requiere mover un registro de CPU.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("fn main() {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.indent("stack_code_inner", |ui| {
                            ui.label(egui::RichText::new("let a: i32 = 42; // 4 bytes en Stack").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label(egui::RichText::new("let arr: [u8; 3] = [1, 2, 3]; // 3 bytes en Stack").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        });
                        ui.label(egui::RichText::new("} // Memoria liberada al instante").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });

                // Columna Derecha: Heap
                let mut heap_frame = egui::Frame::new();
                heap_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                heap_frame.inner_margin = egui::Margin::same(12);
                heap_frame.corner_radius = egui::CornerRadius::same(8);
                heap_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                heap_frame.show(&mut cols[1], |ui| {
                    ui.label(
                        egui::RichText::new("El Montículo (Heap)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Almacena estructuras de datos dinámicas que pueden crecer en tiempo de ejecución:",
                    );
                    ui.add_space(4.0);
                    ui.label("• Strings modificables (`String`), vectores (`Vec<T>`), Boxes.");
                    ui.label("• El sistema operativo busca un bloque contiguo libre y devuelve un puntero.");
                    ui.label("• Al salir de scope, el dueño en Stack libera el bloque en Heap automáticamente.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("fn main() {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.indent("heap_code_inner", |ui| {
                            ui.label(egui::RichText::new("let mut s = String::from(\"Hola\"); // Buffer en Heap").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label(egui::RichText::new("s.push_str(\" Rust\"); // Crece dinamicamente").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        });
                        ui.label(egui::RichText::new("} // drop() libera el buffer en Heap").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });
            });
        }
        1 => {
            ui.label(
                "Ownership es el sistema central de seguridad de memoria de Rust. Se rige por tres reglas simples verificadas en tiempo de compilación para garantizar cero fugas de memoria y prevenir el error de doble liberación (double free).",
            );
            ui.add_space(10.0);

            // Tabla Comparativa: Reglas de Ownership
            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_reglas_ownership")
                    .striped(true)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Regla de Oro").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis en Código").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Comportamiento en Memoria").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Garantía de Seguridad").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        // Fila 1: Un solo dueño
                        ui.label(egui::RichText::new("1. Propietario Único").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("let s1 = String::from(\"a\");").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Cada valor tiene una única variable dueña.");
                        ui.label("Evita punteros colgantes o compartición insegura.");
                        ui.end_row();

                        // Fila 2: Move
                        ui.label(egui::RichText::new("2. Transferencia (Move)").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("let s2 = s1; // s1 queda inválido").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("El nuevo dueño es s2. s1 no puede volver a usarse.");
                        ui.label("Previene double free al salir de scope.");
                        ui.end_row();

                        // Fila 3: Drop
                        ui.label(egui::RichText::new("3. Liberación (Drop)").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("} // fin del bloque léxico").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Al salir del scope, el valor se destruye automáticamente.");
                        ui.label("Cero fugas de memoria (memory leaks).");
                        ui.end_row();
                    });
            });

            ui.add_space(14.0);

            // Dos Columnas: Move vs Drop
            ui.columns(2, |cols| {
                // Columna Izquierda: Move
                let mut move_frame = egui::Frame::new();
                move_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                move_frame.inner_margin = egui::Margin::same(12);
                move_frame.corner_radius = egui::CornerRadius::same(8);
                move_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                move_frame.show(&mut cols[0], |ui| {
                    ui.label(
                        egui::RichText::new("Transferencia de Propiedad (Move)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Al asignar una variable de Heap o pasarla a una función, Rust transfiere su titularidad:",
                    );
                    ui.add_space(4.0);
                    ui.label("• Copia únicamente el puntero, len y cap en el Stack (operación instantánea).");
                    ui.label("• La variable de origen se invalida de inmediato en tiempo de compilación.");
                    ui.label("• No se duplica el contenido en el Heap.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("let s1 = String::from(\"datos\");").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("let s2 = s1; // Se transfiere la propiedad a s2").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("// println!(\"{s1}\"); // Error: uso de valor movido").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                        ui.label(egui::RichText::new("println!(\"{s2}\"); // Valido").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });

                // Columna Derecha: Drop
                let mut drop_frame = egui::Frame::new();
                drop_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                drop_frame.inner_margin = egui::Margin::same(12);
                drop_frame.corner_radius = egui::CornerRadius::same(8);
                drop_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                drop_frame.show(&mut cols[1], |ui| {
                    ui.label(
                        egui::RichText::new("Liberación Automática (RAII / Drop)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Rust no necesita Garbage Collector porque sabe exactamente cuándo destruir cada dato:",
                    );
                    ui.add_space(4.0);
                    ui.label("• Al alcanzar la llave de cierre '}', el dueño del dato sale de su ámbito.");
                    ui.label("• El compilador inserta automáticamente una llamada a la función drop().");
                    ui.label("• Los recursos de memoria, archivos o sockets se liberan en ese milisegundo exacto.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("{").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.indent("drop_code_inner", |ui| {
                            ui.label(egui::RichText::new("let archivo = String::from(\"buffer\");").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label(egui::RichText::new("// Se utiliza archivo dentro de este bloque").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                        });
                        ui.label(egui::RichText::new("} // drop(archivo) se ejecuta automaticamente aqui").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });
            });
        }
        2 => {
            ui.label(
                "Para evitar transferir la propiedad (Move) cuando se necesita conservar el valor original, Rust ofrece dos mecanismos: la copia automática en Stack (Copy) y la duplicación explícita en Heap (Clone).",
            );
            ui.add_space(10.0);

            // Tabla Comparativa: Copy vs Clone
            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_copy_vs_clone")
                    .striped(true)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Mecanismo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ubicación Memoria").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Costo en Rendimiento").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Tipos de Datos").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        // Fila 1: Copy
                        ui.label(egui::RichText::new("Copy").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("let y = x;").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Solo en Stack");
                        ui.label("Cero (copia bit a bit de pocos bytes)");
                        ui.label("Enteros, booleanos, floats, chars, tuplas Copy.");
                        ui.end_row();

                        // Fila 2: Clone
                        ui.label(egui::RichText::new("Clone").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("let y = x.clone();").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Stack + Reserva en Heap");
                        ui.label("Alto (pide nueva memoria dinámica al OS)");
                        ui.label("String, Vec<T>, estructuras complejas.");
                        ui.end_row();
                    });
            });

            ui.add_space(14.0);

            // Dos Columnas: Copy vs Clone en profundidad
            ui.columns(2, |cols| {
                // Columna Izquierda: Copy
                let mut copy_frame = egui::Frame::new();
                copy_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                copy_frame.inner_margin = egui::Margin::same(12);
                copy_frame.corner_radius = egui::CornerRadius::same(8);
                copy_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                copy_frame.show(&mut cols[0], |ui| {
                    ui.label(
                        egui::RichText::new("El Trait Copy (Stack)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Se aplica a tipos que viven exclusivamente en la pila y tienen tamaño fijo:",
                    );
                    ui.add_space(4.0);
                    ui.label("• La copia ocurre de forma automática e invisible al asignar o pasar a función.");
                    ui.label("• La variable original NUNCA se invalida.");
                    ui.label("• No puede implementarse en tipos que gestionan recursos en el Heap.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("let a = 10;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("let b = a; // Copia automatica de bits en Stack").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("println!(\"a: {a}, b: {b}\"); // Ambos son validos").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });

                // Columna Derecha: Clone
                let mut clone_frame = egui::Frame::new();
                clone_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                clone_frame.inner_margin = egui::Margin::same(12);
                clone_frame.corner_radius = egui::CornerRadius::same(8);
                clone_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                clone_frame.show(&mut cols[1], |ui| {
                    ui.label(
                        egui::RichText::new("El Trait Clone (Heap Deep Copy)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Duplica por completo los datos asignando un nuevo bloque de memoria en el Heap:",
                    );
                    ui.add_space(4.0);
                    ui.label("• Exige invocar explícitamente el método .clone() para ser consciente del costo.");
                    ui.label("• Ambas variables conservan su independencia y su propio ciclo de vida.");
                    ui.label("• Usar solo cuando realmente se requieran dos copias independientes de los datos.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("let s1 = String::from(\"original\");").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("let s2 = s1.clone(); // Reserva nuevo espacio en Heap").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("println!(\"s1: {s1}, s2: {s2}\"); // Ambos validos").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });
            });
        }
        3 => {
            ui.label(
                "Borrowing (Préstamo) permite acceder y operar sobre los datos sin transferir su propiedad mediante referencias (&). El Borrow Checker verifica en compilación que nunca existan condiciones de carrera (data races).",
            );
            ui.add_space(10.0);

            // Tabla Comparativa: Borrowing
            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_borrowing_referencias")
                    .striped(true)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Tipo de Préstamo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Permisos").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Cantidad Simultánea").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Regla de Seguridad").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        // Fila 1: Inmutable
                        ui.label(egui::RichText::new("Inmutable (&T)").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("let r = &s;").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Solo Lectura");
                        ui.label("Ilimitadas referencias");
                        ui.label("Múltiples lectores pueden observar los datos a la vez.");
                        ui.end_row();

                        // Fila 2: Mutable
                        ui.label(egui::RichText::new("Mutable (&mut T)").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("let r = &mut s;").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Lectura y Escritura");
                        ui.label("Exactamente UNA sola");
                        ui.label("Acceso exclusivo mientras vive el préstamo.");
                        ui.end_row();

                        // Fila 3: Concurrencia segura
                        ui.label(egui::RichText::new("Exclusividad").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("& y &mut a la vez").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Conflicto prohibido");
                        ui.label("0 referencias mutables si hay lectores");
                        ui.label("Previene lecturas inconsistentes y data races.");
                        ui.end_row();
                    });
            });

            ui.add_space(14.0);

            // Dos Columnas: &T vs &mut T
            ui.columns(2, |cols| {
                // Columna Izquierda: Referencias Inmutables
                let mut imm_frame = egui::Frame::new();
                imm_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                imm_frame.inner_margin = egui::Margin::same(12);
                imm_frame.corner_radius = egui::CornerRadius::same(8);
                imm_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                imm_frame.show(&mut cols[0], |ui| {
                    ui.label(
                        egui::RichText::new("Referencias Inmutables (&T)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Pasa datos a funciones o inspecciona estructuras sin ceder la propiedad:",
                    );
                    ui.add_space(4.0);
                    ui.label("• El valor original sigue perteneciendo a su variable declarada.");
                    ui.label("• Puedes crear tantas referencias de solo lectura como desees.");
                    ui.label("• No puedes modificar el dato a través de la referencia.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("fn calcular_len(s: &String) -> usize {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.indent("imm_code_inner", |ui| {
                            ui.label(egui::RichText::new("s.len() // Lectura del valor sin moverlo").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        });
                        ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });

                // Columna Derecha: Referencias Mutables
                let mut mut_frame = egui::Frame::new();
                mut_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                mut_frame.inner_margin = egui::Margin::same(12);
                mut_frame.corner_radius = egui::CornerRadius::same(8);
                mut_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                mut_frame.show(&mut cols[1], |ui| {
                    ui.label(
                        egui::RichText::new("Referencias Mutables (&mut T)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Permite modificar un valor ajeno de forma controlada y segura:",
                    );
                    ui.add_space(4.0);
                    ui.label("• La variable base debe haberse declarado con 'mut'.");
                    ui.label("• Solo puede haber UNA referencia mutable activa a la vez.");
                    ui.label("• Mientras exista '&mut T', no se permiten otras lecturas '&T'.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("let mut texto = String::from(\"Hola\");").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("let ref_mut = &mut texto;").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("ref_mut.push_str(\" Mundo\"); // Modificacion exclusiva").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("println!(\"{texto}\"); // Imprime: Hola Mundo").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });
            });
        }
        4 => {
            ui.label(
                "En Rust existen dos formas principales de representar texto: el tipo propietario dinámico 'String' y la vista prestada de solo lectura '&str' (string slice). Comprender su diferencia optimiza el uso de memoria.",
            );
            ui.add_space(10.0);

            // Tabla Comparativa: String vs &str
            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_string_vs_str")
                    .striped(true)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Tipo de Texto").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("¿Dónde reside?").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Mutabilidad").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Estructura en Stack").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Cuándo utilizarlo").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        // Fila 1: &str
                        ui.label(egui::RichText::new("&str (Slice)").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Binario o buffer prestado");
                        ui.label("Inmutable");
                        ui.label("Puntero + Longitud (16 bytes)");
                        ui.label("Parámetros de función y lectura eficiente.");
                        ui.end_row();

                        // Fila 2: String
                        ui.label(egui::RichText::new("String (Heap)").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Memoria dinámica (Heap)");
                        ui.label("Modificable (con mut)");
                        ui.label("Puntero + Longitud + Capacidad (24 bytes)");
                        ui.label("Construcción, concatenación y propiedad.");
                        ui.end_row();

                        // Fila 3: Deref Coercion
                        ui.label(egui::RichText::new("&String -> &str").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Conversión automática");
                        ui.label("Inmutable");
                        ui.label("Deref Coercion instantánea");
                        ui.label("Pasar &mi_string a funciones que piden &str.");
                        ui.end_row();
                    });
            });

            ui.add_space(14.0);

            // Dos Columnas: String vs &str
            ui.columns(2, |cols| {
                // Columna Izquierda: String
                let mut string_frame = egui::Frame::new();
                string_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                string_frame.inner_margin = egui::Margin::same(12);
                string_frame.corner_radius = egui::CornerRadius::same(8);
                string_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                string_frame.show(&mut cols[0], |ui| {
                    ui.label(
                        egui::RichText::new("String (Dueño en Heap)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Es un vector de bytes UTF-8 dinámico con capacidad de crecimiento:",
                    );
                    ui.add_space(4.0);
                    ui.label("• Es dueño exclusivo de su buffer de texto en el Heap.");
                    ui.label("• Permite añadir texto con push_str() o push().");
                    ui.label("• Se crea con String::from(\"...\") o \"...\".to_string().");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("let mut saludo = String::from(\"Hola\");").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("saludo.push_str(\" Rust!\"); // Modifica en Heap").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label(egui::RichText::new("println!(\"{saludo}\");").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });

                // Columna Derecha: &str
                let mut str_frame = egui::Frame::new();
                str_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                str_frame.inner_margin = egui::Margin::same(12);
                str_frame.corner_radius = egui::CornerRadius::same(8);
                str_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                str_frame.show(&mut cols[1], |ui| {
                    ui.label(
                        egui::RichText::new("&str (String Slice)")
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        "Es una referencia ligera (puntero + longitud) que observa una secuencia de bytes UTF-8:",
                    );
                    ui.add_space(4.0);
                    ui.label("• Los literales de código \"Hola\" son de tipo &'static str (viven en el binario).");
                    ui.label("• Puede apuntar a un trozo o al total de un String.");
                    ui.label("• Es el tipo idiomático recomendado para parámetros de función.");
                    ui.add_space(8.0);

                    // Contenedor de Código estilo IDE
                    let mut code_box = egui::Frame::new();
                    code_box.fill = egui::Color32::from_rgb(8, 12, 18);
                    code_box.inner_margin = egui::Margin::same(10);
                    code_box.corner_radius = egui::CornerRadius::same(6);
                    code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

                    code_box.show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("fn imprimir_texto(vista: &str) {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.indent("str_code_inner", |ui| {
                            ui.label(egui::RichText::new("println!(\"Longitud: {}\", vista.len());").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        });
                        ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                    });
                });
            });
        }
        _ => {
            ui.label(
                "Visualiza gráficamente cómo se representan los datos en la Pila (Stack) y el Montículo (Heap). Observa cómo MOVE transfiere la titularidad y cómo BORROW añade referencias seguras sin duplicar la memoria.",
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Paso de Ejecución:").strong().color(gris_tab));
                ui.add_space(6.0);
                for (i, label) in [
                    (0, "1. Asignación s1 = String"),
                    (1, "2. MOVE: s2 = s1"),
                    (2, "3. BORROW: s3 = &s2"),
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
}

/// Simulador stack/heap para MOVE y BORROW con paleta unificada.
pub fn mostrar_simulador_ownership_memoria(ui: &mut egui::Ui, step: usize) {
    let azul_codigo = egui::Color32::from_rgb(100, 200, 255);
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let gris_invalido = egui::Color32::from_rgb(120, 130, 145);
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
        egui::Color32::from_rgb(18, 24, 34),
        egui::Stroke::new(1.5, azul_codigo),
        egui::StrokeKind::Middle,
    );
    painter.text(
        stack_rect.left_top() + egui::vec2(12.0, 10.0),
        egui::Align2::LEFT_TOP,
        "STACK (Pila)",
        egui::FontId::proportional(13.0),
        azul_codigo,
    );

    painter.rect(
        heap_rect,
        6.0,
        egui::Color32::from_rgb(28, 22, 14),
        egui::Stroke::new(1.5, naranja),
        egui::StrokeKind::Middle,
    );
    painter.text(
        heap_rect.left_top() + egui::vec2(12.0, 10.0),
        egui::Align2::LEFT_TOP,
        "HEAP (Montículo)",
        egui::FontId::proportional(13.0),
        naranja,
    );

    let heap_data = heap_rect.center() + egui::vec2(0.0, 12.0);
    painter.circle_filled(heap_data, 30.0, egui::Color32::from_rgb(45, 30, 15));
    painter.circle_stroke(heap_data, 30.0, egui::Stroke::new(2.0, naranja));
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
                egui::Color32::from_rgb(20, 35, 55),
                egui::Stroke::new(1.5, azul_codigo),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s1,
                egui::Align2::CENTER_CENTER,
                "s1 (dueño activo)",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s1 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(30.0, 0.0)],
                egui::Stroke::new(2.0, azul_codigo),
            );
        }
        1 => {
            let s1 = slot(55.0);
            let s2 = slot(115.0);
            painter.rect(
                egui::Rect::from_center_size(s1, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(18, 22, 30),
                egui::Stroke::new(1.0, gris_invalido),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s1,
                egui::Align2::CENTER_CENTER,
                "s1 (inválido - movido)",
                egui::FontId::proportional(12.0),
                gris_invalido,
            );
            painter.rect(
                egui::Rect::from_center_size(s2, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(20, 35, 55),
                egui::Stroke::new(1.5, naranja),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s2,
                egui::Align2::CENTER_CENTER,
                "s2 (nuevo dueño)",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s2 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(30.0, 0.0)],
                egui::Stroke::new(2.0, naranja),
            );
        }
        _ => {
            let s2 = slot(55.0);
            let s3 = slot(115.0);
            painter.rect(
                egui::Rect::from_center_size(s2, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(20, 35, 55),
                egui::Stroke::new(1.5, naranja),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s2,
                egui::Align2::CENTER_CENTER,
                "s2 (dueño)",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.rect(
                egui::Rect::from_center_size(s3, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(20, 35, 55),
                egui::Stroke::new(1.5, azul_codigo),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s3,
                egui::Align2::CENTER_CENTER,
                "s3 = &s2 (préstamo)",
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s2 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(30.0, 0.0)],
                egui::Stroke::new(2.0, naranja),
            );
            painter.line_segment(
                [s3 + egui::vec2(0.0, -17.0), s2 + egui::vec2(0.0, 17.0)],
                egui::Stroke::new(2.0, azul_codigo),
            );
        }
    }

    let caption = match step {
        0 => "s1 en el Stack apunta al buffer \"Hola\" en el Heap.",
        1 => "MOVE: la titularidad se transfiere a s2; usar s1 produce error de compilación.",
        _ => "BORROW: s3 toma prestada una referencia a s2; el dueño sigue siendo s2.",
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
            .strong()
            .color(egui::Color32::from_rgb(255, 160, 50)),
    );
    ui.add_space(10.0);
    ui.label("Presiona 'Ejecutar Siguiente Línea' para ver cómo el compilador asigna la memoria.");
    ui.add_space(20.0);

    ui.columns(2, |columns| {
        // --- COLUMNA 1: EDITOR DE CÓDIGO ---
        let mut code_frame = egui::Frame::new();
        code_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        code_frame.inner_margin = egui::Margin::same(12);
        code_frame.corner_radius = egui::CornerRadius::same(8);
        code_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        code_frame.show(&mut columns[0], |ui| {
            ui.label(
                egui::RichText::new("Editor de Código")
                    .strong()
                    .color(egui::Color32::WHITE),
            );
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
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };
                ui.label(
                    egui::RichText::new(*line)
                        .color(color)
                        .monospace()
                        .size(16.0),
                );
            }

            ui.add_space(20.0);
            if ui
                .button(egui::RichText::new("Ejecutar Siguiente Línea").strong())
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
            egui::Color32::from_rgb(18, 24, 34),
            egui::Stroke::new(1.5, egui::Color32::from_rgb(100, 200, 255)),
            egui::StrokeKind::Middle,
        );
        painter.text(
            stack_rect.center_top() - egui::vec2(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "STACK",
            egui::FontId::proportional(16.0),
            egui::Color32::from_rgb(100, 200, 255),
        );

        painter.rect(
            heap_rect,
            5.0,
            egui::Color32::from_rgb(28, 22, 14),
            egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 160, 50)),
            egui::StrokeKind::Middle,
        );
        painter.text(
            heap_rect.center_top() - egui::vec2(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "HEAP",
            egui::FontId::proportional(16.0),
            egui::Color32::from_rgb(255, 160, 50),
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
                egui::Color32::from_rgb(20, 35, 55),
                egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255)),
                egui::StrokeKind::Middle,
            );
            painter.text(
                var_a_rect.center(),
                egui::Align2::CENTER_CENTER,
                "a: i32 = 42",
                egui::FontId::monospace(15.0),
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
                egui::Color32::from_rgb(20, 35, 55),
                egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255)),
                egui::StrokeKind::Middle,
            );
            painter.text(
                var_s_rect.center(),
                egui::Align2::CENTER_CENTER,
                "s (String)\nptr: 0x...",
                egui::FontId::monospace(13.0),
                egui::Color32::WHITE,
            );

            let heap_data_rect = egui::Rect::from_min_size(
                heap_rect.min + egui::vec2(30.0, 150.0 + float_y),
                egui::vec2(160.0, 50.0),
            );
            painter.rect(
                heap_data_rect,
                8.0,
                egui::Color32::from_rgb(45, 30, 15),
                egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 160, 50)),
                egui::StrokeKind::Middle,
            );
            painter.text(
                heap_data_rect.center(),
                egui::Align2::CENTER_CENTER,
                "['H','o','l','a']",
                egui::FontId::monospace(15.0),
                egui::Color32::WHITE,
            );

            let start = var_s_rect.right_center();
            let end = heap_data_rect.left_center();
            let control1 = start + egui::vec2(50.0, 0.0);
            let control2 = end - egui::vec2(50.0, 0.0);

            painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
                [start, control1, control2, end],
                false,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 160, 50)),
            ));
            painter.circle_filled(end, 5.0, egui::Color32::from_rgb(255, 160, 50));
        }
    });
}

