use eframe::egui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use crate::components::modals::{
    mostrar_modal_comparacion_compiladores, mostrar_modal_railroad_let, mostrar_modal_salida_cargo,
    mostrar_modal_settings, mostrar_modal_terminal,
};
use crate::routes::AppRoute;
use crate::views::conceptos::{
    buscar_ruta_proyecto, ejecutar_cargo_run_proyecto, mostrar_comenzando,
};
use crate::views::control_flujo::mostrar_tutorial_control_flujo;
use crate::views::dashboard::mostrar_graficos;
use crate::views::enums::mostrar_tutorial_enums;
use crate::views::errores::mostrar_tutorial_errores;
use crate::views::funciones::mostrar_tutorial_funciones;
use crate::views::iteradores::mostrar_tutorial_iteradores;
use crate::views::landing::mostrar_landing_page;
use crate::views::pilares::mostrar_tutorial_cargo;
use crate::views::playground::{mostrar_editor, mostrar_editor_nube};
use crate::views::memoria::mostrar_tutorial_strings_ownership;
use crate::views::structs::mostrar_tutorial_structs;
use crate::views::genericos::mostrar_tutorial_genericos;
use crate::views::traits::mostrar_tutorial_traits;

#[allow(dead_code)]
pub struct PortfolioState {
    pub ruta_actual: AppRoute,
    pub mostrar_sidebar: bool,
    pub mostrar_nav_superior: bool,
    pub mostrar_status_bar: bool,

    pub show_ingresos: bool,
    pub show_gastos: bool,
    pub show_beneficios: bool,
    pub year: i32,

    pub tutorial_step: usize,
    pub tutorial_time: f64,
    pub anim_trigger: f64,

    pub playground_code: String,
    pub playground_output: Arc<Mutex<String>>,

    pub datatypes_code: String,
    pub datatypes_output: Arc<Mutex<String>>,

    pub strings_code: String,
    pub strings_output: Arc<Mutex<String>>,

    pub playground_nube_code: String,
    pub playground_nube_output: Arc<Mutex<String>>,

    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,

    // Animaciones
    pub anim_compilacion_activa: bool,
    pub compilacion_progreso: f32,
    pub compilacion_etapa_seleccionada: usize,

    // State para Dashboard de Visualización (Plotly / Power BI)
    pub dash_tab: usize,
    pub bcr_playing: bool,
    pub bcr_year: f32,
    pub bcr_speed: f32,
    pub pie_donut_hole: f32,
    pub pie_exploded: bool,
    pub index_baseline_year: f32,
    pub ts_show_ma: bool,
    pub ts_show_volume: bool,
    pub ts_show_rsi: bool,

    pub bool_sim_a: bool,
    pub bool_sim_b: bool,

    pub show_code_modal: Option<(String, String)>,
    pub shared_project_code: String,
    pub shared_project_output: Arc<Mutex<String>>,

    // State para los nuevos Módulos del Curso Interactivo de Rust
    pub controlflujo_code: String,
    pub controlflujo_output: Arc<Mutex<String>>,
    pub controlflujo_val: i32,

    pub ownership_code: String,
    pub ownership_output: Arc<Mutex<String>>,
    pub ownership_step: usize,
    /// Tabs de la sesión unificada Strings & Ownership
    pub strings_ownership_tab: usize,

    pub structs_code: String,
    pub structs_output: Arc<Mutex<String>>,

    pub enums_code: String,
    pub enums_output: Arc<Mutex<String>>,
    pub enum_variant_selected: usize,

    pub colecciones_code: String,
    pub colecciones_output: Arc<Mutex<String>>,
    pub vec_sim_len: usize,
    pub vec_sim_cap: usize,
    pub colecciones_tab: usize,

    pub errores_code: String,
    pub errores_output: Arc<Mutex<String>>,
    pub err_pipeline_fail: bool,

    pub traits_code: String,
    pub traits_output: Arc<Mutex<String>>,
    pub genericos_code: String,
    pub genericos_output: Arc<Mutex<String>>,

    // State para Juegos Interactivos de Tipos Compuestos (Arrays, Slices, Tuplas)
    pub arr_elem_type: usize,
    pub arr_len: usize,
    pub arr_active_idx: usize,
    pub arr_action_msg: String,
    /// Tabs de la sesión Tipos compuestos
    pub compuestos_tab: usize,

    pub slice_start: usize,
    pub slice_end: usize,

    pub tup_t0: usize,
    pub tup_t1: usize,
    pub tup_t2: usize,

    /// Tabs de Structs & impl
    pub structs_tab: usize,

    // State para Funciones e Iteradores
    pub funciones_code: String,
    pub funciones_output: Arc<Mutex<String>>,
    pub funciones_step: usize,
    pub funciones_tab: usize,

    pub iteradores_tab: usize,
    pub enums_tab: usize,
    pub traits_tab: usize,
    pub genericos_tab: usize,
    pub iteradores_code: String,
    pub iteradores_output: Arc<Mutex<String>>,
    pub iter_mode: usize,
    pub iter_filter_even: bool,

    // Editores dedicados para Tipos Compuestos
    pub arr_code: String,
    pub arr_output: Arc<Mutex<String>>,
    pub slice_code: String,
    pub slice_output: Arc<Mutex<String>>,
    pub tup_code: String,
    pub tup_output: Arc<Mutex<String>>,
    pub tup_active_idx: usize,
    pub compuestos_info_tab: usize,

    // State para Estructura de Proyecto y Conceptos Básicos
    pub comenzando_step: usize,
    pub pilares_step: usize,
    pub anatomy_step: usize,
    pub mostrar_explorer_drawer: bool,
    pub mostrar_console_drawer: bool,
    pub show_commands_modal: bool,
    pub show_macro_expansion: bool,
    pub show_cargo_output_modal: Arc<AtomicBool>,
    pub show_rustc_compilador_modal: bool,
    pub show_terminal_modal: bool,
    pub show_settings_modal: bool,
    pub settings_tab: usize,
    pub tipo_primitivo_categoria: usize,
    pub variable_name: String,
    pub variable_type: usize,
    pub variable_value: String,
    pub variable_mutable: bool,
    pub declaration_kind: usize,
    pub estructura_code: String,
    pub estructura_output: Arc<Mutex<String>>,
    pub estructura_tab: usize,
    pub conceptos_tab: usize,
    pub show_railroad_modal: Option<usize>,
    pub controlflujo_tab: usize,
    pub controlflujo_is_practica: bool,
    pub conceptos_code: String,
    pub conceptos_output: Arc<Mutex<String>>,
    pub modulos_code: String,
    pub modulos_output: Arc<Mutex<String>>,
    pub modulos_tab: usize,

    // State para Componente de Terminal de 3 Modos
    pub term_selected_mode: usize,
    pub term_input: String,
    pub term_history: Arc<Mutex<Vec<String>>>,
    pub term_cwd: std::path::PathBuf,
    pub show_terminal_history: bool,
    pub created_project_name: Option<String>,
    pub selected_project: Option<String>,
    pub selected_file: Option<String>,
    pub project_editor_path: Option<String>,
    pub project_editor_code: String,
    pub project_editor_status: String,
    pub project_editor_dirty: bool,
}



impl Default for PortfolioState {
    fn default() -> Self {
        Self {
            ruta_actual: AppRoute::LandingPage,
            mostrar_sidebar: true,
            mostrar_nav_superior: true,
            mostrar_status_bar: true,
            show_ingresos: true,
            show_gastos: true,
            show_beneficios: true,
            year: 2025,
            tutorial_step: 0,
            tutorial_time: 0.0,
            anim_trigger: 0.0,
            show_code_modal: None,
            shared_project_code: String::new(),
            shared_project_output: Arc::new(Mutex::new(String::new())),
            playground_code: "fn main() {\n    println!(\"¡Hola desde el Editor de Egui!\");\n}\n".to_string(),
            playground_output: Arc::new(Mutex::new(String::new())),
            datatypes_code: "// Tipos Compuestos en Rust: Arrays, Slices y Tuplas\nfn main() {\n    let numeros: [i32; 5] = [10, 20, 30, 40, 50];\n    let slice_nums: &[i32] = &numeros[1..4];\n    let tupla: (&str, u8) = (\"Rust\", 10);\n\n    println!(\"Array: {:?}\", numeros);\n    println!(\"Slice: {:?}\", slice_nums);\n    println!(\"Tupla: {}, {}\", tupla.0, tupla.1);\n}\n".to_string(),
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
            
            bool_sim_a: true,
            bool_sim_b: false,

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
            colecciones_tab: 0,

            errores_code: "fn dividir(a: f64, b: f64) -> Result<f64, String> {\n    if b == 0.0 {\n        Err(\"No se puede dividir entre cero\".to_string())\n    } else {\n        Ok(a / b)\n    }\n}\n\nfn calcular() -> Result<f64, String> {\n    let res1 = dividir(100.0, 2.0)?;\n    let res2 = dividir(res1, 5.0)?;\n    Ok(res2)\n}\n\nfn main() {\n    match calcular() {\n        Ok(val) => println!(\"Resultado exitoso: {}\", val),\n        Err(err) => println!(\"Error en cálculo: {}\", err),\n    }\n}\n".to_string(),
            errores_output: Arc::new(Mutex::new(String::new())),
            err_pipeline_fail: false,

            traits_code: "trait Dibujable {\n    fn dibujar(&self);\n}\n\nstruct Circulo { radio: f64 }\nstruct Rectangulo { ancho: f64, alto: f64 }\n\nimpl Dibujable for Circulo {\n    fn dibujar(&self) {\n        println!(\"🔴 Dibujando círculo de radio {}\", self.radio);\n    }\n}\n\nimpl Dibujable for Rectangulo {\n    fn dibujar(&self) {\n        println!(\"🟦 Dibujando rectángulo {}x{}\", self.ancho, self.alto);\n    }\n}\n\nfn renderizar(item: &impl Dibujable) {\n    item.dibujar();\n}\n\nfn main() {\n    let c = Circulo { radio: 5.0 };\n    let r = Rectangulo { ancho: 10.0, alto: 4.0 };\n    renderizar(&c);\n    renderizar(&r);\n}\n".to_string(),
            traits_output: Arc::new(Mutex::new(String::new())),
            genericos_code: "fn identidad<T>(valor: T) -> T {\n    valor\n}\n\nfn main() {\n    let n = identidad(42);\n    let s = identidad(\"Hola Rust\");\n    println!(\"n: {}, s: {}\", n, s);\n}\n".to_string(),
            genericos_output: Arc::new(Mutex::new(String::new())),
            genericos_tab: 0,

            arr_elem_type: 1,
            arr_len: 5,
            arr_active_idx: 2,
            arr_action_msg: "Inspecciona métodos y accesos a elementos del arreglo".to_string(),
            compuestos_tab: 0,
            compuestos_info_tab: 0,
            slice_start: 1,
            slice_end: 4,
            tup_t0: 1,
            tup_t1: 3,
            tup_t2: 2,
            tup_active_idx: 0,
            structs_tab: 0,

            funciones_code: "fn main() {\n    let a = 15;\n    let b = 25;\n\n    // Llamada + retorno implícito (sin ';')\n    let suma = calcular_suma(a, b);\n    println!(\"La suma de {} + {} es: {}\", a, b, suma);\n\n    // Paso por referencia mutable\n    let mut contador = 0;\n    incrementar(&mut contador);\n    println!(\"Contador incrementado: {}\", contador);\n\n    // Closure: captura el entorno\n    let factor = 3;\n    let multiplicar = |x: i32| x * factor;\n    println!(\"10 * factor = {}\", multiplicar(10));\n}\n\nfn calcular_suma(x: i32, y: i32) -> i32 {\n    x + y // última expresión = retorno\n}\n\nfn incrementar(val: &mut i32) {\n    *val += 1;\n}\n".to_string(),
            funciones_output: Arc::new(Mutex::new(String::new())),
            funciones_step: 0,
            funciones_tab: 0,

            iteradores_tab: 0,
            enums_tab: 0,
            traits_tab: 0,
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
            anatomy_step: 0,
            mostrar_explorer_drawer: false,
            mostrar_console_drawer: false,
            show_commands_modal: false,
            show_macro_expansion: false,
            show_cargo_output_modal: Arc::new(AtomicBool::new(false)),
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
            controlflujo_is_practica: false,
            conceptos_code: "fn main() {\n    let edad: u8 = 25;\n    println!(\"edad = {edad}\");\n}\n".to_string(),
            conceptos_output: Arc::new(Mutex::new(String::new())),
            modulos_code: "// Módulos en Rust\nmod redes {\n    pub fn conectar() {\n        println!(\"Conectado a la red!\");\n    }\n}\n\nfn main() {\n    redes::conectar();\n}\n".to_string(),
            modulos_output: Arc::new(Mutex::new(String::new())),

            term_selected_mode: 1,
            term_input: String::new(),
            term_history: Arc::new(Mutex::new(Vec::new())),
            term_cwd: std::env::current_dir()
                .ok()
                .map(|p| {
                    if p.file_name().is_some_and(|n| n == "egui_vnc") {
                        p.parent().unwrap_or(&p).to_path_buf()
                    } else {
                        p
                    }
                })
                .unwrap_or_else(|| std::path::PathBuf::from("/home/alek/VNC/repos")),
            show_terminal_history: false,
            created_project_name: None,
            selected_project: None,
            selected_file: None,
            project_editor_path: None,
            project_editor_code: String::new(),
            project_editor_status: String::new(),
            project_editor_dirty: false,
            modulos_tab: 0,
        }
    }
}


impl PortfolioState {
    pub fn obtener_codigo_activo(&self) -> &str {
        if self.selected_project.is_some() {
            &self.shared_project_code
        } else {
            match self.ruta_actual {
                AppRoute::TutorialControlFlujo => &self.controlflujo_code,
                AppRoute::TutorialTiposDatos => &self.datatypes_code,
                AppRoute::TutorialStrings | AppRoute::TutorialOwnership | AppRoute::TutorialMemoria => &self.ownership_code,
                AppRoute::TutorialFunciones => &self.funciones_code,
                AppRoute::TutorialIteradores => &self.iteradores_code,
                AppRoute::TutorialStructs => &self.structs_code,
                AppRoute::TutorialEnums => &self.enums_code,
                AppRoute::TutorialColecciones => &self.colecciones_code,
                AppRoute::TutorialErrores => &self.errores_code,
                AppRoute::TutorialTraits => &self.traits_code,
                AppRoute::TutorialGenericos => &self.genericos_code,
                AppRoute::TutorialModulos => &self.modulos_code,
                AppRoute::Playground => &self.playground_code,
                AppRoute::PlaygroundNube => &self.playground_nube_code,
                AppRoute::Comenzando => &self.conceptos_code,
                AppRoute::TutorialCargo | AppRoute::TutorialCompilacion => &self.estructura_code,
                _ => &self.conceptos_code,
            }
        }
    }

    pub fn obtener_editor_activo_mut(&mut self) -> (&mut String, Arc<Mutex<String>>) {
        if self.selected_project.is_some() {
            (&mut self.shared_project_code, Arc::clone(&self.shared_project_output))
        } else {
            match self.ruta_actual {
                AppRoute::TutorialControlFlujo => (&mut self.controlflujo_code, Arc::clone(&self.controlflujo_output)),
                AppRoute::TutorialTiposDatos => (&mut self.datatypes_code, Arc::clone(&self.datatypes_output)),
                AppRoute::TutorialStrings | AppRoute::TutorialOwnership | AppRoute::TutorialMemoria => (&mut self.ownership_code, Arc::clone(&self.ownership_output)),
                AppRoute::TutorialFunciones => (&mut self.funciones_code, Arc::clone(&self.funciones_output)),
                AppRoute::TutorialIteradores => (&mut self.iteradores_code, Arc::clone(&self.iteradores_output)),
                AppRoute::TutorialStructs => (&mut self.structs_code, Arc::clone(&self.structs_output)),
                AppRoute::TutorialEnums => (&mut self.enums_code, Arc::clone(&self.enums_output)),
                AppRoute::TutorialColecciones => (&mut self.colecciones_code, Arc::clone(&self.colecciones_output)),
                AppRoute::TutorialErrores => (&mut self.errores_code, Arc::clone(&self.errores_output)),
                AppRoute::TutorialTraits => (&mut self.traits_code, Arc::clone(&self.traits_output)),
                AppRoute::TutorialGenericos => (&mut self.genericos_code, Arc::clone(&self.genericos_output)),
                AppRoute::TutorialModulos => (&mut self.modulos_code, Arc::clone(&self.modulos_output)),
                AppRoute::Playground => (&mut self.playground_code, Arc::clone(&self.playground_output)),
                AppRoute::PlaygroundNube => (&mut self.playground_nube_code, Arc::clone(&self.playground_nube_output)),
                AppRoute::Comenzando => (&mut self.conceptos_code, Arc::clone(&self.conceptos_output)),
                AppRoute::TutorialCargo | AppRoute::TutorialCompilacion => (&mut self.estructura_code, Arc::clone(&self.estructura_output)),
                _ => (&mut self.conceptos_code, Arc::clone(&self.conceptos_output)),
            }
        }
    }

    #[allow(dead_code)]
    pub fn cargar_archivo_proyecto_activo(&mut self) {
        if let Some(ref proj) = self.selected_project.clone() {
            let proj_dir = buscar_ruta_proyecto(&self.term_cwd, proj);
            let target_file = if let Some(ref rel) = self.selected_file {
                proj_dir.join(rel)
            } else {
                let main_rs = proj_dir.join("src/main.rs");
                let lib_rs = proj_dir.join("src/lib.rs");
                if main_rs.exists() {
                    main_rs
                } else if lib_rs.exists() {
                    lib_rs
                } else {
                    main_rs
                }
            };
            if let Ok(content) = std::fs::read_to_string(&target_file) {
                let (code_ref, _) = self.obtener_editor_activo_mut();
                *code_ref = content;
            }
        }
    }

    pub fn obtener_output_activo(&self) -> Arc<Mutex<String>> {
        if self.selected_project.is_some() {
            Arc::clone(&self.shared_project_output)
        } else {
            match self.ruta_actual {
                AppRoute::TutorialControlFlujo => Arc::clone(&self.controlflujo_output),
                AppRoute::TutorialTiposDatos => Arc::clone(&self.datatypes_output),
                AppRoute::TutorialStrings | AppRoute::TutorialOwnership | AppRoute::TutorialMemoria => Arc::clone(&self.ownership_output),
                AppRoute::TutorialFunciones => Arc::clone(&self.funciones_output),
                AppRoute::TutorialIteradores => Arc::clone(&self.iteradores_output),
                AppRoute::TutorialStructs => Arc::clone(&self.structs_output),
                AppRoute::TutorialEnums => Arc::clone(&self.enums_output),
                AppRoute::TutorialColecciones => Arc::clone(&self.colecciones_output),
                AppRoute::TutorialErrores => Arc::clone(&self.errores_output),
                AppRoute::TutorialTraits => Arc::clone(&self.traits_output),
                AppRoute::TutorialGenericos => Arc::clone(&self.genericos_output),
                AppRoute::TutorialModulos => Arc::clone(&self.modulos_output),
                AppRoute::Playground => Arc::clone(&self.playground_output),
                AppRoute::PlaygroundNube => Arc::clone(&self.playground_nube_output),
                AppRoute::Comenzando => Arc::clone(&self.conceptos_output),
                AppRoute::TutorialCargo | AppRoute::TutorialCompilacion => Arc::clone(&self.estructura_output),
                _ => Arc::clone(&self.conceptos_output),
            }
        }
    }

    pub fn guardar_proyecto_activo(&mut self) {
        if let Some(ref proj) = self.selected_project {
            let proj_dir = buscar_ruta_proyecto(&self.term_cwd, proj);
            if self.project_editor_path.as_deref() == self.selected_file.as_deref() {
                if let Some(ref relative_file) = self.project_editor_path {
                    let target_file = proj_dir.join(relative_file);
                    if std::fs::write(target_file, &self.project_editor_code).is_ok() {
                        self.project_editor_dirty = false;
                        self.project_editor_status = "Cambios guardados correctamente.".to_string();
                    }
                    return;
                }
            }
            let target_file = if let Some(ref rel_file) = self.selected_file {
                proj_dir.join(rel_file)
            } else {
                let main_rs = proj_dir.join("src/main.rs");
                let lib_rs = proj_dir.join("src/lib.rs");
                if main_rs.exists() {
                    main_rs
                } else if lib_rs.exists() {
                    lib_rs
                } else {
                    main_rs
                }
            };
            if let Some(parent) = target_file.parent() {
                if !parent.exists() {
                    let _ = std::fs::create_dir_all(parent);
                }
            }
            let _ = std::fs::write(target_file, self.obtener_codigo_activo());
        }
    }
}


impl eframe::App for PortfolioState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // FORZAR desactivación GLOBAL de CUALQUIER modo debug (bordes rojos, etc.)
        ui.style_mut().debug.debug_on_hover = false;
        ui.style_mut().debug.show_expand_width = false;
        ui.style_mut().debug.show_expand_height = false;
        ui.style_mut().debug.show_resize = false;
        ui.style_mut().debug.show_interactive_widgets = false;
        for theme in [egui::Theme::Dark, egui::Theme::Light] {
            ui.ctx().style_mut_of(theme, |style| {
                style.debug.debug_on_hover = false;
                style.debug.show_expand_width = false;
                style.debug.show_expand_height = false;
                style.debug.show_resize = false;
                style.debug.show_interactive_widgets = false;
            });
        }

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

        // --- COLORES PRINCIPALES DEL ENTORNO ---
        // Aquí puedes "jugar" con los colores (R, G, B)
        let color_fondo_principal = egui::Color32::from_rgb(20, 22, 27); // Gris azulado oscuro (Main)
        let color_footer = egui::Color32::from_rgb(13, 15, 19); // Mismo color oscuro que la Barra Lateral

        // --- 0. STATUS BAR GLOBAL (FOOTER) ---
        if self.ruta_actual != AppRoute::LandingPage {
            let mut is_expanded = self.mostrar_status_bar;
            egui::Panel::bottom("status_bar")
                .frame(egui::Frame::default().fill(color_footer).inner_margin(4.0))
                .show_collapsible(ui, &mut is_expanded, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    
                    // Toggle Sidebar Izquierda
                    let img_sidebar = egui::Image::new(egui::include_image!("../assets/icons/sidebar-left.svg")).fit_to_exact_size(egui::Vec2::new(16.0, 16.0));
                    let mut btn_sidebar = egui::Button::image(img_sidebar);
                    if self.mostrar_sidebar {
                        btn_sidebar = btn_sidebar.fill(egui::Color32::from_rgb(50, 60, 80));
                    }
                    if ui.add(btn_sidebar)
                        .on_hover_text("Mostrar/Ocultar Menú Lateral (Ctrl+B)")
                        .clicked() 
                    {
                        self.mostrar_sidebar = !self.mostrar_sidebar;
                    }
                    
                    // Toggle Header
                    let has_top_nav = self.ruta_actual == AppRoute::Comenzando || self.ruta_actual == AppRoute::TutorialCargo;
                    if has_top_nav {
                        let img_top = egui::Image::new(egui::include_image!("../assets/icons/layout-top.svg")).fit_to_exact_size(egui::Vec2::new(16.0, 16.0));
                        let mut btn_top = egui::Button::image(img_top);
                        if self.mostrar_nav_superior {
                            btn_top = btn_top.fill(egui::Color32::from_rgb(50, 60, 80));
                        }
                        if ui.add(btn_top)
                            .on_hover_text("Mostrar/Ocultar Header (Ctrl+H)")
                            .clicked()
                        {
                            self.mostrar_nav_superior = !self.mostrar_nav_superior;
                        }
                    }

                    // Elementos a la derecha (estilo VS Code / Zed)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("FerrisKey v0.1").small().color(egui::Color32::from_rgb(130, 140, 155)));
                        ui.add_space(12.0);

                        if let Some(ref proj) = self.selected_project {
                            if let Some(ref file) = self.selected_file {
                                ui.label(
                                    egui::RichText::new(file)
                                        .size(11.5)
                                        .color(egui::Color32::from_rgb(255, 180, 50)),
                                );
                                ui.label(
                                    egui::RichText::new("/")
                                        .size(11.5)
                                        .color(egui::Color32::from_rgb(70, 90, 120)),
                                );
                            }
                            ui.label(
                                egui::RichText::new(proj)
                                    .size(11.5)
                                    .color(egui::Color32::from_rgb(100, 200, 255)),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new("Libre")
                                    .size(11.5)
                                    .color(egui::Color32::from_rgb(120, 145, 175)),
                            );
                        }
                    });
                });
            });
            self.mostrar_status_bar = is_expanded;
        }

        // --- 1. PANEL DE NAVEGACIÓN (SIDEBAR - Solo si no está en LandingPage) ---
        if self.ruta_actual != AppRoute::LandingPage {
            crate::components::sidebar::mostrar_sidebar(ui, self);
        }

        // --- 1.5 PANEL DERECHO (Inspector - Solo en Conceptos y Rust Foundations) ---
        if self.ruta_actual == AppRoute::Comenzando {
            crate::views::conceptos::mostrar_nav_superior(ui, self);
        } else if self.ruta_actual == AppRoute::TutorialCargo {
            crate::views::pilares::mostrar_nav_superior(ui, self);
        }

        // --- 2. PANEL CENTRAL ---
        let is_code_lab = self.ruta_actual == AppRoute::TutorialCargo && self.pilares_step == 1;
        let central_panel = if self.ruta_actual == AppRoute::LandingPage {
            egui::CentralPanel::default().frame(egui::Frame::new().fill(egui::Color32::BLACK))
        } else if is_code_lab {
            egui::CentralPanel::default().frame(egui::Frame::default().fill(egui::Color32::from_rgb(10, 14, 22)).inner_margin(0.0))
        } else {
            egui::CentralPanel::default().frame(egui::Frame::default().fill(color_fondo_principal).inner_margin(8.0))
        };
        central_panel.show(ui, |ui| {

            match self.ruta_actual {
                AppRoute::LandingPage => mostrar_landing_page(ui, self),
                AppRoute::TutorialCargo => mostrar_tutorial_cargo(ui, self),
                AppRoute::Comenzando => {
                    egui::ScrollArea::vertical().show(ui, |ui| mostrar_comenzando(ui, self));
                }
                AppRoute::TutorialCompilacion => crate::views::pilares::pipeline::mostrar_tutorial_compilacion(ui, self),
                AppRoute::TutorialTiposDatos => crate::views::tipos_compuestos::mostrar_tutorial_tipos_compuestos(ui, self),
                AppRoute::TutorialControlFlujo => mostrar_tutorial_control_flujo(ui, self),
                AppRoute::TutorialFunciones => mostrar_tutorial_funciones(ui, self),
                AppRoute::TutorialIteradores => mostrar_tutorial_iteradores(ui, self),
                AppRoute::TutorialOwnership | AppRoute::TutorialStrings | AppRoute::TutorialMemoria => {
                    mostrar_tutorial_strings_ownership(ui, self)
                }
                AppRoute::TutorialModulos => crate::views::modulos::mostrar_tutorial_modulos(ui, self),
                AppRoute::TutorialStructs => mostrar_tutorial_structs(ui, self),
                AppRoute::TutorialEnums => mostrar_tutorial_enums(ui, self),
                AppRoute::TutorialColecciones => {
                    crate::views::tipos_compuestos::mostrar_tutorial_tipos_compuestos(ui, self)
                }
                AppRoute::TutorialErrores => mostrar_tutorial_errores(ui, self),
                AppRoute::TutorialTraits => mostrar_tutorial_traits(ui, self),
                AppRoute::TutorialGenericos => mostrar_tutorial_genericos(ui, self),
                AppRoute::DashboardGraficos => mostrar_graficos(ui, self),
                AppRoute::Playground => mostrar_editor(ui, self),
                AppRoute::PlaygroundNube => mostrar_editor_nube(ui, self),
            }
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

            // Ctrl + B -> Toggle Sidebar Izquierdo
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::B,
            )) {
                self.mostrar_sidebar = !self.mostrar_sidebar;
            }

            // Ctrl + H -> Toggle Top Nav
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::H,
            )) {
                self.mostrar_nav_superior = !self.mostrar_nav_superior;
            }

            // Ctrl + J -> Toggle Status Bar (Footer)
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL,
                egui::Key::J,
            )) {
                self.mostrar_status_bar = !self.mostrar_status_bar;
            }

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
            self.guardar_proyecto_activo();
            ejecutar_cargo_run_proyecto(self, ui.ctx());
        }

        mostrar_modal_comparacion_compiladores(ui.ctx(), self);
        mostrar_modal_salida_cargo(ui.ctx(), self);
        mostrar_modal_terminal(ui.ctx(), self);
        mostrar_modal_settings(ui.ctx(), self);
        mostrar_modal_railroad_let(ui.ctx(), self);
        crate::components::modals::mostrar_modal_codigo(ui.ctx(), self);

        // Request continuous repaint for animations
        ui.ctx().request_repaint();
    }
}
