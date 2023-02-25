use bevy::prelude::*;

#[derive(Component)]
struct Player;

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: PLAYER_COLOR,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(10.0, 10.0, 10.0)),
        ..default()
    }).insert(Player);
}

fn move_player(key: Res<Input<KeyCode>>, mut positions: Query<&mut Transform, With<Player>>) {
    for mut transform in positions.iter_mut() {
        if key.pressed(KeyCode::H) {
            transform.translation.x -= 2.0;
        }
        if key.pressed(KeyCode::L) {
            transform.translation.x += 2.0;
        }
        if key.pressed(KeyCode::J) {
            transform.translation.y -= 2.0;
        }
        if key.pressed(KeyCode::K) {
            transform.translation.y += 2.0;
        }
    }
}
