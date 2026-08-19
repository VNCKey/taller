use eframe::egui;
use crate::app::PortfolioState;

pub fn mostrar_modal_comparacion_compiladores(ctx: &egui::Context, state: &mut PortfolioState) {
    if !state.show_rustc_compilador_modal {
        return;
    }

    let mut open = true;
    egui::Window::new("⚙️ Pipeline de Compilación de rustc")
        .open(&mut open)
        .resizable(true)
        .default_size([980.0, 640.0])
        .collapsible(false)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(
                        "Flujo completo de transformación: desde tu archivo .rs hasta el binario nativo ejecutable.",
                    )
                    .color(egui::Color32::from_rgb(180, 195, 215))
                    .size(13.0),
                );
                ui.add_space(10.0);

                // Diagrama SVG generado desde Mermaid (pipe.svg)
                let img = egui::Image::from_bytes(
                    "bytes://pipe.svg",
                    include_bytes!("../../../diagramas/pipe.svg"),
                )
                .fit_to_original_size(1.0);

                ui.add(img);

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(10.0);

                // Conclusión didáctica
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
                                egui::RichText::new("Conclusión del Proceso de rustc")
                                    .strong()
                                    .size(15.0)
                                    .color(egui::Color32::from_rgb(255, 180, 100)),
                            );
                            ui.label(
                                egui::RichText::new(
                                    "Cada fase tiene un propósito exacto: el Frontend entiende tu código (.rs -> .hir), el Middle-End garantiza la seguridad de memoria con el Borrow Checker (.mir), y el Backend de LLVM genera código máquina altamente optimizado (.s -> .o -> binario).",
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
