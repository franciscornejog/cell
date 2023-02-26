use bevy::prelude::*;
use crate::AppState;

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
    commands.spawn((NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center, 
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        ..default()
    }, Splash))
    .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "C E L L", TextStyle {
                    color: Color::WHITE,
                    font: asset_server.load("fonts/JetBrainsMonoNL-Regular.ttf"),
                    font_size: 60.0,
                })
            );
            parent.spawn(ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play", TextStyle {
                        color: Color::WHITE,
                        font: asset_server.load("fonts/JetBrainsMonoNL-Regular.ttf"),
                        font_size: 40.0,
                    },
                ));
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

fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
