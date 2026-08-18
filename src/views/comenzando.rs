use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
use crate::views::pilares::mostrar_desglose_template_con_imagen;

pub fn mostrar_comenzando(ui: &mut egui::Ui, state: &mut PortfolioState) {
    mostrar_tutorial_conceptos_basicos(ui, state);
}


pub fn mostrar_componente_terminal_3_modos(
    ui: &mut egui::Ui,
    _cmd_predeterminado: &str,
    state: &mut PortfolioState,
) {
    let mut term_frame = egui::Frame::new();
    term_frame.fill = egui::Color32::from_rgb(13, 17, 23);
    term_frame.inner_margin = egui::Margin::same(12);
    term_frame.corner_radius = egui::CornerRadius::same(8);
    term_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    term_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        // --- CABECERA DE LA TERMINAL LINUX ---
        ui.horizontal(|ui| {
            let history_len = state.term_history.lock().map(|h| h.len()).unwrap_or(0);
            let label = if state.show_terminal_history {
                format!("▼ Historial ({})", history_len)
            } else {
                format!("▶ Historial ({})", history_len)
            };

            if ui
                .button(
                    egui::RichText::new(label)
                        .strong()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                )
                .on_hover_text("Mostrar u ocultar el historial de comandos ejecutados")
                .clicked()
            {
                state.show_terminal_history = !state.show_terminal_history;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Botón Limpiar (Solo Ícono sin fondo)
                if ui
                    .add(egui::Button::new(egui::RichText::new("🗑️").size(13.0)).frame(false))
                    .on_hover_text("Limpiar terminal")
                    .clicked()
                    && let Ok(mut history) = state.term_history.lock()
                {
                    history.clear();
                }

                ui.add_space(6.0);

                // Botón Copiar Comando (Solo Ícono sin fondo)
                if ui
                    .add(egui::Button::new(egui::RichText::new("📋").size(13.0)).frame(false))
                    .on_hover_text("Copiar comando de la terminal")
                    .clicked()
                {
                    let text_to_copy = if !state.term_input.trim().is_empty() {
                        state.term_input.trim().to_string()
                    } else {
                        _cmd_predeterminado.to_string()
                    };
                    ui.ctx().output_mut(|o| {
                        o.commands.push(egui::OutputCommand::CopyText(text_to_copy))
                    });
                }
            });
        });

        ui.add_space(4.0);

        // Formato corto de CWD para el Prompt (ej: ~/VNC/repos/egui_vnc)
        let cwd_full = state.term_cwd.to_string_lossy();
        let short_cwd = if let Ok(home) = std::env::var("HOME") {
            if cwd_full.starts_with(&home) {
                cwd_full.replacen(&home, "~", 1)
            } else {
                cwd_full.to_string()
            }
        } else {
            cwd_full.to_string()
        };

        // --- HISTORIAL DE SALIDA DE COMANDOS DESPLEGABLE ---
        if state.show_terminal_history {
            if let Ok(history) = state.term_history.lock()
                && !history.is_empty()
            {
                egui::ScrollArea::vertical()
                    .max_height(160.0)
                    .id_salt("scroll_terminal_history")
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for (idx, line) in history.iter().enumerate() {
                            ui.push_id(idx, |ui| {
                                ui.label(
                                    egui::RichText::new(line)
                                        .monospace()
                                        .size(12.0)
                                        .color(egui::Color32::from_rgb(200, 230, 255)),
                                );
                            });
                        }
                    });
                ui.add_space(6.0);
            }
        }

        // Línea de entrada limpia sin marcos ni cajas negras flotantes
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("alek@debian:{}$", short_cwd))
                    .strong()
                    .color(egui::Color32::from_rgb(120, 255, 120))
                    .monospace()
                    .size(13.0),
            );

            let input_response = ui.add(
                egui::TextEdit::singleline(&mut state.term_input)
                    .frame(egui::Frame::NONE)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY),
            );

            // Mantener el foco automático en la terminal continuamente
            input_response.request_focus();

            // Ejecutar comando al presionar Enter
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let cmd_str = state.term_input.trim().to_string();
                if !cmd_str.is_empty() {
                    if let Ok(mut history) = state.term_history.lock() {
                        history.push(format!("alek@debian:{}$ {}", short_cwd, cmd_str));
                    }
                    state.term_input.clear();

                    // Detectar si el usuario ejecuta 'cargo new <nombre_proyecto>'
                    if cmd_str.starts_with("cargo new ") || cmd_str.starts_with("cargo  new ") {
                        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
                        if parts.len() >= 3 {
                            let is_lib = parts.contains(&"--lib");
                            for part in parts.iter().skip(2) {
                                if !part.starts_with('-') {
                                    let proj_name = (*part).to_string();
                                    state.created_project_name = Some(proj_name.clone());
                                    state.selected_project = Some(proj_name);
                                    if is_lib {
                                        state.estructura_tab = 2;
                                    } else {
                                        state.estructura_tab = 1;
                                    }
                                    break;
                                }
                            }
                        }
                    }

                    // Manejo especial de comando 'cd' para persistir el directorio de navegación
                    if cmd_str == "cd" || cmd_str.starts_with("cd ") {
                        let target_arg = if cmd_str == "cd" {
                            ""
                        } else {
                            cmd_str[3..].trim()
                        };
                        let new_path = if target_arg.is_empty() || target_arg == "~" {
                            std::env::var("HOME")
                                .map(std::path::PathBuf::from)
                                .unwrap_or_else(|_| state.term_cwd.clone())
                        } else if target_arg == ".." {
                            state
                                .term_cwd
                                .parent()
                                .map(|p| p.to_path_buf())
                                .unwrap_or_else(|| state.term_cwd.clone())
                        } else {
                            let candidate = state.term_cwd.join(target_arg);
                            candidate.canonicalize().unwrap_or(candidate)
                        };

                        if new_path.is_dir() {
                            state.term_cwd = new_path;
                        } else {
                            if let Ok(mut history) = state.term_history.lock() {
                                history.push(format!(
                                    "sh: cd: {}: No existe el directorio",
                                    target_arg
                                ));
                            }
                        }
                    } else {
                        // Ejecución en hilo secundario asíncrono (evita congelar el GUI)
                        let history_arc = Arc::clone(&state.term_history);
                        let output_arc = Arc::clone(&state.conceptos_output);
                        let modal_arc = Arc::clone(&state.show_cargo_output_modal);
                        let cwd = state.term_cwd.clone();
                        let cmd = cmd_str.clone();
                        let ctx = ui.ctx().clone();

                        if cmd.starts_with("cargo ") {
                            if let Ok(mut out) = state.conceptos_output.lock() {
                                *out = "Compilando con Cargo...".to_string();
                            }
                        }

                        std::thread::spawn(move || {
                            let output = std::process::Command::new("sh")
                                .arg("-c")
                                .arg(&cmd)
                                .current_dir(&cwd)
                                .output();

                            match output {
                                Ok(out) => {
                                    let stdout = String::from_utf8_lossy(&out.stdout);
                                    let stderr = String::from_utf8_lossy(&out.stderr);
                                    if let Ok(mut history) = history_arc.lock() {
                                        if !stdout.is_empty() {
                                            for line in stdout.lines() {
                                                history.push(line.to_string());
                                            }
                                        }
                                        if !stderr.is_empty() {
                                            for line in stderr.lines() {
                                                history.push(line.to_string());
                                            }
                                        }
                                    }

                                    // Sincronizar el cuadro de salida dedicado para comandos cargo
                                    if cmd.starts_with("cargo ") {
                                        let mut combined = stdout.into_owned();
                                        if !stderr.is_empty() {
                                            if !combined.is_empty() {
                                                combined.push_str("\n\n");
                                            }
                                            combined.push_str("[Errores/Warnings]:\n");
                                            combined.push_str(&stderr);
                                            if cmd.starts_with("cargo expand")
                                                && (stderr.contains("no such command")
                                                    || stderr.contains("not found"))
                                            {
                                                combined.push_str(
                                                    "\n\n💡 Nota: 'cargo expand' requiere la herramienta externa. Puedes instalarla ejecutando:\ncargo install cargo-expand",
                                                );
                                            }
                                        }
                                        if combined.is_empty() {
                                            combined = "El comando terminó sin salidas.".to_string();
                                        }
                                        if let Ok(mut out_lock) = output_arc.lock() {
                                            *out_lock = combined;
                                        }
                                        // Abrir la ventana modal flotante recién al terminar la compilación
                                        modal_arc.store(true, Ordering::Relaxed);
                                    }
                                }
                                Err(err) => {
                                    if let Ok(mut history) = history_arc.lock() {
                                        history.push(format!("Error ejecutando comando: {}", err));
                                    }
                                }
                            }
                            ctx.request_repaint();
                        });
                    }
                }
                // Mantener foco en el campo de texto tras presionar Enter
                input_response.request_focus();
            }
        });

        /*
        // --- MODOS RESERVADOS PARA EL FUTURO (MODO 0: ESTÁTICA, MODO 2: PTY REAL) ---
        // Si en el futuro necesitas habilitar el modo PTY nativo o estático:
        //
        // MODO 0 (Estática):
        // ui.label(egui::RichText::new(cmd_predeterminado).strong().color(egui::Color32::WHITE).monospace());
        //
        // MODO 2 (PTY Real Linux con portable-pty):
        // std::thread::spawn(move || { ... portable_pty::NativePtySystem ... });
         */
    });
}


fn obtener_repos_base_dir(term_cwd: &std::path::Path) -> std::path::PathBuf {
    let default_repos = std::path::Path::new("/home/alek/VNC/repos");
    if default_repos.exists() && default_repos.is_dir() {
        default_repos.to_path_buf()
    } else if term_cwd.exists() && term_cwd.is_dir() {
        if term_cwd.file_name().is_some_and(|n| n == "egui_vnc") {
            term_cwd.parent().unwrap_or(term_cwd).to_path_buf()
        } else {
            term_cwd.to_path_buf()
        }
    } else {
        std::path::PathBuf::from("/home/alek/VNC/repos")
    }
}


pub fn buscar_ruta_proyecto(base_path: &std::path::Path, proj_name: &str) -> std::path::PathBuf {
    let candidate1 = base_path.join(proj_name);
    if candidate1.exists() {
        return candidate1;
    }
    let repos_dir = obtener_repos_base_dir(base_path);
    let candidate2 = repos_dir.join(proj_name);
    if candidate2.exists() {
        return candidate2;
    }

    if let Ok(entries) = std::fs::read_dir(&repos_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == proj_name) {
                    return path;
                }
                let nested = path.join(proj_name);
                if nested.exists() {
                    return nested;
                }
            }
        }
    }
    base_path.join(proj_name)
}


pub fn listar_proyectos_cargo(base_path: &std::path::Path) -> Vec<String> {
    let mut proyectos = Vec::new();
    let mut dirs_to_scan = Vec::new();

    let repos_dir = obtener_repos_base_dir(base_path);
    dirs_to_scan.push(repos_dir.clone());

    if base_path.exists() && base_path.is_dir() {
        if !dirs_to_scan.contains(&base_path.to_path_buf()) {
            dirs_to_scan.push(base_path.to_path_buf());
        }
        if let Some(parent) = base_path.parent() {
            let p_buf = parent.to_path_buf();
            if !dirs_to_scan.contains(&p_buf) {
                dirs_to_scan.push(p_buf);
            }
        }
    }

    for dir_to_scan in dirs_to_scan {
        if let Ok(entries) = std::fs::read_dir(&dir_to_scan) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let es_valido = path.join("Cargo.toml").exists()
                        || path.join("src/main.rs").exists()
                        || path.join("src/lib.rs").exists();

                    if es_valido {
                        if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                            if !proyectos.contains(&folder_name.to_string()) {
                                proyectos.push(folder_name.to_string());
                            }
                        }
                    } else if let Ok(sub_entries) = std::fs::read_dir(&path) {
                        for sub_entry in sub_entries.flatten() {
                            let sub_path = sub_entry.path();
                            if sub_path.is_dir() {
                                let sub_valido = sub_path.join("Cargo.toml").exists()
                                    || sub_path.join("src/main.rs").exists()
                                    || sub_path.join("src/lib.rs").exists();
                                if sub_valido {
                                    if let Some(sub_folder) =
                                        sub_path.file_name().and_then(|n| n.to_str())
                                    {
                                        if !proyectos.contains(&sub_folder.to_string()) {
                                            proyectos.push(sub_folder.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    proyectos.sort();
    proyectos
}


pub fn mostrar_selector_proyectos_estandar(
    ui: &mut egui::Ui,
    selected_project: &mut Option<String>,
    term_cwd: &mut std::path::PathBuf,
    combo_id: &str,
    code_target: &mut String,
) {
    let proyectos_disponibles = listar_proyectos_cargo(term_cwd);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Selecciona un proyecto:")
                .strong()
                .color(egui::Color32::WHITE),
        );

        let label_seleccionado = match selected_project {
            Some(p) => p.as_str(),
            None => " Selecciona un proyecto ",
        };

        egui::ComboBox::from_id_salt(combo_id)
            .selected_text(
                egui::RichText::new(label_seleccionado)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            )
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(selected_project.is_none(), " Ninguno ")
                    .clicked()
                {
                    *selected_project = None;
                }
                ui.separator();
                for proj in &proyectos_disponibles {
                    let es_sel = selected_project.as_ref() == Some(proj);
                    if ui.selectable_label(es_sel, proj).clicked() {
                        *selected_project = Some(proj.clone());
                        let proj_dir = buscar_ruta_proyecto(term_cwd, proj);
                        *term_cwd = proj_dir.clone();

                        let main_rs = proj_dir.join("src/main.rs");
                        let lib_rs = proj_dir.join("src/lib.rs");
                        let file_to_read = if main_rs.exists() { main_rs } else { lib_rs };
                        if let Ok(content) = std::fs::read_to_string(file_to_read) {
                            *code_target = content;
                        }
                    }
                }
            });
    });
}


pub fn ejecutar_cargo_run_proyecto(state: &mut PortfolioState, ctx: &egui::Context) {
    let proj_dir = if let Some(ref proj) = state.selected_project {
        buscar_ruta_proyecto(&state.term_cwd, proj)
    } else {
        obtener_repos_base_dir(&state.term_cwd)
    };

    let main_rs = proj_dir.join("src/main.rs");
    let lib_rs = proj_dir.join("src/lib.rs");
    let target_file = if main_rs.exists() {
        main_rs
    } else if lib_rs.exists() {
        lib_rs
    } else {
        main_rs.clone()
    };

    // Obtener el código y el búfer de salida de la lección activa
    let codigo_activo = match state.ruta_actual {
        AppRoute::TutorialControlFlujo => state.controlflujo_code.clone(),
        AppRoute::TutorialTiposDatos => state.datatypes_code.clone(),
        AppRoute::TutorialStrings => state.strings_code.clone(),
        AppRoute::Playground => state.playground_code.clone(),
        _ => state.conceptos_code.clone(),
    };

    let output_arc = match state.ruta_actual {
        AppRoute::TutorialControlFlujo => Arc::clone(&state.controlflujo_output),
        AppRoute::TutorialTiposDatos => Arc::clone(&state.datatypes_output),
        AppRoute::TutorialStrings => Arc::clone(&state.strings_output),
        AppRoute::Playground => Arc::clone(&state.playground_output),
        _ => Arc::clone(&state.conceptos_output),
    };

    if state.selected_project.is_some() && target_file.parent().is_some_and(|p| p.exists()) {
        let _ = std::fs::write(&target_file, &codigo_activo);
    }

    if let Ok(mut out) = output_arc.lock() {
        *out = "Compilando y ejecutando con Cargo (cargo run)...".to_string();
    }

    // Abrir inmediatamente la ventana modal flotante centrada en la pantalla
    state.show_cargo_output_modal.store(true, Ordering::Relaxed);

    let history_arc = Arc::clone(&state.term_history);
    let ctx_clone = ctx.clone();
    let is_proj = state.selected_project.is_some();

    std::thread::spawn(move || {
        let output = if is_proj && proj_dir.exists() {
            std::process::Command::new("cargo")
                .arg("run")
                .current_dir(&proj_dir)
                .output()
        } else {
            std::process::Command::new("cargo")
                .arg("run")
                .current_dir(obtener_repos_base_dir(&proj_dir))
                .output()
        };

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let mut combined = stdout.into_owned();
                if !stderr.is_empty() {
                    if !combined.is_empty() {
                        combined.push_str("\n\n");
                    }
                    combined.push_str("[Compilador / Warnings / Errores]:\n");
                    combined.push_str(&stderr);
                }
                if combined.is_empty() {
                    combined = "El programa terminó exitosamente sin salidas.".to_string();
                }
                if let Ok(mut out_lock) = output_arc.lock() {
                    *out_lock = combined;
                }
                if let Ok(mut history) = history_arc.lock() {
                    history.push(format!("$ cargo run (en {})", proj_dir.display()));
                }
            }
            Err(err) => {
                if let Ok(mut out_lock) = output_arc.lock() {
                    *out_lock = format!("Error al ejecutar cargo run: {}", err);
                }
            }
        }
        ctx_clone.request_repaint();
    });
}


pub fn mostrar_contenido_tipos_primitivos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    // Selector de Categoría (Enteros, Decimales, Bool, Char)
    ui.horizontal(|ui| {
        for (cat_idx, (cat_label, cat_color)) in [
            ("Enteros", egui::Color32::from_rgb(255, 160, 50)),
            ("Decimales", egui::Color32::from_rgb(255, 160, 50)),
            ("Booleanos", egui::Color32::from_rgb(255, 160, 50)),
            ("Caracteres", egui::Color32::from_rgb(255, 160, 50)),
        ]
        .iter()
        .enumerate()
        {
            let es_sel = state.tipo_primitivo_categoria == cat_idx;
            let text_rich = egui::RichText::new(*cat_label).strong().color(if es_sel {
                *cat_color
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            });
            if ui.add(egui::Button::new(text_rich).frame(es_sel)).clicked() {
                state.tipo_primitivo_categoria = cat_idx;
            }
            ui.add_space(4.0);
        }
    });

    ui.add_space(12.0);

    match state.tipo_primitivo_categoria {
        0 => mostrar_categoria_enteros(ui),
        1 => mostrar_categoria_flotantes(ui),
        2 => mostrar_categoria_booleanos(ui),
        _ => mostrar_categoria_caracteres(ui),
    }
}


fn centrar_texto_en_rectangulos(raw_svg: &str) -> String {
    let clean_svg = raw_svg.replace(">\n", ">").replace(">\r\n", ">");

    let mut output = String::with_capacity(clean_svg.len());
    let mut search_idx = 0;

    while let Some(g_start) = clean_svg[search_idx..].find("<g class=") {
        let abs_g_start = search_idx + g_start;
        let g_substr = &clean_svg[abs_g_start..];

        if g_substr.starts_with("<g class=\"terminal\"")
            || g_substr.starts_with("<g class=\"nonterminal\"")
        {
            if let Some(g_end) = g_substr.find("</g>") {
                let abs_g_end = abs_g_start + g_end + 4;
                let group_block = &clean_svg[abs_g_start..abs_g_end];

                let mut rx = 0.0f32;
                let mut ry = 0.0f32;
                let mut rw = 0.0f32;
                let mut rh = 0.0f32;

                if let Some(r_pos) = group_block.find("<rect ") {
                    let r_sub = &group_block[r_pos..];
                    if let Some(r_close) = r_sub.find('>') {
                        let r_tag = &r_sub[..r_close];
                        for attr in r_tag.split_whitespace() {
                            if let Some((k, v)) = attr.split_once('=') {
                                let val = v
                                    .trim_matches('"')
                                    .trim_matches('\'')
                                    .trim_matches('>')
                                    .trim_matches('/');
                                match k {
                                    "x" => rx = val.parse().unwrap_or(0.0),
                                    "y" => ry = val.parse().unwrap_or(0.0),
                                    "width" => rw = val.parse().unwrap_or(0.0),
                                    "height" => rh = val.parse().unwrap_or(0.0),
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                if rw > 0.0 && rh > 0.0 {
                    if let (Some(t_start), Some(t_end)) =
                        (group_block.find("<text "), group_block.find("</text>"))
                    {
                        let text_sub = &group_block[t_start..t_end + 7];
                        if let Some(tag_close) = text_sub.find('>') {
                            let content = text_sub[tag_close + 1..text_sub.len() - 7].trim();

                            let cx = rx + rw / 2.0;
                            let cy = ry + rh / 2.0;

                            let is_terminal = group_block.contains("class=\"terminal\"");
                            let fill_color = if is_terminal { "#ffb347" } else { "#64c8ff" };

                            let new_group = format!(
                                "{}\n<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-family=\"sans-serif\" font-size=\"12\" font-weight=\"normal\" text-anchor=\"middle\" dominant-baseline=\"central\" stroke=\"none\">{}</text>\n</g>",
                                &group_block[..t_start].trim_end(),
                                cx,
                                cy,
                                fill_color,
                                content
                            );
                            output.push_str(&clean_svg[search_idx..abs_g_start]);
                            output.push_str(&new_group);
                            search_idx = abs_g_end;
                            continue;
                        }
                    }
                }
            }
        }

        output.push_str(&clean_svg[search_idx..abs_g_start + 8]);
        search_idx = abs_g_start + 8;
    }

    output.push_str(&clean_svg[search_idx..]);
    output
}


fn generar_railroad_color_image() -> Option<egui::ColorImage> {
    use railroad::*;

    let mut seq = Sequence::default();
    let e1: Box<dyn railroad::Node> = Box::new(Terminal::new("let".to_string()));
    let e2: Box<dyn railroad::Node> = Box::new(Optional::new(Terminal::new("mut".to_string())));
    let e3: Box<dyn railroad::Node> = Box::new(NonTerminal::new("identificador".to_string()));
    let e4_sub1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
    let e4_sub2: Box<dyn railroad::Node> = Box::new(NonTerminal::new("tipo".to_string()));
    let e4: Box<dyn railroad::Node> =
        Box::new(Optional::new(Sequence::new(vec![e4_sub1, e4_sub2])));
    let e5: Box<dyn railroad::Node> = Box::new(Terminal::new("=".to_string()));
    let e6: Box<dyn railroad::Node> = Box::new(NonTerminal::new("expresion".to_string()));
    let e7: Box<dyn railroad::Node> = Box::new(Terminal::new(";".to_string()));

    seq.push(e1);
    seq.push(e2);
    seq.push(e3);
    seq.push(e4);
    seq.push(e5);
    seq.push(e6);
    seq.push(e7);

    let dia = Diagram::new(seq);
    let mut raw_svg = dia.to_string();

    if !raw_svg.contains("width=") {
        raw_svg = raw_svg.replace(
            "<svg ",
            "<svg width=\"626\" height=\"60\" xmlns=\"http://www.w3.org/2000/svg\" ",
        );
    } else if !raw_svg.contains("xmlns=") {
        raw_svg = raw_svg.replace("<svg ", "<svg xmlns=\"http://www.w3.org/2000/svg\" ");
    }

    raw_svg = raw_svg
        .replace(
            "<g class=\"terminal\">",
            "<g class=\"terminal\" fill=\"#1e2638\" stroke=\"#ff9d00\" stroke-width=\"2\">",
        )
        .replace(
            "<g class=\"nonterminal\">",
            "<g class=\"nonterminal\" fill=\"#1a2336\" stroke=\"#64c8ff\" stroke-width=\"2\">",
        )
        .replace(
            "<path ",
            "<path stroke=\"#64c8ff\" stroke-width=\"2.5\" fill=\"none\" ",
        );

    raw_svg = centrar_texto_en_rectangulos(&raw_svg);

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    let opt = usvg::Options {
        fontdb: fontdb.into(),
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_str(&raw_svg, &opt).ok()?;
    let width = tree.size().width().ceil() as u32;
    let height = tree.size().height().ceil() as u32;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width.max(1), height.max(1))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    let pixels = pixmap.data();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        pixels,
    ))
}


pub fn mostrar_contenido_macros(ui: &mut egui::Ui) {
    ui.heading(
        egui::RichText::new("Categorías de Macros en Rust")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(6.0);
    ui.label(
        "En Rust, las macros son herramientas de metaprogramación que generan código durante la compilación. Se distinguen fácilmente por llevar un signo de exclamación ! al final.",
    );
    ui.add_space(10.0);

    ui.columns(2, |cols| {
        let mut card1 = egui::Frame::new();
        card1.fill = egui::Color32::from_rgb(18, 22, 32);
        card1.inner_margin = egui::Margin::same(12);
        card1.corner_radius = egui::CornerRadius::same(8);
        card1.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 95));

        card1.show(&mut cols[0], |ui| {
            ui.heading(
                egui::RichText::new("1. Macros Declarativas (macro_rules!)")
                    .size(15.0)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.add_space(4.0);
            ui.label("Basadas en coincidencia de patrones (pattern matching) similares a una sentencia match. Permiten escribir código conciso como vec![] o println!.");
        });

        card1.show(&mut cols[1], |ui| {
            ui.heading(
                egui::RichText::new("2. Macros Procedurales (Derive, Atributos)")
                    .size(15.0)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(4.0);
            ui.label("Operan sobre el Árbol de Sintaxis Abstracta (AST) de Rust como código ejecutable durante la compilación (ej: #[derive(Debug, Serialize)]).");
        });
    });

    ui.add_space(16.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_macros_estandar_rust")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Macro")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Salida / Comportamiento")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                let macros_list = [
                    (
                        "println!",
                        "Impresión a Consola (stdout)",
                        "println!(\"Hola {name}\");",
                        "Escribe en consola con salto de línea al final (\\n).",
                    ),
                    (
                        "print!",
                        "Impresión Continua (stdout)",
                        "print!(\"Cargando...\");",
                        "Escribe en consola sin agregar salto de línea.",
                    ),
                    (
                        "format!",
                        "Creación de String Formateado",
                        "let s = format!(\"x = {}\", 10);",
                        "Devuelve un String dinámico sin imprimir en consola.",
                    ),
                    (
                        "eprintln!",
                        "Impresión de Errores (stderr)",
                        "eprintln!(\"Error: {}\", err);",
                        "Escribe en la salida estándar de errores stderr.",
                    ),
                    (
                        "dbg!",
                        "Macro de Depuración Nativa",
                        "let y = dbg!(x * 2);",
                        "Imprime archivo, línea, expresión y devuelve el valor.",
                    ),
                    (
                        "vec!",
                        "Creación de Vectores Dinámicos",
                        "let v = vec![1, 2, 3];",
                        "Sintaxis conveniente para inicializar un Vec<T>.",
                    ),
                    (
                        "panic!",
                        "Interrupción de Emergencia",
                        "panic!(\"Fallo crítico\");",
                        "Detiene la ejecución del hilo enviando un mensaje de pánico.",
                    ),
                    (
                        "assert_eq!",
                        "Verificación de Pruebas",
                        "assert_eq!(a, b);",
                        "Valida igualdad en tests; entra en pánico si son distintos.",
                    ),
                ];

                for (m_name, m_prop, m_code, m_desc) in macros_list {
                    ui.label(
                        egui::RichText::new(m_name)
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label(
                        egui::RichText::new(m_prop).color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(m_code)
                            .monospace()
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                    ui.label(m_desc);
                    ui.end_row();
                }
            });
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Depuración")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(6.0);
    ui.label(
        "Rust ofrece potentes mecanismos de formateo e inspección de variables. Conocer la diferencia entre Display {}, Debug {:?}, Pretty Debug {:#?} y dbg! es clave para el desarrollo diario.",
    );
    ui.add_space(10.0);

    let mut fmt_frame = egui::Frame::new();
    fmt_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    fmt_frame.inner_margin = egui::Margin::same(12);
    fmt_frame.corner_radius = egui::CornerRadius::same(8);
    fmt_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    fmt_frame.show(ui, |ui| {
        egui::Grid::new("tabla_debug_formato_rust")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Formato / Herramienta")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Especificador")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Uso Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Display {}
                ui.label(
                    egui::RichText::new("Display")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Formato amigable para el usuario final");
                ui.label(
                    egui::RichText::new("println!(\"Score: {}\", puntos);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Debug {:?}
                ui.label(
                    egui::RichText::new("Debug")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{:?}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Inspección técnica de 1 sola línea");
                ui.label(
                    egui::RichText::new("println!(\"{:?}\", arreglo);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Pretty Debug {:#?}
                ui.label(
                    egui::RichText::new("Pretty Debug")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{:#?}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Inspección multilínea indentada (estructuras compuestas)");
                ui.label(
                    egui::RichText::new("println!(\"{:#?}\", persona);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Macro dbg!
                ui.label(
                    egui::RichText::new("Macro dbg!")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("dbg!(exp)")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Imprime archivo, nº línea, expresión y devuelve el valor");
                ui.label(
                    egui::RichText::new("let b = dbg!(a + 5);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    let mut tip_frame = egui::Frame::new();
    tip_frame.fill = egui::Color32::from_rgb(20, 28, 42);
    tip_frame.inner_margin = egui::Margin::same(14);
    tip_frame.corner_radius = egui::CornerRadius::same(8);
    tip_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 90, 140));

    tip_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new("A diferencia de println!, dbg! toma la propiedad de la expresión, imprime su ubicación exacta en el código fuente, el resultado de la expresión, y devuelve el valor evaluado. ¡Eso te permite envolver llamadas intermedias sin romper tu código!")
                .color(egui::Color32::from_rgb(200, 230, 255)),
        );
    });

    ui.add_space(16.0);
    ui.heading(
        egui::RichText::new("🛤️ Generador de Diagramas de Sintaxis SVG (railroad)")
            .size(16.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(6.0);
    ui.label("La crate 'railroad' genera dinámicamente gráficos de sintaxis tipo ferrocarril (Railroad Diagram) en formato SVG a partir de reglas sintácticas:");
    ui.add_space(10.0);

    let mut railroad_frame = egui::Frame::new();
    railroad_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    railroad_frame.inner_margin = egui::Margin::same(12);
    railroad_frame.corner_radius = egui::CornerRadius::same(8);
    railroad_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    railroad_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new(
                "Diagrama de Sintaxis Generado para 'let [mut] ident [: tipo] = expr;':",
            )
            .strong()
            .color(egui::Color32::from_rgb(100, 200, 255)),
        );
        ui.add_space(8.0);

        ui.add(
            egui::Image::from_bytes(
                "bytes://diagrama_let.svg",
                include_bytes!("../../diagramas/diagrama_let.svg"),
            )
            .fit_to_exact_size(egui::vec2(650.0, 80.0))
            .corner_radius(egui::CornerRadius::same(6)),
        );

        ui.add_space(8.0);
        if ui
            .button(
                egui::RichText::new("🌐 Abrir Diagrama SVG NATIVO en Navegador Web")
                    .strong()
                    .color(egui::Color32::from_rgb(100, 220, 255)),
            )
            .clicked()
        {
            let _ = std::process::Command::new("xdg-open")
                .arg("/home/alek/VNC/repos/egui_vnc/diagramas/ver_diagrama.html")
                .spawn();
        }
    });
}


fn generar_railroad_desde_codigo(codigo: &str) -> Option<egui::ColorImage> {
    use railroad::*;

    let mut mut_token: Option<String> = None;
    let mut ident_token = "variable".to_string();
    let mut tipo_token: Option<String> = None;
    let mut expr_token = "expresion".to_string();
    let mut tiene_punto_y_coma = false;

    let mut encontrado = false;
    for orig_line in codigo.lines() {
        let line_without_comment = match orig_line.split_once("//") {
            Some((code, _)) => code,
            None => orig_line,
        };
        let trimmed = line_without_comment.trim();
        if trimmed.starts_with("let ") || trimmed.starts_with("let\t") || trimmed == "let" {
            encontrado = true;
            let mut rest = if trimmed.starts_with("let ") || trimmed.starts_with("let\t") {
                trimmed["let".len()..].trim()
            } else {
                ""
            };

            if rest.contains(';') {
                tiene_punto_y_coma = true;
                rest = rest.trim_end_matches(';').trim();
            }

            let (pat_part, expr_part) = match rest.split_once('=') {
                Some((p, e)) => (p.trim(), e.trim()),
                None => (rest, ""),
            };

            if !expr_part.is_empty() {
                expr_token = expr_part.to_string();
            }

            let mut pat_str = pat_part;
            if pat_str.starts_with("mut ") || pat_str.starts_with("mut\t") || pat_str == "mut" {
                mut_token = Some("mut".to_string());
                if pat_str.len() > 3 {
                    pat_str = pat_str[3..].trim();
                } else {
                    pat_str = "";
                }
            }

            if let Some((ident, ty)) = pat_str.split_once(':') {
                if !ident.trim().is_empty() {
                    ident_token = ident.trim().to_string();
                }
                if !ty.trim().is_empty() {
                    tipo_token = Some(ty.trim().to_string());
                }
            } else if !pat_str.trim().is_empty() {
                ident_token = pat_str.trim().to_string();
            }
            break;
        }
    }

    if !encontrado {
        return None;
    }

    let mut seq = Sequence::default();
    let e1: Box<dyn railroad::Node> = Box::new(Terminal::new("let".to_string()));
    let e2: Box<dyn railroad::Node> = match mut_token {
        Some(m) => Box::new(Terminal::new(m)),
        None => Box::new(Optional::new(Terminal::new("mut".to_string()))),
    };
    let e3: Box<dyn railroad::Node> = Box::new(NonTerminal::new(ident_token));

    let e4: Box<dyn railroad::Node> = match tipo_token {
        Some(ty) => {
            let n1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
            let n2: Box<dyn railroad::Node> = Box::new(NonTerminal::new(ty));
            Box::new(Sequence::new(vec![n1, n2]))
        }
        None => {
            let e4_sub1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
            let e4_sub2: Box<dyn railroad::Node> = Box::new(NonTerminal::new("tipo".to_string()));
            Box::new(Optional::new(Sequence::new(vec![e4_sub1, e4_sub2])))
        }
    };

    let e5: Box<dyn railroad::Node> = Box::new(Terminal::new("=".to_string()));
    let e6: Box<dyn railroad::Node> = Box::new(NonTerminal::new(expr_token));
    let e7: Box<dyn railroad::Node> = if tiene_punto_y_coma {
        Box::new(Terminal::new(";".to_string()))
    } else {
        Box::new(Optional::new(Terminal::new(";".to_string())))
    };

    seq.push(e1);
    seq.push(e2);
    seq.push(e3);
    seq.push(e4);
    seq.push(e5);
    seq.push(e6);
    seq.push(e7);

    let dia = Diagram::new(seq);
    let mut raw_svg = dia.to_string();

    if !raw_svg.contains("width=") {
        raw_svg = raw_svg.replace(
            "<svg ",
            "<svg width=\"650\" height=\"60\" xmlns=\"http://www.w3.org/2000/svg\" ",
        );
    } else if !raw_svg.contains("xmlns=") {
        raw_svg = raw_svg.replace("<svg ", "<svg xmlns=\"http://www.w3.org/2000/svg\" ");
    }

    raw_svg = raw_svg
        .replace(
            "<g class=\"terminal\">",
            "<g class=\"terminal\" fill=\"#1e2638\" stroke=\"#ff9d00\" stroke-width=\"2\">",
        )
        .replace(
            "<g class=\"nonterminal\">",
            "<g class=\"nonterminal\" fill=\"#1a2336\" stroke=\"#64c8ff\" stroke-width=\"2\">",
        )
        .replace(
            "<path ",
            "<path stroke=\"#64c8ff\" stroke-width=\"2.5\" fill=\"none\" ",
        );

    raw_svg = centrar_texto_en_rectangulos(&raw_svg);

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    let opt = usvg::Options {
        fontdb: fontdb.into(),
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_str(&raw_svg, &opt).ok()?;
    let width = tree.size().width().ceil() as u32;
    let height = tree.size().height().ceil() as u32;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width.max(1), height.max(1))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    let pixels = pixmap.data();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        pixels,
    ))
}


pub fn mostrar_tutorial_conceptos_basicos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Comenzando con Rust")
                .size(28.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 160, 50)),
        );
    });

    ui.add_space(15.0);

    // Barra de navegación con el mismo patrón unificado que Pilares
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );

        let tabs_practica = [(0, "Inmutabilidad"), (1, "const y static")];
        for (indice, texto) in tabs_practica {
            let es_activo = state.conceptos_tab == indice;
            let text_color = if es_activo {
                egui::Color32::from_rgb(255, 160, 50)
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(texto).strong().color(text_color))
                        .frame(es_activo),
                )
                .clicked()
            {
                state.conceptos_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let tabs_teoria = [(3, "Macro & Format"), (2, "Data Types")];
            for (indice, texto) in tabs_teoria {
                let es_activo = state.conceptos_tab == indice;
                let text_color = if es_activo {
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(texto).strong().color(text_color))
                            .frame(es_activo),
                    )
                    .clicked()
                {
                    state.conceptos_tab = indice;
                }
                ui.add_space(4.0);
            }
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

    match state.conceptos_tab {
        0 => {
            ui.label(
                "En Rust, las variables son inmutables por defecto. Esto garantiza seguridad de memoria, previene condiciones de carrera en concurrencia (data races) y obliga a declarar explícitamente con 'mut' cuando un valor necesita cambiar.",
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let mut table_frame = egui::Frame::new();
                table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                table_frame.inner_margin = egui::Margin::same(12);
                table_frame.corner_radius = egui::CornerRadius::same(8);
                table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                table_frame.show(ui, |ui| {
                    egui::Grid::new("tabla_let_mut")
                        .striped(true)
                        .spacing([25.0, 8.0])
                        .show(ui, |ui| {
                            // Encabezados
                            ui.label(egui::RichText::new("Declaración").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Mutabilidad").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Ejemplo de Código").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Diagrama").strong().color(egui::Color32::WHITE));
                            ui.end_row();

                            // Fila 1: let (Inmutable)
                            let btn_color_0 = if state.show_railroad_modal == Some(0) {
                                egui::Color32::from_rgb(255, 160, 50)
                            } else {
                                egui::Color32::from_rgb(180, 190, 205)
                            };

                            ui.label(egui::RichText::new("let").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label(egui::RichText::new("No").strong().color(egui::Color32::from_rgb(180, 190, 205)));
                            ui.label(egui::RichText::new("let x = 5;").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label("No puede reasignarse dentro de su alcance salvo que se declare con mut.");
                            if ui
                                .add(
                                    egui::Button::image(
                                        egui::Image::from_bytes(
                                            "bytes://view.svg",
                                            include_bytes!("../../diagramas/view.svg"),
                                        )
                                        .fit_to_exact_size(egui::vec2(20.0, 20.0))
                                        .tint(btn_color_0),
                                    )
                                    .frame(state.show_railroad_modal == Some(0)),
                                )
                                .on_hover_text("Ver diagrama Railroad de sintaxis (let inmutable)")
                                .clicked()
                            {
                                state.show_railroad_modal = if state.show_railroad_modal == Some(0) { None } else { Some(0) };
                            }
                            ui.end_row();

                            // Fila 2: let mut (Mutable)
                            let btn_color_1 = if state.show_railroad_modal == Some(1) {
                                egui::Color32::from_rgb(255, 160, 50)
                            } else {
                                egui::Color32::from_rgb(180, 190, 205)
                            };

                            ui.label(egui::RichText::new("let mut").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label(egui::RichText::new("Sí").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label(egui::RichText::new("let mut x = 5;\nx = 10;").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label("Permite reasignar un nuevo valor a la misma variable de forma explícita.");
                            if ui
                                .add(
                                    egui::Button::image(
                                        egui::Image::from_bytes(
                                            "bytes://view.svg",
                                            include_bytes!("../../diagramas/view.svg"),
                                        )
                                        .fit_to_exact_size(egui::vec2(20.0, 20.0))
                                        .tint(btn_color_1),
                                    )
                                    .frame(state.show_railroad_modal == Some(1)),
                                )
                                .on_hover_text("Ver diagrama Railroad de sintaxis (let mut mutable)")
                                .clicked()
                            {
                                state.show_railroad_modal = if state.show_railroad_modal == Some(1) { None } else { Some(1) };
                            }
                            ui.end_row();
                        });
                });

                ui.add_space(15.0);

                // Imagen de Ferris Crab
                ui.add(
                    egui::Image::new(egui::include_image!("../../assets/taller/1.png"))
                        .max_height(140.0)
                        .texture_options(egui::TextureOptions::LINEAR)
                        .corner_radius(egui::CornerRadius::same(8)),
                );
            });
        }
        1 => {
            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_const_static")
                    .striped(true)
                    .spacing([25.0, 8.0])
                    .show(ui, |ui| {
                        // Encabezados
                        ui.label(
                            egui::RichText::new("Declaración")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Ubicación en Memoria")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Mutabilidad")
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

                        // Fila 1: const
                        ui.label(
                            egui::RichText::new("const")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label("Valor de compilación");
                        ui.label(
                            egui::RichText::new("No")
                                .strong()
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                        ui.label(
                            egui::RichText::new("const MAX: u32 = 100;")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label("Su valor debe poder evaluarse durante la compilación.");
                        ui.end_row();

                        // Fila 2: static
                        ui.label(
                            egui::RichText::new("static")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label("Dirección Única en RAM");
                        ui.label(
                            egui::RichText::new("No por defecto")
                                .strong()
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                        ui.label(
                            egui::RichText::new("static VALOR: &str = \"OK\";")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label(
                            "Tiene una ubicación estable y vive durante todo el programa; static mut sí requiere unsafe.",
                        );
                        ui.end_row();
                    });
            });
        }
        2 => {
            mostrar_contenido_tipos_primitivos(ui, state);
        }
        _ => {
            mostrar_contenido_macros(ui);
        }
    }

    // El selector de proyectos y el editor de código solo se muestran en las pestañas interactivas de código (0 e 1)
    if state.conceptos_tab < 2 {
        ui.add_space(15.0);

        mostrar_selector_proyectos_estandar(
            ui,
            &mut state.selected_project,
            &mut state.term_cwd,
            "combo_proyectos_comenzando",
            &mut state.conceptos_code,
        );

        ui.add_space(10.0);

        let theme = &state.theme_set.themes["base16-ocean.dark"];
        mostrar_editor_interactivo(
            ui,
            &mut state.conceptos_code,
            Arc::clone(&state.conceptos_output),
            "",
            ejecutar_codigo_rust,
            &state.syntax_set,
            theme,
        );
    }
}

#[derive(Clone, Copy)]
