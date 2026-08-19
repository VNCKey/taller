use eframe::egui;
use crate::app::PortfolioState;

pub fn mostrar_modal_comparacion_compiladores(ctx: &egui::Context, state: &mut PortfolioState) {
    if !state.show_rustc_compilador_modal {
        return;
    }

    let mut open = true;
    egui::Window::new("⚙️ Flujo y Arquitectura de Ejecución: ¿Por qué Rust es tan rápido?")
        .open(&mut open)
        .resizable(true)
        .default_size([960.0, 620.0])
        .collapsible(false)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(
                        "Compara cómo viaja una instrucción desde tu código fuente hasta el silicio del procesador en los 3 grandes paradigmas de ejecución.",
                    )
                    .color(egui::Color32::from_rgb(180, 195, 215))
                    .size(13.0),
                );
                ui.add_space(14.0);

                // --- 1. RUST / COMPILACIÓN NATIVA (AOT) ---
                dibujar_tarjeta_paradigma(
                    ui,
                    "🦀 1. Compilado Nativo AOT (Ahead-of-Time)",
                    "Rust, C, C++, Go",
                    egui::Color32::from_rgb(255, 160, 50),
                    1.0,
                    "100% Eficiencia Directa (Sin intermediarios)",
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            dibujar_nodo_flujo(
                                ui,
                                "📄",
                                "Código Fuente",
                                "main.rs",
                                "Fase 1: Edición",
                                egui::Color32::from_rgb(60, 140, 240),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(255, 160, 50));

                            dibujar_nodo_flujo(
                                ui,
                                "⚙️",
                                "rustc + LLVM",
                                "Borrow Check + O3",
                                "Fase 2: Build Time",
                                egui::Color32::from_rgb(255, 160, 50),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(255, 160, 50));

                            dibujar_nodo_flujo(
                                ui,
                                "📦",
                                "Binario Nativo",
                                "ELF / .EXE / Mach-O",
                                "Fase 3: Artefacto",
                                egui::Color32::from_rgb(80, 220, 120),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(80, 220, 120));

                            dibujar_nodo_flujo(
                                ui,
                                "⚡",
                                "CPU / Silicio",
                                "Instrucciones x86/ARM",
                                "Fase 4: Runtime",
                                egui::Color32::from_rgb(255, 215, 0),
                            );
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            badge_caracteristica(ui, "⏱️ Arranque: 0 ms (Instantáneo)", egui::Color32::from_rgb(100, 200, 120));
                            badge_caracteristica(ui, "🧠 Memoria: Sin Garbage Collector", egui::Color32::from_rgb(100, 200, 255));
                            badge_caracteristica(ui, "🛡️ Errores: 100% verificados antes de correr", egui::Color32::from_rgb(255, 180, 100));
                        });
                    },
                );

                ui.add_space(14.0);

                // --- 2. INTERPRETADO (Python / JS) ---
                dibujar_tarjeta_paradigma(
                    ui,
                    "🐍 2. Interpretado en Vivo",
                    "Python, JavaScript, Ruby, PHP",
                    egui::Color32::from_rgb(100, 190, 255),
                    0.35,
                    "~35% Eficiencia (Sobrecarga de traducción en tiempo real)",
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            dibujar_nodo_flujo(
                                ui,
                                "📄",
                                "Código Fuente",
                                "app.py",
                                "Fase 1: Edición",
                                egui::Color32::from_rgb(60, 140, 240),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(100, 190, 255));

                            dibujar_nodo_flujo(
                                ui,
                                "🔄",
                                "Intérprete (Eval Loop)",
                                "Lee y parsea en vivo",
                                "Fase 2: Runtime",
                                egui::Color32::from_rgb(255, 120, 120),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(255, 120, 120));

                            dibujar_nodo_flujo(
                                ui,
                                "⚡",
                                "CPU / Silicio",
                                "Instrucción a instrucción",
                                "Fase 3: Ejecución",
                                egui::Color32::from_rgb(255, 215, 0),
                            );
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            badge_caracteristica(ui, "🚀 Desarrollo: Sin tiempo de compilación", egui::Color32::from_rgb(100, 200, 120));
                            badge_caracteristica(ui, "🐌 Rendimiento: Bucles lentos", egui::Color32::from_rgb(255, 130, 130));
                            badge_caracteristica(ui, "⚠️ Errores: Saltan al ejecutar en producción", egui::Color32::from_rgb(255, 180, 100));
                        });
                    },
                );

                ui.add_space(14.0);

                // --- 3. MÁQUINA VIRTUAL / BYTECODE (Java / C#) ---
                dibujar_tarjeta_paradigma(
                    ui,
                    "☕ 3. Máquina Virtual / Bytecode Híbrido",
                    "Java, C#, Kotlin, Scala",
                    egui::Color32::from_rgb(220, 140, 255),
                    0.75,
                    "~75% Eficiencia (Vía JIT Compiler + Pausas de Garbage Collector)",
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            dibujar_nodo_flujo(
                                ui,
                                "📄",
                                "Código Fuente",
                                "App.java",
                                "Fase 1: Edición",
                                egui::Color32::from_rgb(60, 140, 240),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(220, 140, 255));

                            dibujar_nodo_flujo(
                                ui,
                                "⚙️",
                                "Compilador javac",
                                "Genera Bytecode",
                                "Fase 2: Build Time",
                                egui::Color32::from_rgb(200, 120, 240),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(220, 140, 255));

                            dibujar_nodo_flujo(
                                ui,
                                "📑",
                                "Bytecode Universal",
                                "App.class",
                                "Fase 3: Intermedio",
                                egui::Color32::from_rgb(180, 180, 255),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(220, 140, 255));

                            dibujar_nodo_flujo(
                                ui,
                                "☕",
                                "JVM / CLR + JIT",
                                "Compila en caliente + GC",
                                "Fase 4: Runtime VM",
                                egui::Color32::from_rgb(255, 140, 160),
                            );

                            dibujar_flecha_conector(ui, egui::Color32::from_rgb(255, 140, 160));

                            dibujar_nodo_flujo(
                                ui,
                                "⚡",
                                "CPU / Silicio",
                                "Código Máquina JIT",
                                "Fase 5: Hardware",
                                egui::Color32::from_rgb(255, 215, 0),
                            );
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            badge_caracteristica(ui, "🌐 Portabilidad: Corre en cualquier SO con JVM", egui::Color32::from_rgb(100, 200, 255));
                            badge_caracteristica(ui, "🔥 Warmup: Necesita calentar el JIT", egui::Color32::from_rgb(255, 180, 100));
                            badge_caracteristica(ui, "🧹 GC: Pausas para limpiar memoria", egui::Color32::from_rgb(255, 130, 130));
                        });
                    },
                );

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(10.0);

                // --- CONCLUSIÓN DIDÁCTICA ---
                let mut conclusion_frame = egui::Frame::new();
                conclusion_frame.fill = egui::Color32::from_rgb(18, 26, 38);
                conclusion_frame.inner_margin = egui::Margin::same(12);
                conclusion_frame.corner_radius = egui::CornerRadius::same(8);
                conclusion_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 100, 160));

                conclusion_frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("💡").size(24.0));
                        ui.add_space(6.0);
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Conclusión Didáctica de Rust")
                                    .strong()
                                    .size(15.0)
                                    .color(egui::Color32::from_rgb(255, 180, 100)),
                            );
                            ui.label(
                                egui::RichText::new(
                                    "Rust traslada TODO el esfuerzo y las verificaciones de seguridad al tiempo de compilación (rustc + Borrow Checker). El resultado es un binario autónomo que habla el lenguaje nativo de tu procesador sin pausas, sin máquinas virtuales y con consumo mínimo de batería y memoria.",
                                )
                                .color(egui::Color32::from_rgb(210, 220, 235)),
                            );
                        });
                    });
                });
                ui.add_space(8.0);
            });
        });

    if !open {
        state.show_rustc_compilador_modal = false;
    }
}

fn dibujar_tarjeta_paradigma(
    ui: &mut egui::Ui,
    titulo: &str,
    subtitulo: &str,
    color_titulo: egui::Color32,
    progreso: f32,
    texto_progreso: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    let mut card_frame = egui::Frame::new();
    card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    card_frame.inner_margin = egui::Margin::same(14);
    card_frame.corner_radius = egui::CornerRadius::same(8);
    card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 52, 75));

    card_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new(titulo)
                    .size(16.0)
                    .strong()
                    .color(color_titulo),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!("({})", subtitulo))
                    .color(egui::Color32::from_rgb(140, 155, 175))
                    .italics(),
            );
        });

        ui.add_space(10.0);
        add_contents(ui);
        ui.add_space(10.0);

        // Barra de eficiencia
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Eficiencia de CPU:")
                    .strong()
                    .size(12.0)
                    .color(egui::Color32::from_rgb(200, 210, 225)),
            );
            ui.add(
                egui::ProgressBar::new(progreso)
                    .text(texto_progreso)
                    .fill(color_titulo),
            );
        });
    });
}

fn dibujar_nodo_flujo(
    ui: &mut egui::Ui,
    icono: &str,
    nombre: &str,
    detalle: &str,
    fase: &str,
    color_borde: egui::Color32,
) {
    let mut node_frame = egui::Frame::new();
    node_frame.fill = egui::Color32::from_rgb(20, 26, 38);
    node_frame.inner_margin = egui::Margin::symmetric(10, 8);
    node_frame.corner_radius = egui::CornerRadius::same(6);
    node_frame.stroke = egui::Stroke::new(1.0, color_borde);

    node_frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(fase)
                    .size(10.0)
                    .color(egui::Color32::from_rgb(140, 160, 185)),
            );
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(icono).size(15.0));
                ui.label(
                    egui::RichText::new(nombre)
                        .strong()
                        .size(13.0)
                        .color(egui::Color32::WHITE),
                );
            });
            ui.add_space(2.0);
            ui.label(
                egui::RichText::new(detalle)
                    .monospace()
                    .size(11.0)
                    .color(color_borde),
            );
        });
    });
}

fn dibujar_flecha_conector(ui: &mut egui::Ui, color: egui::Color32) {
    ui.vertical_centered(|ui| {
        ui.add_space(14.0);
        ui.label(
            egui::RichText::new(" ──▶ ")
                .size(16.0)
                .strong()
                .color(color),
        );
    });
}

fn badge_caracteristica(ui: &mut egui::Ui, texto: &str, color: egui::Color32) {
    let mut badge_frame = egui::Frame::new();
    badge_frame.fill = egui::Color32::from_rgb(24, 30, 44);
    badge_frame.inner_margin = egui::Margin::symmetric(8, 4);
    badge_frame.corner_radius = egui::CornerRadius::same(4);
    badge_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 95));

    badge_frame.show(ui, |ui| {
        ui.label(egui::RichText::new(texto).size(11.0).color(color));
    });
    ui.add_space(4.0);
}
