use crate::app::PortfolioState;
use crate::routes::AppRoute;
use eframe::egui;

pub fn mostrar_landing_page(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let avail_w = ui.available_width();
    let avail_h = ui.available_height();

    ui.allocate_ui(egui::vec2(avail_w, avail_h), |ui| {
        // --- HEADER SUPERIOR FLOTANTE CON BORDER BOTTOM ---

        let header_margin_width = 100.0;

        let header_frame = egui::Frame::new()
            // .fill(egui::Color32::BLACK)
            .inner_margin(egui::Margin::symmetric(20, 15)) // Padding interno
            .outer_margin(egui::Margin { left: 100, right: 100, top: 10, bottom: 0 }) // Márgenes exteriores
            .stroke(egui::Stroke::NONE); // Sin borde nativo

        let response = header_frame.show(ui, |ui| {
            ui.set_min_width(ui.available_width()); // Estirar a todo el ancho disponible
            ui.horizontal(|ui| {
                // Logo izquierdo
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(egui::RichText::new("Ferris").size(22.0).strong().color(egui::Color32::from_rgb(255, 160, 50)));
                    ui.label(egui::RichText::new("Key").size(22.0).strong().color(egui::Color32::WHITE));
                });

                // Menú derecho (Comunidad)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let estilo_previo = (**ui.style()).clone();
                    let estilo_mut = ui.style_mut();
                    estilo_mut.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    estilo_mut.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                    estilo_mut.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;

                    estilo_mut.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(25, 30, 40);
                    estilo_mut.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 160, 50));

                    estilo_mut.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(40, 45, 55);

                    ui.menu_button(egui::RichText::new("Comunidad").size(16.0).color(egui::Color32::from_rgb(200, 210, 225)), |ui| {
                        *ui.style_mut() = estilo_previo;
                        ui.set_min_width(120.0);
                        if ui.button("Discord").clicked() { ui.ctx().open_url(egui::OpenUrl::new_tab("https://discord.com")); }
                        if ui.button("Telegram").clicked() { ui.ctx().open_url(egui::OpenUrl::new_tab("https://telegram.org")); }
                        if ui.button("Página Web").clicked() { ui.ctx().open_url(egui::OpenUrl::new_tab("https://ferriskey.com")); }
                    });
                });
            });
        });

        // HACK DEL BORDER-BOTTOM EN EGUI
        let rect = response.response.rect;
        let start = egui::pos2(rect.left() + 100.0, rect.bottom());
        let end = egui::pos2(rect.right() - 100.0, rect.bottom());

        ui.painter().line_segment(
            [start, end],
            egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 160, 50))
        );

        ui.add_space(15.0);
        // ------------------------

        // --- CUERPO PRINCIPAL CON MÁRGENES LATERALES ---
        let margen_lateral = (avail_w * 0.15).max(30.0) as i8;

        // Usamos un Frame invisible para meter el padding lateral de forma nativa sin romper el flujo vertical
        let cuerpo_frame = egui::Frame::new()
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::NONE)
            .inner_margin(egui::Margin {
                left: margen_lateral,
                right: margen_lateral,
                top: 0,
                bottom: 0,
            });

        cuerpo_frame.show(ui, |ui| {
            ui.vertical_centered(|ui| {
                // Propósito / Título grande
                ui.label(
                    egui::RichText::new("Domina Rust como si fuera un MMORPG")
                        .size((avail_w * 0.025).clamp(24.0, 36.0))
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );

                ui.add_space(10.0);

                // Descripción de propósito
                ui.label(
                    egui::RichText::new("Bienvenido a FerrisKey, la plataforma interactiva diseñada para aprender las mecánicas del lenguaje, subir de nivel gestionando la memoria y competir en desafíos de código en tiempo real contra otros jugadores.")
                        .size((avail_w * 0.013).clamp(14.0, 18.0))
                        .color(egui::Color32::from_rgb(180, 200, 230))
                        .line_height(Some(22.0)),
                );

                ui.add_space(20.0);

                // Imagen de la interfaz / Ilustración
                let max_img_w = (avail_w * 0.50).clamp(280.0, 680.0);
                let max_img_h = (avail_h * 0.40).clamp(160.0, 420.0);

                ui.add(
                    egui::Image::new(egui::include_image!("../../assets/taller/home2.png"))
                        .max_width(max_img_w)
                        .max_height(max_img_h)
                        .corner_radius(egui::CornerRadius::same(16)),
                );

                ui.add_space(20.0);

                // Botón de acción principal
                let btn_w = (avail_w * 0.20).clamp(180.0, 280.0);
                let btn_h = (avail_h * 0.06).clamp(42.0, 54.0);

                let btn_jugar = ui.add_sized(
                    [btn_w, btn_h],
                    egui::Button::new(
                        egui::RichText::new("▶  J U G A R")
                            .size((btn_h * 0.40).clamp(16.0, 22.0))
                            .strong()
                            .color(egui::Color32::BLACK),
                    )
                    .fill(egui::Color32::from_rgb(255, 180, 50))
                    .corner_radius(egui::CornerRadius::same((btn_h * 0.5) as u8)),
                );

                if btn_jugar.clicked() {
                    state.ruta_actual = AppRoute::TutorialCargo;
                    state.anim_trigger = ui.input(|i| i.time);
                }
            });
        });
    });
}
