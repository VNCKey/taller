use eframe::egui;

use crate::app::PortfolioState;
use crate::views::pilares::entorno;

pub fn mostrar_ecosystem(ui: &mut egui::Ui, state: &mut PortfolioState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        entorno::mostrar_pilares_entorno_contenido(ui, state);
    });
}
