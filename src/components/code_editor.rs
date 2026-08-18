use eframe::egui;
use std::sync::{Arc, Mutex};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use super::console_output::formatear_salida_consola;

pub fn rust_layouter(
    ui: &egui::Ui,
    string: &str,
    wrap_width: f32,
    syntax_set: &SyntaxSet,
    theme: &Theme,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::default();

    let syntax = syntax_set
        .find_syntax_by_extension("rs")
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, theme);

    for line in LinesWithEndings::from(string) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, syntax_set).unwrap_or_default();
        for (style, text) in ranges {
            let color =
                egui::Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b);
            let font_id = egui::FontId::monospace(14.0);
            job.append(text, 0.0, egui::TextFormat::simple(font_id, color));
        }
    }

    if !string.ends_with('\n') && job.text.ends_with('\n') {
        job.text.pop();
    }

    job.wrap.max_width = wrap_width;
    ui.painter().layout_job(job)
}

pub fn mostrar_editor_interactivo<F>(
    ui: &mut egui::Ui,
    code: &mut String,
    output_mutex: Arc<Mutex<String>>,
    btn_text: &str,
    execute_fn: F,
    syntax_set: &SyntaxSet,
    theme: &Theme,
) where
    F: Fn(&str) -> String + Send + 'static,
{
    let mut editor_frame = egui::Frame::new();
    editor_frame.fill = egui::Color32::from_rgb(13, 17, 23);
    editor_frame.inner_margin = egui::Margin::same(14);
    editor_frame.corner_radius = egui::CornerRadius::same(8);

    editor_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
            rust_layouter(ui, text.as_str(), wrap_width, syntax_set, theme)
        };

        egui::ScrollArea::vertical()
            .max_height(260.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(code)
                        .frame(egui::Frame::NONE)
                        .layouter(&mut layouter)
                        .code_editor()
                        .desired_width(f32::INFINITY)
                        .lock_focus(true),
                );
            });
    });

    if !btn_text.is_empty() {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui
                .button(
                    egui::RichText::new(btn_text)
                        .size(16.0)
                        .color(egui::Color32::LIGHT_GREEN),
                )
                .clicked()
            {
                *output_mutex.lock().unwrap() = "Ejecutando...".to_string();
                let code_clone = code.clone();
                let out_clone = Arc::clone(&output_mutex);
                let ctx = ui.ctx().clone();
                std::thread::spawn(move || {
                    let res = execute_fn(&code_clone);
                    *out_clone.lock().unwrap() = res;
                    ctx.request_repaint();
                });
            }
        });

        let output_text = output_mutex.lock().unwrap().clone();
        if !output_text.is_empty() {
            ui.add_space(10.0);
            let mut out_frame = egui::Frame::new();
            out_frame.fill = egui::Color32::from_rgb(10, 10, 10);
            out_frame.inner_margin = egui::Margin::same(10);
            out_frame.corner_radius = egui::CornerRadius::same(5);
            out_frame.show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                if output_text == "Ejecutando..." || output_text == "Compilando..." {
                    ui.label(
                        egui::RichText::new(output_text)
                            .color(egui::Color32::YELLOW)
                            .monospace(),
                    );
                } else if let Some(idx) = output_text.find("[Errores/Warnings]:\n") {
                    let (stdout, stderr) = output_text.split_at(idx);
                    if !stdout.is_empty() {
                        ui.label(formatear_salida_consola(stdout, false));
                        ui.add_space(5.0);
                        ui.separator();
                        ui.add_space(5.0);
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
        }
    }
}
