use bevy::prelude::*;
use crate::AppState;
use crate::ui::{
    get_button_bundle,
    get_text_bundle,
    get_node_bundle,
    despawn_screen};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(AppState::Splash)
            .with_system(spawn_screen))
        .add_system_set(SystemSet::on_update(AppState::Splash)
            .with_system(interact_button))
        .add_system_set(SystemSet::on_exit(AppState::Splash)
            .with_system(despawn_screen::<Splash>));
    }
}

#[derive(Component)]
struct Splash;

fn spawn_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((get_node_bundle(), Splash))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(Color::WHITE, 60.0, "C E L L", &asset_server));
            parent.spawn(get_button_bundle(Color::DARK_GRAY))
                .with_children(|parent| {
                parent.spawn(get_text_bundle(Color::WHITE, 40.0, "Play", &asset_server));
            });
        });
}

fn interact_button(
    mut state: ResMut<State<AppState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Clicked => { state.set(AppState::Game).unwrap(); }
            Interaction::Hovered => { *color = Color::ORANGE_RED.into(); }
            Interaction::None => { *color = Color::DARK_GRAY.into(); }
        }
    }
}
