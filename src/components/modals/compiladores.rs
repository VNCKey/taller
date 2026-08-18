use eframe::egui;
use crate::app::PortfolioState;

pub fn mostrar_modal_comparacion_compiladores(ctx: &egui::Context, state: &mut PortfolioState) {
    if !state.show_rustc_compilador_modal {
        return;
    }

    let mut open = true;
    egui::Window::new("Arquitectura de ejecución")
        .open(&mut open)
        .resizable(true)
        .default_size([820.0, 500.0])
        .collapsible(false)
        .show(ctx, |ui| {
            let mut card_frame = egui::Frame::new();
            card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            card_frame.inner_margin = egui::Margin::same(12);
            card_frame.corner_radius = egui::CornerRadius::same(8);
            card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            let mut step_frame = egui::Frame::new();
            step_frame.fill = egui::Color32::from_rgb(22, 28, 40);
            step_frame.inner_margin = egui::Margin::symmetric(10, 6);
            step_frame.corner_radius = egui::CornerRadius::same(6);
            step_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 52, 75));

            let title_color = egui::Color32::from_rgb(255, 180, 100);
            let code_color = egui::Color32::from_rgb(200, 230, 255);
            let subtext_color = egui::Color32::from_rgb(180, 195, 215);

            ui.columns(3, |cols| {
                // 1. Compilado Nativo (Rust)
                card_frame.show(&mut cols[0], |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("Compilado Nativo")
                                .size(16.0)
                                .strong()
                                .color(title_color),
                        );
                        ui.label(
                            egui::RichText::new("Rust, C, C++, Go")
                                .strong()
                                .color(subtext_color),
                        );
                    });
                    ui.add_space(10.0);

                    // Pasos en Tarjetas
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 1: Código (.rs)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 2: rustc + LLVM").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 3: Binario (.exe / ELF)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 4: CPU (Directo)").strong().color(egui::Color32::WHITE));
                        });
                    });

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Rendimiento CPU:").strong().size(12.0));
                    ui.add(egui::ProgressBar::new(1.0).text("100% Nativo Directo"));
                });

                // 2. Interpretado (Python)
                card_frame.show(&mut cols[1], |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("Interpretado")
                                .size(16.0)
                                .strong()
                                .color(title_color),
                        );
                        ui.label(
                            egui::RichText::new("Python, JS, PHP")
                                .strong()
                                .color(subtext_color),
                        );
                    });
                    ui.add_space(10.0);

                    // Pasos en Tarjetas
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 1: Código (.py)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 2: Intérprete").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 3: CPU (En vivo)").strong().color(egui::Color32::WHITE));
                        });
                        ui.add_space(28.0);
                    });

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Rendimiento CPU:").strong().size(12.0));
                    ui.add(egui::ProgressBar::new(0.35).text("~35% Traduciendo en vivo"));
                });

                // 3. Máquina Virtual (Java)
                card_frame.show(&mut cols[2], |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("Máquina Virtual")
                                .size(16.0)
                                .strong()
                                .color(title_color),
                        );
                        ui.label(
                            egui::RichText::new("Java, C#, Kotlin")
                                .strong()
                                .color(subtext_color),
                        );
                    });
                    ui.add_space(10.0);

                    // Pasos en Tarjetas
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 1: Código (.java)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 2: Bytecode (.class)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 3: VM / JIT").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 4: CPU Hardware").strong().color(egui::Color32::WHITE));
                        });
                    });

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Rendimiento CPU:").strong().size(12.0));
                    ui.add(egui::ProgressBar::new(0.75).text("~75% Vía VM / JIT"));
                });
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Conclusión Didáctica:").strong().color(egui::Color32::WHITE));
                ui.label("Rust compila a código binario nativo directo al hardware. ¡Por eso no requiere máquinas virtuales y tiene velocidad máxima!");
            });
        });

    if !open {
        state.show_rustc_compilador_modal = false;
    }
}
