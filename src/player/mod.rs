use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) { app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (run_player_logic, run_player_bullet_logic));
    }
}