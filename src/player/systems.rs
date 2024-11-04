use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::resources::*;
use crate::components::*;

use super::components::*;
use super::resources::*;

// Shader Stuff
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DamageFlash {
    #[uniform(0)]
    enabled: bool,
    #[texture(1)]
    #[sampler(2)]
    texture: Image
}
impl Material2d for DamageFlash {
    fn fragment_shader() -> ShaderRef {
        "assets/shaders/enemy_flash.wgsl".into()
    }
}

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut materials: ResMut<Assets<DamageFlash>>,
) {
    let texture = asset_server.load("sprites/gameplay/player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(22), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_entity = commands.spawn( (
        SpriteBundle {
            transform: Transform::from_xyz(-125.0, 0.0, 5.0),
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
        AnimationTools {
            ticks_i: PLAYER_PROPELLER_TICK_LENGTH,
            ..default()
        },
        Complex2dMovement {
            soft_terminal_velocity: 1.25,
            hard_terminal_velocity: 2.5,
            acceleration: 5.5,
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
    mut player_query: Query<(&Children, &mut Transform, &mut TextureAtlas, & GameControls, &mut Complex2dMovement, &mut AnimationTools, ), (With<Player>, Without<PlayerChild>)>,
    mut child_query : Query<(&Parent  , &mut Transform, &mut TextureAtlas, ), (With<PlayerChild>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (player, mut eyes_transform , mut eyes_texture) in child_query.iter_mut() 
    {
    for (eyes, mut p_transform, mut p_texture, g_buttons, mut p_physics, mut a_tools) in player_query.iter_mut()
    {
        
        // PLAYER MOVEMENT CODE **************************************************************************************************************

        let mut movement_direction: Vec2 = Vec2::ZERO;

        let current_sprite_rotation : f32 = p_transform.rotation.z.clone();

        // Capture player key inputs
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

        // Horizontal movement
        if ((movement_direction.x.abs() * 10.0).round() > 0.0) { // This block of code is bad and complex
            p_physics.current_velocity.x = (p_physics.current_velocity.x + ((movement_direction.x * &p_physics.acceleration) * time.delta_seconds()) ).clamp(-p_physics.soft_terminal_velocity, p_physics.soft_terminal_velocity);
        }
        else { // Horizontal deceleration
            let x_sign: i8 = if &p_physics.current_velocity.x >= &0.0 {1} else {-1};
            p_physics.current_velocity.x = (p_physics.current_velocity.x.abs() - (&p_physics.natural_deceleration * time.delta_seconds())).clamp(0.0, p_physics.soft_terminal_velocity) * (x_sign as f32); 
        }
        
        let y_sign: i8 = if &p_physics.current_velocity.y >= &0.0 {1} else {-1};
        
        // Vertical movement
        if ((movement_direction.y.abs() * 10.0).round() > 0.0) { // This block of code is bad and complex too
            p_physics.current_velocity.y = (p_physics.current_velocity.y + ((movement_direction.y * &p_physics.acceleration) * time.delta_seconds()) ).clamp(-p_physics.soft_terminal_velocity, p_physics.soft_terminal_velocity);    
            if p_transform.rotation.z.abs() < 0.3 {
                p_transform.rotate_z((0.0075 * (y_sign as f32)) * (time.delta_seconds() * 60.0)); // TODO: Fix bug where player gets stuck in direction if he turns too far
            }
        }
        else { // Vertical deceleration
            p_physics.current_velocity.y = (p_physics.current_velocity.y.abs() - (&p_physics.natural_deceleration * time.delta_seconds())).clamp(0.0, p_physics.soft_terminal_velocity) * (y_sign as f32);
            if ! (-(current_sprite_rotation / 2.0)).is_nan() {
                p_transform.rotate_z( -(current_sprite_rotation / (12.5) * (time.delta_seconds() * 60.0)) );
            }
        }

        if p_transform.rotation.z.is_nan() {
            p_transform.rotation.z = 0.0;
        }

        let mut normalized_vector : Vec2 = Vec2::new(p_physics.current_velocity.x, p_physics.current_velocity.y);

        if normalized_vector.x.is_nan() { normalized_vector.x = 0.0 }
        if normalized_vector.y.is_nan() { normalized_vector.y = 0.0 }

        p_transform.translation += (p_physics.current_velocity.abs() * Vec3::new(normalized_vector.x, normalized_vector.y, 1.0)) * (time.delta_seconds() * 60.0); // Actually move the player

        // END OF PLAYER MOVEMENT CODE *******************************************************************************************************

        // PLAYER ANIMATION CODE *************************************************************************************************************

        if a_tools.ticks_i <= 0.0 {
            if p_texture.index == 0 { p_texture.index = 1 }
            else if p_texture.index == 1 { p_texture.index = 0 }
            a_tools.ticks_i = PLAYER_PROPELLER_TICK_LENGTH;
        }
        else { a_tools.tick(time.delta_seconds()) }

        if      movement_direction.y > 0.0 { eyes_texture.index = 3 }
        else if movement_direction.y < 0.0 { eyes_texture.index = 5 }
        else if movement_direction.x < 0.0 { eyes_texture.index = 4 }
        else { eyes_texture.index = 2 }
        
        // END OF PLAYER ANIMATION CODE ******************************************************************************************************

        // PLAYER SHOOTING CODE **************************************************************************************************************

        if keyboard_input.just_pressed(g_buttons.shoot) {
            commands.spawn( ( // Player bullet entity
                SpriteBundle {
                    transform : Transform::from_xyz(p_transform.translation.x, p_transform.translation.y, 4.0),
                    texture : asset_server.load("sprites/gameplay/projectiles/player_bullet.png"),
                    ..Default::default()
                },
                Complex2dMovement {
                    soft_terminal_velocity: 5.0,
                    hard_terminal_velocity: 5.0,
                    acceleration: 0.5,
                    natural_deceleration: 22.5,
                    current_velocity: Vec3::new(14.0, 0.0, 0.0),
                },
                TextureAtlas {
                    layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(14), 6, 1, None, None)),
                    index: 0,
                },
                Collision {
                    enabled: true,
                    size: Vec3::new(10.0, 6.0, 250.0),
                },
                PlayerBullet {
                    float_horizontal_acceleration: rand::thread_rng().gen_range(-0.25..0.25),
                },
                AnimationTools {
                    ticks_i: PLAYER_BULLET_TICK_LENGTH,
                    ..default()
                },
            ));
        }

        // END OF PLAYER SHOOTING CODE *******************************************************************************************************

    }
    }
}

pub fn run_player_bullet_logic (
    mut bullet_query: Query<(&mut Transform, &mut Complex2dMovement, &mut TextureAtlas, &mut Collision, & PlayerBullet, Entity, &mut AnimationTools), With<PlayerBullet>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut b_transform, mut b_physics, mut b_texture, mut b_collision, bullet, entity, mut a_tools) in bullet_query.iter_mut() 
    {
    if b_collision.enabled { // Bullet shooting
        
        b_transform.translation += b_physics.current_velocity * (time.delta_seconds() * 60.0);

        b_physics.current_velocity.x -= b_physics.natural_deceleration * time.delta_seconds();

        if b_physics.current_velocity.x <= 0.005 { 

            b_collision.enabled = false;
            b_physics.current_velocity = Vec3::ZERO;

        }

    }
    else { // Bullet as a little bubble
        
        b_transform.translation += b_physics.current_velocity * (time.delta_seconds() * 60.0); // Movement

        b_physics.current_velocity.y += b_physics.acceleration * time.delta_seconds(); //              Floating away |
        b_physics.current_velocity.x += bullet.float_horizontal_acceleration * time.delta_seconds();// Acceleration  |

        if b_transform.translation.y >= 350.0 { // Despawn when off screen
            commands.entity(entity).despawn();
        }

    }

    if b_physics.current_velocity.x <= 0.1 {

        if !(b_texture.index >= 5) // Animation
        {
            if a_tools.ticks_i <= 0.0 {
                b_texture.index += 1;
                a_tools.ticks_i = PLAYER_BULLET_TICK_LENGTH;
            }
            else { a_tools.tick(time.delta_seconds()); }
        }

    }
    }
}