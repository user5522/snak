use bevy::{
    core::FrameCount,
    prelude::*,
    time::{Fixed, Time},
};
use rand::prelude::*;
use std::process;

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
const SNAKE_COLOR: Color = Color::rgb(0.3, 0.9, 0.3);
const APPLE_COLOR: Color = Color::rgb(0.9, 0.1, 0.1);
const MOVEMENT_TIMESTEP: f64 = 0.35;

const CELL_SIZE: f32 = 30.0;
const GRID_WIDTH: f32 = 20.0;
const GRID_HEIGHT: f32 = 15.0;

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
    length: i32,
    positions: Vec<Vec2>,
}

#[derive(Component)]
struct Apple;

#[derive(Component)]
struct SnakeBody;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(MOVEMENT_TIMESTEP))
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (CELL_SIZE * (GRID_WIDTH + 1.), CELL_SIZE * GRID_HEIGHT).into(),
                title: "Snak".to_string().into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                visible: false,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_apple, spawn_snake))
        .add_systems(
            Update,
            (
                make_visible,
                snake_eating,
                snake_movement_input.before(snake_movement),
                snake_body,
                snake_self_collision_check,
            ),
        )
        .add_systems(FixedUpdate, snake_movement)
        .run();
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
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
            length: 1,
            positions: vec![Vec2::new(0., 0.)],
        },
    ));
}

fn spawn_apple(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: APPLE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, 1.),
                translation: Vec3::new(
                    rng.gen_range(-(GRID_WIDTH / 2.) as i32..(GRID_WIDTH / 2.) as i32) as f32
                        * CELL_SIZE,
                    rng.gen_range(-(GRID_HEIGHT / 2.) as i32..(GRID_HEIGHT / 2.) as i32) as f32
                        * CELL_SIZE,
                    0.2,
                ),
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
) {
    let (snake_transform, mut snake) = snake.single_mut();
    let (apple_entity, apple_transform) = apple.single_mut();

    if (snake_transform.translation.x == apple_transform.translation.x)
        && (snake_transform.translation.y == apple_transform.translation.y)
    {
        snake.length += 1;
        commands.entity(apple_entity).despawn();
        spawn_apple(commands);
    }
}

fn snake_body(
    snake: Query<&Snake>,
    mut commands: Commands,
    mut query: Query<Entity, With<SnakeBody>>,
) {
    let snake = snake.single();

    for entity in query.iter_mut() {
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

fn snake_self_collision_check(snake: Query<(&Transform, &Snake)>) {
    let (snake_transform, snake) = snake.single();

    let head_pos = Vec2::new(
        snake_transform.translation.x / CELL_SIZE,
        snake_transform.translation.y / CELL_SIZE,
    );

    if snake.positions.len() > 2 {
        for i in 0..snake.positions.len() - 1 {
            if snake.positions[i] == head_pos {
                process::exit(0x0100);
            }
        }
    }
}

fn snake_movement_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut snake: Query<&mut Snake>) {
    let mut snake = snake.single_mut();

    if snake.curr_direction != Direction::Down
        && (keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp))
    {
        snake.curr_direction = Direction::Up;
    }
    if snake.curr_direction != Direction::Up
        && (keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown))
    {
        snake.curr_direction = Direction::Down;
    }
    if snake.curr_direction != Direction::Right
        && (keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft))
    {
        snake.curr_direction = Direction::Left;
    }
    if snake.curr_direction != Direction::Left
        && (keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight))
    {
        snake.curr_direction = Direction::Right;
    }
}

fn snake_movement(mut snake: Query<(&mut Transform, &mut Snake), With<Snake>>) {
    let (mut snake_transform, mut snake) = snake.single_mut();

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

    if snake_transform.translation.x > GRID_WIDTH / 2.0 * CELL_SIZE {
        process::exit(0x0100);
    }
    if snake_transform.translation.y > GRID_HEIGHT / 2.0 * CELL_SIZE {
        process::exit(0x0100);
    }
    if snake_transform.translation.x < -GRID_WIDTH / 2.0 * CELL_SIZE {
        process::exit(0x0100);
    }
    if snake_transform.translation.y < -GRID_HEIGHT / 2.0 * CELL_SIZE {
        process::exit(0x0100);
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
