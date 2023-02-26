use bevy::prelude::*;

pub fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn get_button_bundle(color: Color) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

pub fn get_text_bundle(
    color: Color, 
    font_size: f32, 
    text: &str, 
    asset_server: &Res<AssetServer>,
) -> TextBundle {
    TextBundle::from_section(text, TextStyle {
        color,
        font: asset_server.load("fonts/JetBrainsMonoNL-Regular.ttf"),
        font_size,
    })
}

pub fn get_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center, 
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        ..default()
    }
}
