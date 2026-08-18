use bevy::prelude::*;
use rand::seq::SliceRandom;

fn main() {
    println!("🦀 Iniciando Ferris Memory Match...");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🦀 Ferris Memory Match 🦀".to_string(),
                resolution: (900, 700).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .insert_resource(GameState::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                card_click_system,
                match_check_system,
                update_card_sprites,
                ui_update_system,
            ),
        )
        .run();
}

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
pub struct GameState {
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

const CARD_WIDTH: f32 = 130.0;
const CARD_HEIGHT: f32 = 130.0;
const CARD_GAP: f32 = 25.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("🎮 Setup: Generando 2D Camera y Tablero de cartas...");

    // 2D Camera
    commands.spawn(Camera2d);

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
    ));

    println!("✅ Setup completado. 8 cartas desplegadas.");
}

fn card_click_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut card_q: Query<(Entity, &mut Card, &Transform)>,
    mut game_state: ResMut<GameState>,
) {
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
                        println!("🃏 Clic en carta {} (Tipo: {:?})", card.id, card.card_type);
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
    mut game_state: ResMut<GameState>,
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
                        println!("✨ Pareja encontrada! ({:?})", t1);
                        for (entity, mut card) in card_q.iter_mut() {
                            if entity == entity1 || entity == entity2 {
                                card.state = CardState::Matched;
                            }
                        }
                        game_state.matched_pairs += 1;
                        if game_state.matched_pairs == 4 {
                            println!("🏆 ¡Juego completado!");
                            game_state.is_won = true;
                        }
                    } else {
                        println!("❌ No coinciden ({:?} vs {:?})", t1, t2);
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
    game_state: Res<GameState>,
    mut move_text_q: Query<&mut Text, (With<MoveCounterText>, Without<WinMessageText>)>,
    mut win_text_q: Query<&mut Visibility, With<WinMessageText>>,
) {
    if game_state.is_changed() {
        for mut text in move_text_q.iter_mut() {
            **text = format!("Movimientos: {}", game_state.moves);
        }

        if game_state.is_won {
            for mut visibility in win_text_q.iter_mut() {
                *visibility = Visibility::Visible;
            }
        }
    }
}
