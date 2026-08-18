use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use rand::seq::SliceRandom;
use egui_plot::{Bar, BarChart, Plot, Legend};

#[derive(PartialEq, Default)]
enum AppRoute {
    #[default]
    Portafolio,
    TutorialCargo,
    TutorialTiposDatos,
    TutorialMemoria,
    DashboardGraficos,
    JuegoMemoriaFerris,
}

#[derive(Resource)]
struct PortfolioState {
    ruta_actual: AppRoute,
    
    show_ingresos: bool,
    show_gastos: bool,
    show_beneficios: bool,
    year: i32,

    tutorial_step: usize,
    tutorial_time: f64,
}

impl Default for PortfolioState {
    fn default() -> Self {
        Self {
            ruta_actual: AppRoute::Portafolio,
            show_ingresos: true,
            show_gastos: true,
            show_beneficios: true,
            year: 2025,
            tutorial_step: 0,
            tutorial_time: 0.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luis Alexander - Rust Portfolio 3D".to_string(),
                resolution: (1100, 750).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .init_resource::<PortfolioState>()
        .insert_resource(MemoryGameState::default())
        .add_systems(Startup, (setup_3d_scene, setup_memory_game))
        .add_systems(EguiPrimaryContextPass, render_ui_system)
        .add_systems(Update, (
            rotate_cube,
            card_click_system,
            match_check_system,
            update_card_sprites,
            ui_update_system,
            toggle_memory_game_visibility
        ))
        .run();
}

#[derive(Component)]
struct RotatingCube;

fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cubo central rotando (NUESTRO FONDO 3D)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.6, 0.8),
            perceptual_roughness: 0.2,
            metallic: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RotatingCube,
    ));

    // Iluminación
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Cámara
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.5 * time.delta_secs());
        transform.rotate_x(0.3 * time.delta_secs());
    }
}

fn render_ui_system(mut contexts: EguiContexts, mut state: ResMut<PortfolioState>, time: Res<Time>) {
    let ctx = contexts.ctx_mut().expect("No egui context");
    
    let mut viewport_ui = egui::Ui::new(
        ctx.clone(),
        "viewport".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );
    
    // Actualizar tiempo de animación usando el motor 3D
    state.tutorial_time += time.delta_secs() as f64;
    
    // --- 1. PANEL DE NAVEGACIÓN (SIDEBAR) ---
    egui::Panel::left("sidebar").resizable(false).show(&mut viewport_ui, |ui| {
        ui.set_min_width(220.0);
        ui.add_space(20.0);
        
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Luis Alexander").size(24.0).strong());
            ui.label(egui::RichText::new("Rust Developer").italics());
        });
        
        ui.add_space(30.0);
        ui.separator();
        ui.add_space(20.0);
        
        ui.label(egui::RichText::new("NAVEGACIÓN").strong().color(egui::Color32::GRAY));
        ui.add_space(10.0);
        if ui.selectable_label(state.ruta_actual == AppRoute::Portafolio, "👤 Inicio / Sobre Mí").clicked() {
            state.ruta_actual = AppRoute::Portafolio;
        }
        
        ui.add_space(20.0);
        ui.label(egui::RichText::new("RUST LEARNING HUB").strong().color(egui::Color32::GRAY));
        ui.add_space(10.0);
        if ui.selectable_label(state.ruta_actual == AppRoute::TutorialCargo, "📦 Ecosistema & Cargo").clicked() {
            state.ruta_actual = AppRoute::TutorialCargo;
        }
        if ui.selectable_label(state.ruta_actual == AppRoute::TutorialTiposDatos, "🔤 Tipos Primitivos").clicked() {
            state.ruta_actual = AppRoute::TutorialTiposDatos;
        }
        if ui.selectable_label(state.ruta_actual == AppRoute::TutorialMemoria, "🧠 Stack vs Heap").clicked() {
            state.ruta_actual = AppRoute::TutorialMemoria;
        }
        
        ui.add_space(20.0);
        ui.label(egui::RichText::new("PROYECTOS TÉCNICOS").strong().color(egui::Color32::GRAY));
        ui.add_space(10.0);
        if ui.selectable_label(state.ruta_actual == AppRoute::DashboardGraficos, "📊 Visualización de Datos").clicked() {
            state.ruta_actual = AppRoute::DashboardGraficos;
        }
    });

    // --- 2. PANEL CENTRAL ---
    // NOTA: Para que podamos ver el mundo 3D por detrás de egui, 
    // tenemos que hacer que el Panel Central tenga fondo transparente.
    let mut frame = egui::Frame::central_panel(viewport_ui.style());
    frame.fill = egui::Color32::from_black_alpha(150); // Translúcido para ver el 3D

    egui::CentralPanel::default().frame(frame).show(&mut viewport_ui, |ui| {
        match state.ruta_actual {
            AppRoute::Portafolio => mostrar_portafolio(ui),
            AppRoute::TutorialCargo => mostrar_tutorial_cargo(ui),
            AppRoute::TutorialTiposDatos => mostrar_tutorial_tipos_datos(ui),
            AppRoute::TutorialMemoria => mostrar_tutorial_memoria(ui, &mut state),
            AppRoute::DashboardGraficos => mostrar_graficos(ui, &mut state),
            AppRoute::JuegoMemoriaFerris => {
                ui.heading("🦀 Juego de Memoria de Ferris");
                ui.label("¡Encuentra todas las parejas de mascotas Rust en el mundo 3D detrás de esta ventana!");
            },
        }
    });
}

// === FUNCIONES DE VISTAS SEPARADAS ===

fn mostrar_portafolio(ui: &mut egui::Ui) {
    ui.add_space(40.0);
    ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new("¡Hola! Soy Luis Alexander").size(36.0).strong());
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Desarrollador de Software especializado en Rust.").size(20.0));
    });
    
    ui.add_space(40.0);
    ui.separator();
    ui.add_space(20.0);
    
    ui.label(egui::RichText::new("¡Bienvenido al Portafolio 3D de Bevy!").size(16.0).strong());
    ui.add_space(10.0);
    ui.label("Esta aplicación demuestra la capacidad de renderizar interfaces (GUI) de alto rendimiento encima de motores de videojuegos.");
}

fn mostrar_tutorial_cargo(ui: &mut egui::Ui) {
    ui.add_space(20.0);
    ui.heading(egui::RichText::new("El Ecosistema Cargo y el Compilador").size(28.0).strong());
}

fn mostrar_tutorial_tipos_datos(ui: &mut egui::Ui) {
    ui.add_space(20.0);
    ui.heading(egui::RichText::new("Tipos de Datos Primitivos en Rust").size(28.0).strong());
}

fn mostrar_tutorial_memoria(ui: &mut egui::Ui, state: &mut PortfolioState) {
    ui.add_space(20.0);
    ui.heading(egui::RichText::new("Gestión de Memoria: Stack vs Heap").size(28.0).strong());
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
                let color = if is_current { egui::Color32::YELLOW } else { egui::Color32::LIGHT_GRAY };
                ui.label(egui::RichText::new(*line).color(color).monospace().size(18.0));
            }

            ui.add_space(30.0);
            if ui.button(egui::RichText::new("▶ Ejecutar Siguiente Línea").size(16.0)).clicked() {
                state.tutorial_step = (state.tutorial_step + 1) % 4;
            }
        });

        // --- COLUMNA 2: VISUALIZACIÓN DE MEMORIA (epaint) ---
        let (response, painter) = columns[1].allocate_painter(
            egui::vec2(columns[1].available_width(), 450.0),
            egui::Sense::hover(),
        );

        let rect = response.rect;
        let stack_rect = egui::Rect::from_min_size(rect.min + egui::vec2(10.0, 40.0), egui::vec2(160.0, 350.0));
        let heap_rect = egui::Rect::from_min_size(rect.min + egui::vec2(200.0, 40.0), egui::vec2(220.0, 350.0));

        painter.rect(stack_rect, 5.0, egui::Color32::TRANSPARENT, egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 250)), egui::StrokeKind::Middle);
        painter.text(stack_rect.center_top() - egui::vec2(0.0, 15.0), egui::Align2::CENTER_CENTER, "STACK", egui::FontId::proportional(18.0), egui::Color32::LIGHT_BLUE);

        painter.rect(heap_rect, 5.0, egui::Color32::TRANSPARENT, egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 250, 100)), egui::StrokeKind::Middle);
        painter.text(heap_rect.center_top() - egui::vec2(0.0, 15.0), egui::Align2::CENTER_CENTER, "HEAP", egui::FontId::proportional(18.0), egui::Color32::LIGHT_GREEN);

        let float_y = (state.tutorial_time * 3.0).sin() as f32 * 5.0;

        if state.tutorial_step >= 1 && state.tutorial_step < 3 {
            let var_a_rect = egui::Rect::from_min_size(stack_rect.min + egui::vec2(10.0, 290.0), egui::vec2(140.0, 40.0));
            painter.rect(var_a_rect, 4.0, egui::Color32::from_rgb(60, 60, 180), egui::Stroke::NONE, egui::StrokeKind::Middle);
            painter.text(var_a_rect.center(), egui::Align2::CENTER_CENTER, "a: i32 = 42", egui::FontId::monospace(16.0), egui::Color32::WHITE);
        }

        if state.tutorial_step >= 2 && state.tutorial_step < 3 {
            let var_s_rect = egui::Rect::from_min_size(stack_rect.min + egui::vec2(10.0, 200.0), egui::vec2(140.0, 70.0));
            painter.rect(var_s_rect, 4.0, egui::Color32::from_rgb(200, 150, 50), egui::Stroke::NONE, egui::StrokeKind::Middle);
            painter.text(var_s_rect.center(), egui::Align2::CENTER_CENTER, "s (String)\nptr: 0x...", egui::FontId::monospace(14.0), egui::Color32::BLACK);

            let heap_data_rect = egui::Rect::from_min_size(heap_rect.min + egui::vec2(30.0, 150.0 + float_y), egui::vec2(160.0, 50.0));
            painter.rect(heap_data_rect, 8.0, egui::Color32::from_rgb(50, 200, 50), egui::Stroke::NONE, egui::StrokeKind::Middle);
            painter.text(heap_data_rect.center(), egui::Align2::CENTER_CENTER, "['H','o','l','a']", egui::FontId::monospace(16.0), egui::Color32::BLACK);

            let start = var_s_rect.right_center();
            let end = heap_data_rect.left_center();
            let control1 = start + egui::vec2(50.0, 0.0);
            let control2 = end - egui::vec2(50.0, 0.0);

            painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
                [start, control1, control2, end],
                false,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(3.0, egui::Color32::YELLOW)
            ));
            painter.circle_filled(end, 6.0, egui::Color32::YELLOW);
        }
    });
}

fn mostrar_graficos(ui: &mut egui::Ui, state: &mut PortfolioState) {
    let base_multiplier = (state.year - 2020) as f64 * 100.0;
    
    let mut ingresos_bars = vec![];
    let mut gastos_bars = vec![];
    let mut beneficios_bars = vec![];

    for i in 1..=12 {
        let x = i as f64;
        let ingreso = base_multiplier + 200.0 + (i as f64 * 40.0) + (i % 3) as f64 * 50.0;
        let gasto = base_multiplier + 150.0 + (i as f64 * 25.0) + (i % 2) as f64 * 30.0;
        let beneficio = ingreso - gasto;

        ingresos_bars.push(Bar::new(x - 0.25, ingreso).width(0.2).fill(egui::Color32::from_rgb(60, 200, 120)));
        gastos_bars.push(Bar::new(x, gasto).width(0.2).fill(egui::Color32::from_rgb(220, 90, 90)));
        beneficios_bars.push(Bar::new(x + 0.25, beneficio).width(0.2).fill(egui::Color32::from_rgb(80, 150, 250)));
    }

    Plot::new("bar_chart_pro")
        .legend(Legend::default().position(egui_plot::Corner::RightBottom))
        .view_aspect(2.5)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .show_grid([false, false])
        .show(ui, |plot_ui| {
            if state.show_ingresos { plot_ui.bar_chart(BarChart::new("Ingresos", ingresos_bars).color(egui::Color32::from_rgb(60, 200, 120))); }
            if state.show_gastos { plot_ui.bar_chart(BarChart::new("Gastos", gastos_bars).color(egui::Color32::from_rgb(220, 90, 90))); }
            if state.show_beneficios { plot_ui.bar_chart(BarChart::new("Beneficios", beneficios_bars).color(egui::Color32::from_rgb(80, 150, 250))); }
        });

    let mut overlay_frame = egui::Frame::window(ui.style());
    overlay_frame.fill = egui::Color32::from_black_alpha(200); 

    egui::Window::new("Filtros Flotantes")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(240.0, 20.0))
        .title_bar(false)
        .resizable(false)
        .frame(overlay_frame)
        .show(ui.ctx(), |ui_overlay| {
            ui_overlay.add_space(5.0);
            ui_overlay.heading("Filtros Interactivos");
            
            ui_overlay.horizontal(|ui_h| {
                ui_h.checkbox(&mut state.show_ingresos, "Ingresos");
                ui_h.checkbox(&mut state.show_gastos, "Gastos");
                ui_h.checkbox(&mut state.show_beneficios, "Beneficios");
            });
            ui_overlay.add_space(10.0);
            ui_overlay.add(egui::Slider::new(&mut state.year, 2022..=2026).text("Año"));
        });
}

// ==========================================
// RUST FERRIS MEMORY GAME INTEGRATION
// ==========================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CardType {
    Coder,
    Wizard,
    Pirate,
    Astronaut,
}

impl CardType {
    pub fn all() -> Vec<CardType> {
        vec![
            CardType::Coder,
            CardType::Wizard,
            CardType::Pirate,
            CardType::Astronaut,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CardState {
    FaceDown,
    FaceUp,
    Matched,
}

#[derive(Component)]
pub struct Card {
    pub id: usize,
    pub card_type: CardType,
    pub state: CardState,
}

#[derive(Resource)]
pub struct TextureHandles {
    pub back: Handle<Image>,
    pub coder: Handle<Image>,
    pub wizard: Handle<Image>,
    pub pirate: Handle<Image>,
    pub astronaut: Handle<Image>,
}

impl TextureHandles {
    pub fn get_handle(&self, card_type: CardType) -> Handle<Image> {
        match card_type {
            CardType::Coder => self.coder.clone(),
            CardType::Wizard => self.wizard.clone(),
            CardType::Pirate => self.pirate.clone(),
            CardType::Astronaut => self.astronaut.clone(),
        }
    }
}

#[derive(Resource, Default)]
pub struct MemoryGameState {
    pub selected_cards: Vec<Entity>,
    pub waiting_timer: Option<Timer>,
    pub moves: u32,
    pub matched_pairs: u32,
    pub is_won: bool,
}

#[derive(Component)]
pub struct MoveCounterText;

#[derive(Component)]
pub struct WinMessageText;

#[derive(Component)]
pub struct MemoryGameElement;

const CARD_WIDTH: f32 = 130.0;
const CARD_HEIGHT: f32 = 130.0;
const CARD_GAP: f32 = 25.0;

fn setup_memory_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D Camera
    commands.spawn((
        Camera2d,
        Camera {
            order: 1, // Render on top of 3D
            ..default()
        },
        MemoryGameElement,
    ));

    // Textures (.png)
    let textures = TextureHandles {
        back: asset_server.load("textures/card_back.png"),
        coder: asset_server.load("textures/ferris_coder.png"),
        wizard: asset_server.load("textures/ferris_wizard.png"),
        pirate: asset_server.load("textures/ferris_pirate.png"),
        astronaut: asset_server.load("textures/ferris_astronaut.png"),
    };

    // Create 4 pairs (8 cards total)
    let mut card_deck = Vec::new();
    for card_type in CardType::all() {
        card_deck.push(card_type);
        card_deck.push(card_type);
    }

    let mut rng = rand::thread_rng();
    card_deck.shuffle(&mut rng);

    let cols = 4;
    let rows = 2;
    let total_width = (cols as f32) * CARD_WIDTH + ((cols - 1) as f32) * CARD_GAP;
    let total_height = (rows as f32) * CARD_HEIGHT + ((rows - 1) as f32) * CARD_GAP;

    let start_x = -total_width / 2.0 + CARD_WIDTH / 2.0;
    let start_y = total_height / 2.0 - CARD_HEIGHT / 2.0 - 30.0;

    for (index, &card_type) in card_deck.iter().enumerate() {
        let col = index % cols;
        let row = index / cols;

        let x = start_x + (col as f32) * (CARD_WIDTH + CARD_GAP);
        let y = start_y - (row as f32) * (CARD_HEIGHT + CARD_GAP);

        commands.spawn((
            Card {
                id: index,
                card_type,
                state: CardState::FaceDown,
            },
            Sprite {
                image: textures.back.clone(),
                custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
            MemoryGameElement,
        ));
    }

    commands.insert_resource(textures);

    // UI Text - Header
    commands.spawn((
        Text::new("🦀 FERRIS MEMORY MATCH 🦀"),
        TextFont {
            font_size: FontSize::Px(38.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.6, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(25.0),
            left: Val::Percent(30.0),
            ..default()
        },
        MemoryGameElement,
    ));

    // UI Text - Move Counter
    commands.spawn((
        Text::new("Movimientos: 0"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Percent(42.0),
            ..default()
        },
        MoveCounterText,
        MemoryGameElement,
    ));

    // UI Text - Win Banner
    commands.spawn((
        Text::new("🎉 ¡FELICITACIONES! ¡HAS ENCONTRADO A TODOS LOS FERRIS! 🎉"),
        TextFont {
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.3, 0.9, 0.4)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(35.0),
            left: Val::Percent(10.0),
            ..default()
        },
        Visibility::Hidden,
        WinMessageText,
        MemoryGameElement,
    ));
}

fn toggle_memory_game_visibility(
    state: Res<PortfolioState>,
    mut q: Query<&mut Visibility, With<MemoryGameElement>>,
) {
    let is_memory = state.ruta_actual == AppRoute::JuegoMemoriaFerris;
    for mut vis in q.iter_mut() {
        if is_memory {
            if *vis == Visibility::Hidden {
                *vis = Visibility::Inherited;
            }
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

fn card_click_system(
    state: Res<PortfolioState>,
    mut contexts: EguiContexts,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MemoryGameElement>>,
    mut card_q: Query<(Entity, &mut Card, &Transform)>,
    mut game_state: ResMut<MemoryGameState>,
) {
    if state.ruta_actual != AppRoute::JuegoMemoriaFerris {
        return;
    }
    
    // No pointer input check needed here

    if game_state.waiting_timer.is_some() || game_state.is_won {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.single() else {
            return;
        };
        let Ok((camera, camera_transform)) = camera_q.single() else {
            return;
        };

        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(world_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            {
                for (entity, mut card, transform) in card_q.iter_mut() {
                    if card.state != CardState::FaceDown {
                        continue;
                    }

                    let card_pos = transform.translation.truncate();
                    let half_width = CARD_WIDTH / 2.0;
                    let half_height = CARD_HEIGHT / 2.0;

                    if world_position.x >= card_pos.x - half_width
                        && world_position.x <= card_pos.x + half_width
                        && world_position.y >= card_pos.y - half_height
                        && world_position.y <= card_pos.y + half_height
                    {
                        card.state = CardState::FaceUp;
                        game_state.selected_cards.push(entity);

                        if game_state.selected_cards.len() == 2 {
                            game_state.moves += 1;
                            game_state.waiting_timer =
                                Some(Timer::from_seconds(0.8, TimerMode::Once));
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn match_check_system(
    time: Res<Time>,
    mut game_state: ResMut<MemoryGameState>,
    mut card_q: Query<(Entity, &mut Card)>,
) {
    if let Some(ref mut timer) = game_state.waiting_timer {
        timer.tick(time.delta());

        if timer.just_finished() {
            game_state.waiting_timer = None;

            if game_state.selected_cards.len() == 2 {
                let entity1 = game_state.selected_cards[0];
                let entity2 = game_state.selected_cards[1];

                let mut card1_type = None;
                let mut card2_type = None;

                for (entity, card) in card_q.iter() {
                    if entity == entity1 {
                        card1_type = Some(card.card_type);
                    }
                    if entity == entity2 {
                        card2_type = Some(card.card_type);
                    }
                }

                if let (Some(t1), Some(t2)) = (card1_type, card2_type) {
                    if t1 == t2 {
                        for (entity, mut card) in card_q.iter_mut() {
                            if entity == entity1 || entity == entity2 {
                                card.state = CardState::Matched;
                            }
                        }
                        game_state.matched_pairs += 1;
                        if game_state.matched_pairs == 4 {
                            game_state.is_won = true;
                        }
                    } else {
                        for (entity, mut card) in card_q.iter_mut() {
                            if entity == entity1 || entity == entity2 {
                                card.state = CardState::FaceDown;
                            }
                        }
                    }
                }

                game_state.selected_cards.clear();
            }
        }
    }
}

fn update_card_sprites(
    textures: Res<TextureHandles>,
    mut card_q: Query<(&Card, &mut Sprite), Changed<Card>>,
) {
    for (card, mut sprite) in card_q.iter_mut() {
        match card.state {
            CardState::FaceDown => {
                sprite.image = textures.back.clone();
            }
            CardState::FaceUp | CardState::Matched => {
                sprite.image = textures.get_handle(card.card_type);
            }
        }
    }
}

fn ui_update_system(
    state: Res<PortfolioState>,
    game_state: Res<MemoryGameState>,
    mut move_text_q: Query<&mut Text, (With<MoveCounterText>, Without<WinMessageText>)>,
    mut win_text_q: Query<&mut Visibility, With<WinMessageText>>,
) {
    if state.ruta_actual != AppRoute::JuegoMemoriaFerris {
        return;
    }

    if game_state.is_changed() {
        for mut text in move_text_q.iter_mut() {
            **text = format!("Movimientos: {}", game_state.moves);
        }

        if game_state.is_won {
            for mut visibility in win_text_q.iter_mut() {
                *visibility = Visibility::Inherited;
            }
        }
    }
}
