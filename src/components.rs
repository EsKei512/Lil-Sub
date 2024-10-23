use bevy::prelude::*;

// A component that allows sprites to rotate
#[derive(Component)]
pub struct Rotation;

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