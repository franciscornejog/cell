use bevy::prelude::{Entity, Vec3, Vec2};

pub struct ShootEvent {
    pub cursor_position: Vec2,
}

pub struct ExplodeEvent {
    pub translation: Vec3,
}

pub struct DropVirusEvent {
    pub translation: Vec3,
}

pub struct CollisionEvent {
    pub entity: Entity,
    pub is_player: bool,
}
