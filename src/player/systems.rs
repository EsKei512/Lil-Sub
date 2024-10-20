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
pub const GAME_RES_X: u32 = 320;
pub const GAME_RES_Y: u32 = 240;

// Layers for rendering. 
pub const PIXEL_PERFECT_RENDERING  : RenderLayers = RenderLayers::layer(0);
pub const PIXEL_IMPERFECT_RENDERING: RenderLayers = RenderLayers::layer(1);

use crate::components::Rotation;

use super::components::Player;

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn( (
        SpriteBundle {
            transform: Transform::from_xyz((GAME_RES_X / 4) as f32, (GAME_RES_Y / 2) as f32, 5.0),
            texture  : asset_server.load("sprites/gameplay/player.png"),
            ..default()
        },
        Rotation,
        Player{is_active: false},
        PIXEL_PERFECT_RENDERING,
    ));
    println!("AAAAAAAAH");
}

pub fn run_player_logic(
    mut player_transform_query: Query<& mut Transform, With<Player>>,
) {
    if let Ok(mut player_transform) = player_transform_query.get_single_mut() {
        let mut rotation  = player_transform.rotation.x;
        let mut transform = player_transform.translation;
        let mut scale     = player_transform.scale;
    }
}