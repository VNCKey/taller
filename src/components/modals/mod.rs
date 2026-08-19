pub mod comandos;
pub mod compiladores;
pub mod railroad;
pub mod salida_cargo;
pub mod settings;
pub mod template_creado;
pub mod terminal;
pub mod tipos_primitivos;

#[allow(unused_imports)]
pub use comandos::mostrar_modal_comandos;
pub use compiladores::mostrar_modal_comparacion_compiladores;
pub use railroad::mostrar_modal_railroad_let;
pub use salida_cargo::mostrar_modal_salida_cargo;
pub use settings::mostrar_modal_settings;
pub use template_creado::mostrar_modal_template_creado;
pub use terminal::mostrar_modal_terminal;
#[allow(unused_imports)]
pub use tipos_primitivos::mostrar_modal_tipos_primitivos;
