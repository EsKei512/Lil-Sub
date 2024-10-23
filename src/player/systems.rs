use bevy::{
    color::palettes::css::RED, input::keyboard::Key, math::VectorSpace, prelude::*, render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    }, sprite::MaterialMesh2dBundle, window::WindowResized
};

use crate::resources::*;

use crate::components::Rotation;
use crate::components::GameControls;
use crate::components::Complex2dMovement;

use super::components::*;

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = asset_server.load("sprites/gameplay/player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(22), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_entity = commands.spawn( (
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        },
        Player{is_active: false},
        GameControls{
            ..Default::default()
        },
        Complex2dMovement {
            soft_terminal_velocity: 0.6,
            hard_terminal_velocity: 1.2,
            acceleration: 4.2,
            natural_deceleration: 6.25,
            current_velocity: Vec3::ZERO,
        },
        PIXEL_PERFECT_RENDERING,
    )).id();
    let player_eyes_entitiy = commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 2,
        },
        PIXEL_PERFECT_RENDERING,
        PlayerChild,
    )).id();
    commands.entity(player_entity).push_children(&[player_eyes_entitiy]); // Spawn the player with the eyes entity
}

pub fn run_player_logic(
    mut player_query: Query<(&Children, &mut Transform, &mut TextureAtlas, & GameControls, &mut Complex2dMovement), (With<Player>, Without<PlayerChild>)>,
    mut child_query : Query<(&Parent  , &mut Transform, &mut TextureAtlas, ), (With<PlayerChild>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (player, mut eyes_texture, mut eyes_transform) in child_query.iter_mut() 
    {
    for (eyes, mut p_transform, mut p_texture, g_buttons, mut p_physics) in player_query.iter_mut()
    {
        let mut movement_direction: Vec2 = Vec2::ZERO;

        if keyboard_input.pressed(g_buttons.up  ) {
            movement_direction.y += 1.0
        }
        if keyboard_input.pressed(g_buttons.down) {
            movement_direction.y -= 1.0
        }
        if keyboard_input.pressed(g_buttons.left) {
            movement_direction.x -= 1.0
        }
        if keyboard_input.pressed(g_buttons.right) {
            movement_direction.x += 1.0
        }

        if ((movement_direction.x.abs() * 10.0).round() > 0.0) { // This block of code is bad and complex
            p_physics.current_velocity.x = (p_physics.current_velocity.x + ((movement_direction.x * &p_physics.acceleration) * time.delta_seconds()) ).clamp(-p_physics.soft_terminal_velocity, p_physics.soft_terminal_velocity);
        }
        else {
            let x_sign: i8 = if &p_physics.current_velocity.x >= &0.0 {1} else {-1};
            p_physics.current_velocity.x = (p_physics.current_velocity.x.abs() - (&p_physics.natural_deceleration * time.delta_seconds())).clamp(0.0, p_physics.soft_terminal_velocity) * (x_sign as f32); 
        }
        
        if ((movement_direction.y.abs() * 10.0).round() > 0.0) { // This block of code is bad and complex
            p_physics.current_velocity.y = (p_physics.current_velocity.y + ((movement_direction.y * &p_physics.acceleration) * time.delta_seconds()) ).clamp(-p_physics.soft_terminal_velocity, p_physics.soft_terminal_velocity);    
            //player_transform.rotate_z((movement_direction.y * 2.5)  * time.delta_seconds());
            //player_transform.rotation.z = player_transform.rotation.z.clamp(-0.4, 0.4);
        }
        else {
            let y_sign: i8 = if &p_physics.current_velocity.y >= &0.0 {1} else {-1};
            p_physics.current_velocity.y = (p_physics.current_velocity.y.abs() - (&p_physics.natural_deceleration * time.delta_seconds())).clamp(0.0, p_physics.soft_terminal_velocity) * (y_sign as f32);
            //player_transform.rotation.z = player_transform.rotation.z.lerp(0.0, 0.05);
        }
               
        p_transform.translation += p_physics.current_velocity;

    }
    }
}