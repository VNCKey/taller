use eframe::egui;
use crate::app::PortfolioState;
use crate::routes::AppRoute;

pub fn mostrar_landing_page(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let avail_w = ui.available_width();
    let avail_h = ui.available_height();

    ui.allocate_ui(egui::vec2(avail_w, avail_h), |ui| {
        ui.vertical_centered(|ui| {
            let top_pad = (avail_h * 0.04).clamp(15.0, 50.0);
            ui.add_space(top_pad);

            let title_size = (avail_w * 0.038).clamp(36.0, 64.0);
            let mut job = egui::text::LayoutJob::default();
            job.append(
                "Ferris",
                0.0,
                egui::TextFormat {
                    font_id: egui::FontId::new(title_size, egui::FontFamily::Proportional),
                    color: egui::Color32::from_rgb(255, 160, 50),
                    ..Default::default()
                },
            );
            job.append(
                "Key",
                0.0,
                egui::TextFormat {
                    font_id: egui::FontId::new(title_size, egui::FontFamily::Proportional),
                    color: egui::Color32::WHITE,
                    ..Default::default()
                },
            );
            ui.label(job);

            ui.add_space(12.0);

            let max_img_w = (avail_w * 0.55).clamp(300.0, 750.0);
            let max_img_h = (avail_h * 0.45).clamp(200.0, 520.0);

            ui.add(
                egui::Image::new(egui::include_image!("../../assets/taller/home2.png"))
                    .max_width(max_img_w)
                    .max_height(max_img_h)
                    .corner_radius(egui::CornerRadius::same(16)),
            );

            ui.add_space(14.0);

            ui.label(
                egui::RichText::new("Diviértete aprendiendo con Rust")
                    .size((avail_w * 0.018).clamp(18.0, 28.0))
                    .strong()
                    .color(egui::Color32::WHITE),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("El ecosistema interactivo para dominar el silicio, la memoria y el compilador.")
                    .size((avail_w * 0.012).clamp(13.0, 18.0))
                    .color(egui::Color32::from_rgb(180, 200, 230)),
            );

            ui.add_space((avail_h * 0.04).clamp(15.0, 35.0));

            let btn_w = (avail_w * 0.22).clamp(220.0, 340.0);
            let btn_h = (avail_h * 0.065).clamp(48.0, 60.0);

            let btn_jugar = ui.add_sized(
                [btn_w, btn_h],
                egui::Button::new(
                    egui::RichText::new("▶  J U G A R")
                        .size((btn_h * 0.42).clamp(18.0, 26.0))
                        .strong()
                        .color(egui::Color32::BLACK),
                )
                .fill(egui::Color32::from_rgb(255, 180, 50))
                .corner_radius(egui::CornerRadius::same((btn_h * 0.5) as u8)),
            );

            if btn_jugar.clicked() {
                state.ruta_actual = AppRoute::TutorialCargo;
            }
        });
    });
}


pub fn mostrar_portafolio(ui: &mut egui::Ui) {
    ui.add_space(40.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("¡Hola! Soy Luis Alexander")
                .size(36.0)
                .strong(),
        );
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Desarrollador de Software especializado en Rust.").size(20.0),
        );
    });

    ui.add_space(40.0);
    ui.separator();
}

