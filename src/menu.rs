use bevy::prelude::*;
use crate::AppState;
use crate::game::GameMessage;
use crate::ui::{
    get_button_bundle,
    get_text_bundle,
    get_node_bundle,
    despawn_screen};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(AppState::Menu)
            .with_system(spawn_screen))
        .add_system_set(SystemSet::on_update(AppState::Menu)
            .with_system(interact_button))
        .add_system_set(SystemSet::on_exit(AppState::Menu)
            .with_system(despawn_screen::<Menu>));
    }
}

#[derive(Component)]
struct Menu;

fn spawn_screen(
    mut commands: Commands, 
    message: Res<GameMessage>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((get_node_bundle(), Menu))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(Color::WHITE, 60.0, &message.0, &asset_server));
            parent.spawn(get_button_bundle(Color::DARK_GRAY))
                .with_children(|parent| {
                parent.spawn(get_text_bundle(Color::WHITE, 40.0, "Play Again", &asset_server));
            });
            parent.spawn(get_button_bundle(Color::DARK_GRAY))
                .with_children(|parent| {
                parent.spawn(get_text_bundle(Color::WHITE, 40.0, "Quit", &asset_server));
            });
        });
}

fn interact_button(
    mut state: ResMut<State<AppState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &Children), 
        (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
) {
    for (interaction, mut color, children) in &mut query {
        let text = text_query.get(children[0]).unwrap();
        let mut next_state = AppState::Game;
        if text.sections[0].value == "Quit" {
            next_state = AppState::Splash;
        }
        match *interaction {
            Interaction::Clicked => { state.set(next_state).unwrap(); }
            Interaction::Hovered => { *color = Color::ORANGE_RED.into(); }
            Interaction::None => { *color = Color::DARK_GRAY.into(); }
        }
    }
}
