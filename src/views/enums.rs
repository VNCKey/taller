use eframe::egui;
use std::sync::Arc;
use crate::app::PortfolioState;
use crate::components::code_editor::mostrar_editor_interactivo;
use crate::execution::ejecutar_codigo_rust;

pub fn mostrar_tutorial_enums(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("🏷️ Lección 8: Enums, Option<T> & Result<T, E>")
                .size(30.0)
                .strong()
                .color(egui::Color32::from_rgb(240, 180, 50)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new(
                "Tipos de datos algebraicos (Sum Types), ausencia de valores nulos y patrones",
            )
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
        ui.heading("💡 El Poder de los Enums en Rust");
        ui.add_space(8.0);
        ui.label("• Las variantes pueden contener datos: `enum Mensaje { Mover { x: i32, y: i32 }, Escribir(String) }`.");
        ui.label("• `Option<T>` elimina NullPointerException: Un valor es `Some(T)` o `None`.");
        ui.label("• `Result<T, E>` maneja errores de forma segura: Es `Ok(T)` o `Err(E)`.");
    });

    ui.add_space(20.0);

    ui.heading("📊 Inspector Interactivo de Memoria de Variantes de Enum");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("Variante Activa:");
        ui.selectable_value(&mut state.enum_variant_selected, 0, "0: Pendiente");
        ui.selectable_value(&mut state.enum_variant_selected, 1, "1: Enviado { guia }");
        ui.selectable_value(&mut state.enum_variant_selected, 2, "2: Entregado");
    });

    ui.add_space(10.0);

    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 140.0));
    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    let tag_rect = egui::Rect::from_min_size(
        rect.left_top() + egui::vec2(30.0, 30.0),
        egui::vec2(100.0, 80.0),
    );
    ui.painter()
        .rect_filled(tag_rect, 6.0, egui::Color32::from_rgb(240, 180, 50));
    ui.painter().text(
        tag_rect.center() - egui::vec2(0.0, 10.0),
        egui::Align2::CENTER_CENTER,
        "Discriminante",
        egui::FontId::proportional(11.0),
        egui::Color32::BLACK,
    );
    ui.painter().text(
        tag_rect.center() + egui::vec2(0.0, 12.0),
        egui::Align2::CENTER_CENTER,
        format!("Tag {}", state.enum_variant_selected),
        egui::FontId::proportional(16.0),
        egui::Color32::BLACK,
    );

    let payload_rect = egui::Rect::from_min_size(
        rect.left_top() + egui::vec2(150.0, 30.0),
        egui::vec2(380.0, 80.0),
    );
    ui.painter()
        .rect_filled(payload_rect, 6.0, egui::Color32::from_rgb(35, 45, 60));
    ui.painter().text(
        payload_rect.left_top() + egui::vec2(10.0, 10.0),
        egui::Align2::LEFT_TOP,
        "Payload (Carga Útil de Datos en Memoria)",
        egui::FontId::proportional(11.0),
        egui::Color32::LIGHT_GRAY,
    );

    let payload_desc = match state.enum_variant_selected {
        0 => "Sin datos adicionales (0 bytes extra)",
        1 => "guia: String (\"RUST-9921\") -> Puntero + Len + Cap (24 bytes)",
        _ => "Sin datos adicionales (0 bytes extra)",
    };
    ui.painter().text(
        payload_rect.center() + egui::vec2(0.0, 5.0),
        egui::Align2::CENTER_CENTER,
        payload_desc,
        egui::FontId::proportional(13.0),
        egui::Color32::WHITE,
    );

    ui.add_space(20.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.enums_code,
        Arc::clone(&state.enums_output),
        "▶ Ejecutar Enums & Pattern Matching",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

