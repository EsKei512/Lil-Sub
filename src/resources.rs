use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
    window::WindowResized,
};

// Game Resolution
pub const GAME_RES_X: u32 = 480;
pub const GAME_RES_Y: u32 = 270;

// Layers for rendering. 
pub const PIXEL_PERFECT_RENDERING  : RenderLayers = RenderLayers::layer(0);
pub const PIXEL_IMPERFECT_RENDERING: RenderLayers = RenderLayers::layer(1);

pub const ENEMY_HURT_TICKS : f32 = 2.5;

// Enemy IDs Enum
#[derive(Copy, Clone, Debug)]
pub enum EnemyIds {
    Debug,
    Goldfish,
}
#[derive(Debug)]
pub struct EnemySpawningInfo {
    pub origin: Vec2,
    pub e_id: EnemyIds,
}