use eframe::egui;
use crate::app::PortfolioState;

fn railroad_modal_img_size(svg_bytes: &[u8]) -> egui::Vec2 {
    const TARGET_H: f32 = 96.0;
    const MAX_W: f32 = 1100.0;
    const FALLBACK_ASPECT: f32 = 12.0; // ~720x60

    let s = std::str::from_utf8(svg_bytes).unwrap_or("");
    let (native_w, native_h) = parse_svg_wh(s).unwrap_or((FALLBACK_ASPECT * 60.0, 60.0));
    let aspect = (native_w / native_h.max(1.0)).clamp(1.0, 40.0);

    let mut h = TARGET_H;
    let mut w = h * aspect;
    if w > MAX_W {
        w = MAX_W;
        h = w / aspect;
    }
    egui::vec2(w, h)
}

fn parse_svg_wh(svg: &str) -> Option<(f32, f32)> {
    // width="764" height="60"  (también admite decimales)
    let w_pos = svg.find("width=\"")?;
    let w_rest = &svg[w_pos + 7..];
    let w_end = w_rest.find('"')?;
    let h_key = "height=\"";
    let h_pos = svg.find(h_key)?;
    let h_rest = &svg[h_pos + h_key.len()..];
    let h_end = h_rest.find('"')?;
    let w: f32 = w_rest[..w_end].parse().ok()?;
    let h: f32 = h_rest[..h_end].parse().ok()?;
    if w > 0.0 && h > 0.0 {
        Some((w, h))
    } else {
        None
    }
}


pub fn mostrar_modal_railroad_let(ctx: &egui::Context, state: &mut PortfolioState) {
    let mode = match state.show_railroad_modal {
        Some(m) => m,
        None => return,
    };

    let mut abierto = true;
    let (titulo, bytes_data, uri) = match mode {
        0 => (
            "Inmutable",
            include_bytes!("../../../diagramas/diagrama_let_immut.svg").as_slice(),
            "bytes://diagrama_let_immut.svg",
        ),
        1 => (
            "Mutable",
            include_bytes!("../../../diagramas/diagrama_let_mut.svg").as_slice(),
            "bytes://diagrama_let_mut.svg",
        ),
        2 => (
            "fn main()",
            include_bytes!("../../../diagramas/diagrama_fn_main.svg").as_slice(),
            "bytes://diagrama_fn_main.svg",
        ),
        3 => (
            "Librería (src/lib.rs)",
            include_bytes!("../../../diagramas/diagrama_lib.svg").as_slice(),
            "bytes://diagrama_lib.svg",
        ),
        4 => (
            "Tiempo de Compilación (Compile Time)",
            include_bytes!("../../../diagramas/diagrama_compile_time.svg").as_slice(),
            "bytes://diagrama_compile_time.svg",
        ),
        _ => (
            "Tiempo de Ejecución (Run Time)",
            include_bytes!("../../../diagramas/diagrama_run_time.svg").as_slice(),
            "bytes://diagrama_run_time.svg",
        ),
    };

    let img_size = railroad_modal_img_size(bytes_data);

    let mut window_frame = egui::Frame::window(&ctx.style_of(egui::Theme::Dark));
    window_frame.inner_margin = egui::Margin::symmetric(24, 16);
    window_frame.fill = egui::Color32::from_rgb(18, 24, 36);

    egui::Window::new(titulo)
        .open(&mut abierto)
        .collapsible(false)
        .resizable(false)
        .frame(window_frame)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .pivot(egui::Align2::CENTER_CENTER)
        .show(ctx, |ui| {
            ui.add(
                egui::Image::from_bytes(uri, bytes_data)
                    .fit_to_exact_size(img_size)
                    .maintain_aspect_ratio(true),
            );
        });

    if !abierto {
        state.show_railroad_modal = None;
    }
}
