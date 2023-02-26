use bevy::{
    prelude::*, 
    time::FixedTimestep,
    sprite::MaterialMesh2dBundle, 
    window::close_on_esc};
// use rand::prelude::random;

const SCREEN_HEIGHT: f32 = 500.0;
const SCREEN_WIDTH: f32 = 500.0;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

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
        .add_event::<ShootEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(move_player))
        .add_system(shoot_particle)
        .add_system(spawn_particle.after(shoot_particle))
        .add_system(move_particle)
        .add_system(close_on_esc)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(20.0, 20.0, 20.0)),
        ..default()
    }).insert(Player);
}

fn move_player(key: Res<Input<KeyCode>>, mut positions: Query<&mut Transform, With<Player>>) {
    let speed = 7.0;
    for mut transform in positions.iter_mut() {
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
    positions: Query<&Transform, With<Player>>,
) {
    if let Some(reader) = shoot_reader.iter().next() {
        let position = positions.single();
        let x = reader.cursor_position.x - position.translation.x;
        let y = reader.cursor_position.y - position.translation.y;
        let velocity = Vec3::new(x, y, 0.0).normalize();
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(position.translation),
            ..default()
        }).insert(Particle { velocity });
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

