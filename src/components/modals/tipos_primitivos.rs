use eframe::egui;
use crate::app::PortfolioState;
use crate::views::tipos_datos::{
    mostrar_categoria_booleanos, mostrar_categoria_caracteres, mostrar_categoria_enteros,
    mostrar_categoria_flotantes,
};

#[allow(dead_code)]
pub fn mostrar_modal_tipos_primitivos(ctx: &egui::Context, state: &mut PortfolioState) {
    if !state.show_tipos_primitivos_modal {
        return;
    }

    let mut open = true;
    egui::Window::new("📦 Tipos de Datos Primitivos en Rust")
        .open(&mut open)
        .resizable(true)
        .default_size([750.0, 520.0])
        .collapsible(false)
        .show(ctx, |ui| {
            // Selector de Categoría (Enteros, Decimales, Bool, Char)
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Categoría:")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.add_space(6.0);

                for (cat_idx, (cat_label, cat_color)) in [
                    ("🔢 Enteros (i / u)", egui::Color32::from_rgb(100, 200, 255)),
                    (
                        "📐 Decimales (f32 / f64)",
                        egui::Color32::from_rgb(255, 180, 100),
                    ),
                    (
                        "🔘 Booleanos (bool)",
                        egui::Color32::from_rgb(120, 255, 150),
                    ),
                    (
                        "🔤 Caracteres (char)",
                        egui::Color32::from_rgb(255, 140, 220),
                    ),
                ]
                .iter()
                .enumerate()
                {
                    let es_sel = state.tipo_primitivo_categoria == cat_idx;
                    let text_rich = egui::RichText::new(*cat_label).strong().color(if es_sel {
                        *cat_color
                    } else {
                        egui::Color32::GRAY
                    });
                    if ui.add(egui::Button::new(text_rich).frame(es_sel)).clicked() {
                        state.tipo_primitivo_categoria = cat_idx;
                    }
                    ui.add_space(4.0);
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            egui::ScrollArea::vertical()
                .max_height(430.0)
                .show(ui, |ui| match state.tipo_primitivo_categoria {
                    0 => mostrar_categoria_enteros(ui),
                    1 => mostrar_categoria_flotantes(ui),
                    2 => mostrar_categoria_booleanos(ui),
                    _ => mostrar_categoria_caracteres(ui),
                });
        });

    if !open {
        state.show_tipos_primitivos_modal = false;
    }
}
