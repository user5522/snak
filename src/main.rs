use bevy::{core::FixedTimestep, prelude::*};
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
    positions: Vec<(i32, i32)>,
}

#[derive(Component)]
struct Apple;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: CELL_SIZE * GRID_WIDTH,
            height: CELL_SIZE * GRID_HEIGHT,
            title: "Snak".to_string(),
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_apple)
        .add_startup_system(spawn_snake)
        .add_system(snake_eating)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(MOVEMENT_TIMESTEP))
                .with_system(snake_movement),
        )
        .add_system(snake_movement_input.before(snake_movement))
        .add_system(snake_body)
        .run();
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut snake: Query<&mut Snake>) {
    let mut snake = snake.single_mut();

    if snake.curr_direction != Direction::Down && keyboard_input.pressed(KeyCode::Z) {
        snake.curr_direction = Direction::Up;
    }
    if snake.curr_direction != Direction::Up && keyboard_input.pressed(KeyCode::S) {
        snake.curr_direction = Direction::Down;
    }
    if snake.curr_direction != Direction::Right && keyboard_input.pressed(KeyCode::Q) {
        snake.curr_direction = Direction::Left;
    }
    if snake.curr_direction != Direction::Left && keyboard_input.pressed(KeyCode::D) {
        snake.curr_direction = Direction::Right;
    }
}

fn snake_movement(mut snake: Query<(&mut Transform, &mut Snake), With<Snake>>) {
    let (mut transform, snake) = snake.single_mut();

    if snake.curr_direction == Direction::Up {
        transform.translation.y += CELL_SIZE;
    }
    if snake.curr_direction == Direction::Down {
        transform.translation.y -= CELL_SIZE;
    }
    if snake.curr_direction == Direction::Left {
        transform.translation.x -= CELL_SIZE;
    }
    if snake.curr_direction == Direction::Right {
        transform.translation.x += CELL_SIZE;
    }

    if transform.translation.x > GRID_WIDTH / 2.0 * CELL_SIZE {
        process::exit(0x0100);
    }
    if transform.translation.y > GRID_HEIGHT / 2.0 * CELL_SIZE {
        process::exit(0x0100);
    }
    if transform.translation.x < -GRID_WIDTH / 2.0 * CELL_SIZE {
        process::exit(0x0100);
    }
    if transform.translation.y < -GRID_HEIGHT / 2.0 * CELL_SIZE {
        process::exit(0x0100);
    }
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, -9.),
                ..default()
            },
            ..default()
        })
        .insert(Snake {
            curr_direction: Direction::None,
            length: 3,
            positions: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        });
}

fn snake_body(
    mut commands: Commands,
    snake: Query<&Snake>,
) {
    let snake = snake.single();
    for i in 0..snake.length {
        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE - 3., CELL_SIZE - 3., -9.),
                ..default()
            },
                ..default()
        });
    }
}

// fn snake_parts_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut snake: Query<&mut Snake>
// ) {
//     let mut snake = snake.single_mut();
//     let prev_pos = snake.positions.clone();
//     for i in 0..snake.positions.len() {
//         if i == 0 {
//             if(matches!(snake.curr_direction, Direction::Left) && keyboard_input.pressed(KeyCode::D)) {
//                 snake.positions[i].0 += 1;
//                 snake.curr_direction = Direction::Right;
//             }
//             if(matches!(snake.curr_direction, Direction::Right) && keyboard_input.pressed(KeyCode::Q)) {
//                 snake.positions[i].0 -= 1;
//                 snake.curr_direction = Direction::Left;
//             }
//             if(matches!(snake.curr_direction, Direction::Down) && keyboard_input.pressed(KeyCode::Z)) {
//                 snake.positions[i].1 += 1;
//                 snake.curr_direction = Direction::Up;
//             }
//             if(matches!(snake.curr_direction, Direction::Up) && keyboard_input.pressed(KeyCode::S)) {
//                 snake.positions[i].1 -= 1;
//                 snake.curr_direction = Direction::Down;
//             }
//         } else {
//             snake.positions[i] = prev_pos[i - 1];
//         }
//     }

// snake.positions.push((prev_pos[prev_pos.len() - 1].0, prev_pos[prev_pos.len() - 1].1));
// }

fn spawn_apple(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: APPLE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, -9.),
                translation: Vec3::new(
                    rng.gen_range(-((GRID_WIDTH / 2.) - 1.) as i32..((GRID_WIDTH / 2.) - 1.) as i32)
                        as f32
                        * CELL_SIZE,
                    rng.gen_range(
                        -((GRID_HEIGHT / 2.) - 1.) as i32..((GRID_HEIGHT / 2.) - 1.) as i32,
                    ) as f32
                        * CELL_SIZE,
                    10.,
                ),
                ..default()
            },
            ..default()
        })
        .insert(Apple);
}

fn snake_eating(
    mut snake: Query<(&mut Transform, &mut Snake), With<Snake>>,
    mut apple: Query<&mut Transform, (Without<Snake>, With<Apple>)>,
) {
    let (snake_transform, mut snake) = snake.single_mut();
    let mut appl_transform = apple.single_mut();
    let mut rng = rand::thread_rng();
    if snake_transform.translation.x == appl_transform.translation.x
        && snake_transform.translation.y == appl_transform.translation.y
    {
        snake.length += 1;
        appl_transform.translation.x =
            rng.gen_range(-((GRID_WIDTH / 2.) - 1.) as i32..((GRID_WIDTH / 2.) - 1.) as i32) as f32
                * CELL_SIZE;
        appl_transform.translation.y = rng
            .gen_range(-((GRID_HEIGHT / 2.) - 1.) as i32..((GRID_HEIGHT / 2.) - 1.) as i32)
            as f32
            * CELL_SIZE;
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;

    commands.spawn_bundle(camera).insert(MainCamera);
}


// Why do I have to pay fucking taxes? They're fucking retarded. "Oh you have to pay ur money 4 roads an hospitals".
// shut the fuck up.
// 1 i didn't pass my goddam driving test because the instructor was a fucking disgusting whore who wouldn't be able to see my 3inch cock if it was in her face,
// and 2 i learnt first aid in middleschool and i have painkillers in my moms bathroom.
// So yeah, i don't need it your stupid ass roads or hospital shit.
// And what the fuck are they supposed to do if i don't pay, huh? Tax me more? You mean make me pay more taxes that i'm already not fucking paying?
// I bet you feel like a fucking buffoon right now. Yeah ok, you're gonna put me in jail? huh? yeah? you wanna put me in fucking jail and slap me on my abnormally small wrists big boy? yeah?
// make me eat ass every day? yeah? suckle on my nips for six hours straight maybe? u-huh? you would like that wouldn't you?
// oh, you mean the jail that is paid for by the taxes? you mean the taxes i'm not paying?!?!?!? Yeaheheah, bitch! see? you can't do shit. try, bro, fucking TRY to tax this juicy ass. i'm invincible. suck my dick bro.
//  yeah. eat my ass cunt. frenchkiss my goddam taint and smear my glans with hotsauce fucker. fuck me til you love me bitch ass head-ass fucking bitch. cum on my face. fuck you. see if i care.