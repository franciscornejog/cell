use bevy::{
    prelude::*, 
    window::close_on_esc};

const SCREEN_HEIGHT: f32 = 500.0;
const SCREEN_WIDTH: f32 = 500.0;

mod cell;
mod components;
mod events;
mod game;

use game::GamePlugin;

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
        .add_plugin(GamePlugin)
        .add_system(close_on_esc)
        .run();
}
