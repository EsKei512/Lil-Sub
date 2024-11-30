use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
use systems::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) { app
        .add_systems(Update, (
            particle_logic,
            particle_spawner_logic,
        ));
    }
}