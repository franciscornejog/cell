use bevy::{app::AppExit, prelude::*};
use crate::AppState;
use crate::events::MenuEvent;
use crate::game::Score;
use crate::util::despawn_screen;
use super::ui::{
    get_button_bundle,
    get_text_bundle,
    get_node_bundle,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(AppState::Menu)
            .with_system(spawn_screen))
        .add_system_set(SystemSet::on_update(AppState::Menu)
            .with_system(interact_button)
            .with_system(act_button))
        .add_system_set(SystemSet::on_exit(AppState::Menu)
            .with_system(despawn_screen::<Menu>));
    }
}

#[derive(Component)]
struct Menu;

fn spawn_screen(
    commands: Commands, 
    mut reader: EventReader<MenuEvent>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Some(event) = reader.iter().next() {
        match event.0.as_str() {
            "Next Level" => spawn_level_menu(commands, asset_server, &event.0, score.0),
            _ => spawn_exit_menu(commands, asset_server, &event.0, score.0),
        }
    }
}

fn spawn_exit_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    message: &str,
    score: u32,
) {
    commands.spawn((get_node_bundle(), Menu))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(Color::WHITE, 60.0, message, &asset_server));
            parent.spawn(get_text_bundle(Color::WHITE, 25.0, "Score", &asset_server));
            parent.spawn(get_text_bundle(Color::WHITE, 50.0, &score.to_string(), &asset_server));
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

fn spawn_level_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    message: &str,
    score: u32,
) {
    commands.spawn((get_node_bundle(), Menu))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(Color::WHITE, 25.0, "Score", &asset_server));
            parent.spawn(get_text_bundle(Color::WHITE, 50.0, &score.to_string(), &asset_server));
            parent.spawn(get_button_bundle(Color::DARK_GRAY))
                .with_children(|parent| {
                parent.spawn(get_text_bundle(Color::WHITE, 40.0, message, &asset_server));
            });
            parent.spawn(get_button_bundle(Color::DARK_GRAY))
                .with_children(|parent| {
                parent.spawn(get_text_bundle(Color::WHITE, 40.0, "Quit", &asset_server));
            });
        });
}

fn act_button(
    mut state: ResMut<State<AppState>>,
    query: Query<(&Interaction, &Children), Changed<Interaction>>,
    text_query: Query<&Text>,
    mut event: EventWriter<AppExit>
) {
    for (interaction, children) in &query {
        let text = text_query.get(children[0]).unwrap();
        if *interaction == Interaction::Clicked {
            if text.sections[0].value == "Quit" {
                event.send(AppExit);
            } else if text.sections[0].value == "Play Again" {
                state.set(AppState::Game).unwrap();
            } else {
                state.pop().unwrap();
            }
        }
    }
}


fn interact_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Clicked => { *color = Color::GREEN.into() }
            Interaction::Hovered => { *color = Color::ORANGE_RED.into(); }
            Interaction::None => { *color = Color::DARK_GRAY.into(); }
        }
    }
}
