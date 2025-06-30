use bevy::{core::FrameCount, prelude::*, window::WindowMode};
use rand::prelude::*;
const MOVEMENT_TIMESTEP: f64 = 0.3;

const CLEAR: Color = Color::srgb(0.1, 0.1, 0.1);

const SNAKE_COLOR: Color = Color::srgb(0.3, 0.9, 0.3);
const APPLE_COLOR: Color = Color::srgb(0.9, 0.1, 0.1);

const CELL_SIZE: f32 = 30.0;
const GRID_WIDTH: f32 = 20.0 + 1.;
const GRID_HEIGHT: f32 = 15.0;

#[derive(Resource, Default)]
struct Score {
    value: i32,
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Component)]
struct Snake {
    curr_direction: Direction,
    next_direction: Direction,
    length: i32,
    positions: Vec<Vec2>,
}

#[derive(Component)]
struct SnakeBody;

#[derive(Component)]
struct Apple;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
struct GameOverMenu;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Playing,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(MOVEMENT_TIMESTEP))
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Score { value: 0 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (CELL_SIZE * GRID_WIDTH, CELL_SIZE * GRID_HEIGHT).into(),
                title: "Snak".to_string().into(),
                mode: WindowMode::Windowed,
                visible: false,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (spawn_camera, spawn_ui, spawn_snake, |commands: Commands| {
                spawn_apple(commands, None)
            }),
        )
        .add_systems(
            Update,
            (
                make_visible,
                pause_input,
                snake_eating.run_if(in_state(GameState::Playing)),
                snake_movement_input.run_if(in_state(GameState::Playing)),
                snake_body.run_if(in_state(GameState::Playing)),
                snake_self_collision_check.run_if(in_state(GameState::Playing)),
                update_score_text,
            ),
        )
        .add_systems(
            FixedUpdate,
            snake_movement.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            game_over_input.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
        .add_systems(OnExit(GameState::Paused), cleanup_pause_menu)
        .add_systems(
            OnEnter(GameState::GameOver),
            (spawn_game_over_menu, cleanup_ui, cleanup_game_entities),
        )
        .add_systems(
            OnExit(GameState::GameOver),
            (
                cleanup_game_over_menu,
                restart_game,
                spawn_ui,
                spawn_snake,
                |commands: Commands| spawn_apple(commands, None),
            ),
        )
        .run();
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}

fn spawn_ui(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                score.value.to_string(),
                TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
}

fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<ScoreText>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "PAUSED\nESC to resume",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            bottom: Val::Px(10.0),
            ..default()
        }),
        PauseMenu,
    ));
}

fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn spawn_game_over_menu(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        TextBundle::from_section(
            format!("GAME OVER\nFinal Score: {}\nR to restart", score.value),
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        }),
        GameOverMenu,
    ));
}

fn cleanup_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn cleanup_game_entities(
    mut commands: Commands,
    snake: Query<Entity, With<Snake>>,
    body_query: Query<Entity, With<SnakeBody>>,
    apple_query: Query<Entity, With<Apple>>,
) {
    for entity in snake
        .iter()
        .chain(body_query.iter())
        .chain(apple_query.iter())
    {
        commands.entity(entity).despawn();
    }
}

fn pause_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let current_state = state.get();
        match current_state {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

fn game_over_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}

fn restart_game(mut score: ResMut<Score>) {
    score.value = 0;
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            text.sections[1].value = format!("{}", score.value);
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, 1.),
                ..default()
            },
            ..default()
        },
        Snake {
            curr_direction: Direction::None,
            next_direction: Direction::None,
            length: 1,
            positions: vec![Vec2::new(0., 0.)],
        },
    ));
}

fn spawn_apple(mut commands: Commands, snake_positions: Option<&Vec<Vec2>>) {
    let mut rng = rand::thread_rng();
    let mut new_pos;

    loop {
        new_pos = Vec2::new(
            rng.gen_range(-(GRID_WIDTH / 2.) as i32..(GRID_WIDTH / 2.) as i32) as f32,
            rng.gen_range(-(GRID_HEIGHT / 2.) as i32..(GRID_HEIGHT / 2.) as i32) as f32,
        );

        if let Some(positions) = snake_positions {
            if !positions.contains(&new_pos) {
                break;
            }
        } else {
            break;
        }
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: APPLE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, 1.),
                translation: Vec3::new(new_pos.x * CELL_SIZE, new_pos.y * CELL_SIZE, 0.2),
                ..default()
            },
            ..default()
        },
        Apple,
    ));
}

fn snake_eating(
    mut commands: Commands,
    mut snake: Query<(&Transform, &mut Snake), With<Snake>>,
    mut apple: Query<(Entity, &Transform), (With<Apple>, Without<Snake>)>,
    mut score: ResMut<Score>,
) {
    let (snake_transform, mut snake) = snake.single_mut();
    let (apple_entity, apple_transform) = apple.single_mut();

    if (snake_transform.translation.x == apple_transform.translation.x)
        && (snake_transform.translation.y == apple_transform.translation.y)
    {
        snake.length += 1;
        score.value += 1;
        commands.entity(apple_entity).despawn();
        spawn_apple(commands, Some(&snake.positions));
    }
}

fn snake_body(
    snake: Query<&Snake>,
    mut commands: Commands,
    mut body_query: Query<Entity, With<SnakeBody>>,
) {
    let snake = snake.single();

    for entity in body_query.iter_mut() {
        commands.entity(entity).despawn();
    }

    if snake.length > 0 {
        for i in 0..snake.positions.len() {
            if let Some(position) = snake.positions.get(i) {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: SNAKE_COLOR,
                            ..default()
                        },
                        transform: Transform {
                            scale: Vec3::new(CELL_SIZE - 3., CELL_SIZE - 3., 1.),
                            translation: Vec3::new(
                                position.x as f32 * CELL_SIZE,
                                position.y as f32 * CELL_SIZE,
                                0.1,
                            ),
                            ..default()
                        },
                        ..default()
                    },
                    SnakeBody,
                ));
            }
        }
    }
}

fn snake_self_collision_check(
    snake: Query<(&Transform, &Snake)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (snake_transform, snake) = snake.single();

    let head_pos = Vec2::new(
        snake_transform.translation.x / CELL_SIZE,
        snake_transform.translation.y / CELL_SIZE,
    );

    if snake.positions.len() > 2 {
        for i in 0..snake.positions.len() - 1 {
            if snake.positions[i] == head_pos {
                next_state.set(GameState::GameOver);
            }
        }
    }
}

fn snake_movement_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut snake: Query<&mut Snake>) {
    let mut snake = snake.single_mut();

    if snake.curr_direction != Direction::Down
        && snake.next_direction != Direction::Down
        && (keyboard_input.just_pressed(KeyCode::KeyW)
            || keyboard_input.just_pressed(KeyCode::ArrowUp))
    {
        snake.next_direction = Direction::Up;
    }
    if snake.curr_direction != Direction::Up
        && snake.next_direction != Direction::Up
        && (keyboard_input.just_pressed(KeyCode::KeyS)
            || keyboard_input.just_pressed(KeyCode::ArrowDown))
    {
        snake.next_direction = Direction::Down;
    }
    if snake.curr_direction != Direction::Right
        && snake.next_direction != Direction::Right
        && (keyboard_input.just_pressed(KeyCode::KeyA)
            || keyboard_input.just_pressed(KeyCode::ArrowLeft))
    {
        snake.next_direction = Direction::Left;
    }
    if snake.curr_direction != Direction::Left
        && snake.next_direction != Direction::Left
        && (keyboard_input.just_pressed(KeyCode::KeyD)
            || keyboard_input.just_pressed(KeyCode::ArrowRight))
    {
        snake.next_direction = Direction::Right;
    }
}

fn snake_movement(
    mut snake: Query<(&mut Transform, &mut Snake), With<Snake>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut snake_transform, mut snake) = snake.single_mut();

    if snake.next_direction != Direction::None {
        snake.curr_direction = snake.next_direction;
        snake.next_direction = Direction::None;
    }

    if snake.curr_direction == Direction::Up {
        snake_transform.translation.y += CELL_SIZE;
    }
    if snake.curr_direction == Direction::Down {
        snake_transform.translation.y -= CELL_SIZE;
    }
    if snake.curr_direction == Direction::Left {
        snake_transform.translation.x -= CELL_SIZE;
    }
    if snake.curr_direction == Direction::Right {
        snake_transform.translation.x += CELL_SIZE;
    }

    if snake_transform.translation.x > GRID_WIDTH / 2.0 * CELL_SIZE
        || snake_transform.translation.y > GRID_HEIGHT / 2.0 * CELL_SIZE
        || snake_transform.translation.x < -GRID_WIDTH / 2.0 * CELL_SIZE
        || snake_transform.translation.y < -GRID_HEIGHT / 2.0 * CELL_SIZE
    {
        next_state.set(GameState::GameOver);
    }

    let head_pos = Vec2::new(
        snake_transform.translation.x / CELL_SIZE,
        snake_transform.translation.y / CELL_SIZE,
    );

    if snake.positions.last() != Some(&head_pos) {
        snake.positions.push(head_pos);
    }

    while snake.positions.len() > snake.length as usize {
        snake.positions.remove(0);
    }
}
