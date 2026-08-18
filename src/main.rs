mod execution;

use egui_plot::{Bar, BarChart, Corner, HLine, Legend, Line, Plot, PlotPoints, VLine};
use execution::{
    ejecutar_codigo_api, ejecutar_codigo_cargo_run, ejecutar_codigo_rust, expandir_macros_rust,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(PartialEq, Default)]
#[allow(dead_code)]
enum AppRoute {
    #[default]
    LandingPage,
    Portafolio,
    TutorialCargo,
    Comenzando,
    TutorialCompilacion,
    TutorialTiposDatos,
    TutorialControlFlujo,
    TutorialFunciones,
    TutorialIteradores,
    TutorialOwnership,
    TutorialStructs,
    TutorialEnums,
    TutorialColecciones,
    TutorialErrores,
    TutorialTraits,
    TutorialStrings,
    TutorialMemoria,
    DashboardGraficos,
    Playground,
    PlaygroundNube,
}

#[allow(dead_code)]
struct PortfolioState {
    ruta_actual: AppRoute,

    show_ingresos: bool,
    show_gastos: bool,
    show_beneficios: bool,
    year: i32,

    tutorial_step: usize,
    tutorial_time: f64,

    playground_code: String,
    playground_output: Arc<Mutex<String>>,

    datatypes_code: String,
    datatypes_output: Arc<Mutex<String>>,

    strings_code: String,
    strings_output: Arc<Mutex<String>>,

    playground_nube_code: String,
    playground_nube_output: Arc<Mutex<String>>,

    syntax_set: SyntaxSet,
    theme_set: ThemeSet,

    // Animaciones
    anim_compilacion_activa: bool,
    compilacion_progreso: f32,
    compilacion_etapa_seleccionada: usize,

    // State para Dashboard de Visualización (Plotly / Power BI)
    dash_tab: usize,
    bcr_playing: bool,
    bcr_year: f32,
    bcr_speed: f32,
    pie_donut_hole: f32,
    pie_exploded: bool,
    index_baseline_year: f32,
    ts_show_ma: bool,
    ts_show_volume: bool,
    ts_show_rsi: bool,

    // State para los nuevos Módulos del Curso Interactivo de Rust
    controlflujo_code: String,
    controlflujo_output: Arc<Mutex<String>>,
    controlflujo_val: i32,

    ownership_code: String,
    ownership_output: Arc<Mutex<String>>,
    ownership_step: usize,
    /// Tabs de la sesión unificada Strings & Ownership
    strings_ownership_tab: usize,

    structs_code: String,
    structs_output: Arc<Mutex<String>>,

    enums_code: String,
    enums_output: Arc<Mutex<String>>,
    enum_variant_selected: usize,

    colecciones_code: String,
    colecciones_output: Arc<Mutex<String>>,
    vec_sim_len: usize,
    vec_sim_cap: usize,

    errores_code: String,
    errores_output: Arc<Mutex<String>>,
    err_pipeline_fail: bool,

    traits_code: String,
    traits_output: Arc<Mutex<String>>,

    // State para Juegos Interactivos de Tipos Compuestos (Arrays, Slices, Tuplas)
    arr_elem_type: usize,
    arr_len: usize,
    arr_active_idx: usize,
    arr_action_msg: String,
    /// Tabs de la sesión Tipos compuestos
    compuestos_tab: usize,

    slice_start: usize,
    slice_end: usize,

    tup_t0: usize,
    tup_t1: usize,
    tup_t2: usize,

    /// Tabs de Structs & impl
    structs_tab: usize,

    // State para Funciones e Iteradores
    funciones_code: String,
    funciones_output: Arc<Mutex<String>>,
    funciones_step: usize,
    funciones_tab: usize,

    iteradores_code: String,
    iteradores_output: Arc<Mutex<String>>,
    iter_mode: usize,
    iter_filter_even: bool,

    // Editores dedicados para Tipos Compuestos
    arr_code: String,
    arr_output: Arc<Mutex<String>>,
    slice_code: String,
    slice_output: Arc<Mutex<String>>,
    tup_code: String,
    tup_output: Arc<Mutex<String>>,

    // State para Estructura de Proyecto y Conceptos Básicos
    comenzando_step: usize,
    pilares_step: usize,
    show_commands_modal: bool,
    show_macro_expansion: bool,
    show_cargo_output_modal: Arc<AtomicBool>,
    show_tipos_primitivos_modal: bool,
    show_rustc_compilador_modal: bool,
    show_terminal_modal: bool,
    show_settings_modal: bool,
    settings_tab: usize,
    tipo_primitivo_categoria: usize,
    variable_name: String,
    variable_type: usize,
    variable_value: String,
    variable_mutable: bool,
    declaration_kind: usize,
    estructura_code: String,
    estructura_output: Arc<Mutex<String>>,
    estructura_tab: usize,
    conceptos_tab: usize,
    show_railroad_modal: Option<usize>,
    controlflujo_tab: usize,
    conceptos_code: String,
    conceptos_output: Arc<Mutex<String>>,

    // State para Componente de Terminal de 3 Modos
    term_selected_mode: usize,
    term_input: String,
    term_history: Arc<Mutex<Vec<String>>>,
    term_cwd: std::path::PathBuf,
    show_terminal_history: bool,
    created_project_name: Option<String>,
    selected_project: Option<String>,
}

impl Default for PortfolioState {
    fn default() -> Self {
        Self {
            ruta_actual: AppRoute::LandingPage,
            show_ingresos: true,
            show_gastos: true,
            show_beneficios: true,
            year: 2025,
            tutorial_step: 0,
            tutorial_time: 0.0,
            playground_code: "fn main() {\n    println!(\"¡Hola desde el Editor de Egui!\");\n}\n".to_string(),
            playground_output: Arc::new(Mutex::new(String::new())),
            datatypes_code: "fn main() {\n    let edad: u8 = 25;\n    let saldo: i32 = -15000;\n    println!(\"Edad: {}, Saldo: {}\", edad, saldo);\n}".to_string(),
            datatypes_output: Arc::new(Mutex::new(String::new())),
            strings_code: "fn main() {\n    let estatico: &str = \"¡Hola desde la memoria de sólo lectura (binario)!\";\n    let mut dinamico: String = String::from(\"¡Hola\");\n    dinamico.push_str(\" desde el Heap!\");\n    \n    // Convertir String a &str usando ref (&)\n    let prestado: &str = &dinamico;\n\n    println!(\"{}\\n{}\\n{}\", estatico, dinamico, prestado);\n}".to_string(),
            strings_output: Arc::new(Mutex::new(String::new())),
            playground_nube_code: "use std::collections::HashMap;\n// ¡Importamos serde sin haberlo configurado en Cargo.toml!\n// (Esto compila en la nube del Rust Playground oficial)\n\nfn main() {\n    let mut scores = HashMap::new();\n    scores.insert(\"Blue\", 10);\n    scores.insert(\"Yellow\", 50);\n\n    for (key, value) in &scores {\n        println!(\"{}: {}\", key, value);\n    }\n}".to_string(),
            playground_nube_output: Arc::new(Mutex::new(String::new())),
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            anim_compilacion_activa: false,
            compilacion_progreso: 1.0,
            compilacion_etapa_seleccionada: 4,
            dash_tab: 0,
            bcr_playing: false,
            bcr_year: 2015.0,
            bcr_speed: 1.0,
            pie_donut_hole: 0.5,
            pie_exploded: false,
            index_baseline_year: 2020.0,
            ts_show_ma: true,
            ts_show_volume: true,
            ts_show_rsi: true,

            controlflujo_code: "fn main() {\n    let numero = 7;\n    \n    // 'if' usado como expresión\n    let estado = if numero % 2 == 0 { \"par\" } else { \"impar\" };\n    println!(\"El número {} es {}\", numero, estado);\n\n    // Bucle 'for' sobre un rango inclusivo\n    print!(\"Conteo: \");\n    for i in 1..=5 {\n        print!(\"{} \", i);\n    }\n    println!();\n}\n".to_string(),
            controlflujo_output: Arc::new(Mutex::new(String::new())),
            controlflujo_val: 7,

            ownership_code: "fn main() {\n    // &str: vista (a menudo al binario o prestada)\n    let saludo: &str = \"Hola\";\n\n    // String: dueño en el Heap (puede crecer)\n    let mut s1 = String::from(\"Rust\");\n    s1.push_str(\" Ownership\");\n\n    // MOVE: s1 deja de ser válido\n    let s2 = s1;\n    // println!(\"{}\", s1); // error: value moved\n\n    // BORROW: prestamos sin regalar el dueño\n    imprimir_len(&s2);\n    println!(\"saludo={}, s2={}\", saludo, s2);\n}\n\nfn imprimir_len(texto: &str) {\n    println!(\"len = {}\", texto.len());\n}\n".to_string(),
            ownership_output: Arc::new(Mutex::new(String::new())),
            ownership_step: 0,
            strings_ownership_tab: 0,

            structs_code: "struct ServidorWeb {\n    puerto: u16,\n    host: String,\n    activo: bool,\n}\n\nimpl ServidorWeb {\n    fn new(puerto: u16, host: &str) -> Self {\n        Self {\n            puerto,\n            host: host.to_string(),\n            activo: false,\n        }\n    }\n\n    fn iniciar(&mut self) {\n        self.activo = true;\n        println!(\"Servidor iniciado en http://{}:{}\", self.host, self.puerto);\n    }\n}\n\nfn main() {\n    let mut mi_servidor = ServidorWeb::new(8080, \"127.0.0.1\");\n    mi_servidor.iniciar();\n}\n".to_string(),
            structs_output: Arc::new(Mutex::new(String::new())),

            enums_code: "enum EstadoPedido {\n    Pendiente,\n    Enviado { guia: String },\n    Entregado,\n}\n\nfn procesar(estado: EstadoPedido) {\n    match estado {\n        EstadoPedido::Pendiente => println!(\"⏳ El pedido está pendiente\"),\n        EstadoPedido::Enviado { guia } => println!(\"🚚 En camino. Guía: {}\", guia),\n        EstadoPedido::Entregado => println!(\"✅ Pedido entregado\"),\n    }\n}\n\nfn main() {\n    let pedido = EstadoPedido::Enviado { guia: \"RUST-9921\".to_string() };\n    procesar(pedido);\n}\n".to_string(),
            enums_output: Arc::new(Mutex::new(String::new())),
            enum_variant_selected: 1,

            colecciones_code: "use std::collections::HashMap;\n\nfn main() {\n    // Vector dinámico\n    let mut numeros = vec![10, 20, 30];\n    numeros.push(40);\n    println!(\"Vector: {:?}, len: {}, cap: {}\", numeros, numeros.len(), numeros.capacity());\n\n    // HashMap\n    let mut puntajes = HashMap::new();\n    puntajes.insert(\"Rustaceans\", 100);\n    puntajes.insert(\"Gophers\", 85);\n\n    for (equipo, puntos) in &puntajes {\n        println!(\"Equipo {}: {} pts\", equipo, puntos);\n    }\n}\n".to_string(),
            colecciones_output: Arc::new(Mutex::new(String::new())),
            vec_sim_len: 3,
            vec_sim_cap: 4,

            errores_code: "fn dividir(a: f64, b: f64) -> Result<f64, String> {\n    if b == 0.0 {\n        Err(\"No se puede dividir entre cero\".to_string())\n    } else {\n        Ok(a / b)\n    }\n}\n\nfn calcular() -> Result<f64, String> {\n    let res1 = dividir(100.0, 2.0)?;\n    let res2 = dividir(res1, 5.0)?;\n    Ok(res2)\n}\n\nfn main() {\n    match calcular() {\n        Ok(val) => println!(\"Resultado exitoso: {}\", val),\n        Err(err) => println!(\"Error en cálculo: {}\", err),\n    }\n}\n".to_string(),
            errores_output: Arc::new(Mutex::new(String::new())),
            err_pipeline_fail: false,

            traits_code: "trait Dibujable {\n    fn dibujar(&self);\n}\n\nstruct Circulo { radio: f64 }\nstruct Rectangulo { ancho: f64, alto: f64 }\n\nimpl Dibujable for Circulo {\n    fn dibujar(&self) {\n        println!(\"🔴 Dibujando círculo de radio {}\", self.radio);\n    }\n}\n\nimpl Dibujable for Rectangulo {\n    fn dibujar(&self) {\n        println!(\"🟦 Dibujando rectángulo {}x{}\", self.ancho, self.alto);\n    }\n}\n\nfn renderizar(item: &impl Dibujable) {\n    item.dibujar();\n}\n\nfn main() {\n    let c = Circulo { radio: 5.0 };\n    let r = Rectangulo { ancho: 10.0, alto: 4.0 };\n    renderizar(&c);\n    renderizar(&r);\n}\n".to_string(),
            traits_output: Arc::new(Mutex::new(String::new())),

            arr_elem_type: 1,
            arr_len: 5,
            arr_active_idx: 2,
            arr_action_msg: "Inspecciona métodos y accesos a elementos del arreglo".to_string(),
            compuestos_tab: 0,
            slice_start: 1,
            slice_end: 4,
            tup_t0: 1,
            tup_t1: 3,
            tup_t2: 2,
            structs_tab: 0,

            funciones_code: "fn main() {\n    let a = 15;\n    let b = 25;\n\n    // Llamada + retorno implícito (sin ';')\n    let suma = calcular_suma(a, b);\n    println!(\"La suma de {} + {} es: {}\", a, b, suma);\n\n    // Paso por referencia mutable\n    let mut contador = 0;\n    incrementar(&mut contador);\n    println!(\"Contador incrementado: {}\", contador);\n\n    // Closure: captura el entorno\n    let factor = 3;\n    let multiplicar = |x: i32| x * factor;\n    println!(\"10 * factor = {}\", multiplicar(10));\n}\n\nfn calcular_suma(x: i32, y: i32) -> i32 {\n    x + y // última expresión = retorno\n}\n\nfn incrementar(val: &mut i32) {\n    *val += 1;\n}\n".to_string(),
            funciones_output: Arc::new(Mutex::new(String::new())),
            funciones_step: 0,
            funciones_tab: 0,

            iteradores_code: "fn main() {\n    let numeros = vec![1, 2, 3, 4, 5, 6];\n\n    println!(\"--- 1. Iterar por referencia (&T) con .iter() ---\");\n    for n in numeros.iter() {\n        print!(\"{} \", n);\n    }\n    println!();\n\n    println!(\"--- 2. Pipeline Lazy: filter -> map -> collect ---\");\n    let pares_cuadrados: Vec<i32> = numeros\n        .iter()\n        .filter(|&&x| x % 2 == 0)\n        .map(|&x| x * x)\n        .collect();\n\n    println!(\"Pares al cuadrado: {:?}\", pares_cuadrados);\n}\n".to_string(),
            iteradores_output: Arc::new(Mutex::new(String::new())),
            iter_mode: 0,
            iter_filter_even: true,

            arr_code: "fn main() {\n    let arr: [i32; 5] = [100, -500, 2048, 42, 0];\n    println!(\"Arreglo completo: {:?}\", arr);\n    println!(\"Longitud (arr.len()): {}\", arr.len());\n    println!(\"Tamaño en Stack: {} bytes\", std::mem::size_of_val(&arr));\n\n    for (idx, elem) in arr.iter().enumerate() {\n        println!(\"arr[{}] = {}\", idx, elem);\n    }\n}\n".to_string(),
            arr_output: Arc::new(Mutex::new(String::new())),

            slice_code: "fn main() {\n    let arreglo = [10, 20, 30, 40, 50, 60];\n    let slice: &[i32] = &arreglo[1..4];\n\n    println!(\"Array base: {:?}\", arreglo);\n    println!(\"Slice &arreglo[1..4]: {:?}\", slice);\n    println!(\"Longitud del slice: {}\", slice.len());\n    println!(\"Primer elemento del slice: {:?}\", slice.first());\n}\n".to_string(),
            slice_output: Arc::new(Mutex::new(String::new())),

            tup_code: "fn main() {\n    let mi_tupla: (i32, bool, f64) = (100, true, 3.1415);\n\n    println!(\"Tupla completa: {:?}\", mi_tupla);\n    println!(\"Campos: .0 = {}, .1 = {}, .2 = {}\", mi_tupla.0, mi_tupla.1, mi_tupla.2);\n\n    let (entero, booleano, flotante) = mi_tupla;\n    println!(\"Desestructurado: {}, {}, {}\", entero, booleano, flotante);\n}\n".to_string(),
            tup_output: Arc::new(Mutex::new(String::new())),

            comenzando_step: 0,
            pilares_step: 0,
            show_commands_modal: false,
            show_macro_expansion: false,
            show_cargo_output_modal: Arc::new(AtomicBool::new(false)),
            show_tipos_primitivos_modal: false,
            show_rustc_compilador_modal: false,
            show_terminal_modal: false,
            show_settings_modal: false,
            settings_tab: 0,
            tipo_primitivo_categoria: 0,
            variable_name: "edad".to_string(),
            variable_type: 6,
            variable_value: "25".to_string(),
            variable_mutable: false,
            declaration_kind: 0,
            estructura_code: "// Demostración de Módulos y Visibilidad en Rust\nmod redes {\n    // Módulo interno público\n    pub mod http {\n        pub fn conectar(url: &str) {\n            println!(\"🔗 Conectando a {}\", url);\n        }\n    }\n}\n\n// Re-exportación fachada con 'pub use'\npub use redes::http::conectar;\n\nfn main() {\n    // Llamada vía alias re-exportado\n    conectar(\"https://rust-lang.org\");\n\n    // Llamada vía ruta completa del módulo\n    redes::http::conectar(\"https://crates.io\");\n}\n".to_string(),
            estructura_output: Arc::new(Mutex::new(String::new())),
            estructura_tab: 0,
            conceptos_tab: 0,
            show_railroad_modal: None,
            controlflujo_tab: 0,
            conceptos_code: "fn main() {\n    let edad: u8 = 25;\n    println!(\"edad = {edad}\");\n}\n".to_string(),
            conceptos_output: Arc::new(Mutex::new(String::new())),

            term_selected_mode: 1,
            term_input: String::new(),
            term_history: Arc::new(Mutex::new(Vec::new())),
            term_cwd: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/home/alek")),
            show_terminal_history: false,
            created_project_name: None,
            selected_project: None,
        }
    }
}

impl PortfolioState {
    fn obtener_codigo_activo(&self) -> &str {
        match self.ruta_actual {
            AppRoute::TutorialControlFlujo => &self.controlflujo_code,
            AppRoute::TutorialTiposDatos => &self.datatypes_code,
            AppRoute::TutorialStrings => &self.strings_code,
            AppRoute::Playground => &self.playground_code,
            _ => &self.conceptos_code,
        }
    }

    fn obtener_output_activo(&self) -> Arc<Mutex<String>> {
        match self.ruta_actual {
            AppRoute::TutorialControlFlujo => Arc::clone(&self.controlflujo_output),
            AppRoute::TutorialTiposDatos => Arc::clone(&self.datatypes_output),
            AppRoute::TutorialStrings => Arc::clone(&self.strings_output),
            AppRoute::Playground => Arc::clone(&self.playground_output),
            _ => Arc::clone(&self.conceptos_output),
        }
    }

    fn guardar_proyecto_activo(&self) {
        if let Some(ref proj) = self.selected_project {
            let proj_dir = buscar_ruta_proyecto(&self.term_cwd, proj);
            let main_rs = proj_dir.join("src/main.rs");
            let lib_rs = proj_dir.join("src/lib.rs");
            let target_file = if main_rs.exists() {
                main_rs
            } else if lib_rs.exists() {
                lib_rs
            } else {
                main_rs.clone()
            };
            if target_file.parent().is_some_and(|p| p.exists()) {
                let _ = std::fs::write(target_file, self.obtener_codigo_activo());
            }
        }
    }
}

fn formatear_salida_consola(text: &str, _es_error: bool) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font_id = egui::FontId::monospace(14.0);

    let color_error = egui::Color32::from_rgb(255, 90, 90);
    let color_warning = egui::Color32::from_rgb(255, 200, 50);
    let color_note = egui::Color32::from_rgb(100, 200, 255);
    let color_help = egui::Color32::from_rgb(100, 255, 150);
    let color_normal = egui::Color32::from_rgb(220, 225, 235);

    let mut current_context_warning = false;

    for line in text.split('\n') {
        let trimmed = line.trim_start();

        if trimmed.starts_with("warning:") || trimmed.starts_with("warning") {
            current_context_warning = true;
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_warning),
            );
        } else if trimmed.starts_with("error:") || trimmed.starts_with("error") {
            current_context_warning = false;
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_error),
            );
        } else if trimmed.starts_with("note:") || trimmed.starts_with("= note:") {
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_note),
            );
        } else if trimmed.starts_with("help:") || trimmed.starts_with("= help:") {
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_help),
            );
        } else if trimmed.starts_with("-->") {
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_note),
            );
        } else if line.contains('|') {
            let parts: Vec<&str> = line.splitn(2, '|').collect();
            if parts.len() == 2 {
                job.append(
                    parts[0],
                    0.0,
                    egui::TextFormat::simple(font_id.clone(), color_note),
                );
                job.append(
                    "|",
                    0.0,
                    egui::TextFormat::simple(font_id.clone(), color_note),
                );

                let code_part = parts[1];
                if code_part.contains("help:") {
                    if let Some(idx) = code_part.find("help:") {
                        let (prefix, help_str) = code_part.split_at(idx);
                        let prefix_color = if current_context_warning {
                            color_warning
                        } else {
                            color_error
                        };
                        job.append(
                            prefix,
                            0.0,
                            egui::TextFormat::simple(font_id.clone(), prefix_color),
                        );
                        job.append(
                            help_str,
                            0.0,
                            egui::TextFormat::simple(font_id.clone(), color_help),
                        );
                    } else {
                        job.append(
                            code_part,
                            0.0,
                            egui::TextFormat::simple(font_id.clone(), color_help),
                        );
                    }
                } else if code_part.contains('^') || code_part.contains('~') {
                    let indicator_color = if current_context_warning {
                        color_warning
                    } else {
                        color_error
                    };
                    job.append(
                        code_part,
                        0.0,
                        egui::TextFormat::simple(font_id.clone(), indicator_color),
                    );
                } else {
                    job.append(
                        code_part,
                        0.0,
                        egui::TextFormat::simple(font_id.clone(), color_normal),
                    );
                }
            } else {
                job.append(
                    line,
                    0.0,
                    egui::TextFormat::simple(font_id.clone(), color_normal),
                );
            }
        } else {
            job.append(
                line,
                0.0,
                egui::TextFormat::simple(font_id.clone(), color_normal),
            );
        }
        job.append(
            "\n",
            0.0,
            egui::TextFormat::simple(font_id.clone(), color_normal),
        );
    }

    job
}

fn rust_layouter(
    ui: &egui::Ui,
    string: &str,
    wrap_width: f32,
    syntax_set: &SyntaxSet,
    theme: &syntect::highlighting::Theme,
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

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_title("FerrisKey 🦀🔑 - El Ecosistema Interactivo de Rust"),
        ..Default::default()
    };

    eframe::run_native(
        "FerrisKey 🦀🔑 - El Ecosistema Interactivo de Rust (Luis Alexander / Alekay)",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(PortfolioState::default()))
        }),
    )
}

impl eframe::App for PortfolioState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Alternar pantalla completa con F11 o salir con la tecla Escape
        let is_fullscreen = ui.ctx().input(|i| i.viewport().fullscreen.unwrap_or(false));
        if ui.ctx().input(|i| i.key_pressed(egui::Key::F11)) {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Fullscreen(!is_fullscreen));
        } else if is_fullscreen && ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        }

        // Actualizar tiempo de animación (dt)
        self.tutorial_time += ui.ctx().input(|i| i.stable_dt) as f64;

        // --- 1. PANEL DE NAVEGACIÓN (SIDEBAR - Solo si no está en LandingPage) ---
        if self.ruta_actual != AppRoute::LandingPage {
            egui::Panel::left("sidebar")
                .resizable(false)
                .show(ui, |ui| {
                    ui.set_min_width(220.0);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_space(20.0);

                        ui.vertical_centered(|ui| {
                            let mut job = egui::text::LayoutJob::default();
                            job.append(
                                "Ferris",
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::FontId::new(
                                        24.0,
                                        egui::FontFamily::Proportional,
                                    ),
                                    color: egui::Color32::from_rgb(255, 160, 50),
                                    ..Default::default()
                                },
                            );
                            job.append(
                                "Key",
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::FontId::new(
                                        24.0,
                                        egui::FontFamily::Proportional,
                                    ),
                                    color: egui::Color32::WHITE,
                                    ..Default::default()
                                },
                            );

                            let logo_response = ui
                                .add(egui::Label::new(job).sense(egui::Sense::click()))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Volver a la pantalla de inicio (FerrisKey)");

                            if logo_response.clicked() {
                                self.ruta_actual = AppRoute::LandingPage;
                            }

                            ui.label(
                                egui::RichText::new("Aprende Rust Jugando")
                                    .size(12.0)
                                    .italics()
                                    .color(egui::Color32::GRAY),
                            );
                        });

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(15.0);

                        ui.label(
                            egui::RichText::new("CURSO RUST COMPLETO")
                                .strong()
                                .color(egui::Color32::GRAY),
                        );
                        ui.add_space(10.0);

                        // 1. Pilares
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialCargo,
                                "📦 Pilares",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialCargo;
                        }

                        // 2. Comenzando
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::Comenzando,
                                "🚀 Comenzando",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::Comenzando;
                        }

                        // 3. Strings & Ownership (reglas de memoria + String/&str)
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialOwnership
                                    || self.ruta_actual == AppRoute::TutorialStrings,
                                "🧵 Strings & Ownership",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialOwnership;
                        }

                        // 4. Control de Flujo
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialControlFlujo,
                                "🔀 Control de Flujo",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialControlFlujo;
                        }

                        // 5. Funciones & Closures
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialFunciones,
                                "⚡ Funciones & Closures",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialFunciones;
                        }

                        // 6. Tipos compuestos
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialTiposDatos,
                                "🧱 Tipos compuestos",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialTiposDatos;
                        }

                        // 7. Structs & impl
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialStructs,
                                "🏗️ Structs & impl",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialStructs;
                        }

                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialCompilacion,
                                "⚙️ Proceso de Compilación",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialCompilacion;
                        }
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialIteradores,
                                "🔄 Iteradores",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialIteradores;
                        }
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialEnums,
                                "🏷️ Enums, Option & Result",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialEnums;
                        }
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialColecciones,
                                "📚 Colecciones (Vec/HashMap)",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialColecciones;
                        }
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialErrores,
                                "🚨 Manejo de Errores (?)",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialErrores;
                        }
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::TutorialTraits,
                                "🧬 Traits & Genéricos",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::TutorialTraits;
                        }

                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("PROYECTOS TÉCNICOS")
                                .strong()
                                .color(egui::Color32::GRAY),
                        );
                        ui.add_space(10.0);
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::DashboardGraficos,
                                "📊 Visualización de Datos",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::DashboardGraficos;
                        }
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::Playground,
                                "💻 Editor Local",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::Playground;
                        }
                        if ui
                            .selectable_label(
                                self.ruta_actual == AppRoute::PlaygroundNube,
                                "☁️ Playground API",
                            )
                            .clicked()
                        {
                            self.ruta_actual = AppRoute::PlaygroundNube;
                        }

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            let is_cargo_open =
                                self.show_cargo_output_modal.load(Ordering::Relaxed);
                            let cargo_text_color = if is_cargo_open {
                                egui::Color32::from_rgb(255, 160, 50)
                            } else {
                                egui::Color32::from_rgb(180, 190, 205)
                            };

                            if ui
                                .button(
                                    egui::RichText::new("ℹ️").size(18.0).color(cargo_text_color),
                                )
                                .on_hover_text("Información / Salida de compilación y macros")
                                .clicked()
                            {
                                self.show_cargo_output_modal
                                    .store(!is_cargo_open, Ordering::Relaxed);
                            }

                            ui.add_space(8.0);

                            let term_text_color = if self.show_terminal_modal {
                                egui::Color32::from_rgb(255, 160, 50)
                            } else {
                                egui::Color32::from_rgb(180, 190, 205)
                            };

                            if ui
                                .button(egui::RichText::new("💻").size(18.0).color(term_text_color))
                                .on_hover_text("Terminal Linux interactiva")
                                .clicked()
                            {
                                self.show_terminal_modal = !self.show_terminal_modal;
                            }

                            ui.add_space(8.0);

                            let config_icon_color = if self.show_settings_modal {
                                egui::Color32::from_rgb(255, 160, 50)
                            } else {
                                egui::Color32::from_rgb(180, 190, 205)
                            };

                            let config_img = egui::Image::from_bytes(
                                "bytes://config.svg",
                                include_bytes!("../diagramas/config.svg"),
                            )
                            .fit_to_exact_size(egui::vec2(20.0, 20.0))
                            .tint(config_icon_color);

                            if ui
                                .add(egui::Button::image(config_img))
                                .on_hover_text("Configuración y Atajos de Teclado")
                                .clicked()
                            {
                                self.show_settings_modal = !self.show_settings_modal;
                            }
                        });
                    });
                });
        }

        // --- 2. PANEL CENTRAL ---
        let central_panel = if self.ruta_actual == AppRoute::LandingPage {
            egui::CentralPanel::default().frame(egui::Frame::new().fill(egui::Color32::BLACK))
        } else {
            egui::CentralPanel::default()
        };
        central_panel.show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.ruta_actual {
                AppRoute::LandingPage => mostrar_landing_page(ui, self),
                AppRoute::Portafolio => mostrar_portafolio(ui),
                AppRoute::TutorialCargo => mostrar_tutorial_cargo(ui, self),
                AppRoute::Comenzando => mostrar_comenzando(ui, self),
                AppRoute::TutorialCompilacion => mostrar_tutorial_compilacion(ui, self),
                AppRoute::TutorialTiposDatos => mostrar_tutorial_tipos_datos(ui, self),
                AppRoute::TutorialControlFlujo => mostrar_tutorial_control_flujo(ui, self),
                AppRoute::TutorialFunciones => mostrar_tutorial_funciones(ui, self),
                AppRoute::TutorialIteradores => mostrar_tutorial_iteradores(ui, self),
                AppRoute::TutorialOwnership | AppRoute::TutorialStrings => {
                    mostrar_tutorial_strings_ownership(ui, self)
                }
                AppRoute::TutorialStructs => mostrar_tutorial_structs(ui, self),
                AppRoute::TutorialEnums => mostrar_tutorial_enums(ui, self),
                AppRoute::TutorialColecciones => mostrar_tutorial_colecciones(ui, self),
                AppRoute::TutorialErrores => mostrar_tutorial_errores(ui, self),
                AppRoute::TutorialTraits => mostrar_tutorial_traits(ui, self),
                AppRoute::TutorialMemoria => mostrar_tutorial_memoria(ui, self),
                AppRoute::DashboardGraficos => mostrar_graficos(ui, self),
                AppRoute::Playground => mostrar_editor(ui, self),
                AppRoute::PlaygroundNube => mostrar_editor_nube(ui, self),
            });
        });

        // --- PROCESAR ATAJOS DE TECLADO GLOBALES ---
        let mut ejecutar_cargo_run_flag = false;
        ui.ctx().input_mut(|i| {
            // Filtrar eventos brutos de Escape para evitar que eframe o el viewport soliciten cambios de pantalla completa
            i.events.retain(|event| {
                !matches!(
                    event,
                    egui::Event::Key {
                        key: egui::Key::Escape,
                        pressed: true,
                        ..
                    }
                )
            });

            // Ctrl + T -> Toggle Terminal
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::T,
            )) {
                self.show_terminal_modal = !self.show_terminal_modal;
            }
            // Ctrl + I -> Toggle Info Output Modal
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::I,
            )) {
                let is_open = self.show_cargo_output_modal.load(Ordering::Relaxed);
                self.show_cargo_output_modal
                    .store(!is_open, Ordering::Relaxed);
            }
            // Esc o Ctrl + W -> Cerrar ventanas modales flotantes sin alterar el modo pantalla completa
            let esc_pressed = i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::Escape,
            ));
            let ctrl_w_pressed = i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::W,
            ));
            if esc_pressed || ctrl_w_pressed {
                self.show_terminal_modal = false;
                self.show_cargo_output_modal.store(false, Ordering::Relaxed);
                self.show_settings_modal = false;
                self.show_tipos_primitivos_modal = false;
                self.show_railroad_modal = None;
                self.created_project_name = None;
            }
            // Ctrl + S -> Guardar proyecto activo
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::S,
            )) {
                self.guardar_proyecto_activo();
            }
            // Ctrl + Enter o F5 -> Ejecutar cargo run en segundo plano y abrir modal centrado
            let ctrl_enter = i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::Enter,
            ));
            let f5_pressed = i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::F5,
            ));
            if ctrl_enter || f5_pressed {
                ejecutar_cargo_run_flag = true;
            }
        });

        if ejecutar_cargo_run_flag {
            ejecutar_cargo_run_proyecto(self, ui.ctx());
        }

        mostrar_modal_comparacion_compiladores(ui.ctx(), self);
        mostrar_modal_salida_cargo(ui.ctx(), self);
        mostrar_modal_terminal(ui.ctx(), self);
        mostrar_modal_settings(ui.ctx(), self);
        mostrar_modal_template_creado(ui.ctx(), self);
        mostrar_modal_railroad_let(ui.ctx(), self);

        // Request continuous repaint for animations
        ui.ctx().request_repaint();
    }
}

// === FUNCIONES DE VISTAS SEPARADAS ===

fn mostrar_landing_page(ui: &mut egui::Ui, state: &mut PortfolioState) {
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
                egui::Image::new(egui::include_image!("../assets/taller/home2.png"))
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

fn mostrar_portafolio(ui: &mut egui::Ui) {
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

fn mostrar_tutorial_cargo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Pilares de Rust")
                .size(28.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 160, 50)),
        );
    });

    ui.add_space(15.0);

    // Barra de navegación de Sub-Pasos de Pilares
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        for (indice, texto) in ["Entorno de Trabajo", "Conceptos"].iter().enumerate() {
            let es_activo = state.pilares_step == indice;
            let text_color = if es_activo {
                egui::Color32::from_rgb(255, 160, 50)
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            };

            let btn_text = egui::RichText::new(*texto).strong().color(text_color);
            if ui
                .add(egui::Button::new(btn_text).frame(es_activo))
                .clicked()
            {
                state.pilares_step = indice;
            }
            ui.add_space(6.0);
        }
    });
    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.pilares_step {
        0 => mostrar_pilares_entorno_trabajo(ui, state),
        _ => mostrar_pilares_conceptos(ui, state),
    }
}

fn mostrar_pilares_entorno_trabajo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Estilo unificado de tarjetas
        let mut card_frame = egui::Frame::new();
        card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
        card_frame.inner_margin = egui::Margin::same(12);
        card_frame.corner_radius = egui::CornerRadius::same(8);
        card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

        let title_color = egui::Color32::from_rgb(255, 180, 100);
        let text_color = egui::Color32::from_rgb(200, 210, 225);

        // --- FILA 1: NÚCLEO DE CONSTRUCCIÓN Y ECOSISTEMA ---
        ui.heading(
            egui::RichText::new("Núcleo de Construcción y Ecosistema")
                .size(18.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(8.0);

        ui.columns(3, |columns| {
            // Pilar 1: rustc
            card_frame.show(&mut columns[0], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://rustc.svg",
                            include_bytes!("../diagramas/rustc.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustc").size(17.0).strong().color(title_color));
                    ui.add_space(4.0);
                    if ui.button(egui::RichText::new("🔍 Comparar").small().color(egui::Color32::from_rgb(100, 200, 255)))
                        .on_hover_text("Abrir comparativa didáctica: Compilador vs Intérprete vs Máquina Virtual")
                        .clicked()
                    {
                        state.show_rustc_compilador_modal = true;
                    }
                });
                ui.label(egui::RichText::new("El Compilador Real").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Traduce tu código Rust (.rs) a código máquina optimizado (ELF/EXE/WASM) usando LLVM.").color(text_color));
                ui.label(egui::RichText::new("• Realiza las verificaciones de seguridad de memoria y el Borrow Checker.").color(text_color));
            });

            // Pilar 2: Cargo
            card_frame.show(&mut columns[1], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://cargo2.svg",
                            include_bytes!("../diagramas/cargo2.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Cargo").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Orquestador / Manager").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Gestor de proyectos y administrador de paquetes oficial de Rust.").color(text_color));
                ui.label(egui::RichText::new("• Automatiza la descarga de dependencias, compilación y pruebas.").color(text_color));
            });

            // Pilar 3: Crates / crates.io
            card_frame.show(&mut columns[2], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://crates.svg",
                            include_bytes!("../diagramas/crates.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Crates").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Las Librerías y crates.io").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Una 'Crate' es la unidad de código ejecutable o biblioteca en Rust.").color(text_color));
                ui.label(egui::RichText::new("• crates.io es el registro público mundial donde la comunidad comparte paquetes.").color(text_color));
            });
        });

        ui.add_space(18.0);

        // --- FILA 2: CALIDAD, ESTILO Y DIAGNÓSTICO ---
        ui.heading(
            egui::RichText::new("Calidad, Estilo y Diagnósticos")
                .size(18.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(8.0);

        ui.columns(3, |cols| {
            // 1. Clippy
            card_frame.show(&mut cols[0], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://clippy.svg",
                            include_bytes!("../diagramas/clippy.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Clippy").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Linter Oficial").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Analiza tu código con más de 650 reglas avanzadas para detectar anti-patrones.").color(text_color));
                ui.label(egui::RichText::new("• Enseña las mejores prácticas del código idiomático en Rust.").color(text_color));
            });

            // 2. Rustfmt
            card_frame.show(&mut cols[1], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://format2.svg",
                            include_bytes!("../diagramas/format2.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Rustfmt").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("El Formateador Estándar").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Aplica automáticamente el libro de estilo unificado a todo el proyecto.").color(text_color));
                ui.label(egui::RichText::new("• Elimina discusiones de sangría y espacios en equipos de trabajo.").color(text_color));
            });

            // 3. Error Index
            card_frame.show(&mut cols[2], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://error.svg",
                            include_bytes!("../diagramas/error.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("Error Index").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Enciclopedia de Errores").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Enciclopedia explicativa completa para cada código de error del compilador.").color(text_color));
                ui.label(egui::RichText::new("• Muestra ejemplos de código correcto e incorrecto para aprender del error.").color(text_color));
            });
        });

        ui.add_space(18.0);

        // --- FILA 3: PRODUCTIVIDAD, IDE Y DOCUMENTACIÓN ---
        ui.heading(
            egui::RichText::new("Productividad, IDE y Documentación")
                .size(18.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(8.0);

        ui.columns(3, |cols| {
            // 1. rustup
            card_frame.show(&mut cols[0], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://rustup.svg",
                            include_bytes!("../diagramas/rustup.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustup").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Administrador de Toolchains").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Administra las versiones de Rust (Stable, Nightly) y la compilación cruzada.").color(text_color));
                ui.label(egui::RichText::new("• Permite añadir objetivos como WebAssembly (wasm32) fácilmente.").color(text_color));
            });

            // 2. rust-analyzer
            card_frame.show(&mut cols[1], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://analyzer.svg",
                            include_bytes!("../diagramas/analyzer.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rust-analyzer").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Servidor de Lenguaje (LSP)").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Proporciona autocompletado en vivo e inlays de tipos inferidos en tu IDE.").color(text_color));
                ui.label(egui::RichText::new("• Soporta VS Code, Antigravity y Neovim.").color(text_color));
            });

            // 3. rustdoc
            card_frame.show(&mut cols[2], |ui| {
                ui.set_min_height(140.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_bytes(
                            "bytes://doc.svg",
                            include_bytes!("../diagramas/doc.svg"),
                        )
                        .fit_to_exact_size(egui::vec2(22.0, 22.0))
                        .tint(title_color),
                    );
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("rustdoc").size(17.0).strong().color(title_color));
                });
                ui.label(egui::RichText::new("Generador de Documentación").strong().color(egui::Color32::WHITE));
                ui.add_space(6.0);
                ui.label(egui::RichText::new("• Lee los comentarios de documentación (///) y genera una web HTML completa.").color(text_color));
                ui.label(egui::RichText::new("• Ejecuta doctests automáticamente para garantizar que la documentación funcione.").color(text_color));
            });
        });
    });
}

fn mostrar_pilares_proyecto(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.heading(
        egui::RichText::new("Estructura de Proyectos en Rust")
            .size(20.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.label(
        "Al crear un proyecto con Cargo (ej: 'cargo new mi_proyecto'), Rust genera automáticamente la jerarquía de archivos y carpetas estándar.",
    );
    ui.add_space(12.0);

    // Tabla comparativa main.rs vs lib.rs
    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(14);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_main_vs_lib_pilares")
            .striped(true)
            .spacing([25.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo de Proyecto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Comando de Creación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Archivo Punto de Entrada")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Diagrama")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Fila 1: Ejecutable (src/main.rs)
                let btn_color_main = if state.show_railroad_modal == Some(2) {
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };

                ui.label(
                    egui::RichText::new("Ejecutable (Binario)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("cargo new <nombre>")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("src/main.rs")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Programas independientes con función main() ejecutables por la CPU.");
                if ui
                    .add(
                        egui::Button::image(
                            egui::Image::from_bytes(
                                "bytes://view.svg",
                                include_bytes!("../diagramas/view.svg"),
                            )
                            .fit_to_exact_size(egui::vec2(20.0, 20.0))
                            .tint(btn_color_main),
                        )
                        .frame(state.show_railroad_modal == Some(2)),
                    )
                    .on_hover_text("Ver diagrama Railroad de sintaxis (fn main ejecutable)")
                    .clicked()
                {
                    state.show_railroad_modal = if state.show_railroad_modal == Some(2) { None } else { Some(2) };
                }
                ui.end_row();

                // Fila 2: Librería (src/lib.rs)
                let btn_color_lib = if state.show_railroad_modal == Some(3) {
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };

                ui.label(
                    egui::RichText::new("Librería (Library)")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("cargo new <nombre> --lib")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("src/lib.rs")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Reutilización de código y módulos para ser consumidos por otros crates.");
                if ui
                    .add(
                        egui::Button::image(
                            egui::Image::from_bytes(
                                "bytes://view.svg",
                                include_bytes!("../diagramas/view.svg"),
                            )
                            .fit_to_exact_size(egui::vec2(20.0, 20.0))
                            .tint(btn_color_lib),
                        )
                        .frame(state.show_railroad_modal == Some(3)),
                    )
                    .on_hover_text("Ver diagrama Railroad de sintaxis (librería lib.rs)")
                    .clicked()
                {
                    state.show_railroad_modal = if state.show_railroad_modal == Some(3) { None } else { Some(3) };
                }
                ui.end_row();
            });
    });

    // Desglose de Template Generado + Imagen 7.png si se ha creado un proyecto
    mostrar_desglose_template_con_imagen(ui, state);
}

fn mostrar_pilares_tiempo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.heading(
        egui::RichText::new("Fases de Vida del Código: Compile Time vs Run Time")
            .size(20.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(8.0);
    ui.label("En Rust existe una clara división entre la fase previa de análisis en desarrollo y la ejecución final por la CPU:");
    ui.add_space(12.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(14);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_compilacion_vs_ejecucion_pilares")
            .striped(true)
            .spacing([25.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Aspecto")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tiempo de Compilación")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tiempo de Ejecución")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("¿Cuándo ocurre?")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Antes de crear el ejecutable, mientras rustc procesa el código.");
                ui.label("Mientras el usuario final tiene abierta la aplicación.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("¿Quién lo ejecuta?")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("El compilador (rustc / LLVM) en la PC del desarrollador.");
                ui.label("La CPU del sistema operativo en la PC del usuario.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("Procesos Clave")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    "Verificación de sintaxis, chequeo de tipos, Borrow Checker y optimización.",
                );
                ui.label(
                    "Interacción con el usuario, lectura de archivos, red y cálculo de lógica.",
                );
                ui.end_row();

                // Fila 4: Diagrama Railroad
                ui.label(
                    egui::RichText::new("Diagrama Railroad")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );

                let btn_color_compile = if state.show_railroad_modal == Some(4) {
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };

                if ui
                    .add(
                        egui::Button::image(
                            egui::Image::from_bytes(
                                "bytes://view.svg",
                                include_bytes!("../diagramas/view.svg"),
                            )
                            .fit_to_exact_size(egui::vec2(20.0, 20.0))
                            .tint(btn_color_compile),
                        )
                        .frame(state.show_railroad_modal == Some(4)),
                    )
                    .on_hover_text("Ver diagrama Railroad (Tiempo de Compilación)")
                    .clicked()
                {
                    state.show_railroad_modal = if state.show_railroad_modal == Some(4) { None } else { Some(4) };
                }

                let btn_color_run = if state.show_railroad_modal == Some(5) {
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };

                if ui
                    .add(
                        egui::Button::image(
                            egui::Image::from_bytes(
                                "bytes://view.svg",
                                include_bytes!("../diagramas/view.svg"),
                            )
                            .fit_to_exact_size(egui::vec2(20.0, 20.0))
                            .tint(btn_color_run),
                        )
                        .frame(state.show_railroad_modal == Some(5)),
                    )
                    .on_hover_text("Ver diagrama Railroad (Tiempo de Ejecución)")
                    .clicked()
                {
                    state.show_railroad_modal = if state.show_railroad_modal == Some(5) { None } else { Some(5) };
                }
                ui.end_row();

                ui.label(
                    egui::RichText::new("Impacto de Errores")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    "El ejecutable no se crea. El programador corrige el error en desarrollo.",
                );
                ui.label("Cierre inesperado (panic!) si no se manejan los errores.");
                ui.end_row();
            });
    });
}

fn mostrar_pilares_conceptos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        mostrar_pilares_tiempo(ui, state);
        ui.add_space(24.0);
        mostrar_pilares_proyecto(ui, state);

        ui.add_space(20.0);
        // Imagen de Ferris abajo de ambos conceptos alineada a la derecha
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.add(
                egui::Image::new(egui::include_image!("../assets/taller/3.png"))
                    .max_width(380.0)
                    .corner_radius(8),
            );
        });
    });
}

fn mostrar_desglose_template_con_imagen(ui: &mut egui::Ui, state: &PortfolioState) {
    if let Some(ref proj_name) = state.created_project_name {
        let is_lib = proj_name.contains("lib") || state.estructura_tab == 2;
        let src_file = if is_lib { "src/lib.rs" } else { "src/main.rs" };
        let src_desc = if is_lib {
            "Archivo raíz de la librería. No lleva fn main(), sino funciones y structs con pub."
        } else {
            "Archivo fuente principal ejecutable con la función de entrada fn main() { ... }."
        };

        ui.add_space(16.0);
        ui.heading(
            egui::RichText::new(format!("Template {}", proj_name))
                .size(18.0)
                .strong()
                .color(egui::Color32::WHITE),
        );
        ui.add_space(8.0);

        ui.horizontal_top(|ui| {
            // Columna Izquierda: Tabla Desglose Template
            let mut info_frame = egui::Frame::new();
            info_frame.fill = egui::Color32::from_rgb(18, 22, 32);
            info_frame.inner_margin = egui::Margin::same(14);
            info_frame.corner_radius = egui::CornerRadius::same(8);
            info_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 95));

            info_frame.show(ui, |ui| {
                egui::Grid::new("desglose_template_grid_pilares")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Archivo / Carpeta").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Cargo.toml").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Manifiesto con metadatos de tu proyecto (nombre, versión, dependencias).");
                        ui.end_row();

                        ui.label(egui::RichText::new("Cargo.lock").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Registro de versiones fijadas de dependencias.");
                        ui.end_row();

                        ui.label(egui::RichText::new(src_file).monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(src_desc);
                        ui.end_row();

                        ui.label(egui::RichText::new("target/").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label("Carpeta binaria donde rustc compila los ejecutables.");
                        ui.end_row();
                    });
            });

            ui.add_space(20.0);

            // Columna Derecha: Imagen 7.png fija inamovible
            ui.add(
                egui::Image::new(egui::include_image!("../assets/taller/7.png"))
                    .fit_to_exact_size(egui::vec2(340.0, 220.0))
                    .corner_radius(8),
            );
        });
    }
}

#[derive(Clone, Copy)]
struct TipoEscalar {
    nombre: &'static str,
    categoria: &'static str,
    bits: &'static str,
    bytes: &'static str,
    minimo: &'static str,
    maximo: &'static str,
    ejemplo: &'static str,
    descripcion: &'static str,
}

const TIPOS_ESCALARES: &[TipoEscalar] = &[
    TipoEscalar {
        nombre: "i8",
        categoria: "Entero con signo",
        bits: "8",
        bytes: "1",
        minimo: "-128",
        maximo: "127",
        ejemplo: "25",
        descripcion: "Entero pequeño que admite valores negativos.",
    },
    TipoEscalar {
        nombre: "i16",
        categoria: "Entero con signo",
        bits: "16",
        bytes: "2",
        minimo: "-32 768",
        maximo: "32 767",
        ejemplo: "1200",
        descripcion: "Útil cuando i8 es insuficiente y el rango sigue siendo moderado.",
    },
    TipoEscalar {
        nombre: "i32",
        categoria: "Entero con signo",
        bits: "32",
        bytes: "4",
        minimo: "-2 147 483 648",
        maximo: "2 147 483 647",
        ejemplo: "42",
        descripcion: "Tipo entero inferido por defecto para literales como 42.",
    },
    TipoEscalar {
        nombre: "i64",
        categoria: "Entero con signo",
        bits: "64",
        bytes: "8",
        minimo: "-9 223 372 036 854 775 808",
        maximo: "9 223 372 036 854 775 807",
        ejemplo: "5000000",
        descripcion: "Entero con un rango amplio para conteos grandes.",
    },
    TipoEscalar {
        nombre: "i128",
        categoria: "Entero con signo",
        bits: "128",
        bytes: "16",
        minimo: "−2^127",
        maximo: "2^127 − 1",
        ejemplo: "1000000",
        descripcion: "Entero de rango extraordinariamente amplio.",
    },
    TipoEscalar {
        nombre: "isize",
        categoria: "Entero con signo",
        bits: "Depende de la plataforma",
        bytes: "4 u 8",
        minimo: "Depende de la plataforma",
        maximo: "Depende de la plataforma",
        ejemplo: "10",
        descripcion: "Tiene el tamaño natural de la arquitectura y se usa en ciertas operaciones de memoria.",
    },
    TipoEscalar {
        nombre: "u8",
        categoria: "Entero sin signo",
        bits: "8",
        bytes: "1",
        minimo: "0",
        maximo: "255",
        ejemplo: "25",
        descripcion: "Ideal para bytes, canales de color y valores pequeños no negativos.",
    },
    TipoEscalar {
        nombre: "u16",
        categoria: "Entero sin signo",
        bits: "16",
        bytes: "2",
        minimo: "0",
        maximo: "65 535",
        ejemplo: "8080",
        descripcion: "Frecuente para puertos de red y cantidades medianas.",
    },
    TipoEscalar {
        nombre: "u32",
        categoria: "Entero sin signo",
        bits: "32",
        bytes: "4",
        minimo: "0",
        maximo: "4 294 967 295",
        ejemplo: "100",
        descripcion: "Entero no negativo con un rango amplio.",
    },
    TipoEscalar {
        nombre: "u64",
        categoria: "Entero sin signo",
        bits: "64",
        bytes: "8",
        minimo: "0",
        maximo: "18 446 744 073 709 551 615",
        ejemplo: "5000000",
        descripcion: "Útil para identificadores y contadores muy grandes.",
    },
    TipoEscalar {
        nombre: "u128",
        categoria: "Entero sin signo",
        bits: "128",
        bytes: "16",
        minimo: "0",
        maximo: "2^128 − 1",
        ejemplo: "1000000",
        descripcion: "El entero sin signo con mayor rango incorporado.",
    },
    TipoEscalar {
        nombre: "usize",
        categoria: "Entero sin signo",
        bits: "Depende de la plataforma",
        bytes: "4 u 8",
        minimo: "0",
        maximo: "Depende de la plataforma",
        ejemplo: "3",
        descripcion: "Tipo utilizado para índices y tamaños de colecciones.",
    },
    TipoEscalar {
        nombre: "f32",
        categoria: "Punto flotante",
        bits: "32",
        bytes: "4",
        minimo: "≈ −3.4 × 10^38",
        maximo: "≈ 3.4 × 10^38",
        ejemplo: "19.99",
        descripcion: "Decimal de precisión simple; ocupa menos memoria.",
    },
    TipoEscalar {
        nombre: "f64",
        categoria: "Punto flotante",
        bits: "64",
        bytes: "8",
        minimo: "≈ −1.8 × 10^308",
        maximo: "≈ 1.8 × 10^308",
        ejemplo: "3.14159",
        descripcion: "Decimal inferido por defecto y con mayor precisión que f32.",
    },
    TipoEscalar {
        nombre: "bool",
        categoria: "Booleano",
        bits: "8 en memoria",
        bytes: "1",
        minimo: "false",
        maximo: "true",
        ejemplo: "true",
        descripcion: "Representa una condición lógica verdadera o falsa.",
    },
    TipoEscalar {
        nombre: "char",
        categoria: "Carácter Unicode",
        bits: "32",
        bytes: "4",
        minimo: "U+0000",
        maximo: "U+10FFFF",
        ejemplo: "'A'",
        descripcion: "Representa un valor escalar Unicode, no solamente un byte ASCII.",
    },
];

const COMANDOS_TALLER: &[(&str, &str)] = &[
    (
        "rustup show",
        "Muestra el toolchain activo y los destinos instalados.",
    ),
    ("rustc --version", "Comprueba la versión del compilador."),
    ("cargo --version", "Comprueba la versión de Cargo."),
    ("cargo new mi_proyecto", "Crea un nuevo proyecto binario."),
    (
        "cargo check",
        "Comprueba el código rápidamente sin generar el ejecutable final.",
    ),
    ("cargo run", "Compila y ejecuta el programa."),
    (
        "cargo build",
        "Construye el proyecto y deja el resultado en target/.",
    ),
    ("cargo test", "Compila y ejecuta las pruebas."),
    ("cargo fmt", "Aplica el formato estándar de Rust."),
    (
        "cargo clippy",
        "Analiza el código y propone mejoras idiomáticas.",
    ),
    (
        "cargo doc --open",
        "Genera la documentación y la abre en el navegador.",
    ),
    (
        "cargo expand",
        "Expande macros; requiere instalar cargo-expand.",
    ),
];

fn codigo_variable(state: &PortfolioState) -> String {
    let tipo = TIPOS_ESCALARES[state.variable_type];
    let nombre = state.variable_name.trim();
    let nombre = if nombre.is_empty() {
        "variable"
    } else {
        nombre
    };
    let declaracion = match state.declaration_kind {
        1 => format!(
            "const {}: {} = {};",
            nombre.to_uppercase(),
            tipo.nombre,
            state.variable_value
        ),
        2 => format!(
            "static {}: {} = {};",
            nombre.to_uppercase(),
            tipo.nombre,
            state.variable_value
        ),
        _ => format!(
            "let {}{}: {} = {};",
            if state.variable_mutable { "mut " } else { "" },
            nombre,
            tipo.nombre,
            state.variable_value
        ),
    };
    let identificador = if state.declaration_kind == 0 {
        nombre.to_owned()
    } else {
        nombre.to_uppercase()
    };

    format!(
        "fn main() {{\n    {declaracion}\n    println!(\"{identificador} = {{{identificador}}}\");\n}}\n"
    )
}

fn mostrar_modal_comandos(ctx: &egui::Context, state: &mut PortfolioState) {
    let mut abierto = state.show_commands_modal;
    let mut comando_elegido = None;
    egui::Window::new("⌨ Referencia rápida de comandos")
        .open(&mut abierto)
        .collapsible(false)
        .resizable(true)
        .default_width(650.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().max_height(460.0).show(ui, |ui| {
                egui::Grid::new("comandos_taller_grid")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        for (comando, descripcion) in COMANDOS_TALLER {
                            ui.code(*comando);
                            ui.label(*descripcion);
                            if ui.small_button("Usar").clicked() {
                                comando_elegido = Some(*comando);
                            }
                            ui.end_row();
                        }
                    });
            });
            ui.separator();
            ui.small("cargo expand no forma parte de Cargo: se instala con `cargo install cargo-expand`.");
        });
    state.show_commands_modal = abierto;
    if let Some(comando) = comando_elegido {
        state.term_input = comando.to_owned();
    }
}

fn mostrar_modal_salida_cargo(ctx: &egui::Context, state: &mut PortfolioState) {
    if !state.show_cargo_output_modal.load(Ordering::Relaxed) {
        return;
    }

    let mut open = true;
    egui::Window::new("Salida")
        .open(&mut open)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(true)
        .default_size([650.0, 420.0])
        .collapsible(false)
        .show(ctx, |ui| {
            let output_text = state.obtener_output_activo().lock().unwrap().clone();

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    let mut out_frame = egui::Frame::new();
                    out_frame.fill = egui::Color32::from_rgb(10, 12, 18);
                    out_frame.inner_margin = egui::Margin::same(12);
                    out_frame.corner_radius = egui::CornerRadius::same(6);
                    out_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 55, 80));

                    out_frame.show(ui, |ui| {
                        ui.set_min_width(ui.available_width());

                        if output_text == "Compilando con Cargo..." {
                            ui.label(
                                egui::RichText::new(&output_text)
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
                });
        });

    if !open {
        state
            .show_cargo_output_modal
            .store(false, Ordering::Relaxed);
    }
}

fn mostrar_modal_tipos_primitivos(ctx: &egui::Context, state: &mut PortfolioState) {
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

fn mostrar_modal_comparacion_compiladores(ctx: &egui::Context, state: &mut PortfolioState) {
    if !state.show_rustc_compilador_modal {
        return;
    }

    let mut open = true;
    egui::Window::new("Arquitectura de ejecución")
        .open(&mut open)
        .resizable(true)
        .default_size([820.0, 500.0])
        .collapsible(false)
        .show(ctx, |ui| {
            let mut card_frame = egui::Frame::new();
            card_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            card_frame.inner_margin = egui::Margin::same(12);
            card_frame.corner_radius = egui::CornerRadius::same(8);
            card_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            let mut step_frame = egui::Frame::new();
            step_frame.fill = egui::Color32::from_rgb(22, 28, 40);
            step_frame.inner_margin = egui::Margin::symmetric(10, 6);
            step_frame.corner_radius = egui::CornerRadius::same(6);
            step_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 52, 75));

            let title_color = egui::Color32::from_rgb(255, 180, 100);
            let code_color = egui::Color32::from_rgb(200, 230, 255);
            let subtext_color = egui::Color32::from_rgb(180, 195, 215);

            ui.columns(3, |cols| {
                // 1. Compilado Nativo (Rust)
                card_frame.show(&mut cols[0], |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("Compilado Nativo")
                                .size(16.0)
                                .strong()
                                .color(title_color),
                        );
                        ui.label(
                            egui::RichText::new("Rust, C, C++, Go")
                                .strong()
                                .color(subtext_color),
                        );
                    });
                    ui.add_space(10.0);

                    // Pasos en Tarjetas
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 1: Código (.rs)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 2: rustc + LLVM").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 3: Binario (.exe / ELF)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 4: CPU (Directo)").strong().color(egui::Color32::WHITE));
                        });
                    });

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Rendimiento CPU:").strong().size(12.0));
                    ui.add(egui::ProgressBar::new(1.0).text("100% Nativo Directo"));
                });

                // 2. Interpretado (Python)
                card_frame.show(&mut cols[1], |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("Interpretado")
                                .size(16.0)
                                .strong()
                                .color(title_color),
                        );
                        ui.label(
                            egui::RichText::new("Python, JS, PHP")
                                .strong()
                                .color(subtext_color),
                        );
                    });
                    ui.add_space(10.0);

                    // Pasos en Tarjetas
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 1: Código (.py)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 2: Intérprete").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 3: CPU (En vivo)").strong().color(egui::Color32::WHITE));
                        });
                        ui.add_space(28.0);
                    });

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Rendimiento CPU:").strong().size(12.0));
                    ui.add(egui::ProgressBar::new(0.35).text("~35% Traduciendo en vivo"));
                });

                // 3. Máquina Virtual (Java)
                card_frame.show(&mut cols[2], |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("Máquina Virtual")
                                .size(16.0)
                                .strong()
                                .color(title_color),
                        );
                        ui.label(
                            egui::RichText::new("Java, C#, Kotlin")
                                .strong()
                                .color(subtext_color),
                        );
                    });
                    ui.add_space(10.0);

                    // Pasos en Tarjetas
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 1: Código (.java)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 2: Bytecode (.class)").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 3: VM / JIT").color(code_color));
                        });
                        ui.add_space(4.0);

                        step_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new("Paso 4: CPU Hardware").strong().color(egui::Color32::WHITE));
                        });
                    });

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Rendimiento CPU:").strong().size(12.0));
                    ui.add(egui::ProgressBar::new(0.75).text("~75% Vía VM / JIT"));
                });
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Conclusión Didáctica:").strong().color(egui::Color32::WHITE));
                ui.label("Rust compila a código binario nativo directo al hardware. ¡Por eso no requiere máquinas virtuales y tiene velocidad máxima!");
            });
        });

    if !open {
        state.show_rustc_compilador_modal = false;
    }
}

fn mostrar_modal_terminal(ctx: &egui::Context, state: &mut PortfolioState) {
    let mut abierto = state.show_terminal_modal;
    if !abierto {
        return;
    }

    egui::Window::new("Terminal Linux")
        .open(&mut abierto)
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -15.0))
        .default_size([760.0, 320.0])
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            mostrar_componente_terminal_3_modos(ui, "cargo run", state);
        });

    state.show_terminal_modal = abierto;
}

fn mostrar_modal_settings(ctx: &egui::Context, state: &mut PortfolioState) {
    let mut abierto = state.show_settings_modal;
    if !abierto {
        return;
    }

    egui::Window::new("⚙️ Centro de Control y Referencia")
        .open(&mut abierto)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .default_size([680.0, 420.0])
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            // Header estilo VS Code / Zed Settings Hub
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut state.settings_tab,
                    0,
                    egui::RichText::new("⌨️ Atajos de Teclado").strong().size(15.0),
                );
                ui.add_space(12.0);
                ui.selectable_value(
                    &mut state.settings_tab,
                    1,
                    egui::RichText::new("📦 Comandos de Cargo").strong().size(15.0),
                );
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(10.0);

            match state.settings_tab {
                0 => {
                    ui.label("Combinaciones de teclas globales habilitadas en toda la aplicación:");
                    ui.add_space(10.0);

                    let mut table_frame = egui::Frame::new();
                    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                    table_frame.inner_margin = egui::Margin::same(12);
                    table_frame.corner_radius = egui::CornerRadius::same(8);
                    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                    table_frame.show(ui, |ui| {
                        egui::Grid::new("tabla_atajos_teclado_grid")
                            .striped(true)
                            .spacing([25.0, 10.0])
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Atajo").strong().color(egui::Color32::WHITE));
                                ui.label(egui::RichText::new("Acción Principal").strong().color(egui::Color32::WHITE));
                                ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                                ui.end_row();

                                // Ctrl + T
                                ui.label(egui::RichText::new("Ctrl + T").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                                ui.label(egui::RichText::new("💻 Terminal").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                                ui.label("Abrir / Cerrar la consola Linux flotante.");
                                ui.end_row();

                                // Ctrl + I
                                ui.label(egui::RichText::new("Ctrl + I").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                                ui.label(egui::RichText::new("ℹ️ Info (Salida)").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                                ui.label("Abrir / Cerrar la ventana de salida de Cargo.");
                                ui.end_row();

                                // Ctrl + S
                                ui.label(egui::RichText::new("Ctrl + S").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                                ui.label(egui::RichText::new("💾 Guardar Proyecto").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                                ui.label("Guardar cambios del archivo de proyecto activo.");
                                ui.end_row();

                                // Esc / Ctrl + W
                                ui.label(egui::RichText::new("Esc / Ctrl + W").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                                ui.label(egui::RichText::new("❌ Cerrar Modales").strong().color(egui::Color32::from_rgb(180, 190, 205)));
                                ui.label("Cerrar todas las ventanas flotantes activas.");
                                ui.end_row();
                            });
                    });
                }
                _ => {
                    ui.label("Guía de referencia rápida de comandos de compilación y herramientas de Cargo:");
                    ui.add_space(10.0);

                    let mut table_frame = egui::Frame::new();
                    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                    table_frame.inner_margin = egui::Margin::same(12);
                    table_frame.corner_radius = egui::CornerRadius::same(8);
                    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                    let mut comando_elegido = None;

                    table_frame.show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(260.0)
                            .show(ui, |ui| {
                                egui::Grid::new("tabla_comandos_cargo_settings")
                                    .striped(true)
                                    .spacing([20.0, 10.0])
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("Comando").strong().color(egui::Color32::WHITE));
                                        ui.label(egui::RichText::new("Propósito / Descripción").strong().color(egui::Color32::WHITE));
                                        ui.label(egui::RichText::new("Acción").strong().color(egui::Color32::WHITE));
                                        ui.end_row();

                                        for (comando, descripcion) in COMANDOS_TALLER {
                                            ui.label(
                                                egui::RichText::new(*comando)
                                                    .monospace()
                                                    .strong()
                                                    .color(egui::Color32::from_rgb(255, 160, 50)),
                                            );
                                            ui.label(
                                                egui::RichText::new(*descripcion)
                                                    .color(egui::Color32::from_rgb(180, 190, 205)),
                                            );
                                            if ui.button(egui::RichText::new("▶ Usar").small().color(egui::Color32::from_rgb(100, 200, 255))).clicked() {
                                                comando_elegido = Some(*comando);
                                            }
                                            ui.end_row();
                                        }
                                    });
                            });
                    });

                    if let Some(cmd) = comando_elegido {
                        state.term_input = cmd.to_owned();
                        state.show_terminal_modal = true;
                    }
                }
            }
        });

    state.show_settings_modal = abierto;
}

/// Tamaño de visualización del diagrama railroad manteniendo la escala de texto.
///
/// Antes cada modal usaba un `fit_to_exact_size` distinto (p.ej. fn 800×80 vs lib 960×80)
/// y el SVG más ancho se escalaba más → el texto de `fn main` se veía más pequeño que el de
/// Librería aunque en el SVG el font-size sea el mismo. Aquí fijamos la **altura** y el
/// ancho sale del aspect ratio nativo del SVG.
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

fn mostrar_modal_railroad_let(ctx: &egui::Context, state: &mut PortfolioState) {
    let mode = match state.show_railroad_modal {
        Some(m) => m,
        None => return,
    };

    let mut abierto = true;
    let (titulo, bytes_data, uri) = match mode {
        0 => (
            "Inmutable",
            include_bytes!("../diagramas/diagrama_let_immut.svg").as_slice(),
            "bytes://diagrama_let_immut.svg",
        ),
        1 => (
            "Mutable",
            include_bytes!("../diagramas/diagrama_let_mut.svg").as_slice(),
            "bytes://diagrama_let_mut.svg",
        ),
        2 => (
            "fn main()",
            include_bytes!("../diagramas/diagrama_fn_main.svg").as_slice(),
            "bytes://diagrama_fn_main.svg",
        ),
        3 => (
            "Librería (src/lib.rs)",
            include_bytes!("../diagramas/diagrama_lib.svg").as_slice(),
            "bytes://diagrama_lib.svg",
        ),
        4 => (
            "Tiempo de Compilación (Compile Time)",
            include_bytes!("../diagramas/diagrama_compile_time.svg").as_slice(),
            "bytes://diagrama_compile_time.svg",
        ),
        _ => (
            "Tiempo de Ejecución (Run Time)",
            include_bytes!("../diagramas/diagrama_run_time.svg").as_slice(),
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

fn mostrar_modal_template_creado(ctx: &egui::Context, state: &mut PortfolioState) {
    let proj_name = match &state.created_project_name {
        Some(name) => name.clone(),
        None => return,
    };

    let mut open = true;
    egui::Window::new(format!("📦 Template {}", proj_name))
        .open(&mut open)
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -410.0))
        .default_size([720.0, 260.0])
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            let is_lib = proj_name.contains("lib") || state.estructura_tab == 2;
            let src_file = if is_lib { "src/lib.rs" } else { "src/main.rs" };
            let src_desc = if is_lib {
                "Archivo raíz de la librería. No lleva fn main(), sino funciones y structs con pub."
            } else {
                "Archivo fuente principal ejecutable con la función de entrada fn main() { ... }."
            };

            ui.horizontal_top(|ui| {
                // Columna Izquierda: Tabla Desglose Template
                let mut info_frame = egui::Frame::new();
                info_frame.fill = egui::Color32::from_rgb(18, 22, 32);
                info_frame.inner_margin = egui::Margin::same(12);
                info_frame.corner_radius = egui::CornerRadius::same(8);
                info_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 95));

                info_frame.show(ui, |ui| {
                    egui::Grid::new("desglose_template_grid_modal")
                        .striped(true)
                        .spacing([16.0, 8.0])
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Archivo / Carpeta").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                            ui.end_row();

                            ui.label(egui::RichText::new("Cargo.toml").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label("Manifiesto con metadatos de tu proyecto (nombre, versión, dependencias).");
                            ui.end_row();

                            ui.label(egui::RichText::new("Cargo.lock").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label("Registro de versiones fijadas de dependencias.");
                            ui.end_row();

                            ui.label(egui::RichText::new(src_file).monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label(src_desc);
                            ui.end_row();

                            ui.label(egui::RichText::new("target/").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label("Carpeta binaria donde rustc compila los ejecutables.");
                            ui.end_row();
                        });
                });

                ui.add_space(15.0);

                // Columna Derecha: Imagen 7.png fija inamovible
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/taller/7.png"))
                        .fit_to_exact_size(egui::vec2(280.0, 180.0))
                        .corner_radius(8),
                );
            });
        });

    if !open {
        state.created_project_name = None;
    }
}

fn mostrar_categoria_enteros(ui: &mut egui::Ui) {
    ui.label("En Rust, los enteros se dividen según si admiten números negativos (signed `i`) o solo positivos y cero (unsigned `u`).");
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_enteros")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Familia")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Bits")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Rango Mínimo .. Máximo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                let datos_enteros = [
                    ("i8", "Con signo", "8", "-128 .. 127", "let x: i8 = -50;"),
                    (
                        "i16",
                        "Con signo",
                        "16",
                        "-32,768 .. 32,767",
                        "let x: i16 = -1500;",
                    ),
                    (
                        "i32",
                        "Con signo",
                        "32",
                        "-2,147,483,648 .. 2,147,483,647",
                        "let x: i32 = -25000; (Por defecto)",
                    ),
                    (
                        "i64",
                        "Con signo",
                        "64",
                        "-9.22×10¹⁸ .. 9.22×10¹⁸",
                        "let x: i64 = -9_000_000_000;",
                    ),
                    (
                        "i128",
                        "Con signo",
                        "128",
                        "-1.70×10³⁸ .. 1.70×10³⁸",
                        "let x: i128 = -100_000_000;",
                    ),
                    (
                        "isize",
                        "Según arquitectura",
                        "32 u 64",
                        "Depende del procesador",
                        "let x: isize = -100;",
                    ),
                    ("u8", "Sin signo", "8", "0 .. 255", "let x: u8 = 255;"),
                    (
                        "u16",
                        "Sin signo",
                        "16",
                        "0 .. 65,535",
                        "let x: u16 = 65535;",
                    ),
                    (
                        "u32",
                        "Sin signo",
                        "32",
                        "0 .. 4,294,967,295",
                        "let x: u32 = 100_000;",
                    ),
                    (
                        "u64",
                        "Sin signo",
                        "64",
                        "0 .. 1.84×10¹⁹",
                        "let x: u64 = 5_000_000;",
                    ),
                    (
                        "u128",
                        "Sin signo",
                        "128",
                        "0 .. 3.40×10³⁸",
                        "let x: u128 = 100_000_000;",
                    ),
                    (
                        "usize",
                        "Según arquitectura",
                        "32 u 64",
                        "0 .. Max Memoria CPU",
                        "let x: usize = 10; (Por defecto arreglo)",
                    ),
                ];

                for (tipo, fam, bits, rango, ej_codigo) in datos_enteros {
                    ui.label(
                        egui::RichText::new(tipo)
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label(
                        egui::RichText::new(fam).color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(bits).color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(rango)
                            .monospace()
                            .color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(ej_codigo)
                            .monospace()
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                    ui.end_row();
                }
            });
    });
}

fn mostrar_categoria_flotantes(ui: &mut egui::Ui) {
    ui.label("Los tipos flotantes representan números con coma o fracción decimal en el estándar IEEE-754.");
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_flotantes")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Precisión")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Descripción")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("f32")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Precisión Simple (~6-9 dígitos)");
                ui.label("32 bits (4 bytes)");
                ui.label(
                    egui::RichText::new("let pi: f32 = 3.14159;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Ideal para gráficos 3D, física de juegos y ahorro de memoria.");
                ui.end_row();

                ui.label(
                    egui::RichText::new("f64")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Precisión Doble (~15-17 dígitos)");
                ui.label("64 bits (8 bytes)");
                ui.label(
                    egui::RichText::new("let pi: f64 = 3.141592653589793;")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Por defecto en Rust para decimales. Alta precisión científica.");
                ui.end_row();
            });
    });
}

fn mostrar_categoria_booleanos(ui: &mut egui::Ui) {
    ui.label("El tipo booleano representa una verdad lógica simple. En Rust solo existen dos valores posibles: true y false.");
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_booleanos")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Valores Posibles")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("bool")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("true  |  false")
                        .strong()
                        .color(egui::Color32::from_rgb(180, 190, 205)),
                );
                ui.label("1 byte");
                ui.label(
                    egui::RichText::new(
                        "let es_activo: bool = true;\nlet mut error: bool = false;",
                    )
                    .monospace()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();
            });
    });
}

fn mostrar_categoria_caracteres(ui: &mut egui::Ui) {
    ui.label("En Rust, un 'char' es un valor escalar Unicode de 4 bytes (32 bits), lo que significa que soporta mucho más que texto ASCII.");
    ui.add_space(10.0);

    let mut frame = egui::Frame::new();
    frame.fill = egui::Color32::from_rgb(14, 18, 26);
    frame.inner_margin = egui::Margin::same(12);
    frame.corner_radius = egui::CornerRadius::same(8);
    frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    frame.show(ui, |ui| {
        egui::Grid::new("grid_tipos_caracteres")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tipo")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Sintaxis")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Tamaño")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Características")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                ui.label(
                    egui::RichText::new("char")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label("Comillas simples ''");
                ui.label("4 bytes (32 bits)");
                ui.label(
                    egui::RichText::new(
                        "let letra: char = 'A';\nlet minuscula: char = 'z';\nlet simbolo: char = '@';\nlet letra_n: char = 'ñ';",
                    )
                    .monospace()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label("Soporta ASCII, acentos, Emojis y caracteres de todo el mundo.");
                ui.end_row();
            });
    });
}

fn mostrar_macro_println(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.heading("🧩 ¿Por qué `println!` termina con `!`?");
    ui.label("El signo `!` indica que estás invocando una macro. Una macro recibe tokens y genera código durante la compilación.");
    ui.add_space(8.0);
    ui.columns(2, |columns| {
        columns[0].group(|ui| {
            ui.label(egui::RichText::new("Lo que escribes").strong());
            ui.code("println!(\"Hola, Ferris!\");");
            ui.label("La macro valida el formato y construye los argumentos de impresión.");
        });
        columns[1].group(|ui| {
            ui.label(egui::RichText::new("Modelo mental").strong());
            ui.code("tokens → expansión → código compilable");
            ui.label("Las macros son más potentes que una simple sustitución de texto.");
        });
    });
    ui.add_space(10.0);
    if ui.button("🔬 Expandir println! con cargo expand").clicked() {
        state.show_macro_expansion = true;
    }

    let mut abierto = state.show_macro_expansion;
    egui::Window::new("Expansión didáctica de println!")
        .open(&mut abierto)
        .collapsible(false)
        .default_width(620.0)
        .show(ui.ctx(), |ui| {
            ui.label("Representación simplificada para entender la idea:");
            ui.code("std::io::_print(format_args!(\"Hola, Ferris!\\n\"));");
            ui.add_space(8.0);
            ui.label("La expansión exacta depende de la versión del compilador y puede usar detalles internos.");
            ui.separator();
            ui.code("cargo install cargo-expand");
            ui.code("cargo expand");
        });
    state.show_macro_expansion = abierto;
}

fn mostrar_comenzando(ui: &mut egui::Ui, state: &mut PortfolioState) {
    mostrar_tutorial_conceptos_basicos(ui, state);
}

fn mostrar_componente_terminal_3_modos(
    ui: &mut egui::Ui,
    _cmd_predeterminado: &str,
    state: &mut PortfolioState,
) {
    let mut term_frame = egui::Frame::new();
    term_frame.fill = egui::Color32::from_rgb(13, 17, 23);
    term_frame.inner_margin = egui::Margin::same(12);
    term_frame.corner_radius = egui::CornerRadius::same(8);
    term_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    term_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        // --- CABECERA DE LA TERMINAL LINUX ---
        ui.horizontal(|ui| {
            let history_len = state.term_history.lock().map(|h| h.len()).unwrap_or(0);
            let label = if state.show_terminal_history {
                format!("▼ Historial ({})", history_len)
            } else {
                format!("▶ Historial ({})", history_len)
            };

            if ui
                .button(
                    egui::RichText::new(label)
                        .strong()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                )
                .on_hover_text("Mostrar u ocultar el historial de comandos ejecutados")
                .clicked()
            {
                state.show_terminal_history = !state.show_terminal_history;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Botón Limpiar (Solo Ícono sin fondo)
                if ui
                    .add(egui::Button::new(egui::RichText::new("🗑️").size(13.0)).frame(false))
                    .on_hover_text("Limpiar terminal")
                    .clicked()
                    && let Ok(mut history) = state.term_history.lock()
                {
                    history.clear();
                }

                ui.add_space(6.0);

                // Botón Copiar Comando (Solo Ícono sin fondo)
                if ui
                    .add(egui::Button::new(egui::RichText::new("📋").size(13.0)).frame(false))
                    .on_hover_text("Copiar comando de la terminal")
                    .clicked()
                {
                    let text_to_copy = if !state.term_input.trim().is_empty() {
                        state.term_input.trim().to_string()
                    } else {
                        _cmd_predeterminado.to_string()
                    };
                    ui.ctx().output_mut(|o| {
                        o.commands.push(egui::OutputCommand::CopyText(text_to_copy))
                    });
                }
            });
        });

        ui.add_space(4.0);

        // Formato corto de CWD para el Prompt (ej: ~/VNC/repos/egui_vnc)
        let cwd_full = state.term_cwd.to_string_lossy();
        let short_cwd = if let Ok(home) = std::env::var("HOME") {
            if cwd_full.starts_with(&home) {
                cwd_full.replacen(&home, "~", 1)
            } else {
                cwd_full.to_string()
            }
        } else {
            cwd_full.to_string()
        };

        // --- HISTORIAL DE SALIDA DE COMANDOS DESPLEGABLE ---
        if state.show_terminal_history {
            if let Ok(history) = state.term_history.lock()
                && !history.is_empty()
            {
                egui::ScrollArea::vertical()
                    .max_height(160.0)
                    .id_salt("scroll_terminal_history")
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for (idx, line) in history.iter().enumerate() {
                            ui.push_id(idx, |ui| {
                                ui.label(
                                    egui::RichText::new(line)
                                        .monospace()
                                        .size(12.0)
                                        .color(egui::Color32::from_rgb(200, 230, 255)),
                                );
                            });
                        }
                    });
                ui.add_space(6.0);
            }
        }

        // Línea de entrada limpia sin marcos ni cajas negras flotantes
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("alek@debian:{}$", short_cwd))
                    .strong()
                    .color(egui::Color32::from_rgb(120, 255, 120))
                    .monospace()
                    .size(13.0),
            );

            let input_response = ui.add(
                egui::TextEdit::singleline(&mut state.term_input)
                    .frame(egui::Frame::NONE)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY),
            );

            // Mantener el foco automático en la terminal continuamente
            input_response.request_focus();

            // Ejecutar comando al presionar Enter
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let cmd_str = state.term_input.trim().to_string();
                if !cmd_str.is_empty() {
                    if let Ok(mut history) = state.term_history.lock() {
                        history.push(format!("alek@debian:{}$ {}", short_cwd, cmd_str));
                    }
                    state.term_input.clear();

                    // Detectar si el usuario ejecuta 'cargo new <nombre_proyecto>'
                    if cmd_str.starts_with("cargo new ") || cmd_str.starts_with("cargo  new ") {
                        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
                        if parts.len() >= 3 {
                            let is_lib = parts.contains(&"--lib");
                            for part in parts.iter().skip(2) {
                                if !part.starts_with('-') {
                                    let proj_name = (*part).to_string();
                                    state.created_project_name = Some(proj_name.clone());
                                    state.selected_project = Some(proj_name);
                                    if is_lib {
                                        state.estructura_tab = 2;
                                    } else {
                                        state.estructura_tab = 1;
                                    }
                                    break;
                                }
                            }
                        }
                    }

                    // Manejo especial de comando 'cd' para persistir el directorio de navegación
                    if cmd_str == "cd" || cmd_str.starts_with("cd ") {
                        let target_arg = if cmd_str == "cd" {
                            ""
                        } else {
                            cmd_str[3..].trim()
                        };
                        let new_path = if target_arg.is_empty() || target_arg == "~" {
                            std::env::var("HOME")
                                .map(std::path::PathBuf::from)
                                .unwrap_or_else(|_| state.term_cwd.clone())
                        } else if target_arg == ".." {
                            state
                                .term_cwd
                                .parent()
                                .map(|p| p.to_path_buf())
                                .unwrap_or_else(|| state.term_cwd.clone())
                        } else {
                            let candidate = state.term_cwd.join(target_arg);
                            candidate.canonicalize().unwrap_or(candidate)
                        };

                        if new_path.is_dir() {
                            state.term_cwd = new_path;
                        } else {
                            if let Ok(mut history) = state.term_history.lock() {
                                history.push(format!(
                                    "sh: cd: {}: No existe el directorio",
                                    target_arg
                                ));
                            }
                        }
                    } else {
                        // Ejecución en hilo secundario asíncrono (evita congelar el GUI)
                        let history_arc = Arc::clone(&state.term_history);
                        let output_arc = Arc::clone(&state.conceptos_output);
                        let modal_arc = Arc::clone(&state.show_cargo_output_modal);
                        let cwd = state.term_cwd.clone();
                        let cmd = cmd_str.clone();
                        let ctx = ui.ctx().clone();

                        if cmd.starts_with("cargo ") {
                            if let Ok(mut out) = state.conceptos_output.lock() {
                                *out = "Compilando con Cargo...".to_string();
                            }
                        }

                        std::thread::spawn(move || {
                            let output = std::process::Command::new("sh")
                                .arg("-c")
                                .arg(&cmd)
                                .current_dir(&cwd)
                                .output();

                            match output {
                                Ok(out) => {
                                    let stdout = String::from_utf8_lossy(&out.stdout);
                                    let stderr = String::from_utf8_lossy(&out.stderr);
                                    if let Ok(mut history) = history_arc.lock() {
                                        if !stdout.is_empty() {
                                            for line in stdout.lines() {
                                                history.push(line.to_string());
                                            }
                                        }
                                        if !stderr.is_empty() {
                                            for line in stderr.lines() {
                                                history.push(line.to_string());
                                            }
                                        }
                                    }

                                    // Sincronizar el cuadro de salida dedicado para comandos cargo
                                    if cmd.starts_with("cargo ") {
                                        let mut combined = stdout.into_owned();
                                        if !stderr.is_empty() {
                                            if !combined.is_empty() {
                                                combined.push_str("\n\n");
                                            }
                                            combined.push_str("[Errores/Warnings]:\n");
                                            combined.push_str(&stderr);
                                            if cmd.starts_with("cargo expand")
                                                && (stderr.contains("no such command")
                                                    || stderr.contains("not found"))
                                            {
                                                combined.push_str(
                                                    "\n\n💡 Nota: 'cargo expand' requiere la herramienta externa. Puedes instalarla ejecutando:\ncargo install cargo-expand",
                                                );
                                            }
                                        }
                                        if combined.is_empty() {
                                            combined = "El comando terminó sin salidas.".to_string();
                                        }
                                        if let Ok(mut out_lock) = output_arc.lock() {
                                            *out_lock = combined;
                                        }
                                        // Abrir la ventana modal flotante recién al terminar la compilación
                                        modal_arc.store(true, Ordering::Relaxed);
                                    }
                                }
                                Err(err) => {
                                    if let Ok(mut history) = history_arc.lock() {
                                        history.push(format!("Error ejecutando comando: {}", err));
                                    }
                                }
                            }
                            ctx.request_repaint();
                        });
                    }
                }
                // Mantener foco en el campo de texto tras presionar Enter
                input_response.request_focus();
            }
        });

        /*
        // --- MODOS RESERVADOS PARA EL FUTURO (MODO 0: ESTÁTICA, MODO 2: PTY REAL) ---
        // Si en el futuro necesitas habilitar el modo PTY nativo o estático:
        //
        // MODO 0 (Estática):
        // ui.label(egui::RichText::new(cmd_predeterminado).strong().color(egui::Color32::WHITE).monospace());
        //
        // MODO 2 (PTY Real Linux con portable-pty):
        // std::thread::spawn(move || { ... portable_pty::NativePtySystem ... });
         */
    });
}

fn obtener_repos_base_dir(term_cwd: &std::path::Path) -> std::path::PathBuf {
    let default_repos = std::path::Path::new("/home/alek/VNC/repos");
    if default_repos.exists() && default_repos.is_dir() {
        default_repos.to_path_buf()
    } else if term_cwd.exists() && term_cwd.is_dir() {
        if term_cwd.file_name().is_some_and(|n| n == "egui_vnc") {
            term_cwd.parent().unwrap_or(term_cwd).to_path_buf()
        } else {
            term_cwd.to_path_buf()
        }
    } else {
        std::path::PathBuf::from("/home/alek/VNC/repos")
    }
}

fn buscar_ruta_proyecto(base_path: &std::path::Path, proj_name: &str) -> std::path::PathBuf {
    let candidate1 = base_path.join(proj_name);
    if candidate1.exists() {
        return candidate1;
    }
    let repos_dir = obtener_repos_base_dir(base_path);
    let candidate2 = repos_dir.join(proj_name);
    if candidate2.exists() {
        return candidate2;
    }

    if let Ok(entries) = std::fs::read_dir(&repos_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == proj_name) {
                    return path;
                }
                let nested = path.join(proj_name);
                if nested.exists() {
                    return nested;
                }
            }
        }
    }
    base_path.join(proj_name)
}

fn listar_proyectos_cargo(base_path: &std::path::Path) -> Vec<String> {
    let mut proyectos = Vec::new();
    let mut dirs_to_scan = Vec::new();

    let repos_dir = obtener_repos_base_dir(base_path);
    dirs_to_scan.push(repos_dir.clone());

    if base_path.exists() && base_path.is_dir() {
        if !dirs_to_scan.contains(&base_path.to_path_buf()) {
            dirs_to_scan.push(base_path.to_path_buf());
        }
        if let Some(parent) = base_path.parent() {
            let p_buf = parent.to_path_buf();
            if !dirs_to_scan.contains(&p_buf) {
                dirs_to_scan.push(p_buf);
            }
        }
    }

    for dir_to_scan in dirs_to_scan {
        if let Ok(entries) = std::fs::read_dir(&dir_to_scan) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let es_valido = path.join("Cargo.toml").exists()
                        || path.join("src/main.rs").exists()
                        || path.join("src/lib.rs").exists();

                    if es_valido {
                        if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                            if !proyectos.contains(&folder_name.to_string()) {
                                proyectos.push(folder_name.to_string());
                            }
                        }
                    } else if let Ok(sub_entries) = std::fs::read_dir(&path) {
                        for sub_entry in sub_entries.flatten() {
                            let sub_path = sub_entry.path();
                            if sub_path.is_dir() {
                                let sub_valido = sub_path.join("Cargo.toml").exists()
                                    || sub_path.join("src/main.rs").exists()
                                    || sub_path.join("src/lib.rs").exists();
                                if sub_valido {
                                    if let Some(sub_folder) =
                                        sub_path.file_name().and_then(|n| n.to_str())
                                    {
                                        if !proyectos.contains(&sub_folder.to_string()) {
                                            proyectos.push(sub_folder.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    proyectos.sort();
    proyectos
}

fn mostrar_selector_proyectos_estandar(
    ui: &mut egui::Ui,
    selected_project: &mut Option<String>,
    term_cwd: &mut std::path::PathBuf,
    combo_id: &str,
    code_target: &mut String,
) {
    let proyectos_disponibles = listar_proyectos_cargo(term_cwd);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Selecciona un proyecto:")
                .strong()
                .color(egui::Color32::WHITE),
        );

        let label_seleccionado = match selected_project {
            Some(p) => p.as_str(),
            None => " Selecciona un proyecto ",
        };

        egui::ComboBox::from_id_salt(combo_id)
            .selected_text(
                egui::RichText::new(label_seleccionado)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            )
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(selected_project.is_none(), " Ninguno ")
                    .clicked()
                {
                    *selected_project = None;
                }
                ui.separator();
                for proj in &proyectos_disponibles {
                    let es_sel = selected_project.as_ref() == Some(proj);
                    if ui.selectable_label(es_sel, proj).clicked() {
                        *selected_project = Some(proj.clone());
                        let proj_dir = buscar_ruta_proyecto(term_cwd, proj);
                        *term_cwd = proj_dir.clone();

                        let main_rs = proj_dir.join("src/main.rs");
                        let lib_rs = proj_dir.join("src/lib.rs");
                        let file_to_read = if main_rs.exists() { main_rs } else { lib_rs };
                        if let Ok(content) = std::fs::read_to_string(file_to_read) {
                            *code_target = content;
                        }
                    }
                }
            });
    });
}

fn ejecutar_cargo_run_proyecto(state: &mut PortfolioState, ctx: &egui::Context) {
    let proj_dir = if let Some(ref proj) = state.selected_project {
        buscar_ruta_proyecto(&state.term_cwd, proj)
    } else {
        obtener_repos_base_dir(&state.term_cwd)
    };

    let main_rs = proj_dir.join("src/main.rs");
    let lib_rs = proj_dir.join("src/lib.rs");
    let target_file = if main_rs.exists() {
        main_rs
    } else if lib_rs.exists() {
        lib_rs
    } else {
        main_rs.clone()
    };

    // Obtener el código y el búfer de salida de la lección activa
    let codigo_activo = match state.ruta_actual {
        AppRoute::TutorialControlFlujo => state.controlflujo_code.clone(),
        AppRoute::TutorialTiposDatos => state.datatypes_code.clone(),
        AppRoute::TutorialStrings => state.strings_code.clone(),
        AppRoute::Playground => state.playground_code.clone(),
        _ => state.conceptos_code.clone(),
    };

    let output_arc = match state.ruta_actual {
        AppRoute::TutorialControlFlujo => Arc::clone(&state.controlflujo_output),
        AppRoute::TutorialTiposDatos => Arc::clone(&state.datatypes_output),
        AppRoute::TutorialStrings => Arc::clone(&state.strings_output),
        AppRoute::Playground => Arc::clone(&state.playground_output),
        _ => Arc::clone(&state.conceptos_output),
    };

    if state.selected_project.is_some() && target_file.parent().is_some_and(|p| p.exists()) {
        let _ = std::fs::write(&target_file, &codigo_activo);
    }

    if let Ok(mut out) = output_arc.lock() {
        *out = "Compilando y ejecutando con Cargo (cargo run)...".to_string();
    }

    // Abrir inmediatamente la ventana modal flotante centrada en la pantalla
    state.show_cargo_output_modal.store(true, Ordering::Relaxed);

    let history_arc = Arc::clone(&state.term_history);
    let ctx_clone = ctx.clone();
    let is_proj = state.selected_project.is_some();

    std::thread::spawn(move || {
        let output = if is_proj && proj_dir.exists() {
            std::process::Command::new("cargo")
                .arg("run")
                .current_dir(&proj_dir)
                .output()
        } else {
            std::process::Command::new("cargo")
                .arg("run")
                .current_dir(obtener_repos_base_dir(&proj_dir))
                .output()
        };

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let mut combined = stdout.into_owned();
                if !stderr.is_empty() {
                    if !combined.is_empty() {
                        combined.push_str("\n\n");
                    }
                    combined.push_str("[Compilador / Warnings / Errores]:\n");
                    combined.push_str(&stderr);
                }
                if combined.is_empty() {
                    combined = "El programa terminó exitosamente sin salidas.".to_string();
                }
                if let Ok(mut out_lock) = output_arc.lock() {
                    *out_lock = combined;
                }
                if let Ok(mut history) = history_arc.lock() {
                    history.push(format!("$ cargo run (en {})", proj_dir.display()));
                }
            }
            Err(err) => {
                if let Ok(mut out_lock) = output_arc.lock() {
                    *out_lock = format!("Error al ejecutar cargo run: {}", err);
                }
            }
        }
        ctx_clone.request_repaint();
    });
}

fn mostrar_contenido_tipos_primitivos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    // Selector de Categoría (Enteros, Decimales, Bool, Char)
    ui.horizontal(|ui| {
        for (cat_idx, (cat_label, cat_color)) in [
            ("Enteros", egui::Color32::from_rgb(255, 160, 50)),
            ("Decimales", egui::Color32::from_rgb(255, 160, 50)),
            ("Booleanos", egui::Color32::from_rgb(255, 160, 50)),
            ("Caracteres", egui::Color32::from_rgb(255, 160, 50)),
        ]
        .iter()
        .enumerate()
        {
            let es_sel = state.tipo_primitivo_categoria == cat_idx;
            let text_rich = egui::RichText::new(*cat_label).strong().color(if es_sel {
                *cat_color
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            });
            if ui.add(egui::Button::new(text_rich).frame(es_sel)).clicked() {
                state.tipo_primitivo_categoria = cat_idx;
            }
            ui.add_space(4.0);
        }
    });

    ui.add_space(12.0);

    match state.tipo_primitivo_categoria {
        0 => mostrar_categoria_enteros(ui),
        1 => mostrar_categoria_flotantes(ui),
        2 => mostrar_categoria_booleanos(ui),
        _ => mostrar_categoria_caracteres(ui),
    }
}

fn centrar_texto_en_rectangulos(raw_svg: &str) -> String {
    let clean_svg = raw_svg.replace(">\n", ">").replace(">\r\n", ">");

    let mut output = String::with_capacity(clean_svg.len());
    let mut search_idx = 0;

    while let Some(g_start) = clean_svg[search_idx..].find("<g class=") {
        let abs_g_start = search_idx + g_start;
        let g_substr = &clean_svg[abs_g_start..];

        if g_substr.starts_with("<g class=\"terminal\"")
            || g_substr.starts_with("<g class=\"nonterminal\"")
        {
            if let Some(g_end) = g_substr.find("</g>") {
                let abs_g_end = abs_g_start + g_end + 4;
                let group_block = &clean_svg[abs_g_start..abs_g_end];

                let mut rx = 0.0f32;
                let mut ry = 0.0f32;
                let mut rw = 0.0f32;
                let mut rh = 0.0f32;

                if let Some(r_pos) = group_block.find("<rect ") {
                    let r_sub = &group_block[r_pos..];
                    if let Some(r_close) = r_sub.find('>') {
                        let r_tag = &r_sub[..r_close];
                        for attr in r_tag.split_whitespace() {
                            if let Some((k, v)) = attr.split_once('=') {
                                let val = v
                                    .trim_matches('"')
                                    .trim_matches('\'')
                                    .trim_matches('>')
                                    .trim_matches('/');
                                match k {
                                    "x" => rx = val.parse().unwrap_or(0.0),
                                    "y" => ry = val.parse().unwrap_or(0.0),
                                    "width" => rw = val.parse().unwrap_or(0.0),
                                    "height" => rh = val.parse().unwrap_or(0.0),
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                if rw > 0.0 && rh > 0.0 {
                    if let (Some(t_start), Some(t_end)) =
                        (group_block.find("<text "), group_block.find("</text>"))
                    {
                        let text_sub = &group_block[t_start..t_end + 7];
                        if let Some(tag_close) = text_sub.find('>') {
                            let content = text_sub[tag_close + 1..text_sub.len() - 7].trim();

                            let cx = rx + rw / 2.0;
                            let cy = ry + rh / 2.0;

                            let is_terminal = group_block.contains("class=\"terminal\"");
                            let fill_color = if is_terminal { "#ffb347" } else { "#64c8ff" };

                            let new_group = format!(
                                "{}\n<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-family=\"sans-serif\" font-size=\"12\" font-weight=\"normal\" text-anchor=\"middle\" dominant-baseline=\"central\" stroke=\"none\">{}</text>\n</g>",
                                &group_block[..t_start].trim_end(),
                                cx,
                                cy,
                                fill_color,
                                content
                            );
                            output.push_str(&clean_svg[search_idx..abs_g_start]);
                            output.push_str(&new_group);
                            search_idx = abs_g_end;
                            continue;
                        }
                    }
                }
            }
        }

        output.push_str(&clean_svg[search_idx..abs_g_start + 8]);
        search_idx = abs_g_start + 8;
    }

    output.push_str(&clean_svg[search_idx..]);
    output
}

fn generar_railroad_color_image() -> Option<egui::ColorImage> {
    use railroad::*;

    let mut seq = Sequence::default();
    let e1: Box<dyn railroad::Node> = Box::new(Terminal::new("let".to_string()));
    let e2: Box<dyn railroad::Node> = Box::new(Optional::new(Terminal::new("mut".to_string())));
    let e3: Box<dyn railroad::Node> = Box::new(NonTerminal::new("identificador".to_string()));
    let e4_sub1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
    let e4_sub2: Box<dyn railroad::Node> = Box::new(NonTerminal::new("tipo".to_string()));
    let e4: Box<dyn railroad::Node> =
        Box::new(Optional::new(Sequence::new(vec![e4_sub1, e4_sub2])));
    let e5: Box<dyn railroad::Node> = Box::new(Terminal::new("=".to_string()));
    let e6: Box<dyn railroad::Node> = Box::new(NonTerminal::new("expresion".to_string()));
    let e7: Box<dyn railroad::Node> = Box::new(Terminal::new(";".to_string()));

    seq.push(e1);
    seq.push(e2);
    seq.push(e3);
    seq.push(e4);
    seq.push(e5);
    seq.push(e6);
    seq.push(e7);

    let dia = Diagram::new(seq);
    let mut raw_svg = dia.to_string();

    if !raw_svg.contains("width=") {
        raw_svg = raw_svg.replace(
            "<svg ",
            "<svg width=\"626\" height=\"60\" xmlns=\"http://www.w3.org/2000/svg\" ",
        );
    } else if !raw_svg.contains("xmlns=") {
        raw_svg = raw_svg.replace("<svg ", "<svg xmlns=\"http://www.w3.org/2000/svg\" ");
    }

    raw_svg = raw_svg
        .replace(
            "<g class=\"terminal\">",
            "<g class=\"terminal\" fill=\"#1e2638\" stroke=\"#ff9d00\" stroke-width=\"2\">",
        )
        .replace(
            "<g class=\"nonterminal\">",
            "<g class=\"nonterminal\" fill=\"#1a2336\" stroke=\"#64c8ff\" stroke-width=\"2\">",
        )
        .replace(
            "<path ",
            "<path stroke=\"#64c8ff\" stroke-width=\"2.5\" fill=\"none\" ",
        );

    raw_svg = centrar_texto_en_rectangulos(&raw_svg);

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    let opt = usvg::Options {
        fontdb: fontdb.into(),
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_str(&raw_svg, &opt).ok()?;
    let width = tree.size().width().ceil() as u32;
    let height = tree.size().height().ceil() as u32;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width.max(1), height.max(1))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    let pixels = pixmap.data();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        pixels,
    ))
}

fn mostrar_contenido_macros(ui: &mut egui::Ui) {
    ui.heading(
        egui::RichText::new("Categorías de Macros en Rust")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(6.0);
    ui.label(
        "En Rust, las macros son herramientas de metaprogramación que generan código durante la compilación. Se distinguen fácilmente por llevar un signo de exclamación ! al final.",
    );
    ui.add_space(10.0);

    ui.columns(2, |cols| {
        let mut card1 = egui::Frame::new();
        card1.fill = egui::Color32::from_rgb(18, 22, 32);
        card1.inner_margin = egui::Margin::same(12);
        card1.corner_radius = egui::CornerRadius::same(8);
        card1.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 95));

        card1.show(&mut cols[0], |ui| {
            ui.heading(
                egui::RichText::new("1. Macros Declarativas (macro_rules!)")
                    .size(15.0)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.add_space(4.0);
            ui.label("Basadas en coincidencia de patrones (pattern matching) similares a una sentencia match. Permiten escribir código conciso como vec![] o println!.");
        });

        card1.show(&mut cols[1], |ui| {
            ui.heading(
                egui::RichText::new("2. Macros Procedurales (Derive, Atributos)")
                    .size(15.0)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 160, 50)),
            );
            ui.add_space(4.0);
            ui.label("Operan sobre el Árbol de Sintaxis Abstracta (AST) de Rust como código ejecutable durante la compilación (ej: #[derive(Debug, Serialize)]).");
        });
    });

    ui.add_space(16.0);

    let mut table_frame = egui::Frame::new();
    table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    table_frame.inner_margin = egui::Margin::same(12);
    table_frame.corner_radius = egui::CornerRadius::same(8);
    table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    table_frame.show(ui, |ui| {
        egui::Grid::new("tabla_macros_estandar_rust")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Macro")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Propósito Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Salida / Comportamiento")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                let macros_list = [
                    (
                        "println!",
                        "Impresión a Consola (stdout)",
                        "println!(\"Hola {name}\");",
                        "Escribe en consola con salto de línea al final (\\n).",
                    ),
                    (
                        "print!",
                        "Impresión Continua (stdout)",
                        "print!(\"Cargando...\");",
                        "Escribe en consola sin agregar salto de línea.",
                    ),
                    (
                        "format!",
                        "Creación de String Formateado",
                        "let s = format!(\"x = {}\", 10);",
                        "Devuelve un String dinámico sin imprimir en consola.",
                    ),
                    (
                        "eprintln!",
                        "Impresión de Errores (stderr)",
                        "eprintln!(\"Error: {}\", err);",
                        "Escribe en la salida estándar de errores stderr.",
                    ),
                    (
                        "dbg!",
                        "Macro de Depuración Nativa",
                        "let y = dbg!(x * 2);",
                        "Imprime archivo, línea, expresión y devuelve el valor.",
                    ),
                    (
                        "vec!",
                        "Creación de Vectores Dinámicos",
                        "let v = vec![1, 2, 3];",
                        "Sintaxis conveniente para inicializar un Vec<T>.",
                    ),
                    (
                        "panic!",
                        "Interrupción de Emergencia",
                        "panic!(\"Fallo crítico\");",
                        "Detiene la ejecución del hilo enviando un mensaje de pánico.",
                    ),
                    (
                        "assert_eq!",
                        "Verificación de Pruebas",
                        "assert_eq!(a, b);",
                        "Valida igualdad en tests; entra en pánico si son distintos.",
                    ),
                ];

                for (m_name, m_prop, m_code, m_desc) in macros_list {
                    ui.label(
                        egui::RichText::new(m_name)
                            .monospace()
                            .strong()
                            .color(egui::Color32::from_rgb(255, 160, 50)),
                    );
                    ui.label(
                        egui::RichText::new(m_prop).color(egui::Color32::from_rgb(180, 190, 205)),
                    );
                    ui.label(
                        egui::RichText::new(m_code)
                            .monospace()
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                    ui.label(m_desc);
                    ui.end_row();
                }
            });
    });

    ui.add_space(18.0);
    ui.heading(
        egui::RichText::new("Depuración")
            .size(18.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(6.0);
    ui.label(
        "Rust ofrece potentes mecanismos de formateo e inspección de variables. Conocer la diferencia entre Display {}, Debug {:?}, Pretty Debug {:#?} y dbg! es clave para el desarrollo diario.",
    );
    ui.add_space(10.0);

    let mut fmt_frame = egui::Frame::new();
    fmt_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    fmt_frame.inner_margin = egui::Margin::same(12);
    fmt_frame.corner_radius = egui::CornerRadius::same(8);
    fmt_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    fmt_frame.show(ui, |ui| {
        egui::Grid::new("tabla_debug_formato_rust")
            .striped(true)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Formato / Herramienta")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Especificador")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Uso Principal")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Ejemplo de Código")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.end_row();

                // Display {}
                ui.label(
                    egui::RichText::new("Display")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Formato amigable para el usuario final");
                ui.label(
                    egui::RichText::new("println!(\"Score: {}\", puntos);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Debug {:?}
                ui.label(
                    egui::RichText::new("Debug")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{:?}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Inspección técnica de 1 sola línea");
                ui.label(
                    egui::RichText::new("println!(\"{:?}\", arreglo);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Pretty Debug {:#?}
                ui.label(
                    egui::RichText::new("Pretty Debug")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("{:#?}")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Inspección multilínea indentada (estructuras compuestas)");
                ui.label(
                    egui::RichText::new("println!(\"{:#?}\", persona);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();

                // Macro dbg!
                ui.label(
                    egui::RichText::new("Macro dbg!")
                        .monospace()
                        .strong()
                        .color(egui::Color32::from_rgb(255, 160, 50)),
                );
                ui.label(
                    egui::RichText::new("dbg!(exp)")
                        .monospace()
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Imprime archivo, nº línea, expresión y devuelve el valor");
                ui.label(
                    egui::RichText::new("let b = dbg!(a + 5);")
                        .monospace()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    let mut tip_frame = egui::Frame::new();
    tip_frame.fill = egui::Color32::from_rgb(20, 28, 42);
    tip_frame.inner_margin = egui::Margin::same(14);
    tip_frame.corner_radius = egui::CornerRadius::same(8);
    tip_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 90, 140));

    tip_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new("A diferencia de println!, dbg! toma la propiedad de la expresión, imprime su ubicación exacta en el código fuente, el resultado de la expresión, y devuelve el valor evaluado. ¡Eso te permite envolver llamadas intermedias sin romper tu código!")
                .color(egui::Color32::from_rgb(200, 230, 255)),
        );
    });

    ui.add_space(16.0);
    ui.heading(
        egui::RichText::new("🛤️ Generador de Diagramas de Sintaxis SVG (railroad)")
            .size(16.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(6.0);
    ui.label("La crate 'railroad' genera dinámicamente gráficos de sintaxis tipo ferrocarril (Railroad Diagram) en formato SVG a partir de reglas sintácticas:");
    ui.add_space(10.0);

    let mut railroad_frame = egui::Frame::new();
    railroad_frame.fill = egui::Color32::from_rgb(14, 18, 26);
    railroad_frame.inner_margin = egui::Margin::same(12);
    railroad_frame.corner_radius = egui::CornerRadius::same(8);
    railroad_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

    railroad_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new(
                "Diagrama de Sintaxis Generado para 'let [mut] ident [: tipo] = expr;':",
            )
            .strong()
            .color(egui::Color32::from_rgb(100, 200, 255)),
        );
        ui.add_space(8.0);

        ui.add(
            egui::Image::from_bytes(
                "bytes://diagrama_let.svg",
                include_bytes!("../diagramas/diagrama_let.svg"),
            )
            .fit_to_exact_size(egui::vec2(650.0, 80.0))
            .corner_radius(egui::CornerRadius::same(6)),
        );

        ui.add_space(8.0);
        if ui
            .button(
                egui::RichText::new("🌐 Abrir Diagrama SVG NATIVO en Navegador Web")
                    .strong()
                    .color(egui::Color32::from_rgb(100, 220, 255)),
            )
            .clicked()
        {
            let _ = std::process::Command::new("xdg-open")
                .arg("/home/alek/VNC/repos/egui_vnc/diagramas/ver_diagrama.html")
                .spawn();
        }
    });
}

fn generar_railroad_desde_codigo(codigo: &str) -> Option<egui::ColorImage> {
    use railroad::*;

    let mut mut_token: Option<String> = None;
    let mut ident_token = "variable".to_string();
    let mut tipo_token: Option<String> = None;
    let mut expr_token = "expresion".to_string();
    let mut tiene_punto_y_coma = false;

    let mut encontrado = false;
    for orig_line in codigo.lines() {
        let line_without_comment = match orig_line.split_once("//") {
            Some((code, _)) => code,
            None => orig_line,
        };
        let trimmed = line_without_comment.trim();
        if trimmed.starts_with("let ") || trimmed.starts_with("let\t") || trimmed == "let" {
            encontrado = true;
            let mut rest = if trimmed.starts_with("let ") || trimmed.starts_with("let\t") {
                trimmed["let".len()..].trim()
            } else {
                ""
            };

            if rest.contains(';') {
                tiene_punto_y_coma = true;
                rest = rest.trim_end_matches(';').trim();
            }

            let (pat_part, expr_part) = match rest.split_once('=') {
                Some((p, e)) => (p.trim(), e.trim()),
                None => (rest, ""),
            };

            if !expr_part.is_empty() {
                expr_token = expr_part.to_string();
            }

            let mut pat_str = pat_part;
            if pat_str.starts_with("mut ") || pat_str.starts_with("mut\t") || pat_str == "mut" {
                mut_token = Some("mut".to_string());
                if pat_str.len() > 3 {
                    pat_str = pat_str[3..].trim();
                } else {
                    pat_str = "";
                }
            }

            if let Some((ident, ty)) = pat_str.split_once(':') {
                if !ident.trim().is_empty() {
                    ident_token = ident.trim().to_string();
                }
                if !ty.trim().is_empty() {
                    tipo_token = Some(ty.trim().to_string());
                }
            } else if !pat_str.trim().is_empty() {
                ident_token = pat_str.trim().to_string();
            }
            break;
        }
    }

    if !encontrado {
        return None;
    }

    let mut seq = Sequence::default();
    let e1: Box<dyn railroad::Node> = Box::new(Terminal::new("let".to_string()));
    let e2: Box<dyn railroad::Node> = match mut_token {
        Some(m) => Box::new(Terminal::new(m)),
        None => Box::new(Optional::new(Terminal::new("mut".to_string()))),
    };
    let e3: Box<dyn railroad::Node> = Box::new(NonTerminal::new(ident_token));

    let e4: Box<dyn railroad::Node> = match tipo_token {
        Some(ty) => {
            let n1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
            let n2: Box<dyn railroad::Node> = Box::new(NonTerminal::new(ty));
            Box::new(Sequence::new(vec![n1, n2]))
        }
        None => {
            let e4_sub1: Box<dyn railroad::Node> = Box::new(Terminal::new(":".to_string()));
            let e4_sub2: Box<dyn railroad::Node> = Box::new(NonTerminal::new("tipo".to_string()));
            Box::new(Optional::new(Sequence::new(vec![e4_sub1, e4_sub2])))
        }
    };

    let e5: Box<dyn railroad::Node> = Box::new(Terminal::new("=".to_string()));
    let e6: Box<dyn railroad::Node> = Box::new(NonTerminal::new(expr_token));
    let e7: Box<dyn railroad::Node> = if tiene_punto_y_coma {
        Box::new(Terminal::new(";".to_string()))
    } else {
        Box::new(Optional::new(Terminal::new(";".to_string())))
    };

    seq.push(e1);
    seq.push(e2);
    seq.push(e3);
    seq.push(e4);
    seq.push(e5);
    seq.push(e6);
    seq.push(e7);

    let dia = Diagram::new(seq);
    let mut raw_svg = dia.to_string();

    if !raw_svg.contains("width=") {
        raw_svg = raw_svg.replace(
            "<svg ",
            "<svg width=\"650\" height=\"60\" xmlns=\"http://www.w3.org/2000/svg\" ",
        );
    } else if !raw_svg.contains("xmlns=") {
        raw_svg = raw_svg.replace("<svg ", "<svg xmlns=\"http://www.w3.org/2000/svg\" ");
    }

    raw_svg = raw_svg
        .replace(
            "<g class=\"terminal\">",
            "<g class=\"terminal\" fill=\"#1e2638\" stroke=\"#ff9d00\" stroke-width=\"2\">",
        )
        .replace(
            "<g class=\"nonterminal\">",
            "<g class=\"nonterminal\" fill=\"#1a2336\" stroke=\"#64c8ff\" stroke-width=\"2\">",
        )
        .replace(
            "<path ",
            "<path stroke=\"#64c8ff\" stroke-width=\"2.5\" fill=\"none\" ",
        );

    raw_svg = centrar_texto_en_rectangulos(&raw_svg);

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    let opt = usvg::Options {
        fontdb: fontdb.into(),
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_str(&raw_svg, &opt).ok()?;
    let width = tree.size().width().ceil() as u32;
    let height = tree.size().height().ceil() as u32;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width.max(1), height.max(1))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    let pixels = pixmap.data();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        pixels,
    ))
}

fn mostrar_tutorial_conceptos_basicos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Comenzando con Rust")
                .size(28.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 160, 50)),
        );
    });

    ui.add_space(15.0);

    // Barra de navegación con el mismo patrón unificado que Pilares
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );

        let tabs_practica = [(0, "Inmutabilidad"), (1, "const y static")];
        for (indice, texto) in tabs_practica {
            let es_activo = state.conceptos_tab == indice;
            let text_color = if es_activo {
                egui::Color32::from_rgb(255, 160, 50)
            } else {
                egui::Color32::from_rgb(180, 190, 205)
            };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(texto).strong().color(text_color))
                        .frame(es_activo),
                )
                .clicked()
            {
                state.conceptos_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let tabs_teoria = [(3, "Macro & Format"), (2, "Data Types")];
            for (indice, texto) in tabs_teoria {
                let es_activo = state.conceptos_tab == indice;
                let text_color = if es_activo {
                    egui::Color32::from_rgb(255, 160, 50)
                } else {
                    egui::Color32::from_rgb(180, 190, 205)
                };
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(texto).strong().color(text_color))
                            .frame(es_activo),
                    )
                    .clicked()
                {
                    state.conceptos_tab = indice;
                }
                ui.add_space(4.0);
            }
            ui.label(
                egui::RichText::new("Teórico:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.conceptos_tab {
        0 => {
            ui.label(
                "En Rust, las variables son inmutables por defecto. Esto garantiza seguridad de memoria, previene condiciones de carrera en concurrencia (data races) y obliga a declarar explícitamente con 'mut' cuando un valor necesita cambiar.",
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let mut table_frame = egui::Frame::new();
                table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
                table_frame.inner_margin = egui::Margin::same(12);
                table_frame.corner_radius = egui::CornerRadius::same(8);
                table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

                table_frame.show(ui, |ui| {
                    egui::Grid::new("tabla_let_mut")
                        .striped(true)
                        .spacing([25.0, 8.0])
                        .show(ui, |ui| {
                            // Encabezados
                            ui.label(egui::RichText::new("Declaración").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Mutabilidad").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Ejemplo de Código").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new("Diagrama").strong().color(egui::Color32::WHITE));
                            ui.end_row();

                            // Fila 1: let (Inmutable)
                            let btn_color_0 = if state.show_railroad_modal == Some(0) {
                                egui::Color32::from_rgb(255, 160, 50)
                            } else {
                                egui::Color32::from_rgb(180, 190, 205)
                            };

                            ui.label(egui::RichText::new("let").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label(egui::RichText::new("No").strong().color(egui::Color32::from_rgb(180, 190, 205)));
                            ui.label(egui::RichText::new("let x = 5;").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label("No puede reasignarse dentro de su alcance salvo que se declare con mut.");
                            if ui
                                .add(
                                    egui::Button::image(
                                        egui::Image::from_bytes(
                                            "bytes://view.svg",
                                            include_bytes!("../diagramas/view.svg"),
                                        )
                                        .fit_to_exact_size(egui::vec2(20.0, 20.0))
                                        .tint(btn_color_0),
                                    )
                                    .frame(state.show_railroad_modal == Some(0)),
                                )
                                .on_hover_text("Ver diagrama Railroad de sintaxis (let inmutable)")
                                .clicked()
                            {
                                state.show_railroad_modal = if state.show_railroad_modal == Some(0) { None } else { Some(0) };
                            }
                            ui.end_row();

                            // Fila 2: let mut (Mutable)
                            let btn_color_1 = if state.show_railroad_modal == Some(1) {
                                egui::Color32::from_rgb(255, 160, 50)
                            } else {
                                egui::Color32::from_rgb(180, 190, 205)
                            };

                            ui.label(egui::RichText::new("let mut").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label(egui::RichText::new("Sí").strong().color(egui::Color32::from_rgb(255, 160, 50)));
                            ui.label(egui::RichText::new("let mut x = 5;\nx = 10;").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                            ui.label("Permite reasignar un nuevo valor a la misma variable de forma explícita.");
                            if ui
                                .add(
                                    egui::Button::image(
                                        egui::Image::from_bytes(
                                            "bytes://view.svg",
                                            include_bytes!("../diagramas/view.svg"),
                                        )
                                        .fit_to_exact_size(egui::vec2(20.0, 20.0))
                                        .tint(btn_color_1),
                                    )
                                    .frame(state.show_railroad_modal == Some(1)),
                                )
                                .on_hover_text("Ver diagrama Railroad de sintaxis (let mut mutable)")
                                .clicked()
                            {
                                state.show_railroad_modal = if state.show_railroad_modal == Some(1) { None } else { Some(1) };
                            }
                            ui.end_row();
                        });
                });

                ui.add_space(15.0);

                // Imagen de Ferris Crab
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/taller/1.png"))
                        .max_height(140.0)
                        .texture_options(egui::TextureOptions::LINEAR)
                        .corner_radius(egui::CornerRadius::same(8)),
                );
            });
        }
        1 => {
            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_const_static")
                    .striped(true)
                    .spacing([25.0, 8.0])
                    .show(ui, |ui| {
                        // Encabezados
                        ui.label(
                            egui::RichText::new("Declaración")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Ubicación en Memoria")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Mutabilidad")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Ejemplo de Código")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Descripción")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.end_row();

                        // Fila 1: const
                        ui.label(
                            egui::RichText::new("const")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label("Valor de compilación");
                        ui.label(
                            egui::RichText::new("No")
                                .strong()
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                        ui.label(
                            egui::RichText::new("const MAX: u32 = 100;")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label("Su valor debe poder evaluarse durante la compilación.");
                        ui.end_row();

                        // Fila 2: static
                        ui.label(
                            egui::RichText::new("static")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label("Dirección Única en RAM");
                        ui.label(
                            egui::RichText::new("No por defecto")
                                .strong()
                                .color(egui::Color32::from_rgb(180, 190, 205)),
                        );
                        ui.label(
                            egui::RichText::new("static VALOR: &str = \"OK\";")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label(
                            "Tiene una ubicación estable y vive durante todo el programa; static mut sí requiere unsafe.",
                        );
                        ui.end_row();
                    });
            });
        }
        2 => {
            mostrar_contenido_tipos_primitivos(ui, state);
        }
        _ => {
            mostrar_contenido_macros(ui);
        }
    }

    // El selector de proyectos y el editor de código solo se muestran en las pestañas interactivas de código (0 e 1)
    if state.conceptos_tab < 2 {
        ui.add_space(15.0);

        mostrar_selector_proyectos_estandar(
            ui,
            &mut state.selected_project,
            &mut state.term_cwd,
            "combo_proyectos_comenzando",
            &mut state.conceptos_code,
        );

        ui.add_space(10.0);

        let theme = &state.theme_set.themes["base16-ocean.dark"];
        mostrar_editor_interactivo(
            ui,
            &mut state.conceptos_code,
            Arc::clone(&state.conceptos_output),
            "",
            ejecutar_codigo_rust,
            &state.syntax_set,
            theme,
        );
    }
}

#[derive(Clone, Copy)]
struct EtapaCompilacion {
    nombre: &'static str,
    subtitulo: &'static str,
    detalle: &'static str,
    salida: &'static str,
    color: egui::Color32,
}

fn color_con_alpha(color: egui::Color32, alpha: f32) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (255.0 * alpha.clamp(0.0, 1.0)) as u8,
    )
}

fn escalar_color(color: egui::Color32, factor: f32, alpha: f32) -> egui::Color32 {
    let canal = |valor: u8| (valor as f32 * factor).clamp(0.0, 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(
        canal(color.r()),
        canal(color.g()),
        canal(color.b()),
        (255.0 * alpha.clamp(0.0, 1.0)) as u8,
    )
}

struct LosaIsometrica {
    ancho: f32,
    fondo: f32,
    grosor: f32,
    color: egui::Color32,
    alpha: f32,
    activa: bool,
}

fn dibujar_losa_isometrica(painter: &egui::Painter, centro: egui::Pos2, losa: LosaIsometrica) {
    let LosaIsometrica {
        ancho,
        fondo,
        grosor,
        color,
        alpha,
        activa,
    } = losa;
    let arriba = egui::pos2(centro.x, centro.y - fondo * 0.5);
    let derecha = egui::pos2(centro.x + ancho * 0.5, centro.y);
    let abajo = egui::pos2(centro.x, centro.y + fondo * 0.5);
    let izquierda = egui::pos2(centro.x - ancho * 0.5, centro.y);
    let abajo_derecha = derecha + egui::vec2(0.0, grosor);
    let abajo_centro = abajo + egui::vec2(0.0, grosor);
    let abajo_izquierda = izquierda + egui::vec2(0.0, grosor);

    if activa {
        for (expansion, opacidad) in [(10.0, 0.05), (6.0, 0.10), (3.0, 0.18)] {
            painter.add(egui::Shape::convex_polygon(
                vec![
                    arriba - egui::vec2(0.0, expansion * 0.5),
                    derecha + egui::vec2(expansion, 0.0),
                    abajo + egui::vec2(0.0, expansion * 0.5),
                    izquierda - egui::vec2(expansion, 0.0),
                ],
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(2.0, color_con_alpha(color, alpha * opacidad)),
            ));
        }
    }

    painter.add(egui::Shape::convex_polygon(
        vec![izquierda, abajo, abajo_centro, abajo_izquierda],
        escalar_color(color, 0.52, alpha),
        egui::Stroke::NONE,
    ));
    painter.add(egui::Shape::convex_polygon(
        vec![abajo, derecha, abajo_derecha, abajo_centro],
        escalar_color(color, 0.30, alpha),
        egui::Stroke::NONE,
    ));
    painter.add(egui::Shape::convex_polygon(
        vec![arriba, derecha, abajo, izquierda],
        escalar_color(color, if activa { 0.62 } else { 0.44 }, alpha),
        egui::Stroke::new(
            if activa { 2.2 } else { 1.2 },
            color_con_alpha(color, alpha * if activa { 0.95 } else { 0.50 }),
        ),
    ));

    let brillo = egui::Stroke::new(1.2, color_con_alpha(egui::Color32::WHITE, alpha * 0.30));
    painter.line_segment([arriba, derecha], brillo);
    painter.line_segment([arriba, izquierda], brillo);
    painter.line_segment(
        [abajo_izquierda, abajo_centro],
        egui::Stroke::new(1.0, color_con_alpha(color, alpha * 0.55)),
    );
    painter.line_segment(
        [abajo_centro, abajo_derecha],
        egui::Stroke::new(1.0, color_con_alpha(color, alpha * 0.35)),
    );
}

fn dibujar_icono_etapa(
    painter: &egui::Painter,
    indice: usize,
    centro: egui::Pos2,
    escala: f32,
    color: egui::Color32,
    alpha: f32,
    tiempo: f32,
) {
    let trazo = egui::Stroke::new(1.8 * escala, color_con_alpha(color, alpha));
    let suave = egui::Stroke::new(1.0 * escala, color_con_alpha(color, alpha * 0.50));

    match indice {
        0 => {
            // Árbol de sintaxis abstracta.
            let nodos = [
                centro + egui::vec2(0.0, -15.0) * escala,
                centro + egui::vec2(-24.0, 4.0) * escala,
                centro + egui::vec2(24.0, 4.0) * escala,
                centro + egui::vec2(-34.0, 23.0) * escala,
                centro + egui::vec2(-13.0, 23.0) * escala,
            ];
            for (a, b) in [(0, 1), (0, 2), (1, 3), (1, 4)] {
                painter.line_segment([nodos[a], nodos[b]], trazo);
            }
            for (n, punto) in nodos.iter().enumerate() {
                painter.circle_filled(
                    *punto,
                    if n == 0 { 5.5 } else { 4.0 } * escala,
                    color_con_alpha(color, alpha),
                );
                painter.circle_stroke(*punto, 8.0 * escala, suave);
            }
        }
        1 => {
            // Escudo del sistema de tipos y borrow checker.
            let s = escala;
            let puntos = vec![
                centro + egui::vec2(-25.0, -18.0) * s,
                centro + egui::vec2(25.0, -18.0) * s,
                centro + egui::vec2(20.0, 12.0) * s,
                centro + egui::vec2(0.0, 28.0) * s,
                centro + egui::vec2(-20.0, 12.0) * s,
            ];
            painter.add(egui::Shape::convex_polygon(
                puntos,
                color_con_alpha(color, alpha * 0.16),
                trazo,
            ));
            painter.line_segment(
                [
                    centro + egui::vec2(-11.0, 2.0) * s,
                    centro + egui::vec2(-2.0, 11.0) * s,
                ],
                trazo,
            );
            painter.line_segment(
                [
                    centro + egui::vec2(-2.0, 11.0) * s,
                    centro + egui::vec2(15.0, -8.0) * s,
                ],
                trazo,
            );
        }
        2 => {
            // Grafo MIR con un pulso viajando entre bloques.
            let s = escala;
            let bloques = [
                egui::Rect::from_center_size(
                    centro + egui::vec2(-28.0, -10.0) * s,
                    egui::vec2(28.0, 15.0) * s,
                ),
                egui::Rect::from_center_size(
                    centro + egui::vec2(7.0, 10.0) * s,
                    egui::vec2(28.0, 15.0) * s,
                ),
                egui::Rect::from_center_size(
                    centro + egui::vec2(36.0, -13.0) * s,
                    egui::vec2(23.0, 15.0) * s,
                ),
            ];
            for bloque in bloques {
                painter.rect(
                    bloque,
                    3.0,
                    color_con_alpha(color, alpha * 0.14),
                    trazo,
                    egui::StrokeKind::Middle,
                );
            }
            painter.line_segment([bloques[0].right_center(), bloques[1].left_center()], suave);
            painter.line_segment([bloques[1].right_center(), bloques[2].left_center()], suave);
            let pulso = (tiempo * 0.8).fract();
            let inicio = bloques[0].center();
            let fin = bloques[2].center();
            painter.circle_filled(
                inicio.lerp(fin, pulso),
                3.5 * s,
                color_con_alpha(egui::Color32::WHITE, alpha),
            );
        }
        3 => {
            // Núcleo de codegen, con anillos en rotación.
            let radio = 17.0 * escala;
            painter.circle_filled(centro, radio * 0.48, color_con_alpha(color, alpha * 0.32));
            painter.circle_stroke(centro, radio, trazo);
            painter.circle_stroke(centro, radio * 1.45, suave);
            for n in 0..6 {
                let angulo = tiempo * 1.8 + n as f32 * std::f32::consts::TAU / 6.0;
                let punto = centro + egui::vec2(angulo.cos(), angulo.sin() * 0.55) * radio * 1.45;
                painter.circle_filled(punto, 2.8 * escala, color_con_alpha(color, alpha));
            }
        }
        _ => {
            // Binario final: bloque con indicador de ejecución.
            let cuerpo = egui::Rect::from_center_size(centro, egui::vec2(72.0, 39.0) * escala);
            painter.rect(
                cuerpo,
                5.0,
                color_con_alpha(color, alpha * 0.16),
                trazo,
                egui::StrokeKind::Middle,
            );
            painter.line_segment(
                [
                    centro + egui::vec2(-20.0, 0.0) * escala,
                    centro + egui::vec2(-7.0, 0.0) * escala,
                ],
                trazo,
            );
            let play = vec![
                centro + egui::vec2(4.0, -10.0) * escala,
                centro + egui::vec2(4.0, 10.0) * escala,
                centro + egui::vec2(20.0, 0.0) * escala,
            ];
            painter.add(egui::Shape::convex_polygon(
                play,
                color_con_alpha(color, alpha * 0.75),
                egui::Stroke::NONE,
            ));
        }
    }
}

fn mostrar_tutorial_compilacion(ui: &mut egui::Ui, state: &mut PortfolioState) {
    const DURACION: f32 = 7.5;
    const ETAPAS: [EtapaCompilacion; 5] = [
        EtapaCompilacion {
            nombre: "01  PARSE / AST",
            subtitulo: "El código se convierte en estructura",
            detalle: "rustc tokeniza el archivo, valida su sintaxis y construye un árbol que representa expresiones, tipos y módulos.",
            salida: "Árbol de sintaxis (AST)",
            color: egui::Color32::from_rgb(67, 205, 255),
        },
        EtapaCompilacion {
            nombre: "02  TYPES + BORROW",
            subtitulo: "Seguridad antes de generar código",
            detalle: "Se resuelven los tipos y el borrow checker comprueba préstamos, ownership y tiempos de vida.",
            salida: "Programa validado",
            color: egui::Color32::from_rgb(255, 91, 125),
        },
        EtapaCompilacion {
            nombre: "03  MIR",
            subtitulo: "Una representación fácil de optimizar",
            detalle: "Rust baja el programa a MIR, simplifica el flujo de control y prepara monomorfización y optimizaciones.",
            salida: "MIR optimizado",
            color: egui::Color32::from_rgb(255, 190, 70),
        },
        EtapaCompilacion {
            nombre: "04  LLVM CODEGEN",
            subtitulo: "Del lenguaje a código de máquina",
            detalle: "LLVM aplica optimizaciones de bajo nivel y produce archivos objeto específicos para tu CPU y sistema.",
            salida: "Archivos objeto (.o)",
            color: egui::Color32::from_rgb(123, 103, 255),
        },
        EtapaCompilacion {
            nombre: "05  LINKER",
            subtitulo: "Todo se une en un ejecutable",
            detalle: "El enlazador combina tu código, dependencias y bibliotecas del sistema para crear el binario final.",
            salida: "Binario ejecutable",
            color: egui::Color32::from_rgb(64, 225, 157),
        },
    ];

    if state.anim_compilacion_activa {
        let dt = ui.ctx().input(|i| i.stable_dt).min(0.1);
        state.compilacion_progreso = (state.compilacion_progreso + dt / DURACION).min(1.0);
        if state.compilacion_progreso >= 1.0 {
            state.anim_compilacion_activa = false;
            state.compilacion_etapa_seleccionada = ETAPAS.len() - 1;
        } else {
            state.compilacion_etapa_seleccionada = (state.compilacion_progreso
                * ETAPAS.len() as f32)
                .floor()
                .min(4.0) as usize;
            ui.ctx().request_repaint();
        }
    }

    ui.add_space(18.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Del código al binario")
                .size(32.0)
                .strong()
                .color(egui::Color32::from_rgb(239, 244, 255)),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new("Explora el pipeline de compilación de Rust, etapa por etapa")
                .size(15.0)
                .color(egui::Color32::from_rgb(148, 160, 184)),
        );
    });
    ui.add_space(18.0);

    egui::Frame::new()
        .fill(egui::Color32::from_rgb(17, 21, 31))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let texto_boton = if state.anim_compilacion_activa {
                    "⏸  Pausar"
                } else if state.compilacion_progreso >= 1.0 {
                    "▶  Reproducir"
                } else {
                    "▶  Continuar"
                };
                if ui.button(texto_boton).clicked() {
                    if state.compilacion_progreso >= 1.0 {
                        state.compilacion_progreso = 0.0;
                        state.compilacion_etapa_seleccionada = 0;
                    }
                    state.anim_compilacion_activa = !state.anim_compilacion_activa;
                }
                if ui.button("↺  Reiniciar").clicked() {
                    state.compilacion_progreso = 0.0;
                    state.compilacion_etapa_seleccionada = 0;
                    state.anim_compilacion_activa = true;
                }

                ui.add_space(10.0);
                let porcentaje = (state.compilacion_progreso * 100.0).round() as u32;
                ui.add(
                    egui::ProgressBar::new(state.compilacion_progreso)
                        .desired_width((ui.available_width() - 70.0).max(100.0))
                        .text(format!("Compilación  {porcentaje}%")),
                );
            });
        });

    ui.add_space(12.0);
    let ancho = ui.available_width();
    let compacto = ancho < 690.0;
    let alto = if compacto { 650.0 } else { 510.0 };
    let (rect, respuesta) = ui.allocate_exact_size(egui::vec2(ancho, alto), egui::Sense::click());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 16.0, egui::Color32::from_rgb(10, 13, 21));
    painter.rect_stroke(
        rect.shrink(0.5),
        16.0,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(37, 45, 63)),
        egui::StrokeKind::Inside,
    );

    // Rejilla de fondo para reforzar la profundidad sin usar un motor 3D.
    let rejilla = egui::Color32::from_rgba_unmultiplied(91, 112, 150, 18);
    let paso = 32.0;
    let mut x = rect.left();
    while x <= rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            egui::Stroke::new(1.0, rejilla),
        );
        x += paso;
    }
    let mut y = rect.top();
    while y <= rect.bottom() {
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(1.0, rejilla),
        );
        y += paso;
    }

    let pila_ancho = if compacto {
        (rect.width() - 50.0).min(330.0)
    } else {
        (rect.width() * 0.42).clamp(260.0, 360.0)
    };
    let pila_fondo = pila_ancho * 0.36;
    let centro_x = if compacto {
        rect.center().x
    } else {
        rect.left() + rect.width() * 0.29
    };
    let inicio_y = rect.top() + 92.0;
    let separacion = 72.0;
    let puntero = respuesta.hover_pos();
    let tiempo = state.tutorial_time as f32;
    let completadas = (state.compilacion_progreso * ETAPAS.len() as f32).clamp(0.0, 5.0);
    let mut etapa_hover = None;

    for indice in 0..ETAPAS.len() {
        let centro_y = inicio_y + indice as f32 * separacion;
        let zona = egui::Rect::from_center_size(
            egui::pos2(centro_x, centro_y),
            egui::vec2(pila_ancho + 30.0, separacion),
        );
        if puntero.is_some_and(|p| zona.contains(p)) {
            etapa_hover = Some(indice);
        }
    }
    if respuesta.clicked()
        && let Some(indice) = etapa_hover
    {
        state.compilacion_etapa_seleccionada = indice;
    }
    let etapa_visible = etapa_hover.unwrap_or(state.compilacion_etapa_seleccionada);

    // Sombra común de la pila.
    let sombra_centro = egui::pos2(centro_x + 12.0, inicio_y + 4.0 * separacion + 38.0);
    painter.add(egui::Shape::convex_polygon(
        vec![
            sombra_centro + egui::vec2(0.0, -pila_fondo * 0.35),
            sombra_centro + egui::vec2(pila_ancho * 0.57, 0.0),
            sombra_centro + egui::vec2(0.0, pila_fondo * 0.55),
            sombra_centro + egui::vec2(-pila_ancho * 0.57, 0.0),
        ],
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 90),
        egui::Stroke::NONE,
    ));

    // Se pinta de abajo hacia arriba para que el solapamiento sea natural.
    for indice in (0..ETAPAS.len()).rev() {
        let etapa = ETAPAS[indice];
        let fraccion = (completadas - indice as f32).clamp(0.0, 1.0);
        let disponible = fraccion > 0.0 || state.compilacion_progreso >= 1.0;
        let activa = indice == etapa_visible;
        let hover_offset = if activa { -7.0 } else { 0.0 };
        let entrada = if disponible {
            (1.0 - fraccion).powi(2) * -22.0
        } else {
            0.0
        };
        let centro = egui::pos2(
            centro_x,
            inicio_y + indice as f32 * separacion + hover_offset + entrada,
        );
        let alpha = if disponible {
            0.62 + fraccion * 0.38
        } else {
            0.20
        };

        dibujar_losa_isometrica(
            &painter,
            centro,
            LosaIsometrica {
                ancho: pila_ancho,
                fondo: pila_fondo,
                grosor: 18.0,
                color: etapa.color,
                alpha,
                activa,
            },
        );
        dibujar_icono_etapa(
            &painter,
            indice,
            centro + egui::vec2(0.0, -6.0),
            (pila_ancho / 330.0).clamp(0.75, 1.0),
            etapa.color,
            alpha,
            tiempo,
        );

        let badge = egui::pos2(centro.x - pila_ancho * 0.5 + 23.0, centro.y - 5.0);
        painter.circle_filled(badge, 11.0, escalar_color(etapa.color, 0.28, alpha));
        painter.circle_stroke(
            badge,
            11.0,
            egui::Stroke::new(1.2, color_con_alpha(etapa.color, alpha)),
        );
        painter.text(
            badge,
            egui::Align2::CENTER_CENTER,
            format!("{}", indice + 1),
            egui::FontId::monospace(10.0),
            color_con_alpha(egui::Color32::WHITE, alpha),
        );
    }

    // Pulso que recorre el pipeline durante la reproducción.
    if state.anim_compilacion_activa {
        let tramo = (state.compilacion_progreso * 5.0).min(4.999);
        let indice = tramo.floor() as usize;
        let local = tramo.fract();
        let y0 = inicio_y + indice as f32 * separacion;
        let y1 = inicio_y + (indice + 1).min(4) as f32 * separacion;
        let punto = egui::pos2(centro_x + pila_ancho * 0.54, egui::lerp(y0..=y1, local));
        painter.circle_filled(punto, 9.0, color_con_alpha(ETAPAS[indice].color, 0.16));
        painter.circle_filled(punto, 4.0, egui::Color32::WHITE);
    }

    let etapa = ETAPAS[etapa_visible];
    if compacto {
        let tarjeta = egui::Rect::from_min_max(
            egui::pos2(rect.left() + 18.0, rect.bottom() - 150.0),
            egui::pos2(rect.right() - 18.0, rect.bottom() - 18.0),
        );
        dibujar_panel_etapa(&painter, tarjeta, etapa, etapa_visible);
    } else {
        let tarjeta = egui::Rect::from_min_max(
            egui::pos2(rect.left() + rect.width() * 0.56, rect.top() + 68.0),
            egui::pos2(rect.right() - 24.0, rect.bottom() - 68.0),
        );
        dibujar_panel_etapa(&painter, tarjeta, etapa, etapa_visible);

        let origen = egui::pos2(
            centro_x + pila_ancho * 0.5 + 8.0,
            inicio_y + etapa_visible as f32 * separacion,
        );
        let codo = egui::pos2(tarjeta.left() - 18.0, origen.y);
        let destino = egui::pos2(tarjeta.left(), tarjeta.top() + 52.0);
        let conector = egui::Stroke::new(1.2, color_con_alpha(etapa.color, 0.65));
        painter.line_segment([origen, codo], conector);
        painter.line_segment([codo, destino], conector);
        painter.circle_filled(origen, 3.5, etapa.color);
    }

    if etapa_hover.is_some() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new("Pasa el cursor o haz clic en una capa para inspeccionarla.")
            .small()
            .color(egui::Color32::from_rgb(124, 137, 160)),
    );
}

fn dibujar_panel_etapa(
    painter: &egui::Painter,
    rect: egui::Rect,
    etapa: EtapaCompilacion,
    indice: usize,
) {
    painter.rect(
        rect,
        12.0,
        egui::Color32::from_rgba_unmultiplied(18, 23, 35, 245),
        egui::Stroke::new(1.0, color_con_alpha(etapa.color, 0.45)),
        egui::StrokeKind::Inside,
    );
    painter.rect_filled(
        egui::Rect::from_min_size(rect.min, egui::vec2(4.0, rect.height())),
        2.0,
        etapa.color,
    );
    painter.text(
        rect.min + egui::vec2(24.0, 22.0),
        egui::Align2::LEFT_TOP,
        etapa.nombre,
        egui::FontId::monospace(15.0),
        etapa.color,
    );
    painter.text(
        rect.min + egui::vec2(24.0, 50.0),
        egui::Align2::LEFT_TOP,
        etapa.subtitulo,
        egui::FontId::proportional(17.0),
        egui::Color32::from_rgb(234, 239, 249),
    );

    let ancho_texto = (rect.width() - 48.0).max(120.0);
    let detalle = painter.layout(
        etapa.detalle.to_owned(),
        egui::FontId::proportional(14.0),
        egui::Color32::from_rgb(158, 170, 192),
        ancho_texto,
    );
    painter.galley(
        rect.min + egui::vec2(24.0, 82.0),
        detalle,
        egui::Color32::WHITE,
    );

    let salida_y = rect.bottom() - 50.0;
    painter.line_segment(
        [
            egui::pos2(rect.left() + 24.0, salida_y - 12.0),
            egui::pos2(rect.right() - 24.0, salida_y - 12.0),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(42, 50, 67)),
    );
    painter.text(
        egui::pos2(rect.left() + 24.0, salida_y),
        egui::Align2::LEFT_TOP,
        format!("SALIDA  →  {}", etapa.salida),
        egui::FontId::monospace(12.0),
        egui::Color32::from_rgb(196, 205, 220),
    );
    painter.text(
        rect.right_top() + egui::vec2(-18.0, 18.0),
        egui::Align2::RIGHT_TOP,
        format!("{}/5", indice + 1),
        egui::FontId::monospace(11.0),
        egui::Color32::from_rgb(112, 124, 146),
    );
}

fn mostrar_tutorial_tipos_datos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Tipos compuestos")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        for (i, label) in [(0, "Array [T; N]"), (1, "Slice &[T]"), (2, "Tupla")] {
            let activo = state.compuestos_tab == i;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.compuestos_tab = i;
            }
            ui.add_space(4.0);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.compuestos_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Comparar").strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.compuestos_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Teórico:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.compuestos_tab {
        0 => mostrar_compuesto_array(ui, state, naranja, cyan, texto),
        1 => mostrar_compuesto_slice(ui, state, naranja, cyan, texto),
        2 => mostrar_compuesto_tupla(ui, state, naranja, cyan, texto),
        _ => mostrar_compuesto_comparar(ui, naranja, cyan, texto),
    }
}

fn mostrar_compuesto_array(
    ui: &mut egui::Ui,
    state: &mut PortfolioState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un array `[T; N]` guarda N valores del mismo tipo, contiguos, con tamaño fijo \
             conocido en compilación. Suele vivir en el stack.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_array_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Pieza").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Nota").strong().color(egui::Color32::WHITE));
                ui.end_row();
                ui.label(egui::RichText::new("Tipo").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("[i32; 5]").monospace().color(cyan));
                ui.label("T y N fijos; N es parte del tipo.");
                ui.end_row();
                ui.label(egui::RichText::new("Acceso").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("arr[i]").monospace().color(cyan));
                ui.label("Fuera de rango → panic en runtime.");
                ui.end_row();
                ui.label(egui::RichText::new("len").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("arr.len()").monospace().color(cyan));
                ui.label("Siempre N; no crece como un Vec.");
                ui.end_row();
            });
    });

    ui.add_space(12.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Tipo T:").strong().color(texto));
        ui.selectable_value(&mut state.arr_elem_type, 0, "i8");
        ui.selectable_value(&mut state.arr_elem_type, 1, "i32");
        ui.selectable_value(&mut state.arr_elem_type, 2, "f64");
        ui.selectable_value(&mut state.arr_elem_type, 3, "bool");
        ui.selectable_value(&mut state.arr_elem_type, 4, "char");
        ui.add_space(12.0);
        ui.label(egui::RichText::new("N:").strong().color(texto));
        ui.add(egui::Slider::new(&mut state.arr_len, 1..=8).text("elems"));
    });

    let mut custom_items: Vec<String> = Vec::new();
    if let Some(pos_eq) = state.arr_code.find("= [") {
        let rest = &state.arr_code[pos_eq + 3..];
        if let Some(pos_end) = rest.find(']') {
            custom_items = rest[..pos_end]
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !custom_items.is_empty() {
                state.arr_len = custom_items.len().clamp(1, 8);
            }
        }
    }
    if state.arr_code.contains("i8") {
        state.arr_elem_type = 0;
    } else if state.arr_code.contains("f64") {
        state.arr_elem_type = 2;
    } else if state.arr_code.contains("bool") {
        state.arr_elem_type = 3;
    } else if state.arr_code.contains("char") {
        state.arr_elem_type = 4;
    } else if state.arr_code.contains("i32") || state.arr_code.contains("u32") {
        state.arr_elem_type = 1;
    }

    let (type_str, elem_size, default_samples) = match state.arr_elem_type {
        0 => ("i8", 1, vec!["-12", "45", "127", "-8", "0", "99", "-50", "12"]),
        1 => (
            "i32",
            4,
            vec!["100", "-500", "2048", "42", "0", "999", "-123", "8888"],
        ),
        2 => (
            "f64",
            8,
            vec!["3.14", "9.81", "-0.5", "2.71", "100.0", "0.001", "-45.2", "1.61"],
        ),
        3 => (
            "bool",
            1,
            vec!["true", "false", "true", "true", "false", "false", "true", "false"],
        ),
        _ => (
            "char",
            4,
            vec!["'R'", "'u'", "'s'", "'t'", "'🦀'", "'⚡'", "'🔥'", "'A'"],
        ),
    };
    let samples: Vec<&str> = if !custom_items.is_empty() {
        custom_items.iter().map(|s| s.as_str()).collect()
    } else {
        default_samples
    };
    let total_stack_bytes = state.arr_len * elem_size;

    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(format!(
            "Firma: [{type_str}; {}]  ·  Stack ≈ {total_stack_bytes} bytes",
            state.arr_len
        ))
        .monospace()
        .color(cyan),
    );

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 120.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let cell_w = (rect.width() - 40.0) / state.arr_len as f32;
    let start_x = rect.left() + 20.0;
    let y = rect.center().y;
    for i in 0..state.arr_len {
        let box_x = start_x + (i as f32 * cell_w) + cell_w / 2.0;
        let box_rect =
            egui::Rect::from_center_size(egui::pos2(box_x, y), egui::vec2(cell_w - 6.0, 48.0));
        let is_active = i == state.arr_active_idx;
        let fill = if is_active {
            egui::Color32::from_rgb(48, 36, 22)
        } else {
            egui::Color32::from_rgb(28, 36, 52)
        };
        let stroke_c = if is_active { naranja } else { cyan };
        painter.rect(
            box_rect,
            5.0,
            fill,
            egui::Stroke::new(1.5, stroke_c),
            egui::StrokeKind::Middle,
        );
        painter.text(
            egui::pos2(box_rect.center().x, box_rect.top() + 6.0),
            egui::Align2::CENTER_TOP,
            format!("[{i}]"),
            egui::FontId::proportional(11.0),
            egui::Color32::LIGHT_GRAY,
        );
        painter.text(
            box_rect.center() + egui::vec2(0.0, 4.0),
            egui::Align2::CENTER_CENTER,
            samples[i % samples.len()],
            egui::FontId::monospace(12.0),
            egui::Color32::WHITE,
        );
    }

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("Índice arr[i]:");
        ui.add(egui::Slider::new(&mut state.arr_active_idx, 0..=state.arr_len.max(1)).text("i"));
        if ui.button("arr.len()").clicked() {
            state.arr_action_msg = format!("arr.len() = {}", state.arr_len);
        }
        if ui.button("size_of").clicked() {
            state.arr_action_msg = format!("≈ {total_stack_bytes} bytes en stack");
        }
    });
    if state.arr_active_idx >= state.arr_len {
        ui.label(
            egui::RichText::new(format!(
                "PANIC: índice {} fuera de rango (len {})",
                state.arr_active_idx, state.arr_len
            ))
            .color(egui::Color32::from_rgb(255, 120, 120)),
        );
    } else {
        ui.label(
            egui::RichText::new(format!(
                "arr[{}] = {}",
                state.arr_active_idx,
                samples[state.arr_active_idx % samples.len()]
            ))
            .color(egui::Color32::from_rgb(120, 220, 140)),
        );
    }
    if !state.arr_action_msg.is_empty() {
        ui.label(
            egui::RichText::new(&state.arr_action_msg)
                .italics()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
    }

    ui.add_space(12.0);
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.arr_code,
        Arc::clone(&state.arr_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

fn mostrar_compuesto_slice(
    ui: &mut egui::Ui,
    state: &mut PortfolioState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Un slice `&[T]` es una vista (préstamo) sobre una secuencia contigua: \
             fat pointer = puntero + longitud. No es dueño de los datos (Ownership).",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_slice_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Pieza").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Nota").strong().color(egui::Color32::WHITE));
                ui.end_row();
                ui.label(egui::RichText::new("Tipo").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("&[i32]").monospace().color(cyan));
                ui.label("Referencia; el array/String sigue siendo dueño.");
                ui.end_row();
                ui.label(egui::RichText::new("Rango").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("&arr[1..4]").monospace().color(cyan));
                ui.label("Inicio inclusivo, fin exclusivo.");
                ui.end_row();
                ui.label(egui::RichText::new("&str").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("&str ≈ &[u8] UTF-8").monospace().color(cyan));
                ui.label("El slice de texto que ya viste en Strings.");
                ui.end_row();
            });
    });

    let slice_max = 6;
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Rango:");
        ui.add(egui::Slider::new(&mut state.slice_start, 0..=slice_max - 1).text("start"));
        ui.add(egui::Slider::new(&mut state.slice_end, 1..=slice_max).text("end"));
    });
    if state.slice_start >= state.slice_end {
        state.slice_end = state.slice_start + 1;
    }
    let slice_len = state.slice_end - state.slice_start;

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 130.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let cell_w = 72.0;
    let start_x = rect.left() + 36.0;
    let y = rect.center().y + 4.0;
    let vals = ["10", "20", "30", "40", "50", "60"];
    for i in 0..slice_max {
        let box_x = start_x + (i as f32 * cell_w) + cell_w / 2.0;
        let box_rect =
            egui::Rect::from_center_size(egui::pos2(box_x, y), egui::vec2(cell_w - 8.0, 44.0));
        let in_slice = i >= state.slice_start && i < state.slice_end;
        painter.rect(
            box_rect,
            4.0,
            if in_slice {
                egui::Color32::from_rgb(28, 48, 72)
            } else {
                egui::Color32::from_rgb(24, 28, 36)
            },
            egui::Stroke::new(1.2, if in_slice { cyan } else { egui::Color32::from_rgb(60, 70, 85) }),
            egui::StrokeKind::Middle,
        );
        painter.text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            vals[i],
            egui::FontId::monospace(13.0),
            egui::Color32::WHITE,
        );
    }
    let slice_min_x = start_x + (state.slice_start as f32 * cell_w);
    let slice_max_x = start_x + (state.slice_end as f32 * cell_w);
    let slice_rect = egui::Rect::from_min_max(
        egui::pos2(slice_min_x + 2.0, y - 30.0),
        egui::pos2(slice_max_x - 2.0, y + 30.0),
    );
    painter.rect_stroke(
        slice_rect,
        6.0,
        egui::Stroke::new(2.0, naranja),
        egui::StrokeKind::Middle,
    );
    painter.text(
        egui::pos2(slice_rect.center().x, slice_rect.top() - 4.0),
        egui::Align2::CENTER_BOTTOM,
        format!(
            "&arr[{}..{}]  len={slice_len}",
            state.slice_start, state.slice_end
        ),
        egui::FontId::proportional(12.0),
        naranja,
    );

    ui.add_space(12.0);
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.slice_code,
        Arc::clone(&state.slice_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

fn mostrar_compuesto_tupla(
    ui: &mut egui::Ui,
    state: &mut PortfolioState,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Una tupla agrupa valores de tipos distintos, sin nombres de campo. \
             Acceso por `.0`, `.1`… o desestructuración. Puente natural hacia `struct`.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_tupla_comp")
            .striped(true)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Pieza").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Nota").strong().color(egui::Color32::WHITE));
                ui.end_row();
                ui.label(egui::RichText::new("Tipo").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("(i32, bool, f64)").monospace().color(cyan));
                ui.label("Heterogénea; el orden define el tipo.");
                ui.end_row();
                ui.label(egui::RichText::new("Campo").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("t.0  t.1").monospace().color(cyan));
                ui.label("Índices fijos desde cero.");
                ui.end_row();
                ui.label(egui::RichText::new("Destruct").monospace().strong().color(naranja));
                ui.label(egui::RichText::new("let (a, b, c) = t;").monospace().color(cyan));
                ui.label("Muy usado al devolver varios valores desde fn.");
                ui.end_row();
            });
    });

    ui.add_space(10.0);
    let row = |ui: &mut egui::Ui, label: &str, slot: &mut usize| {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.selectable_value(slot, 0, "i32");
            ui.selectable_value(slot, 1, "bool");
            ui.selectable_value(slot, 2, "f64");
            ui.selectable_value(slot, 3, "char");
        });
    };
    row(ui, "Campo .0:", &mut state.tup_t0);
    row(ui, "Campo .1:", &mut state.tup_t1);
    row(ui, "Campo .2:", &mut state.tup_t2);

    let info = |id: usize| match id {
        0 => ("i32", "100", egui::Color32::from_rgb(60, 140, 240)),
        1 => ("bool", "true", egui::Color32::from_rgb(40, 180, 100)),
        2 => ("f64", "3.14", egui::Color32::from_rgb(240, 140, 40)),
        _ => ("char", "'R'", egui::Color32::from_rgb(180, 120, 240)),
    };
    let (n0, v0, c0) = info(state.tup_t0);
    let (n1, v1, c1) = info(state.tup_t1);
    let (n2, v2, c2) = info(state.tup_t2);

    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(format!("Firma: ({n0}, {n1}, {n2})"))
            .monospace()
            .color(cyan),
    );

    ui.add_space(8.0);
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), 110.0));
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        8.0,
        egui::Color32::from_rgb(14, 18, 26),
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90)),
        egui::StrokeKind::Inside,
    );
    let y = rect.center().y;
    for (i, (name, val, col)) in [(n0, v0, c0), (n1, v1, c1), (n2, v2, c2)]
        .into_iter()
        .enumerate()
    {
        let x = rect.left() + 90.0 + i as f32 * 180.0;
        let r = egui::Rect::from_center_size(egui::pos2(x, y), egui::vec2(150.0, 52.0));
        painter.rect(
            r,
            6.0,
            egui::Color32::from_rgb(22, 28, 40),
            egui::Stroke::new(2.0, col),
            egui::StrokeKind::Middle,
        );
        painter.text(
            r.center(),
            egui::Align2::CENTER_CENTER,
            format!(".{i}: {val} ({name})"),
            egui::FontId::monospace(12.0),
            egui::Color32::WHITE,
        );
    }

    ui.add_space(12.0);
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.tup_code,
        Arc::clone(&state.tup_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

fn mostrar_compuesto_comparar(
    ui: &mut egui::Ui,
    naranja: egui::Color32,
    cyan: egui::Color32,
    texto: egui::Color32,
) {
    ui.label(
        egui::RichText::new(
            "Elige la forma según homogeneidad, tamaño fijo y si necesitas nombres de campo. \
             Después, `struct` pondrá nombres; `Vec` hará crecer lo homogéneo.",
        )
        .color(texto),
    );
    ui.add_space(10.0);

    card_frame_tutorial().show(ui, |ui| {
        egui::Grid::new("tabla_compuestos_vs")
            .striped(true)
            .spacing([14.0, 10.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Tipo").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Homogéneo").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Tamaño").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Dueño").strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("Siguiente paso").strong().color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("[T; N]").monospace().strong().color(naranja));
                ui.label("Sí");
                ui.label("Fijo N");
                ui.label("El array");
                ui.label(egui::RichText::new("Vec<T>").monospace().color(cyan));
                ui.end_row();

                ui.label(egui::RichText::new("&[T]").monospace().strong().color(naranja));
                ui.label("Sí");
                ui.label("Dinámico (vista)");
                ui.label("No (préstamo)");
                ui.label(egui::RichText::new("&str / APIs").monospace().color(cyan));
                ui.end_row();

                ui.label(egui::RichText::new("(A,B,…)").monospace().strong().color(naranja));
                ui.label("No");
                ui.label("Fijo #campos");
                ui.label("La tupla");
                ui.label(egui::RichText::new("struct").monospace().color(cyan));
                ui.end_row();
            });
    });

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(
            "Cuando la tupla se vuelve confusa (¿qué era .2?) → sesión Structs & impl.",
        )
        .italics()
        .color(egui::Color32::from_rgb(140, 150, 165)),
    );
}


/// Sesión unificada: Strings + Ownership (menú: 🧵 Strings & Ownership).
fn mostrar_tutorial_strings_ownership(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Strings & Ownership")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Tabs al estilo Comenzando / Funciones
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Conceptos:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );

        let tabs = [
            (0, "String vs &str"),
            (1, "Ownership"),
            (2, "Borrowing"),
        ];
        for (indice, label) in tabs {
            let activo = state.strings_ownership_tab == indice;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.strings_ownership_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.strings_ownership_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Stack / Heap").strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.strings_ownership_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Visual:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.strings_ownership_tab {
        0 => {
            ui.label(
                egui::RichText::new(
                    "En Rust hay dos caras del texto: el dueño que puede crecer (`String`) y la \
                     vista de solo lectura (`&str`). Entenderlas es el puente hacia Ownership.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_string_vs_str")
                    .striped(true)
                    .spacing([18.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Tipo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("¿Dónde vive?").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Cuándo usarlo").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("&str").monospace().strong().color(naranja));
                        ui.label("Binario y/o préstamo (vista)");
                        ui.label(egui::RichText::new("let s: &str = \"Hola\";").monospace().color(cyan));
                        ui.label("Leer o pasar texto sin regalar el dueño.");
                        ui.end_row();

                        ui.label(egui::RichText::new("String").monospace().strong().color(naranja));
                        ui.label("Heap (propietario)");
                        ui.label(egui::RichText::new("String::from(\"Hola\")").monospace().color(cyan));
                        ui.label("Crear, modificar, crecer (`push_str`).");
                        ui.end_row();

                        ui.label(egui::RichText::new("&String → &str").monospace().strong().color(naranja));
                        ui.label("Deref coercion");
                        ui.label(egui::RichText::new("let v: &str = &mi_string;").monospace().color(cyan));
                        ui.label("Casi siempre las APIs piden `&str`, no `&String`.");
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(
                    "Regla práctica: dueño y mutable → `String`. Solo mirar o firmar funciones → `&str`.",
                )
                .italics()
                .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        }
        1 => {
            ui.label(
                egui::RichText::new(
                    "Ownership es el modelo de memoria de Rust: cada valor tiene un dueño; al moverse, \
                     el dueño anterior deja de valer; al salir del scope, se libera (`drop`). \
                     `String` en el heap lo hace visible (a diferencia de un `i32` que se copia).",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_ownership_reglas")
                    .striped(true)
                    .spacing([18.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Regla").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("En código").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Significado").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("1. Un dueño").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let s1 = String::from(\"a\");").monospace().color(cyan));
                        ui.label("Cada valor tiene una variable propietaria.");
                        ui.end_row();

                        ui.label(egui::RichText::new("2. Move").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let s2 = s1; // s1 inválido").monospace().color(cyan));
                        ui.label("Solo un dueño a la vez; asignar mueve (no copia el heap).");
                        ui.end_row();

                        ui.label(egui::RichText::new("3. Drop").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("} // fin de scope").monospace().color(cyan));
                        ui.label("Al salir el dueño, Rust libera la memoria automáticamente.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Copy").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let b = a; // i32 sí copia").monospace().color(cyan));
                        ui.label("Tipos chicos en stack (`i32`, `bool`…) se copian; no se invalidan.");
                        ui.end_row();
                    });
            });
        }
        2 => {
            ui.label(
                egui::RichText::new(
                    "Borrowing = prestar sin regalar el dueño. `&T` (lectura, muchas a la vez) o \
                     `&mut T` (escritura, solo una). Nunca ambas al mismo tiempo sobre el mismo dato.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_borrowing")
                    .striped(true)
                    .spacing([18.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Préstamo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Regla").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Inmutable").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let r = &s;").monospace().color(cyan));
                        ui.label("Muchas `&T` simultáneas; no puedes mutar por ellas.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Mutable").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let r = &mut s;").monospace().color(cyan));
                        ui.label("Solo una `&mut T`; exclusivo mientras vive el préstamo.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Exclusividad").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("& y &mut a la vez → error").monospace().color(cyan));
                        ui.label("El borrow checker evita data races en compilación.");
                        ui.end_row();

                        ui.label(egui::RichText::new("API típica").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("fn f(s: &str)").monospace().color(cyan));
                        ui.label("Pides vista; el caller sigue siendo dueño del `String`.");
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.label(
                egui::RichText::new(
                    "Visualiza el heap con el texto real y el stack con el dueño (puntero, len, cap). \
                     El MOVE cambia quién apunta; el BORROW añade una referencia sin quitar el dueño.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Paso:").strong().color(gris_tab));
                ui.add_space(6.0);
                for (i, label) in [
                    (0, "1. s1 = String"),
                    (1, "2. MOVE s2 = s1"),
                    (2, "3. BORROW &s2"),
                ] {
                    let activo = state.ownership_step == i;
                    let color = if activo { naranja } else { gris_tab };
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(label).strong().color(color))
                                .frame(activo),
                        )
                        .clicked()
                    {
                        state.ownership_step = i;
                    }
                    ui.add_space(4.0);
                }
            });

            ui.add_space(12.0);
            mostrar_simulador_ownership_memoria(ui, state.ownership_step);
        }
    }

    // Editor en tabs de concepto; en visual también útil probar el código
    ui.add_space(15.0);
    if state.strings_ownership_tab < 3 {
        mostrar_selector_proyectos_estandar(
            ui,
            &mut state.selected_project,
            &mut state.term_cwd,
            "combo_proyectos_strings_ownership",
            &mut state.ownership_code,
        );
        ui.add_space(10.0);
    }
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.ownership_code,
        Arc::clone(&state.ownership_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

/// Simulador stack/heap para MOVE y BORROW (tema del curso).
fn mostrar_simulador_ownership_memoria(ui: &mut egui::Ui, step: usize) {
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let verde = egui::Color32::from_rgb(80, 200, 120);
    let rojo = egui::Color32::from_rgb(220, 100, 100);
    let morado = egui::Color32::from_rgb(180, 140, 255);
    let canvas_bg = egui::Color32::from_rgb(14, 18, 26);
    let border = egui::Color32::from_rgb(45, 60, 90);

    let height = 220.0;
    let width = ui.available_width().min(780.0);
    let (_, rect) = ui.allocate_space(egui::vec2(width, height));
    let painter = ui.painter_at(rect);

    painter.rect(
        rect,
        8.0,
        canvas_bg,
        egui::Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );

    let stack_rect = egui::Rect::from_min_size(
        egui::pos2(rect.left() + 24.0, rect.top() + 28.0),
        egui::vec2(240.0, 168.0),
    );
    let heap_rect = egui::Rect::from_min_size(
        egui::pos2(rect.left() + 360.0, rect.top() + 28.0),
        egui::vec2(240.0, 168.0),
    );

    painter.rect(
        stack_rect,
        6.0,
        egui::Color32::from_rgb(22, 28, 40),
        egui::Stroke::new(1.5, cyan),
        egui::StrokeKind::Middle,
    );
    painter.text(
        stack_rect.left_top() + egui::vec2(12.0, 10.0),
        egui::Align2::LEFT_TOP,
        "STACK",
        egui::FontId::proportional(13.0),
        cyan,
    );

    painter.rect(
        heap_rect,
        6.0,
        egui::Color32::from_rgb(22, 36, 30),
        egui::Stroke::new(1.5, verde),
        egui::StrokeKind::Middle,
    );
    painter.text(
        heap_rect.left_top() + egui::vec2(12.0, 10.0),
        egui::Align2::LEFT_TOP,
        "HEAP  \"Hola\"",
        egui::FontId::proportional(13.0),
        verde,
    );

    let heap_data = heap_rect.center() + egui::vec2(0.0, 12.0);
    painter.circle_filled(heap_data, 28.0, egui::Color32::from_rgb(36, 90, 60));
    painter.circle_stroke(heap_data, 28.0, egui::Stroke::new(2.0, verde));
    painter.text(
        heap_data,
        egui::Align2::CENTER_CENTER,
        "\"Hola\"",
        egui::FontId::monospace(13.0),
        egui::Color32::WHITE,
    );

    let slot = |y: f32| egui::pos2(stack_rect.center().x, stack_rect.top() + y);

    match step {
        0 => {
            let s1 = slot(70.0);
            painter.rect(
                egui::Rect::from_center_size(s1, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(28, 48, 72),
                egui::Stroke::new(1.5, cyan),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s1,
                egui::Align2::CENTER_CENTER,
                "s1  dueño activo",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s1 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(28.0, 0.0)],
                egui::Stroke::new(2.0, cyan),
            );
        }
        1 => {
            let s1 = slot(55.0);
            let s2 = slot(115.0);
            painter.rect(
                egui::Rect::from_center_size(s1, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(48, 28, 28),
                egui::Stroke::new(1.5, rojo),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s1,
                egui::Align2::CENTER_CENTER,
                "s1  MOVED (inválido)",
                egui::FontId::proportional(12.0),
                rojo,
            );
            painter.rect(
                egui::Rect::from_center_size(s2, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(28, 48, 36),
                egui::Stroke::new(1.5, verde),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s2,
                egui::Align2::CENTER_CENTER,
                "s2  nuevo dueño",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s2 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(28.0, 0.0)],
                egui::Stroke::new(2.0, verde),
            );
        }
        _ => {
            let s2 = slot(55.0);
            let s3 = slot(115.0);
            painter.rect(
                egui::Rect::from_center_size(s2, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(28, 48, 36),
                egui::Stroke::new(1.5, verde),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s2,
                egui::Align2::CENTER_CENTER,
                "s2  dueño",
                egui::FontId::proportional(13.0),
                egui::Color32::WHITE,
            );
            painter.rect(
                egui::Rect::from_center_size(s3, egui::vec2(190.0, 34.0)),
                4.0,
                egui::Color32::from_rgb(40, 32, 56),
                egui::Stroke::new(1.5, morado),
                egui::StrokeKind::Middle,
            );
            painter.text(
                s3,
                egui::Align2::CENTER_CENTER,
                "s3 = &s2  préstamo",
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
            painter.line_segment(
                [s2 + egui::vec2(95.0, 0.0), heap_data - egui::vec2(28.0, 0.0)],
                egui::Stroke::new(2.0, verde),
            );
            painter.line_segment(
                [s3 + egui::vec2(0.0, -17.0), s2 + egui::vec2(0.0, 17.0)],
                egui::Stroke::new(2.0, morado),
            );
        }
    }

    let caption = match step {
        0 => "s1 en el stack apunta al buffer \"Hola\" en el heap.",
        1 => "MOVE: el dueño pasa a s2; usar s1 sería error de compilación.",
        _ => "BORROW: s3 presta a s2; el dueño sigue siendo s2.",
    };
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(caption)
            .small()
            .color(egui::Color32::from_rgb(140, 150, 165)),
    );
}

fn mostrar_tutorial_memoria(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.heading(
        egui::RichText::new("Gestión de Memoria: Stack vs Heap")
            .size(28.0)
            .strong(),
    );
    ui.add_space(10.0);
    ui.label("Presiona 'Ejecutar Siguiente Línea' para ver cómo el compilador asigna la memoria.");
    ui.add_space(20.0);

    ui.columns(2, |columns| {
        // --- COLUMNA 1: EDITOR DE CÓDIGO ---
        columns[0].group(|ui| {
            ui.heading("📝 Editor de Código");
            ui.add_space(15.0);

            let code = [
                "fn main() {",
                "    let a: i32 = 42;",
                "    let s = String::from(\"Hola\");",
                "} // Fin del Scope",
            ];

            for (i, line) in code.iter().enumerate() {
                let is_current = i == state.tutorial_step;
                let color = if is_current {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::LIGHT_GRAY
                };
                ui.label(
                    egui::RichText::new(*line)
                        .color(color)
                        .monospace()
                        .size(18.0),
                );
            }

            ui.add_space(30.0);
            if ui
                .button(egui::RichText::new("▶ Ejecutar Siguiente Línea").size(16.0))
                .clicked()
            {
                state.tutorial_step = (state.tutorial_step + 1) % 4;
            }
        });

        // --- COLUMNA 2: VISUALIZACIÓN DE MEMORIA (epaint) ---
        let (response, painter) = columns[1].allocate_painter(
            egui::vec2(columns[1].available_width(), 450.0),
            egui::Sense::hover(),
        );

        let rect = response.rect;
        let stack_rect =
            egui::Rect::from_min_size(rect.min + egui::vec2(10.0, 40.0), egui::vec2(160.0, 350.0));
        let heap_rect =
            egui::Rect::from_min_size(rect.min + egui::vec2(200.0, 40.0), egui::vec2(220.0, 350.0));

        painter.rect(
            stack_rect,
            5.0,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 250)),
            egui::StrokeKind::Middle,
        );
        painter.text(
            stack_rect.center_top() - egui::vec2(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "STACK",
            egui::FontId::proportional(18.0),
            egui::Color32::LIGHT_BLUE,
        );

        painter.rect(
            heap_rect,
            5.0,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 250, 100)),
            egui::StrokeKind::Middle,
        );
        painter.text(
            heap_rect.center_top() - egui::vec2(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "HEAP",
            egui::FontId::proportional(18.0),
            egui::Color32::LIGHT_GREEN,
        );

        let float_y = (state.tutorial_time * 3.0).sin() as f32 * 5.0;

        if state.tutorial_step >= 1 && state.tutorial_step < 3 {
            let var_a_rect = egui::Rect::from_min_size(
                stack_rect.min + egui::vec2(10.0, 290.0),
                egui::vec2(140.0, 40.0),
            );
            painter.rect(
                var_a_rect,
                4.0,
                egui::Color32::from_rgb(60, 60, 180),
                egui::Stroke::NONE,
                egui::StrokeKind::Middle,
            );
            painter.text(
                var_a_rect.center(),
                egui::Align2::CENTER_CENTER,
                "a: i32 = 42",
                egui::FontId::monospace(16.0),
                egui::Color32::WHITE,
            );
        }

        if state.tutorial_step >= 2 && state.tutorial_step < 3 {
            let var_s_rect = egui::Rect::from_min_size(
                stack_rect.min + egui::vec2(10.0, 200.0),
                egui::vec2(140.0, 70.0),
            );
            painter.rect(
                var_s_rect,
                4.0,
                egui::Color32::from_rgb(200, 150, 50),
                egui::Stroke::NONE,
                egui::StrokeKind::Middle,
            );
            painter.text(
                var_s_rect.center(),
                egui::Align2::CENTER_CENTER,
                "s (String)\nptr: 0x...",
                egui::FontId::monospace(14.0),
                egui::Color32::BLACK,
            );

            let heap_data_rect = egui::Rect::from_min_size(
                heap_rect.min + egui::vec2(30.0, 150.0 + float_y),
                egui::vec2(160.0, 50.0),
            );
            painter.rect(
                heap_data_rect,
                8.0,
                egui::Color32::from_rgb(50, 200, 50),
                egui::Stroke::NONE,
                egui::StrokeKind::Middle,
            );
            painter.text(
                heap_data_rect.center(),
                egui::Align2::CENTER_CENTER,
                "['H','o','l','a']",
                egui::FontId::monospace(16.0),
                egui::Color32::BLACK,
            );

            let start = var_s_rect.right_center();
            let end = heap_data_rect.left_center();
            let control1 = start + egui::vec2(50.0, 0.0);
            let control2 = end - egui::vec2(50.0, 0.0);

            painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
                [start, control1, control2, end],
                false,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(3.0, egui::Color32::YELLOW),
            ));
            painter.circle_filled(end, 6.0, egui::Color32::YELLOW);
        }
    });
}

fn mostrar_graficos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("📊 Dashboard Analítico (Estilo Plotly / Power BI)")
                .size(28.0)
                .strong()
                .color(egui::Color32::from_rgb(100, 200, 255)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("Visualización interactiva multimodelo con egui & egui_plot")
                .size(14.0)
                .italics(),
        );
    });

    ui.add_space(15.0);

    // Barra de Navegación de Pestañas (Tabs)
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.dash_tab, 0, "📊 Power BI Overview");
        ui.selectable_value(&mut state.dash_tab, 1, "🏎️ Bar Chart Race");
        ui.selectable_value(&mut state.dash_tab, 2, "🥧 Pie & Donut Chart");
        ui.selectable_value(&mut state.dash_tab, 3, "📈 Index Chart");
        ui.selectable_value(&mut state.dash_tab, 4, "🔀 Sankey Diagram");
        ui.selectable_value(&mut state.dash_tab, 5, "📉 Time Series Subplots");
    });

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(15.0);

    match state.dash_tab {
        0 => mostrar_dashboard_power_bi(ui, state),
        1 => mostrar_bar_chart_race(ui, state),
        2 => mostrar_pie_donut_chart(ui, state),
        3 => mostrar_index_chart(ui, state),
        4 => mostrar_sankey_diagram(ui, state),
        _ => mostrar_time_series_subplots(ui, state),
    }
}

fn mostrar_dashboard_power_bi(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let avail_w = ui.available_width();

    // 1. KPI Cards Row (Arriba)
    ui.horizontal(|ui| {
        let card_w = ((avail_w - 25.0) / 4.0).max(110.0);

        let kpis = [
            (
                "💰 Ingresos Totales",
                "$1,248,500",
                "+18.4% YoY",
                egui::Color32::from_rgb(60, 200, 120),
            ),
            (
                "📉 Gastos Operativos",
                "$684,200",
                "-4.2% Eficiencia",
                egui::Color32::from_rgb(240, 90, 90),
            ),
            (
                "📈 Margen Neto",
                "45.2%",
                "+5.1% YoY",
                egui::Color32::from_rgb(160, 100, 250),
            ),
            (
                "⚡ Throughput Rust",
                "14,250 ops/s",
                "Nativo",
                egui::Color32::from_rgb(255, 180, 50),
            ),
        ];

        for (title, value, badge, color) in kpis.iter() {
            let mut frame = egui::Frame::new();
            frame.fill = egui::Color32::from_rgb(22, 24, 30);
            frame.inner_margin = egui::Margin::same(10);
            frame.corner_radius = egui::CornerRadius::same(8);
            frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 50, 65));
            frame.show(ui, |ui| {
                ui.set_width(card_w - 20.0);
                ui.label(
                    egui::RichText::new(*title)
                        .size(12.0)
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(4.0);
                ui.heading(
                    egui::RichText::new(*value)
                        .size(17.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(*badge)
                        .size(11.0)
                        .strong()
                        .color(*color),
                );
            });
        }
    });

    ui.add_space(15.0);

    // 2. Barra Horizontal de Filtros Power BI (Slicers)
    let mut slicer_frame = egui::Frame::new();
    slicer_frame.fill = egui::Color32::from_rgb(25, 27, 35);
    slicer_frame.inner_margin = egui::Margin::same(12);
    slicer_frame.corner_radius = egui::CornerRadius::same(8);
    slicer_frame.show(ui, |ui| {
        ui.set_width(avail_w - 24.0);
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new("🎛️ Filtros Power BI:")
                    .size(15.0)
                    .color(egui::Color32::from_rgb(100, 200, 255)),
            );
            ui.add_space(15.0);

            ui.checkbox(&mut state.show_ingresos, "Ingresos");
            ui.add_space(10.0);
            ui.checkbox(&mut state.show_gastos, "Gastos");
            ui.add_space(10.0);
            ui.checkbox(&mut state.show_beneficios, "Beneficios");
            ui.add_space(25.0);

            ui.label(egui::RichText::new("Año Fiscal:").strong());
            ui.add(egui::Slider::new(&mut state.year, 2020..=2026));
        });
    });

    ui.add_space(15.0);

    // 3. Gráfico Principal (Abajo a todo lo ancho)
    let mut plot_frame = egui::Frame::new();
    plot_frame.fill = egui::Color32::from_rgb(20, 20, 25);
    plot_frame.inner_margin = egui::Margin::same(12);
    plot_frame.corner_radius = egui::CornerRadius::same(8);
    plot_frame.show(ui, |ui| {
        ui.set_width(avail_w - 24.0);
        let base_multiplier = (state.year - 2020) as f64 * 80.0;

        let mut ingresos_bars = vec![];
        let mut gastos_bars = vec![];
        let mut beneficios_points = vec![];

        for i in 1..=12 {
            let x = i as f64;
            let ingreso = base_multiplier + 200.0 + (i as f64 * 35.0) + (i % 3) as f64 * 40.0;
            let gasto = base_multiplier + 140.0 + (i as f64 * 20.0) + (i % 2) as f64 * 25.0;
            let beneficio = ingreso - gasto;

            ingresos_bars.push(
                Bar::new(x - 0.2, ingreso)
                    .width(0.35)
                    .fill(egui::Color32::from_rgb(60, 200, 120)),
            );
            gastos_bars.push(
                Bar::new(x + 0.2, gasto)
                    .width(0.35)
                    .fill(egui::Color32::from_rgb(230, 90, 90)),
            );
            beneficios_points.push([x, beneficio]);
        }

        Plot::new("power_bi_plot")
            .legend(Legend::default().position(Corner::RightBottom))
            .height(380.0)
            .show_grid([true, true])
            .show(ui, |plot_ui| {
                if state.show_ingresos {
                    plot_ui.bar_chart(BarChart::new("Ingresos", ingresos_bars));
                }
                if state.show_gastos {
                    plot_ui.bar_chart(BarChart::new("Gastos", gastos_bars));
                }
                if state.show_beneficios {
                    plot_ui.line(
                        Line::new("Tendencia Beneficios", PlotPoints::new(beneficios_points))
                            .color(egui::Color32::from_rgb(160, 100, 250))
                            .width(3.0),
                    );
                }
            });
    });
}

fn mostrar_bar_chart_race(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.horizontal(|ui| {
        if ui
            .button(if state.bcr_playing {
                "⏸ Pausar"
            } else {
                "▶ Iniciar Carrera"
            })
            .clicked()
        {
            state.bcr_playing = !state.bcr_playing;
        }
        if ui.button("🔄 Reiniciar (2015)").clicked() {
            state.bcr_year = 2015.0;
        }
        ui.add_space(10.0);
        ui.label("Velocidad:");
        ui.selectable_value(&mut state.bcr_speed, 0.5, "0.5x");
        ui.selectable_value(&mut state.bcr_speed, 1.0, "1x");
        ui.selectable_value(&mut state.bcr_speed, 2.0, "2x");
        ui.selectable_value(&mut state.bcr_speed, 4.0, "4x");

        ui.add_space(20.0);
        ui.add(egui::Slider::new(&mut state.bcr_year, 2015.0..=2026.0).text("Año"));
    });

    if state.bcr_playing {
        let dt = ui.input(|i| i.stable_dt);
        state.bcr_year += dt * state.bcr_speed * 1.5;
        if state.bcr_year > 2026.0 {
            state.bcr_year = 2015.0;
        }
        ui.ctx().request_repaint();
    }

    ui.add_space(15.0);

    let languages = [
        (
            "🦀 Rust",
            egui::Color32::from_rgb(240, 100, 40),
            [
                (2015.0, 8.0),
                (2018.0, 22.0),
                (2021.0, 52.0),
                (2024.0, 85.0),
                (2026.0, 99.0),
            ],
        ),
        (
            "🐍 Python",
            egui::Color32::from_rgb(60, 140, 230),
            [
                (2015.0, 65.0),
                (2018.0, 78.0),
                (2021.0, 91.0),
                (2024.0, 97.0),
                (2026.0, 100.0),
            ],
        ),
        (
            "🟨 JavaScript",
            egui::Color32::from_rgb(240, 210, 50),
            [
                (2015.0, 88.0),
                (2018.0, 92.0),
                (2021.0, 94.0),
                (2024.0, 95.0),
                (2026.0, 96.0),
            ],
        ),
        (
            "🔷 TypeScript",
            egui::Color32::from_rgb(40, 160, 240),
            [
                (2015.0, 12.0),
                (2018.0, 42.0),
                (2021.0, 72.0),
                (2024.0, 89.0),
                (2026.0, 94.0),
            ],
        ),
        (
            "🐹 Go",
            egui::Color32::from_rgb(50, 210, 210),
            [
                (2015.0, 20.0),
                (2018.0, 48.0),
                (2021.0, 68.0),
                (2024.0, 80.0),
                (2026.0, 87.0),
            ],
        ),
        (
            "⚡ C++",
            egui::Color32::from_rgb(100, 120, 200),
            [
                (2015.0, 75.0),
                (2018.0, 72.0),
                (2021.0, 70.0),
                (2024.0, 72.0),
                (2026.0, 74.0),
            ],
        ),
        (
            "☕ Java",
            egui::Color32::from_rgb(220, 70, 70),
            [
                (2015.0, 82.0),
                (2018.0, 78.0),
                (2021.0, 72.0),
                (2024.0, 66.0),
                (2026.0, 62.0),
            ],
        ),
    ];

    let yr = state.bcr_year;

    let mut current_data: Vec<(&str, egui::Color32, f32)> = languages
        .iter()
        .map(|(name, color, points)| {
            let val = if yr <= points[0].0 {
                points[0].1
            } else if yr >= points[points.len() - 1].0 {
                points[points.len() - 1].1
            } else {
                let mut v = points[0].1;
                for idx in 0..points.len() - 1 {
                    if yr >= points[idx].0 && yr <= points[idx + 1].0 {
                        let t = (yr - points[idx].0) / (points[idx + 1].0 - points[idx].0);
                        v = points[idx].1 + t * (points[idx + 1].1 - points[idx].1);
                        break;
                    }
                }
                v
            };
            (*name, *color, val)
        })
        .collect();

    current_data.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let available_w = ui.available_width();
    let (_, rect) = ui.allocate_space(egui::vec2(available_w, 400.0));

    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    ui.painter().text(
        rect.right_bottom() - egui::vec2(30.0, 30.0),
        egui::Align2::RIGHT_BOTTOM,
        format!("{:.0}", yr),
        egui::FontId::proportional(80.0),
        egui::Color32::from_black_alpha(80),
    );

    let max_val = 105.0;
    let bar_height = 36.0;
    let gap = 14.0;
    let start_y = rect.top() + 20.0;
    let start_x = rect.left() + 160.0;
    let max_bar_width = rect.width() - 250.0;

    for (rank, (name, color, val)) in current_data.iter().enumerate() {
        let bar_y = start_y + rank as f32 * (bar_height + gap);
        let bar_w = (val / max_val) * max_bar_width;

        ui.painter().text(
            egui::pos2(start_x - 15.0, bar_y + bar_height / 2.0),
            egui::Align2::RIGHT_CENTER,
            *name,
            egui::FontId::proportional(15.0).clone(),
            egui::Color32::WHITE,
        );

        let bar_rect = egui::Rect::from_min_size(
            egui::pos2(start_x, bar_y),
            egui::vec2(bar_w.max(10.0), bar_height),
        );
        ui.painter()
            .rect_filled(bar_rect, egui::CornerRadius::same(5), *color);

        ui.painter().text(
            egui::pos2(start_x + bar_w + 12.0, bar_y + bar_height / 2.0),
            egui::Align2::LEFT_CENTER,
            format!("{:.1}%", val),
            egui::FontId::proportional(14.0).clone(),
            egui::Color32::from_rgb(220, 220, 220),
        );
    }
}

fn mostrar_pie_donut_chart(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Radio Rosquilla (Donut):").strong());
        ui.add(egui::Slider::new(&mut state.pie_donut_hole, 0.0..=0.7).text("Hole"));
        ui.add_space(20.0);
        ui.checkbox(
            &mut state.pie_exploded,
            "Explotar Rebanadas al Pasar el Mouse",
        );
    });

    ui.add_space(15.0);

    let slices = [
        (
            "☁️ Infraestructura Cloud",
            1897000.0,
            egui::Color32::from_rgb(60, 140, 240),
        ),
        (
            "🦀 I+D & Rust Core",
            1355000.0,
            egui::Color32::from_rgb(240, 100, 40),
        ),
        (
            "🛡️ Ciberseguridad",
            975000.0,
            egui::Color32::from_rgb(160, 80, 220),
        ),
        (
            "🔑 Licencias & SaaS",
            650000.0,
            egui::Color32::from_rgb(40, 190, 110),
        ),
        (
            "🎓 Capacitación & Equipo",
            542000.0,
            egui::Color32::from_rgb(240, 190, 50),
        ),
    ];

    let total: f64 = slices.iter().map(|s| s.1).sum();

    ui.horizontal(|ui| {
        let (_, rect) = ui.allocate_space(egui::vec2(400.0, 380.0));

        ui.painter()
            .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

        let center = rect.center();
        let r_out = 140.0;
        let r_in = r_out * state.pie_donut_hole;

        let pointer_pos = ui.input(|i| i.pointer.hover_pos());
        let mut current_angle: f32 = -std::f32::consts::FRAC_PI_2;
        let mut hovered_slice: Option<usize> = None;

        for (idx, (name, val, color)) in slices.iter().enumerate() {
            let slice_angle = ((*val / total) as f32) * std::f32::consts::TAU;
            let a1 = current_angle;
            let a2 = current_angle + slice_angle;
            let mid_angle = (a1 + a2) / 2.0;

            let is_hovered = if let Some(p) = pointer_pos {
                let dist = p.distance(center);
                if dist >= r_in && dist <= r_out + 25.0 {
                    let mut click_angle = (p.y - center.y).atan2(p.x - center.x);
                    if click_angle < a1 {
                        click_angle += std::f32::consts::TAU;
                    }
                    click_angle >= a1 && click_angle <= a2
                } else {
                    false
                }
            } else {
                false
            };

            if is_hovered {
                hovered_slice = Some(idx);
            }

            let explode_offset = if is_hovered && state.pie_exploded {
                18.0
            } else {
                0.0
            };
            let slice_center = center
                + egui::vec2(
                    mid_angle.cos() * explode_offset,
                    mid_angle.sin() * explode_offset,
                );

            let steps = (((a2 - a1).abs() / 0.08).ceil() as usize).max(8);

            let mut outer_pts = vec![];
            let mut inner_pts = vec![];

            for step in 0..=steps {
                let t = step as f32 / steps as f32;
                let sub_a = a1 + (a2 - a1) * t;
                outer_pts.push(slice_center + egui::vec2(sub_a.cos() * r_out, sub_a.sin() * r_out));
                inner_pts.push(slice_center + egui::vec2(sub_a.cos() * r_in, sub_a.sin() * r_in));
            }

            for step in 0..steps {
                let v1 = inner_pts[step];
                let v2 = outer_pts[step];
                let v3 = outer_pts[step + 1];
                let v4 = inner_pts[step + 1];

                ui.painter().add(egui::Shape::convex_polygon(
                    vec![v1, v2, v3, v4],
                    *color,
                    egui::Stroke::NONE,
                ));
            }

            let border_stroke = if is_hovered {
                egui::Stroke::new(2.5, egui::Color32::WHITE)
            } else {
                egui::Stroke::new(1.2, egui::Color32::from_rgb(20, 22, 28))
            };

            let mut boundary = vec![];
            boundary.extend_from_slice(&outer_pts);
            for p in inner_pts.iter().rev() {
                boundary.push(*p);
            }
            ui.painter()
                .add(egui::Shape::closed_line(boundary, border_stroke));

            let pct = (*val / total) * 100.0;
            let p_edge =
                slice_center + egui::vec2(mid_angle.cos() * r_out, mid_angle.sin() * r_out);
            let p_elbow = slice_center
                + egui::vec2(
                    mid_angle.cos() * (r_out + 20.0),
                    mid_angle.sin() * (r_out + 20.0),
                );
            let is_right = mid_angle.cos() >= 0.0;
            let p_text = p_elbow + egui::vec2(if is_right { 18.0 } else { -18.0 }, 0.0);

            let leader_color = if is_hovered {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 180)
            };

            ui.painter()
                .line_segment([p_edge, p_elbow], egui::Stroke::new(1.2, leader_color));
            ui.painter()
                .line_segment([p_elbow, p_text], egui::Stroke::new(1.2, leader_color));
            ui.painter().circle_filled(p_edge, 2.5, leader_color);

            ui.painter().text(
                p_text + egui::vec2(if is_right { 4.0 } else { -4.0 }, 0.0),
                if is_right {
                    egui::Align2::LEFT_CENTER
                } else {
                    egui::Align2::RIGHT_CENTER
                },
                format!("{}: {:.1}%", name.split(' ').next().unwrap_or(name), pct),
                egui::FontId::proportional(11.0),
                if is_hovered {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::LIGHT_GRAY
                },
            );

            current_angle = a2;
        }

        if r_in > 30.0 {
            ui.painter()
                .circle_filled(center, r_in - 2.0, egui::Color32::from_rgb(20, 22, 28));
            ui.painter().text(
                center - egui::vec2(0.0, 10.0),
                egui::Align2::CENTER_CENTER,
                "Total Presupuesto",
                egui::FontId::proportional(12.0),
                egui::Color32::GRAY,
            );
            ui.painter().text(
                center + egui::vec2(0.0, 10.0),
                egui::Align2::CENTER_CENTER,
                format!("${:.2}M", total / 1_000_000.0),
                egui::FontId::proportional(18.0),
                egui::Color32::WHITE,
            );
        }

        ui.add_space(20.0);

        ui.vertical(|ui| {
            ui.heading("Desglose del Presupuesto");
            ui.add_space(10.0);
            for (idx, (name, val, color)) in slices.iter().enumerate() {
                let pct = (*val / total) * 100.0;
                let is_sel = hovered_slice == Some(idx);
                let mut text =
                    egui::RichText::new(format!("{}: {:.1}% (${:.0}k)", name, pct, val / 1000.0))
                        .size(14.0)
                        .color(if is_sel {
                            egui::Color32::WHITE
                        } else {
                            egui::Color32::LIGHT_GRAY
                        });
                if is_sel {
                    text = text.strong();
                }
                ui.horizontal(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 3.0, *color);
                    ui.label(text);
                });
                ui.add_space(8.0);
            }
        });
    });
}

fn mostrar_index_chart(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("🎯 Año Base (Punto 100%):").strong());
        ui.add(egui::Slider::new(&mut state.index_baseline_year, 2015.0..=2025.0).text("Año Base"));
        ui.label(
            egui::RichText::new("*(Todos los gráficos se re-escalan a 100% en esta fecha)*")
                .italics()
                .color(egui::Color32::GRAY),
        );
    });

    ui.add_space(15.0);

    let baseline = state.index_baseline_year;

    let series_raw = [
        (
            "🦀 Rust Repositories",
            egui::Color32::from_rgb(240, 100, 40),
            vec![
                (2015.0, 50.0),
                (2017.0, 180.0),
                (2019.0, 450.0),
                (2021.0, 1200.0),
                (2023.0, 3100.0),
                (2026.0, 7800.0),
            ],
        ),
        (
            "🐍 Python AI Packages",
            egui::Color32::from_rgb(60, 140, 230),
            vec![
                (2015.0, 2000.0),
                (2017.0, 4500.0),
                (2019.0, 9800.0),
                (2021.0, 18000.0),
                (2023.0, 32000.0),
                (2026.0, 65000.0),
            ],
        ),
        (
            "🟨 JS/TS Web Frameworks",
            egui::Color32::from_rgb(240, 210, 50),
            vec![
                (2015.0, 5000.0),
                (2017.0, 8500.0),
                (2019.0, 14000.0),
                (2021.0, 21000.0),
                (2023.0, 29000.0),
                (2026.0, 38000.0),
            ],
        ),
        (
            "🐹 Go Microservices",
            egui::Color32::from_rgb(50, 210, 210),
            vec![
                (2015.0, 300.0),
                (2017.0, 900.0),
                (2019.0, 2200.0),
                (2021.0, 5100.0),
                (2023.0, 11000.0),
                (2026.0, 22000.0),
            ],
        ),
    ];

    let mut indexed_lines = vec![];

    for (name, color, points) in series_raw.iter() {
        let base_val = if baseline <= points[0].0 {
            points[0].1
        } else if baseline >= points[points.len() - 1].0 {
            points[points.len() - 1].1
        } else {
            let mut v = points[0].1;
            for i in 0..points.len() - 1 {
                if baseline >= points[i].0 && baseline <= points[i + 1].0 {
                    let t = (baseline - points[i].0) / (points[i + 1].0 - points[i].0);
                    v = points[i].1 + t * (points[i + 1].1 - points[i].1);
                    break;
                }
            }
            v
        };

        let norm_pts: Vec<[f64; 2]> = points
            .iter()
            .map(|(x, y)| [*x as f64, (100.0 * (y / base_val)) as f64])
            .collect();

        indexed_lines.push(
            Line::new(*name, PlotPoints::new(norm_pts))
                .color(*color)
                .width(2.5),
        );
    }

    Plot::new("index_chart_plot")
        .legend(Legend::default().position(Corner::LeftTop))
        .height(380.0)
        .show_grid([true, true])
        .show(ui, |plot_ui| {
            plot_ui.hline(
                HLine::new("Base 100%", 100.0)
                    .color(egui::Color32::WHITE)
                    .width(1.5),
            );
            plot_ui.vline(
                VLine::new("Año Base", baseline as f64)
                    .color(egui::Color32::LIGHT_YELLOW)
                    .width(1.5),
            );

            for line in indexed_lines {
                plot_ui.line(line);
            }
        });
}

fn mostrar_sankey_diagram(ui: &mut egui::Ui, _state: &mut PortfolioState) {
    ui.label(
        egui::RichText::new("🔀 Diagrama de Flujo de Recursos y Asignación Financiera")
            .size(15.0)
            .italics(),
    );
    ui.add_space(10.0);

    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 420.0));
    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    let sources = [
        (
            "Ventas SaaS Directas",
            1200.0,
            egui::Color32::from_rgb(60, 200, 120),
        ),
        (
            "Licencias Enterprise",
            600.0,
            egui::Color32::from_rgb(80, 150, 240),
        ),
        (
            "Servicios Consultoría",
            200.0,
            egui::Color32::from_rgb(180, 100, 240),
        ),
    ];

    let hub = (
        "Ingresos Totales Brutos",
        2000.0,
        egui::Color32::from_rgb(240, 180, 50),
    );

    let targets = [
        (
            "Desarrollo Core Rust",
            800.0,
            egui::Color32::from_rgb(240, 100, 40),
        ),
        (
            "Infraestructura Cloud",
            500.0,
            egui::Color32::from_rgb(60, 160, 240),
        ),
        (
            "Marketing & Ventas",
            400.0,
            egui::Color32::from_rgb(240, 140, 180),
        ),
        (
            "Ganancia Neta Límpida",
            300.0,
            egui::Color32::from_rgb(40, 200, 100),
        ),
    ];

    let col1_x = rect.left() + 40.0;
    let col2_x = rect.center().x - 60.0;
    let col3_x = rect.right() - 200.0;
    let node_w = 160.0;

    let mut src_ports = vec![];
    let mut curr_y = rect.top() + 40.0;
    for (name, val, color) in sources.iter() {
        let h = (val / 2000.0) * 320.0;
        let n_rect = egui::Rect::from_min_size(egui::pos2(col1_x, curr_y), egui::vec2(node_w, h));
        ui.painter().rect_filled(n_rect, 5.0, *color);
        ui.painter().text(
            n_rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{}\n${:.0}k", name, val),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        src_ports.push((egui::pos2(n_rect.right(), n_rect.center().y), h, *color));
        curr_y += h + 20.0;
    }

    let hub_h = 320.0;
    let hub_y = rect.top() + 40.0;
    let hub_rect = egui::Rect::from_min_size(egui::pos2(col2_x, hub_y), egui::vec2(node_w, hub_h));
    ui.painter().rect_filled(hub_rect, 5.0, hub.2);
    ui.painter().text(
        hub_rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{}\n${:.0}k", hub.0, hub.1),
        egui::FontId::proportional(13.0),
        egui::Color32::WHITE,
    );

    let mut tgt_ports = vec![];
    curr_y = rect.top() + 40.0;
    for (name, val, color) in targets.iter() {
        let h = (val / 2000.0) * 320.0;
        let n_rect = egui::Rect::from_min_size(egui::pos2(col3_x, curr_y), egui::vec2(node_w, h));
        ui.painter().rect_filled(n_rect, 5.0, *color);
        ui.painter().text(
            n_rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{}\n${:.0}k", name, val),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        tgt_ports.push((egui::pos2(n_rect.left(), n_rect.center().y), h, *color));
        curr_y += h + 15.0;
    }

    for (port, h, color) in src_ports.iter() {
        let p1 = *port;
        let p2 = egui::pos2(hub_rect.left(), p1.y);

        let steps = 20;
        let mut top_pts = vec![];
        let mut bot_pts = vec![];

        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let x = p1.x + t * (p2.x - p1.x);
            let s_t = 3.0 * t * t - 2.0 * t * t * t;
            let y = p1.y + s_t * (p2.y - p1.y);
            top_pts.push(egui::pos2(x, y - h / 2.0));
            bot_pts.push(egui::pos2(x, y + h / 2.0));
        }

        let ribbon_color = egui::Color32::from_rgba_premultiplied(
            color.r() / 2,
            color.g() / 2,
            color.b() / 2,
            100,
        );

        for step in 0..steps {
            let quad = vec![
                top_pts[step],
                top_pts[step + 1],
                bot_pts[step + 1],
                bot_pts[step],
            ];
            ui.painter().add(egui::Shape::convex_polygon(
                quad,
                ribbon_color,
                egui::Stroke::NONE,
            ));
        }
    }

    for (port, h, color) in tgt_ports.iter() {
        let p1 = egui::pos2(hub_rect.right(), port.y);
        let p2 = *port;

        let steps = 20;
        let mut top_pts = vec![];
        let mut bot_pts = vec![];

        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let x = p1.x + t * (p2.x - p1.x);
            let s_t = 3.0 * t * t - 2.0 * t * t * t;
            let y = p1.y + s_t * (p2.y - p1.y);
            top_pts.push(egui::pos2(x, y - h / 2.0));
            bot_pts.push(egui::pos2(x, y + h / 2.0));
        }

        let ribbon_color = egui::Color32::from_rgba_premultiplied(
            color.r() / 2,
            color.g() / 2,
            color.b() / 2,
            100,
        );

        for step in 0..steps {
            let quad = vec![
                top_pts[step],
                top_pts[step + 1],
                bot_pts[step + 1],
                bot_pts[step],
            ];
            ui.painter().add(egui::Shape::convex_polygon(
                quad,
                ribbon_color,
                egui::Stroke::NONE,
            ));
        }
    }
}

fn mostrar_time_series_subplots(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.horizontal(|ui| {
        ui.checkbox(&mut state.ts_show_ma, "Medias Móviles (SMA 20/50)");
        ui.checkbox(&mut state.ts_show_volume, "Volumen");
        ui.checkbox(&mut state.ts_show_rsi, "RSI Indicator");
    });

    ui.add_space(10.0);

    let days = 100;
    let mut price_pts = vec![];
    let mut sma20_pts = vec![];
    let mut sma50_pts = vec![];
    let mut bollinger_upper = vec![];
    let mut bollinger_lower = vec![];
    let mut volume_bars = vec![];
    let mut rsi_pts = vec![];

    let mut curr_price = 150.0;

    for d in 1..=days {
        let x = d as f64;
        let change = ((d % 7) as f64 - 3.2) * 2.5 + ((d % 3) as f64 - 1.0) * 1.5;
        curr_price = (curr_price + change).max(50.0);

        price_pts.push([x, curr_price]);

        let sma20 = curr_price * 0.95 + (d as f64 * 0.1);
        let sma50 = curr_price * 0.90 + (d as f64 * 0.15);
        let b_upper = sma20 + 15.0;
        let b_lower = sma20 - 15.0;

        sma20_pts.push([x, sma20]);
        sma50_pts.push([x, sma50]);
        bollinger_upper.push([x, b_upper]);
        bollinger_lower.push([x, b_lower]);

        let vol = 1000.0 + ((d % 5) as f64 * 400.0) + (change.abs() * 200.0);
        let vol_color = if change >= 0.0 {
            egui::Color32::from_rgb(60, 200, 120)
        } else {
            egui::Color32::from_rgb(230, 80, 80)
        };
        volume_bars.push(Bar::new(x, vol).width(0.7).fill(vol_color));

        let rsi = 50.0 + (change * 5.0) + ((d % 4) as f64 * 4.0);
        rsi_pts.push([x, rsi.clamp(10.0, 90.0)]);
    }

    ui.label(egui::RichText::new("📈 Panel 1: Precio de Cotización & Indicadores").strong());
    Plot::new("subplot_price")
        .height(200.0)
        .legend(Legend::default().position(Corner::LeftTop))
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Precio", PlotPoints::new(price_pts))
                    .color(egui::Color32::WHITE)
                    .width(2.0),
            );
            if state.ts_show_ma {
                plot_ui.line(
                    Line::new("SMA 20", PlotPoints::new(sma20_pts))
                        .color(egui::Color32::from_rgb(240, 180, 50))
                        .width(1.5),
                );
                plot_ui.line(
                    Line::new("SMA 50", PlotPoints::new(sma50_pts))
                        .color(egui::Color32::from_rgb(60, 160, 240))
                        .width(1.5),
                );
                plot_ui.line(
                    Line::new("Bollinger Sup", PlotPoints::new(bollinger_upper))
                        .color(egui::Color32::GRAY)
                        .width(1.0),
                );
                plot_ui.line(
                    Line::new("Bollinger Inf", PlotPoints::new(bollinger_lower))
                        .color(egui::Color32::GRAY)
                        .width(1.0),
                );
            }
        });

    ui.add_space(10.0);

    if state.ts_show_volume {
        ui.label(egui::RichText::new("📊 Panel 2: Volumen Operado Diario").strong());
        Plot::new("subplot_volume")
            .height(100.0)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(BarChart::new("Volumen", volume_bars));
            });
        ui.add_space(10.0);
    }

    if state.ts_show_rsi {
        ui.label(egui::RichText::new("📉 Panel 3: Índice de Fuerza Relativa (RSI)").strong());
        Plot::new("subplot_rsi").height(100.0).show(ui, |plot_ui| {
            plot_ui.hline(
                HLine::new("Límite Sobrecompra (70)", 70.0)
                    .color(egui::Color32::RED)
                    .width(1.0),
            );
            plot_ui.hline(
                HLine::new("Límite Sobrevendido (30)", 30.0)
                    .color(egui::Color32::GREEN)
                    .width(1.0),
            );
            plot_ui.line(
                Line::new("RSI (14)", PlotPoints::new(rsi_pts))
                    .color(egui::Color32::from_rgb(180, 100, 240))
                    .width(1.5),
            );
        });
    }
}

fn mostrar_editor_interactivo<F>(
    ui: &mut egui::Ui,
    code: &mut String,
    output_mutex: Arc<Mutex<String>>,
    btn_text: &str,
    execute_fn: F,
    syntax_set: &SyntaxSet,
    theme: &syntect::highlighting::Theme,
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

fn mostrar_tutorial_control_flujo(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.heading(
        egui::RichText::new("Control de Flujo")
            .size(22.0)
            .strong()
            .color(egui::Color32::WHITE),
    );
    ui.add_space(10.0);

    // Selector de pestañas interactivas de Control de Flujo
    ui.horizontal(|ui| {
        if ui
            .selectable_label(
                state.controlflujo_tab == 0,
                egui::RichText::new("Condicionales").strong(),
            )
            .clicked()
        {
            state.controlflujo_tab = 0;
        }
        if ui
            .selectable_label(
                state.controlflujo_tab == 1,
                egui::RichText::new("Bucles").strong(),
            )
            .clicked()
        {
            state.controlflujo_tab = 1;
        }
        if ui
            .selectable_label(
                state.controlflujo_tab == 2,
                egui::RichText::new("Match").strong(),
            )
            .clicked()
        {
            state.controlflujo_tab = 2;
        }
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(15.0);

    match state.controlflujo_tab {
        0 => {
            ui.label(
                "En Rust, 'if' no es solo una declaración de control, sino una expresión que devuelve un valor. Esto permite asignar el resultado de una condición directamente a una variable.",
            );
            ui.add_space(10.0);

            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_if_else_rust")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Estructura").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("if / else").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("if c { a } else { b }").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Evalúa una condición booleana; ambos bloques deben retornar el mismo tipo.");
                        ui.end_row();

                        ui.label(egui::RichText::new("else if").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("if c1 { } else if c2 { }").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Permite encadenar múltiples evaluaciones de forma secuencial.");
                        ui.end_row();
                    });
            });
        }
        1 => {
            ui.label(
                "Rust ofrece 3 construcciones para bucles: 'loop' para repetición infinita o con retorno de valor, 'while' para ejecución condicional y 'for' para iterar de forma segura sobre rangos y colecciones.",
            );
            ui.add_space(10.0);

            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_bucles_rust")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Bucle")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Ejemplo de Sintaxis")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Descripción")
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("loop")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label(
                            egui::RichText::new("loop { break valor; }")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label(
                            "Bucle infinito. Permite retornar un valor mediante 'break valor;'.",
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("while")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label(
                            egui::RichText::new("while condicion { ... }")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label("Se ejecuta repetidamente mientras la condición sea 'true'.");
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("for")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(255, 160, 50)),
                        );
                        ui.label(
                            egui::RichText::new("for i in 1..=5 { ... }")
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                        ui.label("Itera sobre un rango o colección de elementos de forma segura.");
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.label(
                "La sentencia 'match' permite comparar un valor contra una serie de patrones y ejecutar código basado en el primer patrón que coincida. El compilador de Rust exige exhaustividad total.",
            );
            ui.add_space(10.0);

            let mut table_frame = egui::Frame::new();
            table_frame.fill = egui::Color32::from_rgb(14, 18, 26);
            table_frame.inner_margin = egui::Margin::same(12);
            table_frame.corner_radius = egui::CornerRadius::same(8);
            table_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));

            table_frame.show(ui, |ui| {
                egui::Grid::new("tabla_match_rust")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Patrón").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo de Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Valor Literal").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("1 => println!(\"Uno\"),").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Coincidencia exacta con un valor explícito.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Rangos").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("1..=5 => println!(\"1 a 5\"),").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Coincidencia con cualquier número dentro del rango.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Comodín _").monospace().strong().color(egui::Color32::from_rgb(255, 160, 50)));
                        ui.label(egui::RichText::new("_ => println!(\"Cualquier otro\"),").monospace().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Captura cualquier caso no especificado previamente (obligatorio para cumplir exhaustividad).");
                        ui.end_row();
                    });
            });
        }
    }

    ui.add_space(15.0);

    mostrar_selector_proyectos_estandar(
        ui,
        &mut state.selected_project,
        &mut state.term_cwd,
        "combo_proyectos_control_flujo",
        &mut state.controlflujo_code,
    );

    ui.add_space(10.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.controlflujo_code,
        Arc::clone(&state.controlflujo_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

fn card_frame_tutorial() -> egui::Frame {
    let mut f = egui::Frame::new();
    f.fill = egui::Color32::from_rgb(14, 18, 26);
    f.inner_margin = egui::Margin::same(12);
    f.corner_radius = egui::CornerRadius::same(8);
    f.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 90));
    f
}

fn mostrar_tutorial_funciones(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Funciones & Closures")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    // Tabs: mismo patrón que Comenzando / Control de Flujo
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Práctica:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );

        let tabs_izq = [(0, "fn y parámetros"), (1, "Retorno"), (2, "Closures")];
        for (indice, label) in tabs_izq {
            let activo = state.funciones_tab == indice;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.funciones_tab = indice;
            }
            ui.add_space(4.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.funciones_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("Call Stack")
                            .strong()
                            .color(color),
                    )
                    .frame(activo),
                )
                .clicked()
            {
                state.funciones_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Visual:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.funciones_tab {
        0 => {
            ui.label(
                egui::RichText::new(
                    "Una función empaqueta lógica reutilizable. En Rust la firma declara tipos de \
                     parámetros y (si devuelve algo) el tipo de retorno. El cuerpo es un bloque `{}`.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_fn_params")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Pieza").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Declaración").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("fn nombre(args) { ... }").monospace().color(cyan));
                        ui.label("Define la función. Los tipos de cada parámetro son obligatorios.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Parámetro").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("x: i32").monospace().color(cyan));
                        ui.label("Nombre + tipo. Se pasan por valor salvo que uses referencias.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Referencia").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("val: &mut i32").monospace().color(cyan));
                        ui.label("Presta el valor sin moverlo; &mut permite modificarlo.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Llamada").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("calcular_suma(a, b)").monospace().color(cyan));
                        ui.label("Ejecuta la función y (si hay retorno) produce un valor.");
                        ui.end_row();
                    });
            });
        }
        1 => {
            ui.label(
                egui::RichText::new(
                    "En Rust casi todo es una expresión. La última línea de un bloque sin `;` es el \
                     valor que devuelve. `return` existe, pero el estilo idiomático es el retorno implícito.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_fn_retorno")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Forma").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Notas").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Firma -> T").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("fn f(x: i32) -> i32").monospace().color(cyan));
                        ui.label("Obligatorio si la función devuelve un valor distinto de ().");
                        ui.end_row();

                        ui.label(egui::RichText::new("Implícito").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("x + y   // sin ';'").monospace().color(cyan));
                        ui.label("Última expresión del bloque = valor de retorno.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Con ;").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("x + y;  // devuelve ()").monospace().color(cyan));
                        ui.label("El `;` convierte la expresión en declaración → no hay valor.");
                        ui.end_row();

                        ui.label(egui::RichText::new("return").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("return x + y;").monospace().color(cyan));
                        ui.label("Salida temprana; útil en ramas, no obligatorio al final.");
                        ui.end_row();
                    });
            });
        }
        2 => {
            ui.label(
                egui::RichText::new(
                    "Un closure es una función anónima que puede capturar variables del entorno. \
                     Se escribe con `|params| cuerpo` y es la base de iteradores (`.map`, `.filter`).",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_closures")
                    .striped(true)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Idea").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Sintaxis").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Descripción").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Básico").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("|x| x * 2").monospace().color(cyan));
                        ui.label("Un parámetro; el tipo suele inferirse del uso.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Tipado").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("|x: i32| -> i32 { x + 1 }").monospace().color(cyan));
                        ui.label("Puedes anotar tipos y usar bloque `{}` si hay varias líneas.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Captura").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("let f = |x| x * factor;").monospace().color(cyan));
                        ui.label("`factor` vive fuera: el closure la toma prestada o la mueve.");
                        ui.end_row();

                        ui.label(egui::RichText::new("fn vs closure").monospace().strong().color(naranja));
                        ui.label(egui::RichText::new("fn f(x: i32) vs |x|").monospace().color(cyan));
                        ui.label("`fn` no captura el entorno; el closure sí (Fn / FnMut / FnOnce).");
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.label(
                egui::RichText::new(
                    "Cada llamada apila un frame (variables locales + punto de retorno). Al terminar, \
                     el frame se desapila y el valor vuelve al llamador — aquí con retorno implícito.",
                )
                .color(texto),
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Paso:").strong().color(gris_tab));
                ui.add_space(6.0);
                for (i, label) in [
                    (0, "1. main()"),
                    (1, "2. Apilar suma"),
                    (2, "3. Retorno 40"),
                ] {
                    let activo = state.funciones_step == i;
                    let color = if activo { naranja } else { gris_tab };
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(label).strong().color(color))
                                .frame(activo),
                        )
                        .clicked()
                    {
                        state.funciones_step = i;
                    }
                    ui.add_space(4.0);
                }
            });

            ui.add_space(12.0);
            mostrar_simulador_call_stack(ui, state.funciones_step);
        }
    }

    // Editor en todas las pestañas de práctica (0–2); también útil tras ver el stack
    if state.funciones_tab < 3 {
        ui.add_space(15.0);
        mostrar_selector_proyectos_estandar(
            ui,
            &mut state.selected_project,
            &mut state.term_cwd,
            "combo_proyectos_funciones",
            &mut state.funciones_code,
        );
        ui.add_space(10.0);
        let theme = &state.theme_set.themes["base16-ocean.dark"];
        mostrar_editor_interactivo(
            ui,
            &mut state.funciones_code,
            Arc::clone(&state.funciones_output),
            "",
            ejecutar_codigo_rust,
            &state.syntax_set,
            theme,
        );
    } else {
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(
                "Prueba el mismo flujo en el editor: pestaña «fn y parámetros» o «Retorno».",
            )
            .small()
            .italics()
            .color(egui::Color32::from_rgb(140, 150, 165)),
        );
    }
}

/// Simulador visual de call stack (tema naranja/cyan del curso).
fn mostrar_simulador_call_stack(ui: &mut egui::Ui, step: usize) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let frame_bg = egui::Color32::from_rgb(30, 40, 60);
    let canvas_bg = egui::Color32::from_rgb(14, 18, 26);
    let border = egui::Color32::from_rgb(45, 60, 90);

    let height = 168.0;
    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width().min(720.0), height));
    let painter = ui.painter_at(rect);

    painter.rect(
        rect,
        8.0,
        canvas_bg,
        egui::Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );

    let main_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 150.0, rect.center().y),
        egui::vec2(230.0, 108.0),
    );
    painter.rect(
        main_rect,
        6.0,
        frame_bg,
        egui::Stroke::new(1.5, cyan),
        egui::StrokeKind::Middle,
    );
    painter.text(
        main_rect.left_top() + egui::vec2(12.0, 10.0),
        egui::Align2::LEFT_TOP,
        "Frame: main()",
        egui::FontId::proportional(13.0),
        cyan,
    );
    painter.text(
        main_rect.center() + egui::vec2(0.0, 8.0),
        egui::Align2::CENTER_CENTER,
        "let a = 15;\nlet b = 25;",
        egui::FontId::monospace(13.0),
        egui::Color32::WHITE,
    );

    if step >= 1 {
        let sub_rect = egui::Rect::from_center_size(
            egui::pos2(rect.left() + 470.0, rect.center().y),
            egui::vec2(250.0, 108.0),
        );
        let (fill, stroke_c, title, body) = if step == 1 {
            (
                egui::Color32::from_rgb(48, 36, 22),
                naranja,
                "Frame: calcular_suma",
                "x = 15, y = 25\nx + y   // sin ';'",
            )
        } else {
            (
                egui::Color32::from_rgb(22, 42, 32),
                egui::Color32::from_rgb(80, 200, 120),
                "Retorno → main",
                "valor = 40\nframe desapilado",
            )
        };
        painter.rect(
            sub_rect,
            6.0,
            fill,
            egui::Stroke::new(2.0, stroke_c),
            egui::StrokeKind::Middle,
        );
        painter.text(
            sub_rect.left_top() + egui::vec2(12.0, 10.0),
            egui::Align2::LEFT_TOP,
            title,
            egui::FontId::proportional(13.0),
            stroke_c,
        );
        painter.text(
            sub_rect.center() + egui::vec2(0.0, 8.0),
            egui::Align2::CENTER_CENTER,
            body,
            egui::FontId::monospace(13.0),
            egui::Color32::WHITE,
        );

        let a = main_rect.right_center();
        let b = sub_rect.left_center();
        painter.line_segment([a, b], egui::Stroke::new(2.0, naranja));
        // Flechita simple hacia el frame hijo / retorno
        let tip = if step == 1 { b } else { a };
        let dir = if step == 1 { -1.0 } else { 1.0 };
        painter.line_segment(
            [
                tip,
                egui::pos2(tip.x + 8.0 * dir, tip.y - 5.0),
            ],
            egui::Stroke::new(2.0, naranja),
        );
        painter.line_segment(
            [
                tip,
                egui::pos2(tip.x + 8.0 * dir, tip.y + 5.0),
            ],
            egui::Stroke::new(2.0, naranja),
        );
    }

    let caption = match step {
        0 => "Solo main está en la pila.",
        1 => "Se apila calcular_suma; main espera el retorno.",
        _ => "Se desapila el frame hijo; main recibe 40.",
    };
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(caption)
            .small()
            .color(egui::Color32::from_rgb(140, 150, 165)),
    );
}

fn mostrar_tutorial_iteradores(ui: &mut egui::Ui, state: &mut PortfolioState) {
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

fn mostrar_tutorial_structs(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let naranja = egui::Color32::from_rgb(255, 160, 50);
    let cyan = egui::Color32::from_rgb(100, 200, 255);
    let gris_tab = egui::Color32::from_rgb(180, 190, 205);
    let texto = egui::Color32::from_rgb(200, 210, 225);

    ui.add_space(15.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("Structs & impl")
                .size(28.0)
                .strong()
                .color(naranja),
        );
    });
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Conceptos:")
                .small()
                .color(egui::Color32::from_rgb(140, 150, 165)),
        );
        for (i, label) in [
            (0, "struct"),
            (1, "impl / métodos"),
            (2, "Asociadas"),
        ] {
            let activo = state.structs_tab == i;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).strong().color(color))
                        .frame(activo),
                )
                .clicked()
            {
                state.structs_tab = i;
            }
            ui.add_space(4.0);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let activo = state.structs_tab == 3;
            let color = if activo { naranja } else { gris_tab };
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("vs tupla")
                            .strong()
                            .color(color),
                    )
                    .frame(activo),
                )
                .clicked()
            {
                state.structs_tab = 3;
            }
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Comparar:")
                    .small()
                    .color(egui::Color32::from_rgb(140, 150, 165)),
            );
        });
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);

    match state.structs_tab {
        0 => {
            ui.label(
                egui::RichText::new(
                    "Un `struct` agrupa datos relacionados con campos nombrados. \
                     Es la forma idiomática de modelar entidades (usuario, servidor, punto…).",
                )
                .color(texto),
            );
            ui.add_space(10.0);
            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_struct_formas")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Forma").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Uso").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Nombrado").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("struct User { id: u64, name: String }")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Campos con nombre; lo más habitual.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Tupla struct").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("struct Color(u8, u8, u8);")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Como tupla, pero con tipo propio.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Unit").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("struct Marcador;")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Sin datos; útil como marca de tipo.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Instancia").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("User { id: 1, name: s }")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Hay que nombrar todos los campos (o ..base).");
                        ui.end_row();
                    });
            });
        }
        1 => {
            ui.label(
                egui::RichText::new(
                    "El bloque `impl` asocia funciones al tipo. Los métodos reciben `self`, \
                     `&self` o `&mut self` (Ownership + borrowing aplicados a tus datos).",
                )
                .color(texto),
            );
            ui.add_space(10.0);
            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_impl_metodos")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Receptor").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Firma").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Significado").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("&self").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("fn len(&self) -> usize")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Solo lee; no toma el dueño.");
                        ui.end_row();

                        ui.label(egui::RichText::new("&mut self").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("fn iniciar(&mut self)")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Modifica el struct en el sitio.");
                        ui.end_row();

                        ui.label(egui::RichText::new("self").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("fn into_parts(self)")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Consume el valor (move).");
                        ui.end_row();

                        ui.label(egui::RichText::new("Llamada").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("obj.metodo()")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Azúcar de Tipo::metodo(&obj) / similar.");
                        ui.end_row();
                    });
            });
        }
        2 => {
            ui.label(
                egui::RichText::new(
                    "Las funciones asociadas no llevan `self`: viven en el tipo \
                     (`ServidorWeb::new`). `Self` es alias del tipo del `impl`.",
                )
                .color(texto),
            );
            ui.add_space(10.0);
            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_asociadas")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Idea").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Ejemplo").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Nota").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Constructor").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("fn new(...) -> Self")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Convención; no es palabra clave.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Self").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("Self { campo: v }")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Igual que escribir el nombre del struct.");
                        ui.end_row();

                        ui.label(egui::RichText::new("Ruta").monospace().strong().color(naranja));
                        ui.label(
                            egui::RichText::new("Tipo::new(args)")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label("Sin instancia previa.");
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.label(
                egui::RichText::new(
                    "La tupla es anónima y posicional; el struct da nombres y puede llevar `impl`. \
                     Cuando `.2` ya no se entiende solo, es hora de campos nombrados.",
                )
                .color(texto),
            );
            ui.add_space(10.0);
            card_frame_tutorial().show(ui, |ui| {
                egui::Grid::new("tabla_struct_vs_tupla")
                    .striped(true)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("Tupla").strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("struct").strong().color(egui::Color32::WHITE));
                        ui.end_row();

                        ui.label(egui::RichText::new("Campos").strong().color(naranja));
                        ui.label(egui::RichText::new(".0 .1").monospace().color(cyan));
                        ui.label(egui::RichText::new(".nombre").monospace().color(cyan));
                        ui.end_row();

                        ui.label(egui::RichText::new("Métodos").strong().color(naranja));
                        ui.label("No (de serie)");
                        ui.label(egui::RichText::new("impl").monospace().color(cyan));
                        ui.end_row();

                        ui.label(egui::RichText::new("Legibilidad").strong().color(naranja));
                        ui.label("Pocas piezas");
                        ui.label("Datos de dominio");
                        ui.end_row();

                        ui.label(egui::RichText::new("Ejemplo").strong().color(naranja));
                        ui.label(
                            egui::RichText::new("(u16, String)")
                                .monospace()
                                .color(cyan),
                        );
                        ui.label(
                            egui::RichText::new("ServidorWeb { puerto, host }")
                                .monospace()
                                .color(cyan),
                        );
                        ui.end_row();
                    });
            });
        }
    }

    ui.add_space(15.0);
    mostrar_selector_proyectos_estandar(
        ui,
        &mut state.selected_project,
        &mut state.term_cwd,
        "combo_proyectos_structs",
        &mut state.structs_code,
    );
    ui.add_space(10.0);
    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.structs_code,
        Arc::clone(&state.structs_output),
        "",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}


fn mostrar_tutorial_enums(ui: &mut egui::Ui, state: &mut PortfolioState) {
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

fn mostrar_tutorial_colecciones(ui: &mut egui::Ui, state: &mut PortfolioState) {
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

fn mostrar_tutorial_errores(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("🚨 Lección 10: Manejo de Errores & Operador ?")
                .size(30.0)
                .strong()
                .color(egui::Color32::from_rgb(240, 90, 90)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("Gestión explícita de fallos sin excepciones irrecuperables")
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
        ui.heading("💡 Excepciones vs Result en Rust");
        ui.add_space(8.0);
        ui.label("• `panic!`: Para condiciones catastróficas e irrecuperables (ej. desbordamiento de índice en array).");
        ui.label("• `Result<T, E>`: Para fallos esperados (ej. archivo no encontrado, error de red).");
        ui.label("• El Operador `?`: Retorna `Err` inmediatamente si la función interna falla, o desenvuelve `Ok(v)`.");
    });

    ui.add_space(20.0);

    ui.heading("📊 Simulador de Tubería de Propagación de Errores (?)");
    ui.add_space(10.0);

    ui.checkbox(
        &mut state.err_pipeline_fail,
        "Simular fallo en la función interna (dividir por cero)",
    );

    ui.add_space(10.0);

    let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 160.0));
    ui.painter()
        .rect_filled(rect, 8.0, egui::Color32::from_rgb(20, 22, 28));

    let f1_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 100.0, rect.center().y),
        egui::vec2(120.0, 50.0),
    );
    let f2_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 300.0, rect.center().y),
        egui::vec2(120.0, 50.0),
    );
    let f3_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 500.0, rect.center().y),
        egui::vec2(120.0, 50.0),
    );

    ui.painter()
        .rect_filled(f1_rect, 6.0, egui::Color32::from_rgb(40, 50, 70));
    ui.painter().text(
        f1_rect.center(),
        egui::Align2::CENTER_CENTER,
        "dividir(a, b)",
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );

    ui.painter()
        .rect_filled(f2_rect, 6.0, egui::Color32::from_rgb(40, 50, 70));
    ui.painter().text(
        f2_rect.center(),
        egui::Align2::CENTER_CENTER,
        "calcular() ?",
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );

    ui.painter()
        .rect_filled(f3_rect, 6.0, egui::Color32::from_rgb(40, 50, 70));
    ui.painter().text(
        f3_rect.center(),
        egui::Align2::CENTER_CENTER,
        "main()",
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );

    if state.err_pipeline_fail {
        ui.painter().line_segment(
            [f1_rect.right_center(), f2_rect.left_center()],
            egui::Stroke::new(3.0, egui::Color32::RED),
        );
        ui.painter().line_segment(
            [f2_rect.right_center(), f3_rect.left_center()],
            egui::Stroke::new(3.0, egui::Color32::RED),
        );
        ui.painter().text(
            rect.center() + egui::vec2(0.0, 50.0),
            egui::Align2::CENTER_CENTER,
            "❌ Err(\"No se puede dividir entre cero\") propogado hasta main()",
            egui::FontId::proportional(13.0),
            egui::Color32::LIGHT_RED,
        );
    } else {
        ui.painter().line_segment(
            [f1_rect.right_center(), f2_rect.left_center()],
            egui::Stroke::new(3.0, egui::Color32::GREEN),
        );
        ui.painter().line_segment(
            [f2_rect.right_center(), f3_rect.left_center()],
            egui::Stroke::new(3.0, egui::Color32::GREEN),
        );
        ui.painter().text(
            rect.center() + egui::vec2(0.0, 50.0),
            egui::Align2::CENTER_CENTER,
            "✅ Ok(10.0) retornado exitosamente",
            egui::FontId::proportional(13.0),
            egui::Color32::GREEN,
        );
    }

    ui.add_space(20.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.errores_code,
        Arc::clone(&state.errores_output),
        "▶ Ejecutar Manejo de Errores",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

fn mostrar_tutorial_traits(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(
            egui::RichText::new("🧬 Lección 11: Traits, Genéricos & Dispatch")
                .size(30.0)
                .strong()
                .color(egui::Color32::from_rgb(100, 220, 200)),
        );
        ui.add_space(5.0);
        ui.label(
            egui::RichText::new("Polimorfismo en Rust: Static Monomorphization vs Dynamic Vtables")
                .size(15.0)
                .italics(),
        );
    });

    ui.add_space(25.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.traits_code,
        Arc::clone(&state.traits_output),
        "▶ Ejecutar Traits & Genéricos",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

fn mostrar_editor(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new("💻 Code Playground").size(32.0).strong().color(egui::Color32::from_rgb(100, 200, 255)));
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Editor 100% funcional. Escribe código Rust, haz clic en compilar y se ejecutará de verdad usando rustc en segundo plano.").size(16.0).italics());
    });

    ui.add_space(30.0);
    ui.separator();
    ui.add_space(20.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.playground_code,
        Arc::clone(&state.playground_output),
        "▶ Ejecutar Local (rustc)",
        ejecutar_codigo_rust,
        &state.syntax_set,
        theme,
    );
}

fn mostrar_editor_nube(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new("☁️ Rust Playground API").size(32.0).strong().color(egui::Color32::from_rgb(255, 150, 100)));
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Envía tu código a los servidores oficiales de Rust para compilarlo. ¡Soporta el Top 100 crates (ej: serde, rand)!").size(16.0).italics());
    });

    ui.add_space(30.0);
    ui.separator();
    ui.add_space(20.0);

    let theme = &state.theme_set.themes["base16-ocean.dark"];
    mostrar_editor_interactivo(
        ui,
        &mut state.playground_nube_code,
        Arc::clone(&state.playground_nube_output),
        "☁️ Compilar en la Nube",
        ejecutar_codigo_api,
        &state.syntax_set,
        theme,
    );
}
