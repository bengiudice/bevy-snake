use bevy::prelude::*;
fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
#[derive(Component)]
struct SnakeHead;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite, mut tx) in q.iter_mut() {
        tx.scale = Vec3::new(
            sprite.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite.height / ARENA_HEIGHT as f32 * window.height(),
            1.0,
        );
    }
}
fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - bound_window / 2.0 + tile_size / 2.0
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
fn snake_movement(key: Res<Input<KeyCode>>, mut head_positions: Query<&mut Position, With<SnakeHead>>) {
    for mut pos in head_positions.iter_mut() {
        if key.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if key.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if key.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if key.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}
