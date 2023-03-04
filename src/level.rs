use bevy::prelude::*;
use crate::components::*;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(Resource)]
pub struct Level(pub usize);

const ROW_SIZE: usize = 21;
const COL_SIZE: usize = 21;
const TILE_SIZE: f32 = SCREEN_WIDTH / ROW_SIZE as f32;

pub const LEVEL_0: &str = "
|||||||||||||||||||||
|...................|
|...................|
|.........P.........|
|...................|
|...................|
|...................|
|.........*.........|
|...................|
|...................|
|....|||||||||||....|
|...................|
|...................|
|...................|
|...................|
|.........E.........|
|...................|
|...................|
|...................|
|...................|
|||||||||||||||||||||
";

pub const LEVEL_1: &str = "
|||||||||||||||||||||
|...................|
|...................|
|..............E....|
|....||||||.........|
|.........|.........|
|.........|.........|
|.........|.........|
|.........|.........|
|.........|.........|
|.........|....*....|
|.........|.........|
|.........|.........|
|.........|.........|
|.........|.........|
|.........|.........|
|.........||||||....|
|....P..............|
|...................|
|...................|
|||||||||||||||||||||
";

pub const LEVELS: [&str; 2] = [
    LEVEL_0, LEVEL_1,
];

pub fn generate_level(mut commands: Commands, mut level: ResMut<Level>) {
    for (i, c) in LEVELS[LEVELS.len() - 1 - level.0].chars().filter(|c| *c != '\n').enumerate() {
        let col = (i % COL_SIZE) as f32;
        let row = (i / ROW_SIZE) as f32;
        let translation = Vec3::new(col, row, 0.0);
        if c == '|' {
            commands.spawn((
                get_tile(Color::DARK_GRAY, TILE_SIZE, translation), 
                Wall,
            ));
        } else if c == '*' {
            commands.spawn((
                get_tile(Color::GREEN, TILE_SIZE, translation),
                StatusEffect::Speed,
            ));
        } else if c == 'P' {
            commands.spawn((
                get_tile(Color::ORANGE_RED, TILE_SIZE, translation),
                Cell,
                Lifespan(1),
                Player,
                Velocity(Vec3::ZERO),
            ));
        } else if c == 'E' {
            commands.spawn((
                get_tile(Color::FUCHSIA, TILE_SIZE, translation),
                Cell,
                Enemy(Timer::from_seconds(2.0, TimerMode::Repeating)),
                Lifespan(1),
            ));
        }
    }
    if level.0.checked_sub(1).is_none() {
        level.0 = LEVELS.len() - 1;
    }
}

fn get_tile(color: Color, size: f32, translation: Vec3) -> SpriteBundle {
    let x = translation.x * size - SCREEN_WIDTH / 2.0 + size / 2.0;
    let y = translation.y * -size + SCREEN_HEIGHT / 2.0 - size / 2.0;
    let translation = Vec3::new(x, y, 0.0);
    SpriteBundle {
        sprite: Sprite {
            color,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(size, size, 0.0))
            .with_translation(translation),
        ..default()
    }
}

fn get_rectangle(
    color: Color, 
    height: f32, 
    width: f32, 
    translation: Vec3,
    side: f32,
) -> SpriteBundle {
    let mut x = translation.x * TILE_SIZE - SCREEN_WIDTH / 2.0 + width / 2.0;
    let mut y = translation.y * -TILE_SIZE + SCREEN_HEIGHT / 2.0 - height / 2.0;
    if width == TILE_SIZE {
        y -= side;
    } else {
        x += side;
    }
    let translation = Vec3::new(x, y, 0.0);
    SpriteBundle {
        sprite: Sprite {
            color,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(width, height, 0.0))
            .with_translation(translation),
        ..default()
    }
}
