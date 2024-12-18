use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
use systems::*;

pub struct EsBvDebugPlugin;

impl Plugin for EsBvDebugPlugin {
    fn build(&self, app: &mut App) { app
        .add_systems(Startup, (
            spawn_debug_menu
        ))
        .add_systems(Update, (
            debug_menu_logic,
            debug_menu_window_logic,
        ));
    }
}