use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
use systems::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) { app
        .add_systems(Startup, (
            
        ))
        .add_systems(Update, (

        ));
    }
}