use eframe::egui;
use crate::app::PortfolioState;

pub fn mostrar_tutorial_cargo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Pilares de Rust")
                .size(28.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 160, 50)),
        );
    });

    ui.add_space(15.0);

    // Barra de navegación de Sub-Pasos de Pilares
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        for (indice, texto) in ["Entorno de Trabajo", "Conceptos"].iter().enumerate() {
            let es_activo = state.pilares_step == indice;
            let text_color = if es_activo {
                egui::Color32::from_rgb(255, 160, 50)
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            };

            let btn_text = egui::RichText::new(*texto).strong().color(text_color);
            if ui
                .add(egui::Button::new(btn_text).frame(es_activo))
                .clicked()
            {
                state.pilares_step = indice;
            }
            ui.add_space(6.0);
        }
    });
    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.pilares_step {
        0 => mostrar_pilares_entorno_trabajo(ui, state),
        _ => mostrar_pilares_conceptos(ui, state),
    }
}


pub fn mostrar_pilares_entorno_trabajo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Estilo unificado de tarjetas
        let mut card_frame = egui::Frame::new();
        card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        card_frame.inner_margin = egui::Margin::same(12);
        card_frame.corner_radius = egui::CornerRadius::same(8);
        card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        let title_color = egui::Color32::from_rgb(255, 180, 100);
        let text_color = egui::Color32::from_rgb(200, 210, 225);

        // --- FILA 1: NÚCLEO DE CONSTRUCCIÓN Y ECOSISTEMA ---
        ui.heading(
            egui::RichText::new("Núcleo de Construcción y Ecosistema")
                .size(18.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(8.0);

        ui.columns(3, |columns| {
            // Pilar 1: rustc
            card_frame.show(&mut columns[0], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://rustc.svg",
                            include_bytes!("../../diagramas/rustc.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustc").size(17.0).strong().color(title_color));
                    ui.add_space(4.0);
                    if ui.button(egui::RichText::new("🔍 Ver").small().color(egui::Color32::from_rgb(100, 200, 255)))
                        .on_hover_text("Abrir diagrama del pipeline de compilación de rustc")
                        .clicked()
                    {
                        state.show_rustc_compilador_modal = true;
                    }
                });
                ui.label(egui::RichText::new("El Compilador Real").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Traduce tu código Rust (.rs) a código máquina optimizado (ELF/EXE/WASM) usando LLVM.").color(text_color));
                ui.label(egui::RichText::new("• Realiza las verificaciones de seguridad de memoria y el Borrow Checker.").color(text_color));
            });

            // Pilar 2: Cargo
            card_frame.show(&mut columns[1], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://cargo2.svg",
                            include_bytes!("../../diagramas/cargo2.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Cargo").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Orquestador / Manager").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Gestor de proyectos y administrador de paquetes oficial de Rust.").color(text_color));
                ui.label(egui::RichText::new("• Automatiza la descarga de dependencias, compilación y pruebas.").color(text_color));
            });

            // Pilar 3: Crates / crates.io
            card_frame.show(&mut columns[2], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://crates.svg",
                            include_bytes!("../../diagramas/crates.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Crates").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Las Librerías y crates.io").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Una 'Crate' es la unidad de código ejecutable o biblioteca en Rust.").color(text_color));
                ui.label(egui::RichText::new("• crates.io es el registro público mundial donde la comunidad comparte paquetes.").color(text_color));
            });
        });

        ui.add_space(18.0);

        // --- FILA 2: CALIDAD, ESTILO Y DIAGNÓSTICO ---
        ui.heading(
            egui::RichText::new("Calidad, Estilo y Diagnósticos")
                .size(18.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(8.0);

        ui.columns(3, |cols| {
            // 1. Clippy
            card_frame.show(&mut cols[0], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://clippy.svg",
                            include_bytes!("../../diagramas/clippy.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Clippy").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Linter Oficial").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Analiza tu código con más de 650 reglas avanzadas para detectar anti-patrones.").color(text_color));
                ui.label(egui::RichText::new("• Enseña las mejores prácticas del código idiomático en Rust.").color(text_color));
            });

            // 2. Rustfmt
            card_frame.show(&mut cols[1], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://format2.svg",
                            include_bytes!("../../diagramas/format2.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Rustfmt").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Formateador Estándar").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Aplica automáticamente el libro de estilo unificado a todo el proyecto.").color(text_color));
                ui.label(egui::RichText::new("• Elimina discusiones de sangría y espacios en equipos de trabajo.").color(text_color));
            });

            // 3. Error Index
            card_frame.show(&mut cols[2], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://error.svg",
                            include_bytes!("../../diagramas/error.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Error Index").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Enciclopedia de Errores").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Enciclopedia explicativa completa para cada código de error del compilador.").color(text_color));
                ui.label(egui::RichText::new("• Muestra ejemplos de código correcto e incorrecto para aprender del error.").color(text_color));
            });
        });

        ui.add_space(18.0);

        // --- FILA 3: PRODUCTIVIDAD, IDE Y DOCUMENTACIÓN ---
        ui.heading(
            egui::RichText::new("Productividad, IDE y Documentación")
                .size(18.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(8.0);

        ui.columns(3, |cols| {
            // 1. rustup
            card_frame.show(&mut cols[0], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://rustup.svg",
                            include_bytes!("../../diagramas/rustup.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustup").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Administrador de Toolchains").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Administra las versiones de Rust (Stable, Nightly) y la compilación cruzada.").color(text_color));
                ui.label(egui::RichText::new("• Permite añadir objetivos como WebAssembly (wasm32) fácilmente.").color(text_color));
            });

            // 2. rust-analyzer
            card_frame.show(&mut cols[1], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://analyzer.svg",
                            include_bytes!("../../diagramas/analyzer.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rust-analyzer").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Servidor de Lenguaje (LSP)").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Proporciona autocompletado en vivo e inlays de tipos inferidos en tu IDE.").color(text_color));
                ui.label(egui::RichText::new("• Soporta VS Code, Antigravity y Neovim.").color(text_color));
            });

            // 3. rustdoc
            card_frame.show(&mut cols[2], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://doc.svg",
                            include_bytes!("../../diagramas/doc.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustdoc").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Generador de Documentación").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Lee los comentarios de documentación (///) y genera una web HTML completa.").color(text_color));
                ui.label(egui::RichText::new("• Ejecuta doctests automáticamente para garantizar que la documentación funcione.").color(text_color));
            });
        });
    });
}


pub fn mostrar_pilares_proyecto(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.heading(
        egui::RichText::new("Estructura de Proyectos en Rust")
            .size(20.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.label(
        "Al crear un proyecto con Cargo (ej: 'cargo new mi_proyecto'), Rust genera automáticamente la jerarquía de archivos y carpetas estándar.",
    );
    ui.add_space(12.0);

    // Tabla comparativa main.rs vs lib.rs
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(14);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_main_vs_lib_pilares")
            .striped(true)
            .spacing([25.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo de Proyecto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Comando de Creación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Archivo Punto de Entrada")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Ejecutable (src/main.rs)
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Ejecutable (Binario)")
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(4.0);
                    let btn_color_main = if state.show_railroad_modal == Some(2) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_main),
                            )
                            .frame(state.show_railroad_modal == Some(2)),
                        )
                        .on_hover_text("Ver diagrama Railroad de sintaxis (fn main ejecutable)")
                        .clicked()
                    {
                        state.show_railroad_modal = if state.show_railroad_modal == Some(2) { None } else { Some(2) };
                    }
                });
                ui.label(
                    egui::RichText::new("cargo new <nombre>")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("src/main.rs")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Programas independientes con función main() ejecutables por la CPU.");
                ui.end_row();

                // Fila 2: Librería (src/lib.rs)
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Librería (Library)")
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.add_space(4.0);
                    let btn_color_lib = if state.show_railroad_modal == Some(3) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_lib),
                            )
                            .frame(state.show_railroad_modal == Some(3)),
                        )
                        .on_hover_text("Ver diagrama Railroad de sintaxis (librería lib.rs)")
                        .clicked()
                    {
                        state.show_railroad_modal = if state.show_railroad_modal == Some(3) { None } else { Some(3) };
                    }
                });
                ui.label(
                    egui::RichText::new("cargo new <nombre> --lib")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("src/lib.rs")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Reutilización de código y módulos para ser consumidos por otros crates.");
                ui.end_row();
            });
    });

    // Desglose de Template Generado + Imagen 7.png si se ha creado un proyecto
    mostrar_desglose_template_con_imagen(ui, state);
}


pub fn mostrar_pilares_tiempo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.heading(
        egui::RichText::new("Fases de Vida del Código: Compile Time vs Run Time")
            .size(20.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.label("En Rust existe una clara división entre la fase previa de análisis en desarrollo y la ejecución final por la CPU:");
    ui.add_space(12.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(14);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_compilacion_vs_ejecucion_pilares")
            .striped(true)
            .spacing([25.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Aspecto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );

                // Columna 2: Tiempo de Compilación + Botón Ver Diagrama
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Tiempo de Compilación")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    let btn_color_compile = if state.show_railroad_modal == Some(4) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_compile),
                            )
                            .frame(state.show_railroad_modal == Some(4)),
                        )
                        .on_hover_text("Ver diagrama de flujo (Tiempo de Compilación)")
                        .clicked()
                    {
                        state.show_railroad_modal = if state.show_railroad_modal == Some(4) { None } else { Some(4) };
                    }
                });

                // Columna 3: Tiempo de Ejecución + Botón Ver Diagrama
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Tiempo de Ejecución")
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    let btn_color_run = if state.show_railroad_modal == Some(5) {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .add(
                            egui::Button::image(
                                egui::Image::from_bytes(
                                    "bytes://view.svg",
                                    include_bytes!("../../diagramas/view.svg"),
                                )
                                .fit_to_exact_size(egui::vec2(18.0, 18.0))
                                .tint(btn_color_run),
                            )
                            .frame(state.show_railroad_modal == Some(5)),
                        )
                        .on_hover_text("Ver diagrama de flujo (Tiempo de Ejecución)")
                        .clicked()
                    {
                        state.show_railroad_modal = if state.show_railroad_modal == Some(5) { None } else { Some(5) };
                    }
                });
                ui.end_row();

                ui.label(
                    egui::RichText::new("¿Cuándo ocurre?")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Antes de crear el ejecutable, mientras rustc procesa el código.");
                ui.label("Mientras el usuario final tiene abierta la aplicación.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("¿Quién lo ejecuta?")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("El compilador (rustc / LLVM) en la PC del desarrollador.");
                ui.label("La CPU del sistema operativo en la PC del usuario.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Procesos Clave")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    "Verificación de sintaxis, chequeo de tipos, Borrow Checker y optimización.",
                );
                ui.label(
                    "Interacción con el usuario, lectura de archivos, red y cálculo de lógica.",
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("Impacto de Errores")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    "El ejecutable no se crea. El programador corrige el error en desarrollo.",
                );
                ui.label("Cierre inesperado (panic!) si no se manejan los errores.");
                ui.end_row();
            });
    });
}


pub fn mostrar_pilares_conceptos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        mostrar_pilares_tiempo(ui, state);
        ui.add_space(24.0);
        mostrar_pilares_proyecto(ui, state);

        ui.add_space(20.0);
        // Imagen de Ferris abajo de ambos conceptos alineada a la derecha
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.add(
                egui::Image::new(egui::include_image!("../../assets/taller/3.png"))
                    .max_width(380.0)
                    .corner_radius(8),
            );
        });
    });
}


pub fn mostrar_desglose_template_con_imagen(ui: &mut egui::Ui, state: &PortfolioState) {
    if let Some(ref proj_name) = state.created_project_name {
        let is_lib = proj_name.contains("lib") || state.estructura_tab == 2;
        let src_file = if is_lib { "src/lib.rs" } else { "src/main.rs" };
        let src_desc = if is_lib {
            "Archivo raíz de la librería. No lleva fn main(), sino funciones y structs con pub."
        } else {
            "Archivo fuente principal ejecutable con la función de entrada fn main() { ... }."
        };

        ui.add_space(16.0);
        ui.heading(
            egui::RichText::new(format!("Template {}", proj_name))
                .size(18.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(8.0);

        ui.horizontal_top(|ui| {
            // Columna Izquierda: Tabla Desglose Template
            let mut info_frame = egui::Frame::new();
            info_frame.fill = egui::Color32::from_rgb(18, 22, 32);
            info_frame.inner_margin = egui::Margin::same(14);
            info_frame.corner_radius = egui::CornerRadius::same(8);
            info_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 95));

            info_frame.show(ui, |ui| {
                egui::Grid::new("desglose_template_grid_pilares")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Archivo / Carpeta").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Cargo.toml").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Manifiesto con metadatos de tu proyecto (nombre, versión, dependencias).");
                        ui.end_row();

                        ui.label(egui::RichText::new("Cargo.lock").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Registro de versiones fijadas de dependencias.");
                        ui.end_row();

                        ui.label(egui::RichText::new(src_file).monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(src_desc);
                        ui.end_row();

                        ui.label(egui::RichText::new("target/").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Carpeta binaria donde rustc compila los ejecutables.");
                        ui.end_row();
                    });
            });

            ui.add_space(20.0);

            // Columna Derecha: Imagen 7.png fija inamovible
            ui.add(
                egui::Image::new(egui::include_image!("../../assets/taller/7.png"))
                    .fit_to_exact_size(egui::vec2(340.0, 220.0))
                    .corner_radius(8),
            );
        });
    }
}


#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TipoEscalar {
    nombre: &'static str,
    categoria: &'static str,
    bits: &'static str,
    bytes: &'static str,
    minimo: &'static str,
    maximo: &'static str,
    ejemplo: &'static str,
    descripcion: &'static str,
}

#[allow(dead_code)]
const TIPOS_ESCALARES: &[TipoEscalar] = &[
    TipoEscalar {
        nombre: "i8",
        categoria: "Entero con signo",
        bits: "8",
        bytes: "1",
        minimo: "-128",
        maximo: "127",
        ejemplo: "25",
        descripcion: "Entero pequeño que admite valores negativos.",
    },
    TipoEscalar {
        nombre: "i16",
        categoria: "Entero con signo",
        bits: "16",
        bytes: "2",
        minimo: "-32 768",
        maximo: "32 767",
        ejemplo: "1200",
        descripcion: "Útil cuando i8 es insuficiente y el rango sigue siendo moderado.",
    },
    TipoEscalar {
        nombre: "i32",
        categoria: "Entero con signo",
        bits: "32",
        bytes: "4",
        minimo: "-2 147 483 648",
        maximo: "2 147 483 647",
        ejemplo: "42",
        descripcion: "Tipo entero inferido por defecto para literales como 42.",
    },
    TipoEscalar {
        nombre: "i64",
        categoria: "Entero con signo",
        bits: "64",
        bytes: "8",
        minimo: "-9 223 372 036 854 775 808",
        maximo: "9 223 372 036 854 775 807",
        ejemplo: "5000000",
        descripcion: "Entero con un rango amplio para conteos grandes.",
    },
    TipoEscalar {
        nombre: "i128",
        categoria: "Entero con signo",
        bits: "128",
        bytes: "16",
        minimo: "−2^127",
        maximo: "2^127 − 1",
        ejemplo: "1000000",
        descripcion: "Entero de rango extraordinariamente amplio.",
    },
    TipoEscalar {
        nombre: "isize",
        categoria: "Entero con signo",
        bits: "Depende de la plataforma",
        bytes: "4 u 8",
        minimo: "Depende de la plataforma",
        maximo: "Depende de la plataforma",
        ejemplo: "10",
        descripcion: "Tiene el tamaño natural de la arquitectura y se usa en ciertas operaciones de memoria.",
    },
    TipoEscalar {
        nombre: "u8",
        categoria: "Entero sin signo",
        bits: "8",
        bytes: "1",
        minimo: "0",
        maximo: "255",
        ejemplo: "25",
        descripcion: "Ideal para bytes, canales de color y valores pequeños no negativos.",
    },
    TipoEscalar {
        nombre: "u16",
        categoria: "Entero sin signo",
        bits: "16",
        bytes: "2",
        minimo: "0",
        maximo: "65 535",
        ejemplo: "8080",
        descripcion: "Frecuente para puertos de red y cantidades medianas.",
    },
    TipoEscalar {
        nombre: "u32",
        categoria: "Entero sin signo",
        bits: "32",
        bytes: "4",
        minimo: "0",
        maximo: "4 294 967 295",
        ejemplo: "100",
        descripcion: "Entero no negativo con un rango amplio.",
    },
    TipoEscalar {
        nombre: "u64",
        categoria: "Entero sin signo",
        bits: "64",
        bytes: "8",
        minimo: "0",
        maximo: "18 446 744 073 709 551 615",
        ejemplo: "5000000",
        descripcion: "Útil para identificadores y contadores muy grandes.",
    },
    TipoEscalar {
        nombre: "u128",
        categoria: "Entero sin signo",
        bits: "128",
        bytes: "16",
        minimo: "0",
        maximo: "2^128 − 1",
        ejemplo: "1000000",
        descripcion: "El entero sin signo con mayor rango incorporado.",
    },
    TipoEscalar {
        nombre: "usize",
        categoria: "Entero sin signo",
        bits: "Depende de la plataforma",
        bytes: "4 u 8",
        minimo: "0",
        maximo: "Depende de la plataforma",
        ejemplo: "3",
        descripcion: "Tipo utilizado para índices y tamaños de colecciones.",
    },
    TipoEscalar {
        nombre: "f32",
        categoria: "Punto flotante",
        bits: "32",
        bytes: "4",
        minimo: "≈ −3.4 × 10^38",
        maximo: "≈ 3.4 × 10^38",
        ejemplo: "19.99",
        descripcion: "Decimal de precisión simple; ocupa menos memoria.",
    },
    TipoEscalar {
        nombre: "f64",
        categoria: "Punto flotante",
        bits: "64",
        bytes: "8",
        minimo: "≈ −1.8 × 10^308",
        maximo: "≈ 1.8 × 10^308",
        ejemplo: "3.14159",
        descripcion: "Decimal inferido por defecto y con mayor precisión que f32.",
    },
    TipoEscalar {
        nombre: "bool",
        categoria: "Booleano",
        bits: "8 en memoria",
        bytes: "1",
        minimo: "false",
        maximo: "true",
        ejemplo: "true",
        descripcion: "Representa una condición lógica verdadera o falsa.",
    },
    TipoEscalar {
        nombre: "char",
        categoria: "Carácter Unicode",
        bits: "32",
        bytes: "4",
        minimo: "U+0000",
        maximo: "U+10FFFF",
        ejemplo: "'A'",
        descripcion: "Representa un valor escalar Unicode, no solamente un byte ASCII.",
    },
];


#[allow(dead_code)]
fn codigo_variable(state: &PortfolioState) -> String {
    let tipo = TIPOS_ESCALARES[state.variable_type];
    let nombre = state.variable_name.trim();
    let nombre = if nombre.is_empty() {
        "variable"
    } else {
        nombre
    };
    let declaracion = match state.declaration_kind {
        1 => format!(
            "const {}: {} = {};",
            nombre.to_uppercase(),
            tipo.nombre,
            state.variable_value
        ),
        2 => format!(
            "static {}: {} = {};",
            nombre.to_uppercase(),
            tipo.nombre,
            state.variable_value
        ),
        _ => format!(
            "let {}{}: {} = {};",
            if state.variable_mutable { "mut " } else { "" },
            nombre,
            tipo.nombre,
            state.variable_value
        ),
    };
    let identificador = if state.declaration_kind == 0 {
        nombre.to_owned()
    } else {
        nombre.to_uppercase()
    };

    format!(
        "fn main() {{\n    {declaracion}\n    println!(\"{identificador} = {{{identificador}}}\");\n}}\n"
    )
}

