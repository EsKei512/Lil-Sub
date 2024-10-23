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
use crate::components::GameControls;
use crate::components::Complex2dMovement;

use super::components::Player;

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        GameControls{
            ..Default::default()
        },
        Complex2dMovement {
            global_transform: Vec2::new((GAME_RES_X / 4) as f32, (GAME_RES_Y / 2) as f32),
            soft_terminal_velocity: 1.5,
            hard_terminal_velocity: 3.0,
            acceleration: 4.5,
            natural_deceleration: 8.65,
            current_velocity: Vec2::ZERO,
        },
        PIXEL_PERFECT_RENDERING,
    ));
}

pub fn run_player_logic(
    mut player_transform_query: Query<& mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_keys: Query<& GameControls, With<Player>>,
    mut velocity_info: Query<& mut Complex2dMovement, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_transform_query.get_single_mut() 
    {
    if let Ok(game_key_list) = game_keys.get_single() 
    {
    if let Ok(mut pv) = velocity_info.get_single_mut() { // "pv" = Player Velocity

        let mut rotation  = player_transform.rotation.x;
        let mut transform = player_transform.translation;
        let mut scale     = player_transform.scale;

        let mut movement_direction: Vec2 = Vec2::ZERO;

        if keyboard_input.pressed(game_key_list.up  ) {
            movement_direction.y -= 1.0
        }
        if keyboard_input.pressed(game_key_list.down) {
            movement_direction.y += 1.0
        }
        if keyboard_input.pressed(game_key_list.left) {
            movement_direction.x -= 1.0
        }
        if keyboard_input.pressed(game_key_list.right) {
            movement_direction.x += 1.0
        }

        if ((movement_direction.x.abs() * 10.0).round() > 0.0) || ((movement_direction.y.abs() * 10.0).round() > 0.0) { // This block of code is bad and complex
            pv.current_velocity.x = (pv.current_velocity.x + ((movement_direction.x * &pv.acceleration) * time.delta_seconds()) ).clamp(-pv.soft_terminal_velocity, pv.soft_terminal_velocity);
            pv.current_velocity.y = (pv.current_velocity.y + ((movement_direction.y * &pv.acceleration) * time.delta_seconds()) ).clamp(-pv.soft_terminal_velocity, pv.soft_terminal_velocity);    
            println!("{}", pv.current_velocity);
        }
        else {

            //let x_sign: i8 = if &pv.current_velocity.x >= &0.0 {1} else {-1};
            //let y_sign: i8 = if &pv.current_velocity.y >= &0.0 {1} else {-1};
            

        }
        
        
        //println!("{}", pv.current_velocity);

        }
    }
    }
}