use bevy::{
    prelude::*, 
    sprite::collide_aabb::collide};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, AppState};
use crate::components::{
    MainCamera, 
    Cell, Player, Direction, Hostile, Enemy, Lifespan,
    Explosion, Virus, Particle, Wall};
use crate::events::{DropVirusEvent, EjectEvent, ExplodeEvent};
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
        .add_event::<EjectEvent>()
        .add_event::<ExplodeEvent>()
        .add_system_set(SystemSet::on_enter(AppState::Game)
            .with_system(spawn_cell)
            .with_system(spawn_wall))
        .add_system_set(SystemSet::on_update(AppState::Game)
            .with_system(input_player)
            .with_system(input_particle)
            .with_system(input_virus)
            .with_system(move_player)
            .with_system(move_particle)
            .with_system(spawn_enemy_particle)
            .with_system(spawn_particle.after(input_particle))
            .with_system(spawn_virus.after(input_virus))
            .with_system(spawn_explosion.after(despawn_virus))
            .with_system(despawn_virus)
            .with_system(despawn_explosion) 
            .with_system(despawn_lifespan) 
            .with_system(collide_hostile))
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
    commands.spawn((get_rectangle(Color::DARK_GRAY, SCREEN_WIDTH, size, top_translation), Wall));
    commands.spawn((get_rectangle(Color::DARK_GRAY, SCREEN_WIDTH, size, bottom_translation), Wall));
    commands.spawn((get_rectangle(Color::DARK_GRAY, size, SCREEN_HEIGHT, left_translation), Wall));
    commands.spawn((get_rectangle(Color::DARK_GRAY, size, SCREEN_HEIGHT, right_translation), Wall));
}

fn spawn_cell(
    mut commands: Commands, 
    wall_query: Query<&Transform, (With<Wall>, Without<Direction>)>,
    cell_query: Query<&Transform, (With<Cell>, Without<Wall>)>,
) {
    let translation = get_random_translation(&wall_query, &cell_query);
    commands.spawn((
        get_rectangle(Color::ORANGE_RED, CELL_SIZE, CELL_SIZE, translation),
        Lifespan(1),
        Direction::None,
        Cell,
        Player,
    ));
    let translation = get_random_translation(&wall_query, &cell_query);
    commands.spawn((
        get_rectangle(Color::FUCHSIA, CELL_SIZE, CELL_SIZE, translation),
        Lifespan(1),
        Cell,
        Enemy(Timer::from_seconds(2.0, TimerMode::Repeating)),
    ));
}

fn spawn_virus(mut commands: Commands, mut reader: EventReader<DropVirusEvent>) {
    if let Some(reader) = reader.iter().next() {
        let size = CELL_SIZE / 2.0;
        commands.spawn(get_rectangle(Color::YELLOW_GREEN, size, size, reader.translation))
            .insert(Virus(Timer::from_seconds(5.0, TimerMode::Once)));
    }
}

fn spawn_particle(
    mut commands: Commands, 
    mut reader: EventReader<EjectEvent>,
    asset_server: Res<AssetServer>,
) {
    if let Some(reader) = reader.iter().next() {
        let x = reader.target_position.x - reader.translation.x;
        let y = reader.target_position.y - reader.translation.y;
        let velocity = Vec3::new(x, y, 1.0).normalize();
        let particle_translation = reader.translation + CELL_SIZE * velocity;
        let radius = 0.05;
        let texture: Handle<Image> = asset_server.load("components/particle.png");
        let sprite = get_sprite(radius, particle_translation, texture);
        commands.spawn(sprite)
            .insert(Lifespan(2))
            .insert(Hostile)
            .insert(Particle { velocity: velocity * 250.0 });
    }
}

fn spawn_explosion(
    mut commands: Commands,
    mut explode_reader: EventReader<ExplodeEvent>,
    asset_server: Res<AssetServer>,
) {
    if let Some(reader) = explode_reader.iter().next() {
        let radius = 1.0;
        let texture: Handle<Image> = asset_server.load("components/explosion.png");
        let sprite = get_sprite(radius, reader.translation, texture);
        commands.spawn(sprite)
            .insert(Hostile)
            .insert(Explosion(Timer::from_seconds(1.0, TimerMode::Once)));
    }
}

fn get_sprite(size: f32, translation: Vec3, texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        texture,
        transform: Transform::from_scale(Vec3::new(size, size, 1.0))
            .with_translation(translation),
        ..default()
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

fn move_particle(
    time: Res<Time>, 
    wall_query: Query<&Transform, (With<Wall>, Without<Direction>)>,
    mut particle_query: Query<(&mut Transform, &mut Lifespan, &mut Particle), Without<Wall>>,
) {
    for (mut transform, mut lifespan, mut particle) in particle_query.iter_mut() {
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
                lifespan.0 -= 1;
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
    let speed = time.delta_seconds() * 60.0;
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
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut writer: EventWriter<EjectEvent>,
) {
    if key.just_pressed(KeyCode::Space) {
        let (camera, transform) = camera_query.single();
        let window = windows.get_primary().unwrap();
        if let Some(target_position) = window.cursor_position() 
            .and_then(|cursor| camera.viewport_to_world(transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let translation = player_query.single().translation;
            writer.send(EjectEvent { translation, target_position }); 
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

fn despawn_lifespan(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut message: ResMut<GameMessage>,
    query: Query<(Entity, &Lifespan, Option<&Player>, Option<&Enemy>)>,
) {
    for (entity, lifespan, player, enemy) in query.iter() {
        if lifespan.0 <= 0 {
            if player.is_some() {
                state.set(AppState::Menu).unwrap(); 
                message.0 = "Game Over".to_string();
            } else if enemy.is_some() {
                state.set(AppState::Menu).unwrap(); 
                message.0 = "Victory".to_string();
            }
            commands.entity(entity).despawn();
        }
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

fn spawn_enemy_particle(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    mut writer: EventWriter<EjectEvent>,
) {
    for (transform, mut enemy) in enemy_query.iter_mut() {
        if enemy.0.tick(time.delta()).just_finished() {
            let player_position = player_query.single().translation.truncate();
            let translation = transform.translation;
            writer.send(EjectEvent { translation, target_position: player_position });
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

fn collide_hostile(
    hostile_query: Query<&Transform, With<Hostile>>,
    mut cell_query: Query<(&Transform, &mut Lifespan), With<Cell>>,
) {
    for hostile_transform in hostile_query.iter() {
        for (cell_transform, mut lifespan) in cell_query.iter_mut() {
            if has_collided(hostile_transform, cell_transform) {
                lifespan.0 -= 1;
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

