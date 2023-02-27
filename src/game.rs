use bevy::{
    prelude::*, 
    sprite::{collide_aabb::collide, MaterialMesh2dBundle}};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, AppState};
use crate::components::{
    MainCamera, 
    Cell, Player, Direction, Enemy, 
    Explosion, Virus, Particle, Wall};
use crate::events::{DropVirusEvent, ShootEvent, ExplodeEvent, CollisionEvent};
use crate::ui::despawn_screen;
use rand::prelude::random;

const CELL_SIZE: f32 = 20.0;

pub struct GamePlugin;

#[derive(Resource)]
pub struct GameMessage(pub String);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GameMessage("Paused".to_string()))
        .add_event::<DropVirusEvent>()
        .add_event::<ShootEvent>()
        .add_event::<ExplodeEvent>()
        .add_event::<CollisionEvent>()
        .add_system_set(SystemSet::on_enter(AppState::Game)
            .with_system(spawn_cell)
            .with_system(spawn_wall))
        .add_system_set(SystemSet::on_update(AppState::Game)
            .with_system(input_player)
            .with_system(input_particle)
            .with_system(input_virus)
            .with_system(move_player)
            .with_system(move_particle)
            .with_system(spawn_particle.after(input_particle))
            .with_system(spawn_virus.after(input_virus))
            .with_system(spawn_explosion.after(despawn_virus))
            .with_system(despawn_virus)
            .with_system(despawn_explosion) 
            .with_system(despawn_cell) 
            // .with_system(collide_particle)
            .with_system(collide_explosion))
        .add_system_set(SystemSet::on_exit(AppState::Game)
            .with_system(despawn_screen::<Wall>)
            .with_system(despawn_screen::<Cell>)
            .with_system(despawn_screen::<Particle>)
            .with_system(despawn_screen::<Virus>)
            .with_system(despawn_screen::<Explosion>));
    }
}

fn spawn_wall(mut commands: Commands) {
    let size = 20.0;
    let top_translation = Vec3::new(0.0, SCREEN_HEIGHT / 2.0 - (size / 2.0), 1.0);
    let bottom_translation = Vec3::new(0.0, -SCREEN_HEIGHT / 2.0 + (size / 2.0), 1.0);
    let left_translation = Vec3::new(-SCREEN_WIDTH / 2.0 + (size / 2.0), 0.0, 1.0);
    let right_translation = Vec3::new(SCREEN_WIDTH / 2.0 - (size / 2.0), 0.0, 1.0);
    commands.spawn(get_rectangle(Color::DARK_GRAY, SCREEN_WIDTH, size, top_translation))
        .insert(Wall);
    commands.spawn(get_rectangle(Color::DARK_GRAY, SCREEN_WIDTH, size, bottom_translation))
        .insert(Wall);
    commands.spawn(get_rectangle(Color::DARK_GRAY, size, SCREEN_HEIGHT, left_translation))
        .insert(Wall);
    commands.spawn(get_rectangle(Color::DARK_GRAY, size, SCREEN_HEIGHT, right_translation))
        .insert(Wall);
}

fn spawn_cell(
    mut commands: Commands, 
    wall_query: Query<&Transform, (With<Wall>, Without<Direction>)>,
    cell_query: Query<&Transform, (With<Cell>, Without<Wall>)>,
) {
    let translation = get_random_translation(&wall_query, &cell_query);
    commands.spawn(get_rectangle(Color::ORANGE_RED, CELL_SIZE, CELL_SIZE, translation))
        .insert(Cell)
        .insert(Direction::None)
        .insert(Player);
    let translation = get_random_translation(&wall_query, &cell_query);
    commands.spawn(get_rectangle(Color::FUCHSIA, CELL_SIZE, CELL_SIZE, translation))
        .insert(Cell)
        .insert(Enemy);
}

fn spawn_virus(mut commands: Commands, mut reader: EventReader<DropVirusEvent>) {
    if let Some(reader) = reader.iter().next() {
        let size = CELL_SIZE / 2.0;
        commands.spawn(get_rectangle(Color::VIOLET, size, size, reader.translation))
            .insert(Virus(Timer::from_seconds(5.0, TimerMode::Once)));
    }
}

fn spawn_particle(
    mut commands: Commands, 
    meshes: ResMut<Assets<Mesh>>, 
    materials: ResMut<Assets<ColorMaterial>>,
    mut reader: EventReader<ShootEvent>,
    query: Query<&Transform, With<Player>>,
) {
    if let Some(reader) = reader.iter().next() {
        let translation = query.single().translation; 
        let x = reader.cursor_position.x - translation.x;
        let y = reader.cursor_position.y - translation.y;
        let velocity = Vec3::new(x, y, 1.0).normalize();
        let particle_translation = translation + CELL_SIZE * velocity;
        let radius = 5.0;
        let circle = get_circle(meshes, materials, Color::WHITE, particle_translation, radius);
        commands.spawn(circle).insert(Particle { velocity: velocity * 300.0 });
    }
}

fn spawn_explosion(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut explode_reader: EventReader<ExplodeEvent>,
) {
    if let Some(reader) = explode_reader.iter().next() {
        let circle = get_circle(meshes, materials, Color::RED, reader.translation, 100.0);
        commands.spawn(circle).insert(Explosion(Timer::from_seconds(1.0, TimerMode::Once)));
    }
}

fn get_rectangle(color: Color, height: f32, width: f32, translation: Vec3) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(height, width, 1.0))
            .with_translation(translation),
        ..default()
    }
}

fn get_random_translation(
    wall_query: &Query<&Transform, (With<Wall>, Without<Direction>)>,
    cell_query: &Query<&Transform, (With<Cell>, Without<Wall>)>,
) -> Vec3 {
    let mut width = get_random_position(SCREEN_WIDTH);
    let mut height = get_random_position(SCREEN_HEIGHT);
    let new_transform = Transform {
        translation: Vec3::new(width, height, 1.0),
        scale: Vec3::new(CELL_SIZE, CELL_SIZE, 1.0),
        ..default()
    };
    while cell_query.iter()
        .any(|cell_transform| has_collided(cell_transform, &new_transform))
        && wall_query.iter()
            .any(|wall_transform| has_collided(wall_transform, &new_transform))
    {
        width = get_random_position(SCREEN_WIDTH);
        height = get_random_position(SCREEN_HEIGHT);
    }
    Vec3::new(width, height, 1.0)
}

fn get_random_position(size: f32) -> f32 {
    (random::<f32>() * (size - CELL_SIZE)) - (size / 2.0 - (CELL_SIZE / 2.0))
}

fn get_circle(
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    color: Color,
    translation: Vec3,
    radius: f32,
) -> MaterialMesh2dBundle<ColorMaterial> {
     MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(color)),
        transform: Transform::from_scale(Vec3::new(radius, radius, 1.0))
            .with_translation(translation),
        ..default() }
}

fn move_particle(
    time: Res<Time>, 
    wall_query: Query<&Transform, (With<Wall>, Without<Direction>)>,
    mut particle_query: Query<(&mut Transform, &mut Particle), Without<Wall>>,
) {
    for (mut transform, mut particle) in particle_query.iter_mut() {
        let mut new_transform = transform.clone();
        new_transform.translation += particle.velocity * time.delta_seconds();
        let mut wall_iter = wall_query.iter()
            .filter(|wall_transform| has_collided(&new_transform, wall_transform));
        match wall_iter.next() {
            None => { *transform = new_transform; }
            Some(wall_transform) => {
                if wall_transform.scale.x < SCREEN_WIDTH {
                    particle.velocity.x = -particle.velocity.x;
                } else {
                    particle.velocity.y = -particle.velocity.y;
                }
                transform.translation += particle.velocity * time.delta_seconds();
            }
        }
    }
}

fn move_player(
    time: Res<Time>, 
    wall_query: Query<&Transform, (With<Wall>, Without<Direction>)>,
    mut player_query: Query<(&mut Transform, &Direction), With<Player>>,
) {
    let speed = time.delta_seconds() * 40.0;
    for (mut transform, direction) in player_query.iter_mut() {
        let mut new_transform = transform.clone();
        match direction {
            Direction::North => { new_transform.translation.y += speed; }
            Direction::South => { new_transform.translation.y -= speed; }
            Direction::East => { new_transform.translation.x += speed; }
            Direction::West => { new_transform.translation.x -= speed; }
            _ => {}
        }
        let has_not_collided = !wall_query.iter()
            .any(|wall_transform| has_collided(&new_transform, wall_transform));
        if has_not_collided {
            *transform = new_transform;
        }
    }
}

fn input_player(key: Res<Input<KeyCode>>, mut query: Query<&mut Direction, With<Player>>) {
    for mut direction in query.iter_mut() {
        *direction = if key.pressed(KeyCode::A) {
            Direction::West
        } else if key.pressed(KeyCode::D) {
            Direction::East
        } else if key.pressed(KeyCode::W) {
            Direction::North
        } else if key.pressed(KeyCode::S) {
            Direction::South
        } else {
            Direction::None
        }
    }
}

fn input_particle(
    windows: Res<Windows>,
    key: Res<Input<KeyCode>>, 
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut writer: EventWriter<ShootEvent>,
) {
    if key.just_pressed(KeyCode::Space) {
        let (camera, transform) = query.single();
        let window = windows.get_primary().unwrap();
        if let Some(position) = window.cursor_position() 
            .and_then(|cursor| camera.viewport_to_world(transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            writer.send(ShootEvent { cursor_position: position }); 
        }
    }
}
 
fn input_virus(
    key: Res<Input<KeyCode>>, 
    query: Query<&Transform, With<Player>>,
    mut writer: EventWriter<DropVirusEvent>,
) {
    if key.just_pressed(KeyCode::Q) {
        let translation = query.single().translation;
        writer.send(DropVirusEvent { translation });
    }
}

fn despawn_cell(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut message: ResMut<GameMessage>,
    mut reader: EventReader<CollisionEvent>,
) {
    if let Some(collision) = reader.iter().next() {
        if collision.is_player {
            state.set(AppState::Menu).unwrap(); 
            message.0 = "Game Over".to_string();
        } else {
            state.set(AppState::Menu).unwrap(); 
            message.0 = "Victory".to_string();
        }
        commands.entity(collision.entity).despawn();
    }
}

fn despawn_virus(
    mut commands: Commands, 
    time: Res<Time>, 
    mut query: Query<(Entity, &mut Virus, &Transform)>,
    mut writer: EventWriter<ExplodeEvent>,
) {
    for (entity, mut virus, transform) in query.iter_mut() {
        if virus.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
            let translation = transform.translation;
            writer.send(ExplodeEvent { translation });
        }
    }
}

fn despawn_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion)>,
) {
    for (entity, mut explosion) in query.iter_mut() {
        if explosion.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    } 
}

fn collide_explosion(
    explosion_query: Query<&Transform, With<Explosion>>,
    cell_query: Query<(Entity, &Transform, Option<&Player>), With<Cell>>,
    mut writer: EventWriter<CollisionEvent>,
) {
    for explosion_transform in explosion_query.iter() {
        for (entity, cell_transform, player) in cell_query.iter() {
            if has_collided(explosion_transform, cell_transform) {
                writer.send(CollisionEvent { entity, is_player: player.is_some() });
            }
        }
    }
}

fn collide_particle(
    particle_query: Query<&Transform,  With<Particle>>,
    cell_query: Query<(Entity, &Transform, Option<&Player>), With<Cell>>,
    mut writer: EventWriter<CollisionEvent>,
) {
    for particle_transform in particle_query.iter() {
        for (entity, cell_transform, player) in cell_query.iter() {
            if has_collided(particle_transform, cell_transform) {
                writer.send(CollisionEvent { entity, is_player: player.is_some() });
            }
        }
    }
}

fn has_collided(a: &Transform, b: &Transform) -> bool {
    collide(
        a.translation, a.scale.truncate(),
        b.translation, b.scale.truncate(),
    ).is_some()
}

