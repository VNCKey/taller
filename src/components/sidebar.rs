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
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialCargo,
                                "📦 Pilares",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialCargo;
                        }

                        // 2. Comenzando
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::Comenzando,
                                "🚀 Comenzando",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::Comenzando;
                        }

                        // 3. Strings & Ownership (reglas de memoria + String/&str)
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialOwnership
                                    || state.ruta_actual == AppRoute::TutorialStrings,
                                "🧵 Strings & Ownership",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialOwnership;
                        }

                        // 4. Control de Flujo
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialControlFlujo,
                                "🔀 Control de Flujo",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialControlFlujo;
                        }

                        // 5. Funciones & Closures
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialFunciones,
                                "⚡ Funciones & Closures",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialFunciones;
                        }

                        // 6. Tipos compuestos
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialTiposDatos,
                                "🧱 Tipos compuestos",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialTiposDatos;
                        }

                        // 7. Structs & impl
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialStructs,
                                "🏗️ Structs & impl",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialStructs;
                        }

                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialCompilacion,
                                "⚙️ Proceso de Compilación",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialCompilacion;
                        }
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialIteradores,
                                "🔄 Iteradores",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialIteradores;
                        }
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialEnums,
                                "🏷️ Enums, Option & Result",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialEnums;
                        }
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialColecciones,
                                "📚 Colecciones (Vec/HashMap)",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialColecciones;
                        }
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialErrores,
                                "🚨 Manejo de Errores (?)",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::TutorialErrores;
                        }
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::TutorialTraits,
                                "🧬 Traits & Genéricos",
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
                                "📊 Visualización de Datos",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::DashboardGraficos;
                        }
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::Playground,
                                "💻 Editor Local",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::Playground;
                        }
                        if ui
                            .selectable_label(
                                state.ruta_actual == AppRoute::PlaygroundNube,
                                "☁️ Playground API",
                            )
                            .clicked()
                        {
                            state.ruta_actual = AppRoute::PlaygroundNube;
                        }

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);

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

