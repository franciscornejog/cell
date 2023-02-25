use bevy::prelude::*;

#[derive(Component)]
struct SnakeHead;

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_camera)
        .add_startup_system(spawn_player)
        .add_system(hello)
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: PLAYER_COLOR,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        ..default()
    }).insert(SnakeHead);
}

fn hello() {
    println!("hello");
}
