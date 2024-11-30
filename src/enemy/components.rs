use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use crate::resources::EnemyIds;

use super::resources::EnemyStates;

#[derive(Component)]
pub struct GameEnemy {
    pub enemy_id      : EnemyIds,
    pub origin        : Vec2,
    pub current_state : EnemyStates,
    pub state_ticks   : f32,
    pub hitbox_size   : f32,
    pub hurt_ticks    : f32,
    pub enabled       : bool,
    pub target_color  : Color,
    pub target_scale  : Vec2,
}
impl GameEnemy {
    pub fn tick(&mut self, delta_time: f32) {
        self.state_ticks -= delta_time * 60.0;
    }
}

#[derive(Component)]
pub struct EnDebug;
impl Default for EnDebug {
    fn default() -> EnDebug {
        EnDebug
    }
}

#[derive(Component)]
pub struct EnGoldfish {
    pub goldfish_target_position: Vec2,
    pub goldfish_movement: Vec3,
    pub goldfish_angle: f32,
    pub goldfish_angle_offset: f32,
    pub calculations_finished: bool,
}
impl Default for EnGoldfish {
    fn default() -> EnGoldfish {
        EnGoldfish {
            goldfish_target_position: Vec2::ZERO,
            goldfish_movement: Vec3::ZERO,
            goldfish_angle: 0.0,
            goldfish_angle_offset: 0.0,
            calculations_finished: false,
        }
    }
}

#[derive(Component)]
pub struct EnemySounds {
    pub hurt_sound  : Handle<AudioSource>,
    pub death_sound : Handle<AudioSource>,
}