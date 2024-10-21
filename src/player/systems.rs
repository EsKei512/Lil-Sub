use bevy::{
    color::palettes::css::RED, input::keyboard::Key, math::VectorSpace, prelude::*, render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    }, sprite::MaterialMesh2dBundle, window::WindowResized
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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let texture = asset_server.load("sprites/gameplay/player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn( (
        SpriteBundle {
            transform: Transform::from_xyz((GAME_RES_X / 4) as f32, (GAME_RES_Y / 2) as f32, 5.0),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        Rotation,
        Player{is_active: false},
        PIXEL_PERFECT_RENDERING,
    ));
}

pub fn run_player_logic(
    mut player_transform_query: Query<& mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if let Ok(mut player_transform) = player_transform_query.get_single_mut() {
        let mut rotation  = player_transform.rotation.x;
        let mut transform = player_transform.translation;
        let mut scale     = player_transform.scale;

        let mut movement_direction: Vec2 = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::Up  ) {
            movement_direction.y -= 1.0
        }
        if keyboard_input.pressed(KeyCode::Down) {
            movement_direction.y += 1.0
        }
        if keyboard_input.pressed(KeyCode::Left) {
            movement_direction.x -= 1.0
        }
        if keyboard_input.pressed(KeyCode::Right) {
            movement_direction.x += 1.0
        }
    }
}