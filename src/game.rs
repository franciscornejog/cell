use bevy::{
    prelude::*, 
    time::FixedTimestep};
use crate::events::ShootEvent;
use crate::cell::{
    spawn_camera, spawn_player, spawn_enemy, spawn_particle,
    despawn_cell, move_player, move_particle, shoot_particle,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ShootEvent>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(move_player))
        .add_system(shoot_particle)
        .add_system(spawn_particle.after(shoot_particle))
        .add_system(move_particle)
        .add_system(despawn_cell);
    }
}
