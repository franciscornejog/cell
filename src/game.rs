use bevy::{
    prelude::*, 
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
    time::FixedTimestep};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, AppState};
use crate::components::{MainCamera, Cell, Player, Enemy, Particle};
use crate::events::ShootEvent;
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
        .add_event::<ShootEvent>()
        .add_system_set(SystemSet::on_enter(AppState::Game)
            .with_system(spawn_player)
            .with_system(spawn_enemy))
        .add_system_set(SystemSet::on_update(AppState::Game)
            .with_system(shoot_particle)
            .with_system(spawn_particle.after(shoot_particle))
            .with_system(move_particle)
            .with_system(despawn_cell))
        .add_system_set(SystemSet::on_update(AppState::Game)
            .with_run_criteria(FixedTimestep::step(0.150))
            .with_system(move_player))
        .add_system_set(SystemSet::on_exit(AppState::Game)
            .with_system(despawn_screen::<Cell>)
            .with_system(despawn_screen::<Particle>));
    }
}

fn spawn_player(mut commands: Commands, query: Query<&Transform, With<Cell>>) {
    commands.spawn(spawn_cell(Color::ORANGE_RED, query))
        .insert(Player);
}

fn spawn_enemy(mut commands: Commands, query: Query<&Transform, With<Cell>>) {
    commands.spawn(spawn_cell(Color::FUCHSIA, query))
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
    mut state: ResMut<State<AppState>>,
    mut message: ResMut<GameMessage>,
    particle_query: Query<&Transform,  With<Particle>>,
    cell_query: Query<(Entity, &Transform, Option<&Player>), With<Cell>>,
) {
    for particle_transform in particle_query.iter() {
        for (cell, cell_transform, player) in cell_query.iter() {
            let collision = collide(
                particle_transform.translation,
                particle_transform.scale.truncate(),
                cell_transform.translation,
                cell_transform.scale.truncate(),
            );
            if collision.is_some() {
                if player.is_some() {
                    state.set(AppState::Menu).unwrap(); 
                    message.0 = "Game Over".to_string();
                } else {
                    state.set(AppState::Menu).unwrap(); 
                    message.0 = "Victory".to_string();
                }
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

