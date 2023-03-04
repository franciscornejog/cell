use bevy::prelude::{Vec3, Vec2};

pub struct MenuEvent(pub String);

pub struct EjectEvent {
    pub translation: Vec3,
    pub target_position: Vec2,
}

pub struct ExplodeEvent {
    pub translation: Vec3,
}

pub struct DropVirusEvent {
    pub translation: Vec3,
}
