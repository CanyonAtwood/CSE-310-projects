#[derive(Component)]
struct SnakeHead;
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0); // <--
fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
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
.add_startup_system(setup_camera)
.add_startup_system(spawn_snake) // <--

fn snake_movement(mut head_positions: Query<(&SnakeHead, &mut Transform)>) {
    for (_head, mut transform) in head_positions.iter_mut() {
        transform.translation.y += 2.;
    }
}
.add_system(snake_movement) // <--
.add_plugins(DefaultPlugins)

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}


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

commands
    .spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..default()
        },
        ..default()
    })
    .insert(SnakeHead)
    .insert(Position { x: 3, y: 3 }) // <--
    .insert(Size::square(0.8)); // <--

    fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
        let window = windows.get_primary().unwrap();
        for (sprite_size, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
                1.0,
            );
        }
    }

    fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
        let window = windows.get_primary().unwrap();
        for (sprite_size, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
                1.0,
            );
        }
    }

    .add_system(snake_movement)
.add_system_set_to_stage(
    CoreStage::PostUpdate,
    SystemSet::new()
        .with_system(position_translation)
        .with_system(size_scaling),
)
.add_plugins(DefaultPlugins)
.run();

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}

App::new()
        .insert_resource(WindowDescriptor { // <--
            title: "Snake!".to_string(), // <--
            width: 500.0,                 // <--
            height: 500.0,                // <--
            ..default()         // <--
        })
        .add_startup_system(setup_camera)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        use rand::prelude::random;
        use bevy::core::FixedTimestep;

        #[derive(Component)]
        struct Food;
        fn food_spawner(mut commands: Commands) {
            commands
                .spawn_bundle(SpriteBundle {
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
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner),
        )

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

struct SnakeHead {
    direction: Direction,
}

.insert(SnakeHead {
    direction: Direction::Up,
})

.add_system_set(
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.150))
        .with_system(snake_movement),
)

.add_system(snake_movement_input.before(snake_movement))

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn snake_movement(mut heads: Query<(&mut Position, &SnakeHead)>) {
    if let Some((mut head_pos, head)) = heads.iter_mut().next() {
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
    }
}

const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component)]
struct SnakeSegment;

#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

.insert_resource(SnakeSegments::default()) // <--

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
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

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

SystemSet::new()
    .with_run_criteria(FixedTimestep::step(0.150))
    .with_system(snake_movement)
    .with_system(snake_eating.after(snake_movement))

    #[derive(Default)]
struct LastTailPosition(Option<Position>);

.insert_resource(LastTailPosition::default())

fn snake_movement(
    // ...
    mut last_tail_position: ResMut<LastTailPosition>,
    // ...


    
    *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));

    fn snake_growth(
        commands: Commands,
        last_tail_position: Res<LastTailPosition>,
        mut segments: ResMut<SnakeSegments>,
        mut growth_reader: EventReader<GrowthEvent>,
    ) {
        if growth_reader.iter().next().is_some() {
            segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
        }
    }

    fn game_over(
        mut commands: Commands,
        mut reader: EventReader<GameOverEvent>,
        segments_res: ResMut<SnakeSegments>,
        food: Query<Entity, With<Food>>,
        segments: Query<Entity, With<SnakeSegment>>,
    ) {
        if reader.iter().next().is_some() {
            for ent in food.iter().chain(segments.iter()) {
                commands.entity(ent).despawn();
            }
            spawn_snake(commands, segments_res);
        }
    }