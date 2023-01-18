use bevy::prelude::*;
fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
#[derive(Component)]
struct SnakeHead;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
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
        .insert(SnakeHead);
}
fn snake_movement(key: Res<Input<KeyCode>>, mut heads: Query<&mut Transform, With<SnakeHead>>) {
    for mut tx in heads.iter_mut() {
        if key.pressed(KeyCode::Left) {
            tx.translation.x -= 2.0;
        }
        if key.pressed(KeyCode::Right) {
            tx.translation.x += 2.0;
        }
        if key.pressed(KeyCode::Down) {
            tx.translation.y -= 2.0;
        }
        if key.pressed(KeyCode::Up) {
            tx.translation.y += 2.0;
        }
    }
}
