use bevy::{
    prelude::*, 
    window::close_on_esc};

const SCREEN_HEIGHT: f32 = 500.0;
const SCREEN_WIDTH: f32 = 500.0;

mod menu;

mod game;
mod components;
mod events;
mod level;

mod util;

mod scene;

use menu::{menu::MenuPlugin, splash::SplashPlugin};
use game::GamePlugin;
// use scene::ScenePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Splash,
    Menu,
    Game,
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
        .add_state(AppState::Splash)
        .add_plugin(SplashPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        // .add_plugin(ScenePlugin)
        .add_system(close_on_esc)
        .run();
}
