use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
use systems::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) { app
        .add_systems(Startup, (
            spawn_ui,
        ))
        .add_systems(Update, (
            universal_ui_logic,
            hp_ui_logic,
            money_ui_logic,
        ));
    }
}