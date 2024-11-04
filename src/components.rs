use bevy::prelude::*;
use super::resources::*;

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
pub struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
pub struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
pub struct OuterCamera;

#[derive(Component)]
#[derive(Debug)]
pub struct GameControls {
    pub left  : KeyCode,
    pub right : KeyCode,
    pub up    : KeyCode,
    pub down  : KeyCode,
    pub shoot : KeyCode,
    pub rocket: KeyCode,
    pub pause : KeyCode,

    pub ui_confirm: KeyCode,
    pub ui_deny   : KeyCode,
}
impl Default for GameControls {
    fn default() -> GameControls {
        GameControls {
            left  : KeyCode::ArrowLeft,
            right : KeyCode::ArrowRight,
            up    : KeyCode::ArrowUp,
            down  : KeyCode::ArrowDown,
            shoot : KeyCode::KeyZ,
            rocket: KeyCode::KeyX,
            pause : KeyCode::Escape,
        
            ui_confirm: KeyCode::KeyZ,
            ui_deny   : KeyCode::KeyX,
        }
    }
}

#[derive(Component)]
pub struct Complex2dMovement {
    pub soft_terminal_velocity : f32, // The entity with this cannot NATURALLY go faster than this, but can be pushed to go faster (I.e., with an explosion)
    pub hard_terminal_velocity : f32, // The entity with this component CANNOT go faster than this

    pub acceleration : f32,
    pub natural_deceleration : f32,
    
    pub current_velocity : Vec3, // The natural deceleration

}

#[derive(Component)]
pub struct AnimationTools {
    pub ticks_i   : f32,
    pub ticks_ii  : f32,
    pub ticks_iii : f32,
    pub ticks_iv  : f32,
    pub generic_counter_i   : i32,
    pub generic_counter_ii  : i32,
    pub generic_counter_iii : i32,
    pub generic_counter_iv  : i32,
    pub generic_counter_v   : i32,
    pub generic_counter_vi  : i32,
}
impl AnimationTools {
    pub fn tick(&mut self, delta_time: f32) {
        self.ticks_i   -= delta_time * 60.0;
        self.ticks_ii  -= delta_time * 60.0;
        self.ticks_iii -= delta_time * 60.0;
        self.ticks_iv  -= delta_time * 60.0;
    }
}
impl Default for AnimationTools {
    fn default() -> AnimationTools {
        AnimationTools {
            ticks_i   : 0.0,
            ticks_ii  : 0.0,
            ticks_iii : 0.0,
            ticks_iv  : 0.0,
            generic_counter_i   : 0,
            generic_counter_ii  : 0,
            generic_counter_iii : 0,
            generic_counter_iv  : 0,
            generic_counter_v   : 0,
            generic_counter_vi  : 0,
        }
    }
}

#[derive(Component)]
pub struct Collision {
    pub enabled: bool,
    pub size   : Vec3,
}

#[derive(Component)]
pub struct Stats {
    pub max_hp : i32,
    pub cur_hp : i32,

    pub damage : i32,
}


#[derive(Component)]
pub struct Player {
    pub is_active: bool,
}

#[derive(Component)]
pub struct PlayerChild;

#[derive(Component)]
pub struct PlayerBullet {
    pub float_horizontal_acceleration : f32,
}

#[derive(Component)]
pub struct EnemySpawningQueue {
    pub queue: Vec<EnemySpawningInfo>,
}

#[derive(Component)]
pub struct GlobalEnt;