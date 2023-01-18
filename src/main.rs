use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::prelude::random;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut, Resource)]
struct SnakeSegments(Vec<Entity>);

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct Food;

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

struct GrowthEvent;

struct GameOverEvent;

#[derive(Default, Resource)]
struct LastTailPosition(Option<Position>);

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn snake_eating(
    mut cmds: Commands,
    mut grow_evw: EventWriter<GrowthEvent>,
    food_pos: Query<(Entity, &Position), With<Food>>,
    head_pos: Query<&Position, With<SnakeHead>>,
) {
    for head in head_pos.iter() {
        for (ent, food) in food_pos.iter() {
            if food == head {
                cmds.entity(ent).despawn();
                grow_evw.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    cmds: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_ev: EventReader<GrowthEvent>,
) {
    if growth_ev.iter().next().is_some() {
        segments.push(spawn_segment(cmds, last_tail_position.0.unwrap()));
    }
}

fn food_spawner(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
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

fn snake_movement_input(key: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    let heads = heads.iter_mut().next();
    if heads.is_none() {
        return;
    }
    let mut head = heads.unwrap();
    let mut dir = head.direction;
    if key.pressed(KeyCode::Left) {
        dir = Direction::Left;
    }
    if key.pressed(KeyCode::Right) {
        dir = Direction::Right;
    }
    if key.pressed(KeyCode::Down) {
        dir = Direction::Down;
    }
    if key.pressed(KeyCode::Up) {
        dir = Direction::Up;
    }
    if dir != head.direction.opposite() {
        head.direction = dir;
    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_evw: EventWriter<GameOverEvent>,
) {
    let heads = heads.iter_mut().next();
    if heads.is_none() {
        return;
    }
    let (head_entity, head) = heads.unwrap();
    let segment_positions = segments
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();
    let mut head_pos = positions.get_mut(head_entity).unwrap();
    match &head.direction {
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
        Direction::Down => head_pos.y -= 1,
        Direction::Up => head_pos.y += 1,
    };
    if head_pos.x < 0
        || head_pos.y < 0
        || head_pos.x as u32 >= ARENA_WIDTH
        || head_pos.y as u32 >= ARENA_HEIGHT
    {
        game_over_evw.send(GameOverEvent);
    }
    if segment_positions.contains(&head_pos) {
        game_over_evw.send(GameOverEvent);
    }
    segment_positions
        .iter()
        .zip(segments.iter().skip(1))
        .for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });
    *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
}

fn game_over(
    mut cmds: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            cmds.entity(ent).despawn();
        }
        spawn_snake(cmds, segments_res);
    }
}

fn spawn_segment(mut cmds: Commands, pos: Position) -> Entity {
    cmds.spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..default()
        },
        ..default()
    })
    .insert(SnakeSegment)
    .insert(pos)
    .insert(Size::square(0.65))
    .id()
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner),
        )
        .add_system(snake_movement_input.before(snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.15))
                .with_system(snake_movement)
                .with_system(snake_eating.after(snake_movement))
                .with_system(snake_growth.after(snake_eating)),
        )
        .add_system(game_over.after(snake_movement))
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Snake!".to_string(),
                width: 500.0,
                height: 500.0,
                ..default()
            },
            ..default()
        }))
        .run();
}
