use bevy::{
    prelude::*, 
    time::FixedTimestep,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
    window::close_on_esc};
use rand::prelude::random;

const SCREEN_HEIGHT: f32 = 500.0;
const SCREEN_WIDTH: f32 = 500.0;
const CELL_SIZE: f32 = 20.0;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Cell;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Particle {
    velocity: Vec3,
}

struct ShootEvent {
    cursor_position: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Cell".to_string(),
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_event::<ShootEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(move_player))
        .add_system(shoot_particle)
        .add_system(spawn_particle.after(shoot_particle))
        .add_system(move_particle)
        .add_system(despawn_cell)
        .add_system(close_on_esc)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn spawn_player(mut commands: Commands, query: Query<&Transform, With<Cell>>) {
    commands.spawn(spawn_cell(Color::RED, query))
        .insert(Player);
}

fn spawn_enemy(mut commands: Commands, query: Query<&Transform, With<Cell>>) {
    commands.spawn(spawn_cell(Color::BEIGE, query))
        .insert(Enemy);
}

fn spawn_cell(color: Color, query: Query<&Transform, With<Cell>>) -> (SpriteBundle, Cell) {
    (SpriteBundle {
        sprite: Sprite {
            color,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(CELL_SIZE, CELL_SIZE, 1.0))
            .with_translation(get_translation(query)),
        ..default()
    }, Cell)
}

fn get_translation(query: Query<&Transform, With<Cell>>) -> Vec3 {
    let mut width = get_random_position(SCREEN_WIDTH);
    let mut height = get_random_position(SCREEN_HEIGHT);
    for transform in query.iter() {
        let collision = collide(
            Vec3::new(width, height, 1.0),
            Vec2::new(CELL_SIZE, CELL_SIZE),
            transform.translation,
            transform.scale.truncate(),
        );
        if collision.is_some() {
            width = get_random_position(SCREEN_WIDTH);
            height = get_random_position(SCREEN_HEIGHT);
        }
    }
    Vec3::new(width, height, 1.0)
}

fn get_random_position(size: f32) -> f32 {
    (random::<f32>() * (size - CELL_SIZE)) - (size / 2.0 - (CELL_SIZE / 2.0))
}

fn despawn_cell(
    mut commands: Commands, 
    particle_query: Query<&Transform,  With<Particle>>,
    cell_query: Query<(Entity, &Transform), With<Cell>>,
) {
    for particle_transform in particle_query.iter() {
        for (cell, cell_transform) in cell_query.iter() {
            let collision = collide(
                particle_transform.translation,
                particle_transform.scale.truncate(),
                cell_transform.translation,
                cell_transform.scale.truncate(),
            );
            if collision.is_some() {
                commands.entity(cell).despawn();
            }
        }
    }
}

fn move_player(key: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    let speed = 7.0;
    for mut transform in query.iter_mut() {
        if key.pressed(KeyCode::A) {
            transform.translation.x -= speed;
        }
        if key.pressed(KeyCode::D) {
            transform.translation.x += speed;
        }
        if key.pressed(KeyCode::W) {
            transform.translation.y += speed;
        }
        if key.pressed(KeyCode::S) {
            transform.translation.y -= speed;
        }
    }
}

fn spawn_particle(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shoot_reader: EventReader<ShootEvent>,
    query: Query<&Transform, With<Player>>,
) {
    if let Some(reader) = shoot_reader.iter().next() {
        let translation = query.single().translation; 
        let x = reader.cursor_position.x - translation.x;
        let y = reader.cursor_position.y - translation.y;
        let velocity = Vec3::new(x, y, 1.0).normalize();
        let particle_translation = translation + CELL_SIZE * velocity;
        let particle_size = 5.0;
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_scale(Vec3::new(particle_size, particle_size, 1.0))
                .with_translation(particle_translation),
            ..default()
        }, Particle { velocity }));
    }
}

fn move_particle(mut particles: Query<(&mut Transform, &Particle)>) {
    for (mut transform, particle) in particles.iter_mut() {
        transform.translation += particle.velocity;
    }
}

fn shoot_particle(
    key: Res<Input<KeyCode>>, 
    windows: Res<Windows>,
    mut shoot_writer: EventWriter<ShootEvent>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if key.just_pressed(KeyCode::Space) {
        let (camera, transform) = camera.single();
        let window = windows.get_primary().unwrap();
        if let Some(position) = window.cursor_position() 
            .and_then(|cursor| camera.viewport_to_world(transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            shoot_writer.send(ShootEvent { cursor_position: position }); 
        }
    }
}

