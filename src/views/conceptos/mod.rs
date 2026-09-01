use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;
#[allow(unused_imports)]
use crate::routes::AppRoute;
pub mod primitivos;
use self::primitivos::{
    mostrar_categoria_booleanos, mostrar_categoria_caracteres, mostrar_categoria_casting,
    mostrar_categoria_enteros, mostrar_categoria_flotantes,
};
pub mod mutabilidad;
pub mod scopes;
pub mod globales;
pub mod statements;
pub mod funciones;
use std::sync::Arc;
use std::sync::atomic::Ordering;

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

            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("👁️").size(13.0),
                    )
                    .frame(false),
                )
                .on_hover_text(format!(
                    "{} historial de comandos ({})",
                    if state.show_terminal_history { "Ocultar" } else { "Mostrar" },
                    history_len
                ))
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
                    .auto_shrink([false, false])
                    .id_salt("scroll_terminal_history")
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
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
                        let output_arc = state.obtener_output_activo();
                        let modal_arc = Arc::clone(&state.show_cargo_output_modal);
                        let cwd = state.term_cwd.clone();
                        let cmd = cmd_str.clone();
                        let ctx = ui.ctx().clone();

                        if cmd.starts_with("cargo ") {
                            if let Ok(mut out) = output_arc.lock() {
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
                                        // Mostrar siempre la salida real del comando, incluido cargo new.
                                        if debe_mostrar_modal_cargo(&cmd) {
                                            modal_arc.store(true, Ordering::Relaxed);
                                        }
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

fn debe_mostrar_modal_cargo(command: &str) -> bool {
    let mut parts = command.split_whitespace();
    if parts.next() != Some("cargo") {
        return false;
    }

    let subcommand = match parts.next() {
        Some("+stable" | "+beta" | "+nightly") => parts.next(),
        other => other,
    };

    matches!(
        subcommand,
        Some(
            "new"
                | "init"
                | "check"
                | "build"
                | "run"
                | "test"
                | "clippy"
                | "doc"
                | "bench"
                | "clean"
                | "expand"
        )
    )
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

pub fn listar_archivos_proyecto(proj_dir: &std::path::Path) -> Vec<String> {
    let mut archivos = Vec::new();
    scan_project_files(proj_dir, proj_dir, &mut archivos);
    archivos.sort();
    archivos
}

fn scan_project_files(
    root: &std::path::Path,
    current_dir: &std::path::Path,
    results: &mut Vec<String>,
) {
    if let Ok(entries) = std::fs::read_dir(current_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
            if name == ".git" || name == "target" || name == "Cargo.lock" {
                continue;
            }

            if path.is_dir() {
                scan_project_files(root, &path, results);
            } else if let Ok(relative) = path.strip_prefix(root) {
                results.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileTreeNode {
    Directory {
        name: String,
        children: Vec<FileTreeNode>,
    },
    File {
        name: String,
        rel_path: String,
    },
}

pub fn build_file_tree(paths: &[String]) -> Vec<FileTreeNode> {
    let mut root_children: Vec<FileTreeNode> = Vec::new();

    for path_str in paths {
        let parts: Vec<&str> = path_str.split('/').collect();
        insert_into_file_tree(&mut root_children, &parts, path_str);
    }

    sort_file_tree(&mut root_children);
    root_children
}

fn insert_into_file_tree(nodes: &mut Vec<FileTreeNode>, parts: &[&str], full_path: &str) {
    if parts.is_empty() {
        return;
    }
    if parts.len() == 1 {
        nodes.push(FileTreeNode::File {
            name: parts[0].to_string(),
            rel_path: full_path.to_string(),
        });
    } else {
        let dir_name = parts[0];
        let rest = &parts[1..];
        if let Some(existing) = nodes.iter_mut().find(|n| match n {
            FileTreeNode::Directory { name, .. } => name == dir_name,
            _ => false,
        }) {
            if let FileTreeNode::Directory { children, .. } = existing {
                insert_into_file_tree(children, rest, full_path);
            }
        } else {
            let mut children = Vec::new();
            insert_into_file_tree(&mut children, rest, full_path);
            nodes.push(FileTreeNode::Directory {
                name: dir_name.to_string(),
                children,
            });
        }
    }
}

fn sort_file_tree(nodes: &mut [FileTreeNode]) {
    nodes.sort_by(|a, b| {
        match (a, b) {
            (FileTreeNode::Directory { name: na, .. }, FileTreeNode::Directory { name: nb, .. }) => na.cmp(nb),
            (FileTreeNode::Directory { .. }, FileTreeNode::File { .. }) => std::cmp::Ordering::Less,
            (FileTreeNode::File { .. }, FileTreeNode::Directory { .. }) => std::cmp::Ordering::Greater,
            (FileTreeNode::File { name: na, .. }, FileTreeNode::File { name: nb, .. }) => na.cmp(nb),
        }
    });
    for node in nodes.iter_mut() {
        if let FileTreeNode::Directory { children, .. } = node {
            sort_file_tree(children);
        }
    }
}

fn render_file_tree(
    ui: &mut egui::Ui,
    nodes: &[FileTreeNode],
    selected_file: &mut Option<String>,
    proj_dir_opt: Option<&std::path::PathBuf>,
    code_target: &mut String,
    close_popup: &mut bool,
    alpha: u8,
    depth: usize,
    combo_id: &str,
) {
    for node in nodes {
        match node {
            FileTreeNode::Directory { name, children } => {
                let dir_id = ui.make_persistent_id(format!("{}_tree_dir_{}_{}", combo_id, depth, name));
                let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(dir_id).unwrap_or(true));

                ui.horizontal(|ui| {
                    if depth > 0 {
                        ui.add_space((depth as f32) * 12.0);
                    }
                    let arrow = if is_open { "▾" } else { "▸" };
                    let btn_text = egui::RichText::new(format!("{} {}/", arrow, name))
                        .size(11.0)
                        .strong()
                        .color(egui::Color32::from_rgba_unmultiplied(140, 180, 220, alpha));

                    if ui.add(egui::Button::new(btn_text).frame(false)).clicked() {
                        is_open = !is_open;
                        ui.data_mut(|d| d.insert_temp(dir_id, is_open));
                    }
                });

                if is_open {
                    render_file_tree(
                        ui,
                        children,
                        selected_file,
                        proj_dir_opt,
                        code_target,
                        close_popup,
                        alpha,
                        depth + 1,
                        combo_id,
                    );
                }
            }
            FileTreeNode::File { name, rel_path } => {
                let is_sel = selected_file.as_ref() == Some(rel_path);
                ui.horizontal(|ui| {
                    if depth > 0 {
                        ui.add_space((depth as f32) * 12.0);
                    }
                    let txt_color = if is_sel {
                        egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha)
                    } else {
                        egui::Color32::from_rgba_unmultiplied(190, 205, 225, alpha)
                    };
                    let txt_file = egui::RichText::new(name).size(11.0).color(txt_color);

                    if ui.selectable_label(is_sel, txt_file).clicked() {
                        *selected_file = Some(rel_path.clone());
                        if let Some(proj_dir) = proj_dir_opt {
                            let target_file = proj_dir.join(rel_path);
                            if let Ok(content) = std::fs::read_to_string(&target_file) {
                                *code_target = content;
                            }
                        }
                        *close_popup = true;
                    }
                });
            }
        }
    }
}

#[allow(dead_code)]
pub fn mostrar_selector_proyectos_estandar(
    ui: &mut egui::Ui,
    selected_project: &mut Option<String>,
    term_cwd: &mut std::path::PathBuf,
    combo_id: &str,
    code_target: &mut String,
) {
    let mut dummy_file = None;
    mostrar_selector_proyectos_estandar_con_archivos(ui, selected_project, &mut dummy_file, term_cwd, combo_id, code_target);
}

fn pintar_icono_badge_tile(
    ui: &mut egui::Ui,
    img: egui::Image,
    activo: bool,
    color_hover: egui::Color32,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(32.0, 26.0), egui::Sense::click());
    let is_hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    // Fondo interactivo idéntico al resto de botones
    let bg_color = if is_down {
        egui::Color32::from_rgb(34, 46, 68)
    } else if activo {
        egui::Color32::from_rgb(26, 36, 54)
    } else if is_hovered {
        egui::Color32::from_rgb(24, 32, 48)
    } else {
        egui::Color32::from_rgb(16, 22, 32)
    };

    // Borde interactivo idéntico al resto de botones
    let border_stroke = if activo {
        egui::Stroke::new(1.0, color_hover)
    } else if is_hovered {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 95, 135))
    } else {
        egui::Stroke::new(1.0, egui::Color32::from_rgb(38, 54, 80))
    };

    // Color del icono SVG (tenue en reposo, se ilumina en Cyan con hover o activo)
    let icon_tint = if activo || is_hovered {
        color_hover
    } else {
        egui::Color32::from_rgb(160, 180, 205)
    };

    ui.painter().rect(rect, egui::CornerRadius::same(4), bg_color, border_stroke, egui::StrokeKind::Inside);
    let icon_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(15.0, 15.0));
    img.tint(icon_tint).paint_at(ui, icon_rect);

    response.on_hover_text(tooltip)
}

pub fn mostrar_selector_proyectos_estandar_con_archivos(
    ui: &mut egui::Ui,
    selected_project: &mut Option<String>,
    selected_file: &mut Option<String>,
    term_cwd: &mut std::path::PathBuf,
    combo_id: &str,
    code_target: &mut String,
) {
    let proyectos_disponibles = listar_proyectos_cargo(term_cwd);

    let mut proj_dir_opt = None;
    let mut archivos_disponibles = Vec::new();

    if let Some(proj) = selected_project.as_ref() {
        let proj_dir = buscar_ruta_proyecto(term_cwd, proj);
        archivos_disponibles = listar_archivos_proyecto(&proj_dir);
        proj_dir_opt = Some(proj_dir);
    }

    // Asegurar que selected_file tenga un valor válido si hay archivos
    if selected_file.is_none() && !archivos_disponibles.is_empty() {
        if archivos_disponibles.contains(&"src/main.rs".to_string()) {
            *selected_file = Some("src/main.rs".to_string());
        } else {
            *selected_file = Some(archivos_disponibles[0].clone());
        }
    }

    // Cargar contenido inicial del proyecto si el búfer compartido está vacío
    if let Some(proj_dir) = proj_dir_opt.as_ref() {
        if code_target.is_empty() {
            let file_rel = selected_file.as_deref().unwrap_or("src/main.rs");
            let target_file = proj_dir.join(file_rel);
            if target_file.exists() {
                if let Ok(real_content) = std::fs::read_to_string(&target_file) {
                    *code_target = real_content;
                }
            }
        }
    }

    ui.horizontal(|ui| {
        let cyan = egui::Color32::from_rgb(100, 200, 255);

        // 1. Botón Tile de Proyecto (Icono puro)
        let proj_popup_id = ui.make_persistent_id(format!("{}_proj_popup_menu", combo_id));
        let mut proj_popup_open = ui.data_mut(|d| d.get_temp::<bool>(proj_popup_id).unwrap_or(false));
        let proj_anim = ui.ctx().animate_bool(proj_popup_id, proj_popup_open);

        let img_folder = egui::Image::new(egui::include_image!("../../../assets/icons/folder-off-svgrepo-com.svg"))
            .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
        let btn_proj = pintar_icono_badge_tile(
            ui,
            img_folder,
            proj_popup_open,
            cyan,
            "Seleccionar Proyecto Cargo",
        );

        if btn_proj.clicked() {
            proj_popup_open = !proj_popup_open;
            ui.data_mut(|d| d.insert_temp(proj_popup_id, proj_popup_open));
        }

        // Cierre al hacer clic fuera o presionar Escape
        let proj_rect_id = proj_popup_id.with("rect");
        let last_proj_rect = ui.data_mut(|d| d.get_temp::<egui::Rect>(proj_rect_id).unwrap_or(egui::Rect::NOTHING));
        if proj_popup_open {
            let escape = ui.input(|i| i.key_pressed(egui::Key::Escape));
            let click_outside = ui.input(|i| {
                if i.pointer.any_click() {
                    if let Some(pos) = i.pointer.interact_pos() {
                        !btn_proj.rect.contains(pos) && !last_proj_rect.contains(pos)
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
            if escape || click_outside {
                ui.data_mut(|d| d.insert_temp(proj_popup_id, false));
            }
        }

        // Popup Dropdown de Proyectos animado con Area
        if proj_anim > 0.001 {
            let slide_y = (1.0 - proj_anim) * -6.0;
            let popup_pos = btn_proj.rect.left_bottom() + egui::vec2(0.0, 4.0 + slide_y);
            let mut close_popup = false;

            let alpha = (proj_anim * 255.0).clamp(0.0, 255.0) as u8;
            let bg_color = egui::Color32::from_rgba_unmultiplied(16, 22, 32, alpha);
            let border_color = egui::Color32::from_rgba_unmultiplied(45, 65, 95, alpha);

            egui::Area::new(proj_popup_id)
                .fixed_pos(popup_pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    // Sincronizar opacidad en todos los estilos visuales (textos, hovers, separadores)
                    ui.style_mut().visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(190, 205, 225, alpha);
                    ui.style_mut().visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha);
                    ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(30, 42, 60, alpha);
                    ui.style_mut().visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                    ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(40, 58, 85, alpha);
                    ui.style_mut().visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::from_rgba_unmultiplied(38, 54, 80, alpha);
                    ui.style_mut().visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(30, 46, 70, alpha);
                    ui.style_mut().visuals.selection.stroke.color = egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha);

                    egui::Frame::popup(ui.style())
                        .fill(bg_color)
                        .stroke(egui::Stroke::new(1.0, border_color))
                        .corner_radius(egui::CornerRadius::same(6))
                        .inner_margin(egui::Margin::same(6))
                        .show(ui, |ui| {
                            ui.data_mut(|d| d.insert_temp(proj_rect_id, ui.min_rect()));
                            ui.set_width(256.0);
                            ui.label(egui::RichText::new("PROYECTOS CARGO").size(9.5).strong().color(egui::Color32::from_rgba_unmultiplied(120, 145, 175, alpha)));
                            ui.separator();

                            egui::ScrollArea::vertical()
                                .max_height(150.0)
                                .auto_shrink([false, true])
                                .show(ui, |ui| {
                                    ui.style_mut().spacing.item_spacing.y = 2.0;

                                    let txt_libre = egui::RichText::new("Libre").color(if selected_project.is_none() {
                                        egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha)
                                    } else {
                                        egui::Color32::from_rgba_unmultiplied(190, 205, 225, alpha)
                                    });

                                    if ui.selectable_label(selected_project.is_none(), txt_libre).clicked() {
                                        *selected_project = None;
                                        *selected_file = None;
                                        close_popup = true;
                                    }

                                    for proj in &proyectos_disponibles {
                                        let es_sel = selected_project.as_ref() == Some(proj);
                                        let txt_proj = egui::RichText::new(proj).color(if es_sel {
                                            egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha)
                                        } else {
                                            egui::Color32::from_rgba_unmultiplied(190, 205, 225, alpha)
                                        });

                                        if ui.selectable_label(es_sel, txt_proj).clicked() {
                                            *selected_project = Some(proj.clone());
                                            let proj_dir = buscar_ruta_proyecto(term_cwd, proj);
                                            *term_cwd = proj_dir.clone();
                                            let nuevos_archivos = listar_archivos_proyecto(&proj_dir);
                                            let main_rel = "src/main.rs".to_string();
                                            if nuevos_archivos.contains(&main_rel) {
                                                *selected_file = Some(main_rel);
                                            } else if !nuevos_archivos.is_empty() {
                                                *selected_file = Some(nuevos_archivos[0].clone());
                                            }

                                            let target = if let Some(rel) = selected_file.as_ref() {
                                                proj_dir.join(rel)
                                            } else {
                                                proj_dir.join("src/main.rs")
                                            };
                                            if let Ok(content) = std::fs::read_to_string(target) {
                                                *code_target = content;
                                            }
                                            close_popup = true;
                                        }
                                    }
                                });
                        });
                });

            if close_popup {
                ui.data_mut(|d| d.insert_temp(proj_popup_id, false));
            }
        }

        // 2. Botón Tile de Archivo (Icono puro al lado del proyecto)
        if selected_project.is_some() && !archivos_disponibles.is_empty() {
            ui.add_space(2.0);

            let file_popup_id = ui.make_persistent_id(format!("{}_file_popup_menu", combo_id));
            let mut file_popup_open = ui.data_mut(|d| d.get_temp::<bool>(file_popup_id).unwrap_or(false));
            let file_anim = ui.ctx().animate_bool(file_popup_id, file_popup_open);

            let img_file = egui::Image::new(egui::include_image!("../../../assets/icons/file-svgrepo-com.svg"))
                .fit_to_exact_size(egui::Vec2::new(15.0, 15.0));
            let btn_file = pintar_icono_badge_tile(
                ui,
                img_file,
                file_popup_open,
                cyan,
                "Seleccionar Archivo del Proyecto",
            );

            if btn_file.clicked() {
                file_popup_open = !file_popup_open;
                ui.data_mut(|d| d.insert_temp(file_popup_id, file_popup_open));
            }

            // Cierre al hacer clic fuera o presionar Escape
            let file_rect_id = file_popup_id.with("rect");
            let last_file_rect = ui.data_mut(|d| d.get_temp::<egui::Rect>(file_rect_id).unwrap_or(egui::Rect::NOTHING));
            if file_popup_open {
                let escape = ui.input(|i| i.key_pressed(egui::Key::Escape));
                let click_outside = ui.input(|i| {
                    if i.pointer.any_click() {
                        if let Some(pos) = i.pointer.interact_pos() {
                            !btn_file.rect.contains(pos) && !last_file_rect.contains(pos)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });
                if escape || click_outside {
                    ui.data_mut(|d| d.insert_temp(file_popup_id, false));
                }
            }

            // Popup Dropdown de Archivos animado con Area
            if file_anim > 0.001 {
                let slide_y = (1.0 - file_anim) * -6.0;
                let popup_pos = btn_file.rect.left_bottom() + egui::vec2(0.0, 4.0 + slide_y);
                let mut close_file_popup = false;

                let alpha = (file_anim * 255.0).clamp(0.0, 255.0) as u8;
                let bg_color = egui::Color32::from_rgba_unmultiplied(16, 22, 32, alpha);
                let border_color = egui::Color32::from_rgba_unmultiplied(45, 65, 95, alpha);

                egui::Area::new(file_popup_id)
                    .fixed_pos(popup_pos)
                    .order(egui::Order::Foreground)
                    .show(ui.ctx(), |ui| {
                        // Sincronizar opacidad en todos los estilos visuales (textos, hovers, separadores)
                        ui.style_mut().visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(190, 205, 225, alpha);
                        ui.style_mut().visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha);
                        ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(30, 42, 60, alpha);
                        ui.style_mut().visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                        ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(40, 58, 85, alpha);
                        ui.style_mut().visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::from_rgba_unmultiplied(38, 54, 80, alpha);
                        ui.style_mut().visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(30, 46, 70, alpha);
                        ui.style_mut().visuals.selection.stroke.color = egui::Color32::from_rgba_unmultiplied(100, 200, 255, alpha);

                        egui::Frame::popup(ui.style())
                            .fill(bg_color)
                            .stroke(egui::Stroke::new(1.0, border_color))
                            .corner_radius(egui::CornerRadius::same(6))
                            .inner_margin(egui::Margin::same(6))
                            .show(ui, |ui| {
                                ui.data_mut(|d| d.insert_temp(file_rect_id, ui.min_rect()));
                                ui.set_width(256.0);
                                ui.label(egui::RichText::new("ARCHIVOS DEL PROYECTO").size(9.5).strong().color(egui::Color32::from_rgba_unmultiplied(120, 145, 175, alpha)));
                                ui.separator();

                                egui::ScrollArea::vertical()
                                    .max_height(150.0)
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        ui.style_mut().spacing.item_spacing.y = 2.0;

                                        let tree = build_file_tree(&archivos_disponibles);
                                        render_file_tree(
                                            ui,
                                            &tree,
                                            selected_file,
                                            proj_dir_opt.as_ref(),
                                            code_target,
                                            &mut close_file_popup,
                                            alpha,
                                            0,
                                            combo_id,
                                        );
                                    });
                            });
                    });

                if close_file_popup {
                    ui.data_mut(|d| d.insert_temp(file_popup_id, false));
                }
            }
        }
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
    let codigo_activo = if state.project_editor_path.as_deref() == Some("src/main.rs") {
        state.project_editor_code.clone()
    } else {
        state.obtener_codigo_activo().to_string()
    };
    let output_arc = state.obtener_output_activo();

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
    // Selector de Categoría (Enteros, Decimales, Bool, Char, Casting)
    ui.horizontal(|ui| {
        for (cat_idx, (cat_label, cat_color)) in [
            ("Enteros", egui::Color32::from_rgb(255, 160, 50)),
            ("Decimales", egui::Color32::from_rgb(255, 160, 50)),
            ("Booleanos", egui::Color32::from_rgb(255, 160, 50)),
            ("Caracteres", egui::Color32::from_rgb(255, 160, 50)),
            ("Casting (as)", egui::Color32::from_rgb(255, 160, 50)),
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
        2 => mostrar_categoria_booleanos(ui, state),
        3 => mostrar_categoria_caracteres(ui),
        _ => mostrar_categoria_casting(ui),
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

    ui.add_space(20.0);
    ui.heading(
        egui::RichText::new("Macros y atributos: cómo se relacionan")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(6.0);
    ui.label("La sintaxis y la implementación son conceptos distintos: ! indica una invocación, mientras que #[...] indica un atributo. Algunos atributos activan macros procedurales.");
    ui.add_space(10.0);

    let mut mapa_frame = egui::Frame::new();
    mapa_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    mapa_frame.inner_margin = egui::Margin::same(12);
    mapa_frame.corner_radius = egui::CornerRadius::same(8);
    mapa_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    mapa_frame.show(ui, |ui| {
        egui::Grid::new("mapa_macros_atributos_rust")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                for (forma, categoria, ejemplo, explicacion) in [
                    ("!", "Macro declarativa", "println!(\"Hola\");", "macro_rules! usa patrones para generar código."),
                    ("!", "Macro procedural function-like", "mi_macro!(dato);", "recibe tokens y genera código durante la compilación."),
                    ("#[derive(...)]", "Macro procedural derive", "#[derive(Debug, Clone)]", "cada trait, como Debug o Clone, puede ser una macro derive."),
                    ("#[atributo]", "Macro procedural attribute", "#[tokio::main]", "transforma la función, struct o módulo al que se aplica."),
                    ("#[atributo]", "Atributo del compilador", "#[cfg(test)]", "configura la compilación; no todos los atributos son macros."),
                ] {
                    ui.label(egui::RichText::new(forma).monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                    ui.label(egui::RichText::new(categoria).strong().color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new(ejemplo).monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                    ui.label(explicacion);
                    ui.end_row();
                }
            });
    });

    ui.add_space(14.0);
    let mut nota_frame = egui::Frame::new();
    nota_frame.fill = egui::Color32::from_rgb(20, 28, 42);
    nota_frame.inner_margin = egui::Margin::same(14);
    nota_frame.corner_radius = egui::CornerRadius::same(8);
    nota_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 90, 140));
    nota_frame.show(ui, |ui| {
        ui.label(egui::RichText::new("Regla rápida").strong().color(egui::Color32::from_rgb(255, 160, 50)));
        ui.label("println!() es una macro declarativa invocada con !. #[derive(Debug)] usa una macro derive mediante sintaxis de atributo. #[tokio::main] sí es una macro procedural de atributo. #[allow(...)] y #[cfg(...)] son atributos integrados del compilador.");
    });
}

#[allow(dead_code)]
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

pub fn mostrar_seccion_documentacion(ui: &mut egui::Ui) {
    ui.label(
        "Rust cuenta con soporte nativo de primera clase para comentarios de código y documentación. La herramienta 'cargo doc' compila automáticamente los Doc Comments con sintaxis Markdown en un sitio web de documentación HTML interactivo.",
    );
    ui.add_space(10.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_doc_comentarios")
            .striped(true)
            .spacing([20.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("¿Genera HTML?")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Destino / Ámbito")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Uso Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Comentario de Línea
                ui.label(
                    egui::RichText::new("Línea")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("// texto")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("No")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label("Ignorado por compilador");
                ui.label("Notas breves e internas de lógica.");
                ui.end_row();

                // Fila 2: Comentario de Bloque
                ui.label(
                    egui::RichText::new("Bloque")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("/* texto */")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("No")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label("Ignorado por compilador");
                ui.label("Desactivar temporalmente código.");
                ui.end_row();

                // Fila 3: Doc Comment Externo
                ui.label(
                    egui::RichText::new("Doc Externo")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("/// texto")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Sí (cargo doc)")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Elemento siguiente");
                ui.label("Documentar funciones, structs y enums.");
                ui.end_row();

                // Fila 4: Doc Comment Interno
                ui.label(
                    egui::RichText::new("Doc Interno")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("//! texto")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Sí (cargo doc)")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Módulo contenedor");
                ui.label("Cabecera de crate, lib.rs o main.rs.");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Dos Columnas: Comentarios vs Doc Comments con Markdown
    ui.columns(2, |cols| {
        // Columna Izquierda: Comentarios Normales
        let mut card_comm = egui::Frame::new();
        card_comm.fill = egui::Color32::from_rgb(14, 18, 26);
        card_comm.inner_margin = egui::Margin::same(12);
        card_comm.corner_radius = egui::CornerRadius::same(8);
        card_comm.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        card_comm.show(&mut cols[0], |ui| {
            ui.label(
                egui::RichText::new("Comentarios Internos (// y /* */)")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("Son notas para ti y tu equipo que el compilador elimina por completo durante el análisis léxico:");
            ui.add_space(8.0);

            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("// Este es un comentario de una sola línea").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.label(egui::RichText::new("let x = 5; // Nota al final de la línea").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("/* Comentario multilínea").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
                ui.label(egui::RichText::new("   útil para grandes bloques */").monospace().size(12.0).color(egui::Color32::from_rgb(140, 160, 185)));
            });
        });

        // Columna Derecha: Doc Comments y cargo doc
        let mut card_doc = egui::Frame::new();
        card_doc.fill = egui::Color32::from_rgb(14, 18, 26);
        card_doc.inner_margin = egui::Margin::same(12);
        card_doc.corner_radius = egui::CornerRadius::same(8);
        card_doc.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        card_doc.show(&mut cols[1], |ui| {
            ui.label(
                egui::RichText::new("Doc Comments (/// & cargo doc)")
                    .strong()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(6.0);
            ui.label("Admiten Markdown y generan la documentación ejecutando 'cargo doc --open' en la terminal:");
            ui.add_space(8.0);

            let mut code_box = egui::Frame::new();
            code_box.fill = egui::Color32::from_rgb(8, 12, 18);
            code_box.inner_margin = egui::Margin::same(10);
            code_box.corner_radius = egui::CornerRadius::same(6);
            code_box.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 50, 75));

            code_box.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new("/// Calcula el área de un rectángulo.").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("/// # Argumentos").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("/// * `base` - Longitud en metros").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.label(egui::RichText::new("fn area(base: f64, altura: f64) -> f64 {").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.indent("doc_code_inner", |ui| {
                    ui.label(egui::RichText::new("base * altura").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
                });
                ui.label(egui::RichText::new("}").monospace().size(12.0).color(egui::Color32::from_rgb(100, 200, 255)));
            });
        });
    });
}

pub fn mostrar_nav_superior(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let mut is_expanded = state.mostrar_nav_superior;

    let color_header = egui::Color32::from_rgb(13, 15, 19);

    egui::Panel::top("nav_top_global")
        .frame(egui::Frame::default().fill(color_header).inner_margin(4.0))
        .resizable(false)
        .show_collapsible(ui, &mut is_expanded, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space(5.0);
                
                // --- LADO IZQUIERDO: Título y Teoría ---
                ui.label(
                    egui::RichText::new("Conceptos de Rust")
                        .size(16.0)
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                
                ui.separator();

                let img_book = egui::Image::new(egui::include_image!("../../../assets/icons/book-line.svg")).fit_to_exact_size(egui::Vec2::new(24.0, 24.0));
                ui.add(img_book);
                let tabs_teoria = [
                    (7, "Core Mechanics"),
                    (4, "Data Types"),
                    (5, "Macros y Atributos"),
                    (6, "Doc & Comentarios"),
                ];
                for (indice, texto) in tabs_teoria {
                    let es_activo = state.conceptos_tab == indice;
                    if ui.selectable_label(es_activo, texto).clicked() {
                        state.conceptos_tab = indice;
                    }
                }

                // --- LADO DERECHO: Práctica ---
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(5.0);
                    
                    let tabs_practica = [
                        (0, "Code Lab"),
                    ];
                    // Iteramos al revés por el right_to_left
                    for (indice, texto) in tabs_practica.iter().rev() {
                        let es_activo = state.conceptos_tab == *indice;
                        if ui.selectable_label(es_activo, *texto).clicked() {
                            state.conceptos_tab = *indice;
                        }
                    }

                    let img_code = egui::Image::new(egui::include_image!("../../../assets/icons/monitor-code-line.svg")).fit_to_exact_size(egui::Vec2::new(24.0, 24.0));
                    ui.add(img_code);

                    ui.separator();
                });
            });
            ui.add_space(6.0);
        });

    state.mostrar_nav_superior = is_expanded;
}

pub fn mostrar_tutorial_conceptos_basicos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    // El selector de proyectos y el editor de código interactivo solo para "Funciones Básicas" (y futuros tabs prácticos si los agregamos)
    // El índice 0 es el único práctico que queda actualmente.
    if state.conceptos_tab == 0 {
        let code_target = if state.selected_project.is_some() {
            &mut state.shared_project_code
        } else {
            &mut state.conceptos_code
        };

        mostrar_selector_proyectos_estandar_con_archivos(
            ui,
            &mut state.selected_project,
            &mut state.selected_file,
            &mut state.term_cwd,
            "combo_proyectos_comenzando",
            code_target,
        );

        ui.add_space(10.0);

        let syntax_set = state.syntax_set.clone();
        let theme = state.theme_set.themes["base16-ocean.dark"].clone();
        let (code_ref, output_arc) = state.obtener_editor_activo_mut();
        mostrar_editor_interactivo(
            ui,
            code_ref,
            output_arc,
            "",
            ejecutar_codigo_rust,
            &syntax_set,
            &theme,
        );

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(12.0);
    }

    match state.conceptos_tab {
        0 => funciones::mostrar(ui, state),
        4 => mostrar_contenido_tipos_primitivos(ui, state),
        5 => mostrar_contenido_macros(ui),
        6 => mostrar_seccion_documentacion(ui),
        7 => {
            ui.heading(
                egui::RichText::new("Mecánicas Centrales de Rust")
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.add_space(20.0);
            
            // Fusión de las 3 vistas teóricas
            mutabilidad::mostrar(ui, state);
            
            ui.add_space(30.0);
            ui.separator();
            ui.add_space(30.0);
            
            scopes::mostrar(ui, state);
            
            ui.add_space(30.0);
            ui.separator();
            ui.add_space(30.0);
            
            statements::mostrar(ui, state);
        }
        _ => {}
    }
}
