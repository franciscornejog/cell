use bevy::prelude::{Component, Vec3, Timer};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Cell;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Explosion(pub Timer);

#[derive(Component)]
pub struct Virus(pub Timer);

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec3,
}

