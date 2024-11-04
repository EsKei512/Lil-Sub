use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) { app
        .add_systems(Startup, (
            debug_wave_spawn.after(super::systems::initialize_game),
        ))
        .add_systems(Update, (
            universal_enemy_logic, 
            run_enemy_bullet_logic,
            spawn_enemy,
            debug_enemy_logic.before(universal_enemy_logic),
            goldfish_enemy_logic.before(universal_enemy_logic),
        ));
    }
}