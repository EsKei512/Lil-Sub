use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
use systems::*;

pub struct CollectablesPlugin;

impl Plugin for CollectablesPlugin {
    fn build(&self, app: &mut App) { app
        .add_systems(Update, (
            universal_collectable_logic,
            health_collectable_logic,
            money_collectable_logic,
            spawn_collectables,
        ));
    }
}