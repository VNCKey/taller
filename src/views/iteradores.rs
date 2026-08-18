use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;

pub fn mostrar_tutorial_iteradores(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("🔄 Lección: Iteradores & Bucles for")
                .size(30.0)
                .strong()
                .color(egui::Color32::from_rgb(60, 220, 140)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("El trait Iterator, bucles for desazucarados y pipelines perezosos (.iter, .map, .filter)")
                .size(15.0)
                .italics(),
        );
    });

    ui.add_space(25.0);

    let mut concept_frame = egui::Frame::new();
    concept_frame.fill = egui::Color32::from_rgb(22, 24, 32);
    concept_frame.inner_margin = egui::Margin::same(15);
    concept_frame.corner_radius = egui::CornerRadius::same(8);
    concept_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 50, 65));
    concept_frame.show(ui, |ui| {
        ui.heading("💡 Las 3 Formas de Iterar en Rust");
        ui.add_space(8.0);
        ui.label("• `.iter()` -> Recorre por Referencia Inmutable `&T` (no consume ni destruye el vector).");
        ui.label("• `.iter_mut()` -> Recorre por Referencia Mutable `&mut T` (permite modificar elementos in-place).");
        ui.label("• `.into_iter()` -> Recorre por Valor `T` (MUEVE/consume el vector original).");
        ui.label("• Pipelines Perezosos (Lazy): `.filter()` y `.map()` no ejecutan nada hasta que se llama a `.collect()` o un bucle `for`.");
    });

    ui.add_space(20.0);

    ui.heading("📊 Simulador de Pipeline Perezoso de Iteradores");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Modo de Iteración:").strong());
        ui.selectable_value(&mut state.iter_mode, 0, "1. .iter() (&T)");
        ui.selectable_value(&mut state.iter_mode, 1, "2. .iter_mut() (&mut T)");
        ui.selectable_value(&mut state.iter_mode, 2, "3. .into_iter() (T)");
        ui.add_space(20.0);
        ui.checkbox(&mut state.iter_filter_even, "Filtrar solo Pares (.filter)");
    });

    ui.add_space(10.0);

    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 150.0));
    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    let input_nums = vec![1, 2, 3, 4, 5, 6];
    let y = rect.center().y;

    // 1. Array Origen
    let start_x = rect.left() + 40.0;
    ui.painter().text(
        egui::pos2(start_x + 60.0, y - 45.0),
        egui::Align2::CENTER_CENTER,
        "origen.iter()",
        egui::FontId::proportional(12.0),
        egui::Color32::LIGHT_GRAY,
    );
    for (i, num) in input_nums.iter().enumerate() {
        let box_rect = egui::Rect::from_center_size(
            egui::pos2(start_x + (i as f32 * 25.0), y),
            egui::vec2(22.0, 30.0),
        );
        ui.painter()
            .rect_filled(box_rect, 3.0, egui::Color32::from_rgb(40, 50, 70));
        ui.painter().text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            num.to_string(),
            egui::FontId::proportional(11.0),
            egui::Color32::WHITE,
        );
    }

    // 2. Filtro / Map
    let mid_x = rect.left() + 280.0;
    ui.painter().line_segment(
        [egui::pos2(start_x + 160.0, y), egui::pos2(mid_x - 40.0, y)],
        egui::Stroke::new(2.0, egui::Color32::GRAY),
    );

    let filter_rect = egui::Rect::from_center_size(egui::pos2(mid_x, y), egui::vec2(100.0, 50.0));
    ui.painter()
        .rect_filled(filter_rect, 5.0, egui::Color32::from_rgb(240, 140, 40));
    ui.painter().text(
        filter_rect.center(),
        egui::Align2::CENTER_CENTER,
        "filter & map\n(x * x)",
        egui::FontId::proportional(11.0),
        egui::Color32::BLACK,
    );

    // 3. Resultado .collect()
    let out_x = rect.left() + 480.0;
    ui.painter().line_segment(
        [egui::pos2(mid_x + 60.0, y), egui::pos2(out_x - 40.0, y)],
        egui::Stroke::new(2.0, egui::Color32::GREEN),
    );

    let res_nums: Vec<i32> = input_nums
        .into_iter()
        .filter(|&x| !state.iter_filter_even || x % 2 == 0)
        .map(|x| x * x)
        .collect();
    ui.painter().text(
        egui::pos2(out_x + (res_nums.len() as f32 * 15.0), y - 45.0),
        egui::Align2::CENTER_CENTER,
        ".collect::<Vec<_>>()",
        egui::FontId::proportional(12.0),
        egui::Color32::GREEN,
    );

    for (i, num) in res_nums.iter().enumerate() {
        let box_rect = egui::Rect::from_center_size(
            egui::pos2(out_x + (i as f32 * 35.0), y),
            egui::vec2(30.0, 32.0),
        );
        ui.painter()
            .rect_filled(box_rect, 4.0, egui::Color32::from_rgb(60, 180, 100));
        ui.painter().text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            num.to_string(),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }

    ui.add_space(20.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.iteradores_code,
        Arc::clone(&state.iteradores_output),
        "▶ Ejecutar Iteradores & Bucles for",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

