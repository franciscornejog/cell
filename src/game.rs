use bevy::{
    prelude::*, 
    time::FixedTimestep};
use crate::AppState;
use crate::events::ShootEvent;
use crate::cell::{
    spawn_player, spawn_enemy, spawn_particle,
    despawn_cell, move_player, move_particle, shoot_particle,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ShootEvent>()
        .add_system_set(SystemSet::on_enter(AppState::Game)
            .with_system(spawn_player)
            .with_system(spawn_enemy))
        .add_system_set(SystemSet::on_update(AppState::Game)
            .with_system(shoot_particle)
            .with_system(spawn_particle.after(shoot_particle))
            .with_system(move_particle)
            .with_system(despawn_cell))
        .add_system_set(SystemSet::on_update(AppState::Game)
            .with_run_criteria(FixedTimestep::step(0.150))
            .with_system(move_player));
    }
}
