use eframe::egui;
use std::sync::atomic::Ordering;
use crate::app::PortfolioState;
use crate::routes::AppRoute;

pub fn mostrar_sidebar(ui: &mut egui::Ui, state: &mut PortfolioState) {
    egui::Panel::left("sidebar")
        .resizable(false)
        .show(ui, |ui| {
            ui.set_min_width(220.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(20.0);

                ui.vertical_centered(|ui| {
                    let mut job = egui::text::LayoutJob::default();
                    job.append(
                        "Ferris",
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(
                                24.0,
                                egui::FontFamily::Proportional,
                            ),
                            color: egui::Color32::from_rgb(255, 160, 50),
                            ..Default::default()
                        },
                    );
                    job.append(
                        "Key",
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(
                                24.0,
                                egui::FontFamily::Proportional,
                            ),
                            color: egui::Color32::WHITE,
                            ..Default::default()
                        },
                    );

                    let logo_response = ui
                        .add(egui::Label::new(job).sense(egui::Sense::click()))
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .on_hover_text("Volver a la pantalla de inicio (FerrisKey)");

                    if logo_response.clicked() {
                        state.ruta_actual = AppRoute::LandingPage;
                    }

                    ui.label(
                        egui::RichText::new("Aprende Rust Jugando")
                            .size(12.0)
                            .italics()
                            .color(egui::Color32::GRAY),
                    );
                });

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(15.0);

                ui.label(
                    egui::RichText::new("CURSO RUST COMPLETO")
                        .strong()
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(10.0);

                // 1. Pilares
                // 1. Pilares
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialCargo,
                        "Pilares",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialCargo;
                }

                // 2. Conceptos
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::Comenzando,
                        "Conceptos",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::Comenzando;
                }

                // 3. Memoria (reglas de memoria + String/&str)
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialOwnership
                            || state.ruta_actual == AppRoute::TutorialStrings,
                        "Memoria",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialOwnership;
                }

                // 4. Módulos
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialModulos,
                        "Módulos",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialModulos;
                }

                // 5. Tipos Compuestos
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialTiposDatos,
                        "Tipos Compuestos",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialTiposDatos;
                }

                // 6. Colecciones
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialColecciones,
                        "Colecciones",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialColecciones;
                }

                // 6. Control de Flujo
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialControlFlujo,
                        "Control de Flujo",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialControlFlujo;
                }

                // 7. Funciones & Closures
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialFunciones,
                        "Closures",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialFunciones;
                }

                // 8. Iteradores
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialIteradores,
                        "Iteradores",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialIteradores;
                }

                // 9. Structs & impl
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialStructs,
                        "Custom Types",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialStructs;
                }

                // 10. Error Handling
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialEnums,
                        "Error Handling",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialEnums;
                }

                // 11. Generics
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialGenericos,
                        "Generics",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialGenericos;
                }


                // 13. Traits & Genéricos
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::TutorialTraits,
                        "Traits",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::TutorialTraits;
                }

                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("PROYECTOS TÉCNICOS")
                        .strong()
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(10.0);
                if ui
                    .selectable_label(
                        state.ruta_actual == AppRoute::DashboardGraficos,
                        "Visualización de Datos",
                    )
                    .clicked()
                {
                    state.ruta_actual = AppRoute::DashboardGraficos;
                }

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                // Botones de Utilidades: Salida/Logs, Terminal y Configuración
                ui.horizontal(|ui| {
                    let is_cargo_open =
                        state.show_cargo_output_modal.load(Ordering::Relaxed);
                    let cargo_text_color = if is_cargo_open {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .button(
                            egui::RichText::new("ℹ️").size(18.0).color(cargo_text_color),
                        )
                        .on_hover_text("Información / Salida de compilación y macros")
                        .clicked()
                    {
                        state.show_cargo_output_modal
                            .store(!is_cargo_open, Ordering::Relaxed);
                    }

                    ui.add_space(8.0);

                    let term_text_color = if state.show_terminal_modal {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    if ui
                        .button(egui::RichText::new("💻").size(18.0).color(term_text_color))
                        .on_hover_text("Terminal Linux interactiva")
                        .clicked()
                    {
                        state.show_terminal_modal = !state.show_terminal_modal;
                    }

                    ui.add_space(8.0);

                    let config_icon_color = if state.show_settings_modal {
                        egui::Color32::from_rgb(255, 160, 50)
                    } else {
                        egui::Color32::from_rgb(180, 190, 205)
                    };

                    let config_img = egui::Image::from_bytes(
                        "bytes://config.svg",
                        include_bytes!("../../diagramas/config.svg"),
                    )
                    .fit_to_exact_size(egui::vec2(20.0, 20.0))
                    .tint(config_icon_color);

                    if ui
                        .add(egui::Button::image(config_img))
                        .on_hover_text("Configuración y Atajos de Teclado")
                        .clicked()
                    {
                        state.show_settings_modal = !state.show_settings_modal;
                    }
                });
            });
        });
}
