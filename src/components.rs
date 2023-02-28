use bevy::prelude::{Component, Timer, Vec3};

#[derive(Component)]
pub struct Cell;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum Direction {
    North, South, East, West, None
}

#[derive(Component)]
pub struct Lifespan(pub i32);

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Enemy(pub Timer);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Hostile;

#[derive(Component)]
pub struct Explosion(pub Timer);

#[derive(Component)]
pub struct Virus(pub Timer);

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec3,
}
