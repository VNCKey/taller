use std::path::{Path, PathBuf};

use eframe::egui;

use crate::app::PortfolioState;
use crate::components::code_editor::rust_layouter;
use crate::components::console_output::formatear_salida_consola;
use crate::execution::ejecutar_codigo_rust;
use crate::views::conceptos::{buscar_ruta_proyecto, mostrar_selector_proyectos_estandar_con_archivos};

pub fn mostrar_anatomia_cargo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let orange = egui::Color32::from_rgb(255, 180, 80);
    let cyan = egui::Color32::from_rgb(100, 200, 255);

    let syntax_set = state.syntax_set.clone();
    let theme = state.theme_set.themes["base16-ocean.dark"].clone();

    // --- CONTENEDOR PRINCIPAL A PANTALLA COMPLETA 100% EDGE-TO-EDGE ---
    let mut main_frame = egui::Frame::new();
    main_frame.fill = egui::Color32::from_rgb(10, 14, 22);
    main_frame.stroke = egui::Stroke::NONE;
    main_frame.corner_radius = egui::CornerRadius::ZERO;
    main_frame.inner_margin = egui::Margin {
        left: 10,
        right: 0, // Pegado 100% al borde derecho para el scrollbar
        top: 6,
        bottom: 6,
    };

    main_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        let mut run_clicked = false;

        // 1. TOOLBAR SUPERIOR DEL EDITOR (Selector + Ejecutar + Consola + Explorador)
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Selector de Proyecto y Archivos a la izquierda
            let code_target = if state.selected_project.is_some() {
                &mut state.shared_project_code
            } else {
                &mut state.estructura_code
            };

            mostrar_selector_proyectos_estandar_con_archivos(
                ui,
                &mut state.selected_project,
                &mut state.selected_file,
                &mut state.term_cwd,
                "combo_proyectos_anatomy_codelab",
                code_target,
            );

            // 1. Botón Ejecutar (play-svgrepo-com.svg con iluminación interactiva Cyan en hover)
            let img_play = egui::Image::new(egui::include_image!("../../../assets/icons/play-svgrepo-com.svg"))
                .fit_to_exact_size(egui::Vec2::new(14.0, 14.0));
            let btn_run = boton_icono_toolbar(
                ui,
                img_play,
                false,
                cyan,
                "Ejecutar Código (F5 / Ctrl+Enter)",
            );

            if btn_run.clicked() {
                run_clicked = true;
                state.mostrar_console_drawer = true;
            }

            ui.add_space(2.0);

            // 2. Botón Consola / Terminal (terminal-svgrepo-com.svg con iluminación interactiva Cyan en hover)
            let console_abierta = state.mostrar_console_drawer;
            let img_terminal = egui::Image::new(egui::include_image!("../../../assets/icons/terminal-svgrepo-com.svg"))
                .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
            let btn_console = boton_icono_toolbar(
                ui,
                img_terminal,
                console_abierta,
                cyan,
                if console_abierta { "Ocultar Terminal de Salida" } else { "Mostrar Terminal de Salida" },
            );

            if btn_console.clicked() {
                state.mostrar_console_drawer = !state.mostrar_console_drawer;
            }

            // --- Espaciador elástico seguro hacia la derecha ---
            let drawer_activo = state.mostrar_explorer_drawer;
            let available_w = ui.available_width();
            let needed_for_btn = 32.0 + 10.0;
            if available_w > needed_for_btn {
                ui.add_space(available_w - needed_for_btn);
            }

            // 3. Botón Enunciado / Descripción del Reto (article svg con iluminación interactiva Cyan en hover)
            let img_article = egui::Image::new(egui::include_image!("../../../assets/icons/design-distribution-of-elements-of-an-article-svgrepo-com.svg"))
                .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
            let btn_guide = boton_icono_toolbar(
                ui,
                img_article,
                drawer_activo,
                cyan,
                if drawer_activo { "Ocultar Descripción del Reto" } else { "Descripción del Reto / Instrucciones" },
            );

            if btn_guide.clicked() {
                state.mostrar_explorer_drawer = !state.mostrar_explorer_drawer;
            }
        });

        ui.add_space(5.0);
        let sep_y = ui.cursor().top();
        let frame_rect = ui.max_rect();
        // Línea divisoria horizontal de borde a borde exterior exacto
        ui.painter().line_segment(
            [
                egui::pos2(frame_rect.min.x - 10.0, sep_y),
                egui::pos2(frame_rect.max.x, sep_y),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95)),
        );
        ui.add_space(5.0);

        // 2. CUERPO DEL EDITOR DE CÓDIGO (A PANTALLA COMPLETA)
        let console_drawer_abierto = state.mostrar_console_drawer;
        let mut cerrar_console = false;
        let (code_ref, output_arc) = state.obtener_editor_activo_mut();

        if run_clicked {
            *output_arc.lock().unwrap() = "Ejecutando...".to_string();
            let code_clone = code_ref.clone();
            let out_clone = std::sync::Arc::clone(&output_arc);
            let ctx = ui.ctx().clone();
            std::thread::spawn(move || {
                let res = ejecutar_codigo_rust(&code_clone);
                *out_clone.lock().unwrap() = res;
                ctx.request_repaint();
            });
        }

        let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
            rust_layouter(ui, text.as_str(), wrap_width, &syntax_set, &theme)
        };

        let output_text = output_arc.lock().unwrap().clone();
        let has_output = !output_text.is_empty();

        // Animación suave del panel de salida inferior (Bottom Drawer)
        let console_anim = ui.ctx().animate_bool(
            egui::Id::new("bottom_console_anim"),
            console_drawer_abierto && has_output,
        );

        let code_max_h = ui.available_height();

        // Personalización de la barra de scroll (Pegada al ras en el extremo derecho, 0px margen)
        let old_style = (**ui.style()).clone();
        ui.style_mut().spacing.scroll.floating = false;
        ui.style_mut().spacing.scroll.bar_width = 8.0;
        ui.style_mut().spacing.scroll.bar_inner_margin = 0.0;
        ui.style_mut().spacing.scroll.bar_outer_margin = 0.0;
        ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 60, 85);
        ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
        ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::ZERO;
        ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 85, 120);
        ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
        ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::ZERO;
        ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 115, 160);
        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
        ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::ZERO;

        egui::ScrollArea::vertical()
            .max_height(code_max_h)
            .auto_shrink([false, false])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    // Generación dinámica de la columna de números de línea
                    let num_lines = code_ref.split('\n').count().max(1);
                    let mut line_numbers = String::new();
                    for i in 1..=num_lines {
                        use std::fmt::Write;
                        let _ = writeln!(line_numbers, "{:>2}", i);
                    }

                    // Columna de numeración (Gutter)
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(line_numbers.trim_end())
                                .font(egui::FontId::monospace(14.0))
                                .color(egui::Color32::from_rgb(85, 105, 135)),
                        )
                        .selectable(false),
                    );

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    ui.add(
                        egui::TextEdit::multiline(code_ref)
                            .frame(egui::Frame::NONE)
                            .layouter(&mut layouter)
                            .code_editor()
                            .desired_width(ui.available_width())
                            .lock_focus(true),
                    );
                });
            });

        ui.set_style(old_style);

        // 3. CONSOLA DE SALIDA TERMINAL (DRAWER SOBREPUESTO QUE SUBE DESDE EL FONDO SIN ENCOGER EL EDITOR)
        if console_anim > 0.001 {
            let frame_rect = ui.max_rect();
            let console_target_h = 180.0_f32.min(frame_rect.height() * 0.45);
            let console_h = console_target_h * console_anim;

            let top_y = frame_rect.max.y - console_h;
            let bottom_y = frame_rect.max.y;
            let left_x = frame_rect.min.x - 10.0;
            let right_x = frame_rect.max.x;

            egui::Area::new(egui::Id::new("console_bottom_overlay_sheet"))
                .fixed_pos(egui::pos2(left_x, top_y))
                .order(egui::Order::Middle)
                .show(ui.ctx(), |ui| {
                    // RECORTE ESTRICTO: Garantiza que NADA se dibuje debajo del límite del editor (el footer queda 100% protegido)
                    let console_clip = egui::Rect::from_min_max(
                        egui::pos2(left_x, top_y),
                        egui::pos2(right_x, bottom_y),
                    );
                    ui.set_clip_rect(console_clip);

                    let mut console_frame = egui::Frame::new();
                    console_frame.fill = egui::Color32::from_rgb(8, 11, 16);
                    console_frame.stroke = egui::Stroke::NONE;
                    console_frame.inner_margin = egui::Margin {
                        left: 10,
                        right: 8,
                        top: 6,
                        bottom: 6,
                    };
                    console_frame.corner_radius = egui::CornerRadius::ZERO;

                    console_frame.show(ui, |ui| {
                        let total_w = right_x - left_x;
                        ui.set_width((total_w - 18.0).max(10.0));
                        ui.set_height((console_h - 12.0).max(10.0));

                        // 1. Línea superior divisoria exacta de borde a borde
                        ui.painter().line_segment(
                            [egui::pos2(left_x, top_y), egui::pos2(right_x, top_y)],
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95)),
                        );

                        // Barra superior de la consola
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("📟 Salida de Consola")
                                    .strong()
                                    .size(12.5)
                                    .color(cyan),
                            );

                            // Estado de la ejecución
                            if output_text == "Ejecutando..." || output_text == "Compilando..." {
                                ui.label(
                                    egui::RichText::new("● Ejecutando...")
                                        .size(11.5)
                                        .color(egui::Color32::YELLOW),
                                );
                            } else if output_text.contains("Error") || output_text.contains("[Errores/Warnings]") {
                                ui.label(
                                    egui::RichText::new("● Con Errores / Advertencias")
                                        .size(11.5)
                                        .color(egui::Color32::from_rgb(255, 110, 110)),
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new("● Éxito (0)")
                                        .size(11.5)
                                        .color(egui::Color32::LIGHT_GREEN),
                                );
                            }

                            // Botón para minimizar
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui
                                    .button(
                                        egui::RichText::new("▼ Minimizar")
                                            .size(11.5)
                                            .color(egui::Color32::from_rgb(180, 195, 215)),
                                    )
                                    .clicked()
                                {
                                    cerrar_console = true;
                                }
                            });
                        });

                        ui.add_space(3.0);
                        let sep_header_y = top_y + 24.0;
                        ui.painter().line_segment(
                            [egui::pos2(left_x, sep_header_y), egui::pos2(right_x, sep_header_y)],
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 42, 65)),
                        );
                        ui.add_space(3.0);

                        // Contenido scrolleable de la consola
                        let console_scroll_h = (console_h - 38.0).max(10.0);
                        egui::ScrollArea::vertical()
                            .max_height(console_scroll_h)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                if output_text == "Ejecutando..." || output_text == "Compilando..." {
                                    ui.label(
                                        egui::RichText::new(&output_text)
                                            .color(egui::Color32::YELLOW)
                                            .monospace(),
                                    );
                                } else if let Some(idx) = output_text.find("[Errores/Warnings]:\n") {
                                    let (stdout, stderr) = output_text.split_at(idx);
                                    if !stdout.is_empty() {
                                        ui.label(formatear_salida_consola(stdout, false));
                                        ui.add_space(4.0);
                                        ui.separator();
                                        ui.add_space(4.0);
                                    }
                                    let solo_error = stderr
                                        .strip_prefix("[Errores/Warnings]:\n")
                                        .unwrap_or(stderr);
                                    ui.label(formatear_salida_consola(solo_error, true));
                                } else if output_text.starts_with("Error") {
                                    ui.label(formatear_salida_consola(&output_text, true));
                                } else {
                                    ui.label(formatear_salida_consola(&output_text, false));
                                }
                            });
                    });
                });
        }

        if cerrar_console {
            state.mostrar_console_drawer = false;
        }
    });

    // --- DRAWER LATERAL DERECHO SOBREPUESTO CON ANIMACIÓN SUAVE ---
    let anim_factor = ui.ctx().animate_bool(egui::Id::new("explorer_drawer_anim"), state.mostrar_explorer_drawer);
    if anim_factor > 0.001 {
        let bounds = ui.max_rect();
        let full_drawer_width = 360.0_f32.min(bounds.width() * 0.45);
        let current_width = full_drawer_width * anim_factor;
        let top_y = bounds.min.y;
        let drawer_height = bounds.height();
        let left_edge_x = bounds.max.x - current_width;

        egui::Area::new(egui::Id::new("explorer_right_drawer_overlay"))
            .fixed_pos(egui::pos2(left_edge_x, top_y))
            .show(ui.ctx(), |ui| {
                let drawer_rect = egui::Rect::from_min_max(
                    egui::pos2(left_edge_x, top_y),
                    egui::pos2(bounds.max.x, top_y + drawer_height),
                );
                ui.set_clip_rect(drawer_rect);

                let mut drawer_frame = egui::Frame::new();
                drawer_frame.fill = egui::Color32::from_rgb(13, 15, 19);
                drawer_frame.stroke = egui::Stroke::NONE;
                drawer_frame.inner_margin = egui::Margin::symmetric(16, 14);
                drawer_frame.corner_radius = egui::CornerRadius::ZERO;

                drawer_frame.show(ui, |ui| {
                    ui.set_width((full_drawer_width - 32.0).max(10.0));
                    ui.set_height(drawer_height - 28.0);

                    // Encabezado del Drawer
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("📁 Explorador de Proyecto")
                                .strong()
                                .size(16.0)
                                .color(cyan),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button(
                                    egui::RichText::new("✖ Cerrar")
                                        .size(13.0)
                                        .color(egui::Color32::from_rgb(255, 120, 120)),
                                )
                                .clicked()
                            {
                                state.mostrar_explorer_drawer = false;
                            }
                        });
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Contenido scrolleable del árbol
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            mostrar_project_explorer_drawer(ui, state, orange, cyan);
                        });
                });

                // Línea divisoria vertical nítida dibujada encima
                ui.painter().line_segment(
                    [egui::pos2(left_edge_x, top_y), egui::pos2(left_edge_x, top_y + drawer_height)],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 65, 95)),
                );
            });
    }
}

fn mostrar_project_explorer_drawer(
    ui: &mut egui::Ui,
    state: &mut PortfolioState,
    orange: egui::Color32,
    cyan: egui::Color32,
) {
    let project_dir = state
        .selected_project
        .as_deref()
        .map(|project| buscar_ruta_proyecto(&state.term_cwd, project));

    match project_dir.as_ref() {
        Some(dir) if dir.exists() => {
            let project_name = dir.file_name().and_then(|name| name.to_str()).unwrap_or("proyecto");
            ui.label(egui::RichText::new(format!("{project_name}/")).monospace().strong().color(orange));
            ui.add_space(6.0);
            mostrar_arbol(ui, dir, dir, "", state);
        }
        Some(_) => {
            ui.label("Esperando a que Cargo cree el proyecto...");
        }
        None => {
            ui.label("No hay ningún proyecto seleccionado.");
            ui.label("Crea o selecciona uno en la barra de herramientas.");
        }
    }

    if let Some(ref relative) = state.selected_file {
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Archivo seleccionado:").strong().color(orange));
        ui.label(egui::RichText::new(relative).monospace().color(cyan));
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(descripcion_archivo(relative))
                .size(12.5)
                .color(egui::Color32::from_rgb(180, 195, 215)),
        );
    }
}

fn mostrar_arbol(ui: &mut egui::Ui, root: &Path, directory: &Path, prefix: &str, state: &mut PortfolioState) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(directory)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.flatten().map(|entry| entry.path()))
        .filter(|path| path.file_name().is_none_or(|name| name != ".git"))
        .collect();
    entries.sort_by_key(|path| (!path.is_dir(), path.file_name().map(|name| name.to_os_string())));

    for (index, path) in entries.iter().enumerate() {
        let is_last = index + 1 == entries.len();
        let branch = if is_last { "└── " } else { "├── " };
        let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("?");
        let relative = path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/");
        if path.is_dir() {
            if directory == root && name == "target" {
                if ui
                    .selectable_label(
                        state.selected_file.as_deref() == Some("target/"),
                        egui::RichText::new(format!("{prefix}{branch}{name}/"))
                            .monospace()
                            .color(egui::Color32::LIGHT_BLUE),
                    )
                    .clicked()
                {
                    state.selected_file = Some("target/".to_string());
                }
                continue;
            }
            ui.label(egui::RichText::new(format!("{prefix}{branch}{name}/")).monospace().color(egui::Color32::LIGHT_BLUE));
            mostrar_arbol(ui, root, path, &format!("{prefix}{}", if is_last { "    " } else { "│   " }), state);
        } else {
            let is_selected = state.selected_file.as_deref() == Some(relative.as_str());
            if ui.selectable_label(is_selected, egui::RichText::new(format!("{prefix}{branch}{name}")).monospace().color(egui::Color32::WHITE)).clicked() {
                state.selected_file = Some(relative.clone());
                state.cargar_archivo_proyecto_activo();
            }
        }
    }
}

fn descripcion_archivo(relative: &str) -> &'static str {
    match relative {
        "Cargo.toml" => "Manifiesto del proyecto: contiene sus metadatos, edición, dependencias y configuración.",
        "Cargo.lock" => "Guarda las versiones exactas de las dependencias resueltas por Cargo.",
        "src/main.rs" => "Punto de entrada de un proyecto ejecutable. Contiene la función fn main().",
        "src/lib.rs" => "Punto de entrada de una librería reutilizable por otros módulos o paquetes.",
        "target/" => "Carpeta generada por Cargo para guardar artefactos de compilación.",
        "build.rs" => "Script opcional que Cargo ejecuta antes de compilar el proyecto.",
        path if path.starts_with("tests/") => "Test de integración del proyecto.",
        path if path.starts_with("examples/") => "Ejemplo ejecutable del proyecto.",
        path if path.starts_with("benches/") => "Benchmark del proyecto.",
        _ => "Archivo o carpeta perteneciente al proyecto Cargo.",
    }
}

fn boton_icono_toolbar(
    ui: &mut egui::Ui,
    img: egui::Image,
    activo: bool,
    color_activo: egui::Color32,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(32.0, 26.0), egui::Sense::click());
    let is_hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    // Fondo interactivo
    let bg_color = if is_down {
        egui::Color32::from_rgb(34, 46, 68)
    } else if activo {
        egui::Color32::from_rgb(26, 36, 54)
    } else if is_hovered {
        egui::Color32::from_rgb(24, 32, 48)
    } else {
        egui::Color32::from_rgb(16, 22, 32)
    };

    // Borde interactivo
    let border_stroke = if activo {
        egui::Stroke::new(1.0, color_activo)
    } else if is_hovered {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 95, 135))
    } else {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 54, 80))
    };

    // Color del icono SVG (se ilumina suavemente en hover o activo)
    let icon_tint = if activo {
        color_activo
    } else if is_hovered {
        color_activo
    } else {
        egui::Color32::from_rgb(160, 180, 205)
    };

    ui.painter().rect(rect, egui::CornerRadius::same(4), bg_color, border_stroke, egui::StrokeKind::Inside);
    let icon_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(15.0, 15.0));
    img.tint(icon_tint).paint_at(ui, icon_rect);

    response.on_hover_text(tooltip)
}
