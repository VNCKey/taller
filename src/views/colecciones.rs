use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;

pub fn mostrar_tutorial_colecciones(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("📚 Lección 9: Colecciones Dinámicas (Vec<T>, HashMap)")
                .size(30.0)
                .strong()
                .color(egui::Color32::from_rgb(60, 200, 120)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("Estructuras de datos dinámicas almacenadas en el Heap")
                .size(15.0)
                .italics(),
        );
    });

    ui.add_space(25.0);

    ui.heading("📊 Simulador de Reasignación de Capacidad en Vec<T>");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        if ui.button("➕ Push Elemento").clicked() {
            state.vec_sim_len += 1;
            if state.vec_sim_len > state.vec_sim_cap {
                state.vec_sim_cap *= 2;
            }
        }
        if ui.button("➖ Pop Elemento").clicked() && state.vec_sim_len > 0 {
            state.vec_sim_len -= 1;
        }
        ui.add_space(20.0);
        ui.label(format!(
            "len: {} | cap: {}",
            state.vec_sim_len, state.vec_sim_cap
        ));
    });

    ui.add_space(10.0);

    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 140.0));
    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    let start_x = rect.left() + 30.0;
    let y = rect.center().y;

    for i in 0..state.vec_sim_cap {
        let box_x = start_x + (i as f32 * 50.0);
        let box_rect = egui::Rect::from_center_size(egui::pos2(box_x, y), egui::vec2(42.0, 42.0));
        let filled = i < state.vec_sim_len;

        ui.painter().rect_filled(
            box_rect,
            4.0,
            if filled {
                egui::Color32::from_rgb(60, 180, 120)
            } else {
                egui::Color32::from_rgb(40, 45, 55)
            },
        );
        ui.painter().text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            if filled {
                format!("[{}]", i)
            } else {
                "_".to_string()
            },
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }

    ui.add_space(20.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.colecciones_code,
        Arc::clone(&state.colecciones_output),
        "▶ Ejecutar Colecciones",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

