use bevy::{
    prelude::*, 
    sprite::collide_aabb::collide};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, AppState};
use crate::components::{
    MainCamera, 
    Cell, Player, Enemy, 
    Hostile, Lifespan,
    Velocity, StatusEffect,
    Explosion, Virus, 
    Particle, Wall
};
use crate::events::{DropVirusEvent, EjectEvent, ExplodeEvent};
use crate::util::despawn_screen;
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
            .with_system(spawn_status_effect)
            .with_system(spawn_wall))
        .add_system_set(SystemSet::on_update(AppState::Game)
            .with_system(input_player)
            .with_system(input_particle)
            .with_system(input_virus)
            .with_system(move_player.after(input_player))
            .with_system(move_particle)
            .with_system(spawn_enemy_particle)
            .with_system(spawn_particle.after(input_particle))
            .with_system(spawn_virus.after(input_virus))
            .with_system(spawn_explosion.after(despawn_virus))
            .with_system(despawn_virus)
            .with_system(despawn_explosion) 
            .with_system(despawn_lifespan) 
            .with_system(collide_status_effect) 
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
    commands.spawn_batch(vec![
        (get_rectangle(Color::DARK_GRAY, SCREEN_WIDTH, size, top_translation), Wall),
        (get_rectangle(Color::DARK_GRAY, SCREEN_WIDTH, size, bottom_translation), Wall),
        (get_rectangle(Color::DARK_GRAY, size, SCREEN_HEIGHT, left_translation), Wall),
        (get_rectangle(Color::DARK_GRAY, size, SCREEN_HEIGHT, right_translation), Wall),
    ]);
}

fn spawn_status_effect(mut commands: Commands) {
    commands.spawn((
        get_rectangle(Color::GREEN, 10.0, 10.0, Vec3::Z),
        StatusEffect::Speed,
    ));
}

fn spawn_cell(
    mut commands: Commands, 
    wall_query: Query<&Transform, (With<Wall>, Without<Cell>)>,
    cell_query: Query<&Transform, (With<Cell>, Without<Wall>)>,
) {
    let translation = get_random_translation(&wall_query, &cell_query);
    commands.spawn((
        get_rectangle(Color::ORANGE_RED, CELL_SIZE, CELL_SIZE, translation),
        Cell,
        Lifespan(1),
        Player,
        Velocity(Vec3::Z),
    ));
    let translation = get_random_translation(&wall_query, &cell_query);
    commands.spawn((
        get_rectangle(Color::FUCHSIA, CELL_SIZE, CELL_SIZE, translation),
        Cell,
        Enemy(Timer::from_seconds(2.0, TimerMode::Repeating)),
        Lifespan(1),
    ));
}

fn spawn_virus(mut commands: Commands, mut reader: EventReader<DropVirusEvent>) {
    if let Some(reader) = reader.iter().next() {
        let size = CELL_SIZE / 2.0;
        commands.spawn((
            get_rectangle(Color::YELLOW, size, size, reader.translation),
            Virus(Timer::from_seconds(5.0, TimerMode::Once)),
        ));
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
        let initial_velocity = 250.0;
        commands.spawn((
            get_sprite(radius, particle_translation, texture),
            Hostile,
            Lifespan(2),
            Particle,
            Velocity(velocity * initial_velocity),
        ));
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
        commands.spawn((
            get_sprite(radius, reader.translation, texture),
            Hostile,
            Explosion(Timer::from_seconds(1.0, TimerMode::Once)),
        ));
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
    wall_query: &Query<&Transform, (With<Wall>, Without<Cell>)>,
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
    wall_query: Query<&Transform, (With<Wall>, Without<Particle>)>,
    mut particle_query: Query<(&mut Transform, &mut Lifespan, &mut Velocity), With<Particle>>,
) {
    for (mut transform, mut lifespan, mut velocity) in particle_query.iter_mut() {
        let mut new_transform = transform.clone();
        new_transform.translation += velocity.0 * time.delta_seconds();
        let mut wall_iter = wall_query.iter()
            .filter(|wall_transform| has_collided(&new_transform, wall_transform));
        match wall_iter.next() {
            None => { *transform = new_transform; }
            Some(wall_transform) => {
                if wall_transform.scale.x < SCREEN_WIDTH {
                    velocity.0.x = -velocity.0.x;
                } else {
                    velocity.0.y = -velocity.0.y;
                }
                lifespan.0 -= 1;
                transform.translation += velocity.0 * time.delta_seconds();
            }
        }
    }
}

fn move_player(
    time: Res<Time>, 
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &Velocity), (With<Player>, Changed<Velocity>)>,
) {
    let (mut transform, velocity) = player_query.single_mut();
    let mut new_transform = transform.clone();
    new_transform.translation += velocity.0 * time.delta_seconds();
    let has_not_collided = !wall_query.iter()
        .any(|wall_transform| has_collided(&new_transform, wall_transform));
    if has_not_collided {
        transform.translation = new_transform.translation;
    }
}

fn input_player(
    key: Res<Input<KeyCode>>, 
    mut query: Query<(&mut Velocity, Option<&StatusEffect>), With<Player>>,
) {
    let (mut velocity, status_effect) = query.single_mut();
    let default_speed = if let Some(StatusEffect::Speed) = status_effect {
        150.0
    } else {
        60.0
    };
    if key.pressed(KeyCode::A) {
        velocity.0.x = -default_speed;
    }
    if key.pressed(KeyCode::D) {
        velocity.0.x = default_speed;
    } 
    if key.pressed(KeyCode::W) {
        velocity.0.y = default_speed;
    } 
    if key.pressed(KeyCode::S) {
        velocity.0.y = -default_speed;
    }
    if !key.any_pressed([KeyCode::A, KeyCode::D, KeyCode::W, KeyCode::S]) {
        velocity.0.x = 0.0;
        velocity.0.y = 0.0;
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

fn collide_status_effect(
    mut commands: Commands,
    status_effect_query: Query<(Entity, &Transform, &StatusEffect)>,
    mut cell_query: Query<(Entity, &Transform), (With<Cell>, Without<StatusEffect>)>,
) {
    for (status_effect_entity, status_effect_transform, status_effect) in status_effect_query.iter() {
        for (cell_entity, cell_transform) in cell_query.iter_mut() {
            if has_collided(status_effect_transform, cell_transform) {
                commands.entity(cell_entity).insert(status_effect.clone());
                commands.entity(status_effect_entity).despawn_recursive();
            }
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

