use std::f32::consts::PI;

use bevy::math::VectorSpace;
use bevy_kira_audio::prelude::*;

use bevy::math::quat;
use bevy::prelude::*;
use bevy::tasks::futures_lite::stream::Once;
use bevy::transform::commands;
use rand::Rng;

use crate::particles::components::*;
use crate::particles::resources::*;
use crate::resources::*;
use crate::components::*;

use super::components::*;
use super::resources::*;

trait FetchUniqueId {
    fn fetch_unique_id(&mut self) -> u64;
}


pub fn spawn_enemy(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut global_query: Query<(&mut EnemySpawningQueue, &mut GlobalEnt, &GameSettings), With<GlobalEnt>>,
) {
    for (mut e_spawining_queue, mut unique_id_handler, game_settings) in global_query.iter_mut()
    {

    while e_spawining_queue.queue.len() > 0 {

        let enemy_id : EnemyIds = e_spawining_queue.queue[0].e_id.clone();
        let origin   : Vec2     = e_spawining_queue.queue[0].origin.clone();

        e_spawining_queue.queue.remove(0);

        let texture : Handle<Image> = match enemy_id { // Get the right texture for the enemy
            EnemyIds::Debug    => asset_server.load("sprites/gameplay/enemies/debug.png"),
            EnemyIds::Goldfish => asset_server.load("sprites/gameplay/enemies/goldfish.png"),
        };

        let layout : Handle<TextureAtlasLayout> = match enemy_id { // Get the right texture for the enemy
            EnemyIds::Debug    => texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 1, 1, None, None)),
            EnemyIds::Goldfish => texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 1, None, None)),
        };

        let enemy_to_spawn = commands.spawn((
            SpriteBundle {
                transform : Transform::from_xyz(origin.x, origin.y, 3.0),
                texture,
                ..default()
            },
            TextureAtlas {
                layout,
                index: 0,
            },
            GameEnemy {
                enemy_id,
                origin,
                current_state : EnemyStates::Spawining,
                state_ticks : 0.0,
                hitbox_size : 0.0,
                enabled : true,
                hurt_ticks: 0.0,
                target_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                target_scale: Vec2::ONE,
            }, 
            AnimationTools {
                ..default()
            },
            Stats {
                max_hp: 5,
                cur_hp: 5,
                damage: 0,
            },
            Complex2dMovement {
                soft_terminal_velocity: 2.0,
                hard_terminal_velocity: 2.0,
                acceleration: 0.0,
                natural_deceleration: 0.0,
                current_velocity: Vec3::ZERO,
            },
        )).id();
        
        match enemy_id {
            EnemyIds::Debug    => commands.entity(enemy_to_spawn).insert((EnDebug{..default()},    EnemySounds{ hurt_sound: asset_server.load("sfx/enemy_hit_med.wav"), death_sound: asset_server.load("sfx/kill.wav") })),
            EnemyIds::Goldfish => commands.entity(enemy_to_spawn).insert((EnGoldfish{..default()}, EnemySounds{ hurt_sound: asset_server.load("sfx/enemy_hit_med.wav"), death_sound: asset_server.load("sfx/kill.wav") })),
        };

        }
    }
}

pub fn universal_enemy_logic(
    mut enemy_query : Query<(Entity, &mut TextureAtlas, &mut Complex2dMovement, &mut Stats, &mut GameEnemy, &mut AnimationTools, &mut Transform, &mut Sprite, &EnemySounds), (With<GameEnemy>, Without<Player>, Without<PlayerBullet>)>,
    player_query: Query<&Transform, (With<Player>, Without<GameEnemy>, Without<PlayerBullet>)>,
    p_bullet_query: Query<(Entity, &Transform, &Collision), (With<PlayerBullet>, Without<Player>, Without<GameEnemy>)>,
    mut commands : Commands,
    time: Res<Time>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut e_texture, mut e_velocity, mut e_stats, mut enemy, mut a_tools, mut e_transform, mut e_sprite, e_audio) in enemy_query.iter_mut()
    {
    for (p_transform) in player_query.iter() 
    {
    if enemy.enabled {

        if e_stats.cur_hp <= 0
            {
            if let EnemyStates::Dead = enemy.current_state {
                audio.play(e_audio.death_sound.clone()).with_panning(((e_transform.translation.x + (GAME_RES_X as f32 / 2.0)) / GAME_RES_X as f32) as f64);
                enemy.enabled = false;
                commands.spawn((
                    ParticleSpawner{ 
                        particle_graphics: asset_server.load("sprites/gameplay/particles/smoke_poof.png"), 
                        particle_graphics_dimensions: UVec2::new(8, 1), 
                        particle_graphics_texture_size: 14,
                        current_tick: 0.0, 
                        tick_max: 0.01,
                        tick_min: 0.0,
                        spawning_shape: ParticleSpawnerShapes::Point, 
                        spawning_shape_dimensions: Vec2::ZERO, 
                        particle_to_spawn: Particle{ 
                            particle_behavior: ParticleBehaviorIDs::Burst, 
                            argument_one: 6.2, 
                            argument_two: 0.0, 
                            time_scale: 1.0,
                            lifetime: 1.0,
                            time_before_animation_advance: 0.125,
                            animation_random_offset: 0.05,
                            movement: Complex2dMovement {
                                soft_terminal_velocity: 10.0,
                                hard_terminal_velocity: 10.0,
                                acceleration: 0.0,
                                natural_deceleration: 250.0,
                                current_velocity: Vec3::new(0.0, 0.0, 0.0),
                            },
                            initial_force: 125.0,
                            rotation_offset: 0.0,
                        },
                        limited_particles: true,
                        how_many_particles: 9,
                        spawner_position: e_transform.translation, 
                    },
                    ParticleSpawnerIdentifier,
                ));

                commands.entity(entity).despawn();
            } else { enemy.current_state = EnemyStates::Dead }
            }
        else {
            for (b_entity, b_transform, b_collision) in p_bullet_query.iter() { // Bullet Collision Thingy

                let distance : f32 = Vec2::new(b_transform.translation.x, b_transform.translation.y).distance(Vec2::new(e_transform.translation.x, e_transform.translation.y));

                if (distance <= enemy.hitbox_size) && (b_collision.enabled) {
                    e_stats.cur_hp -= 1;
                    enemy.hurt_ticks = ENEMY_HURT_TICKS;
                    commands.entity(b_entity).despawn();
                    if e_stats.cur_hp <= 0 {
                        enemy.current_state = EnemyStates::Dead;
                    }
                    else {audio.play(e_audio.hurt_sound.clone()).with_panning(((e_transform.translation.x + (GAME_RES_X as f32 / 2.0)) / GAME_RES_X as f32) as f64);}
                }
            }
            
            if enemy.hurt_ticks > 0.0 {

                e_sprite.color = Color::srgb((255.0 / ENEMY_HURT_TICKS) * enemy.hurt_ticks, (255.0 / ENEMY_HURT_TICKS) * enemy.hurt_ticks, (255.0 / ENEMY_HURT_TICKS) * enemy.hurt_ticks);
                e_transform.scale.x = (0.12 * enemy.hurt_ticks) + 1.0;
                e_transform.scale.y = (0.12 * enemy.hurt_ticks) + 1.0;
                enemy.hurt_ticks -= time.delta_seconds() * 60.0;

            }
            else {
                e_sprite.color = enemy.target_color;
                e_transform.scale.x = enemy.target_scale.x;
                e_transform.scale.y = enemy.target_scale.y;
            }
        }


    }

    }}
    
}

pub fn run_enemy_bullet_logic(

) {
    
}

pub fn debug_enemy_logic(
    mut enemy_query : Query<(Entity, &mut TextureAtlas, &mut Complex2dMovement, &mut Stats, &mut GameEnemy, &mut AnimationTools, &mut Transform, &mut EnDebug), (With<GameEnemy>, Without<Player>, Without<PlayerBullet>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (entity, mut e_texture, mut e_velocity, mut e_stats, mut enemy, mut a_tools, mut e_transform, mut extra) in enemy_query.iter_mut()
    {
    for (p_transform) in player_query.iter() 
    {

    match enemy.current_state {
        
        EnemyStates::Paused => return,

        EnemyStates::Spawining => return,

        EnemyStates::Idle => return,

        EnemyStates::PreparingToAttack => todo!(),

        EnemyStates::Attacking => todo!(),

        EnemyStates::Returning => todo!(),

        EnemyStates::Dead => todo!(),

    }

    }
    }
}

pub fn goldfish_enemy_logic(
    mut enemy_query : Query<(Entity, &mut TextureAtlas, &mut Complex2dMovement, &mut Stats, &mut GameEnemy, &mut AnimationTools, &mut Transform, &mut EnGoldfish), (With<GameEnemy>, Without<Player>, Without<PlayerBullet>)>,
    player_query: Query<&Transform, (With<Player>, Without<GameEnemy>, Without<PlayerBullet>)>,
    time: Res<Time>,
) {
    for (entitiy, mut e_texture, mut e_velocity, mut e_stats, mut enemy, mut a_tools, mut e_transform, mut extra) in enemy_query.iter_mut()
    {
    for (p_transform) in player_query.iter() 
    {
    if enemy.enabled {
        
        match enemy.current_state {
            
            EnemyStates::Paused => return,

            EnemyStates::Spawining => {
                a_tools.ticks_i = 20.0;
                a_tools.generic_counter_i = 3;
                a_tools.generic_counter_ii = rand::thread_rng().gen_range(2..6);
                e_velocity.acceleration = 300.0;
                enemy.hitbox_size = 15.0;
                enemy.current_state = EnemyStates::Idle;
            },

            EnemyStates::Idle => {
                a_tools.tick(time.delta_seconds());
                if a_tools.ticks_i <= 0.0 {
                    a_tools.ticks_i = 20.0;
                    a_tools.generic_counter_i -= 1;
                    if a_tools.generic_counter_i < 0 {
                        a_tools.generic_counter_i = 3;
                        a_tools.generic_counter_ii -= 1;
                        a_tools.ticks_i = 40.0
                    }
                    else if a_tools.generic_counter_i == 0 {
                        a_tools.ticks_i = 10.0
                    }

                    if a_tools.generic_counter_ii <= 0 {
                        extra.calculations_finished = false;
                        enemy.current_state = EnemyStates::PreparingToAttack;
                    }
                }

                if a_tools.generic_counter_i == 0 {
                    e_transform.translation.y = enemy.origin.y + 1.0;
                    if a_tools.ticks_ii <= 0.0 {
                        a_tools.ticks_ii = 6.5;
                        if e_texture.index <= 2 {
                            e_texture.index += 1;
                        }
                        else {
                            e_texture.index = 0;
                        }
                    }
                }
                if a_tools.generic_counter_i == 1 {
                    e_transform.translation.y = enemy.origin.y + 2.0;
                    if a_tools.ticks_ii <= 0.0 {
                        a_tools.ticks_ii = 3.0;
                        if e_texture.index <= 2 {
                            e_texture.index += 1;
                        }
                        else {
                            e_texture.index = 0;
                        }
                    }
                }
                if a_tools.generic_counter_i == 2 {
                    e_transform.translation.y = enemy.origin.y + 1.0;
                    if a_tools.ticks_ii <= 0.0 {
                        a_tools.ticks_ii = 5.0;
                        if e_texture.index <= 2 {
                            e_texture.index += 1;
                        }
                        else {
                            e_texture.index = 0;
                        }
                    }
                }
                else if a_tools.generic_counter_i == 3 {
                    e_transform.translation.y = enemy.origin.y;
                    if a_tools.ticks_ii <= 0.0 {
                        a_tools.ticks_ii = 12.5;
                        if e_texture.index <= 2 {
                            e_texture.index += 1;
                        }
                        else {
                            e_texture.index = 0;
                        }
                    }
                }

            },

            EnemyStates::PreparingToAttack => {
                if !(extra.calculations_finished) { // Only do the calculations if we need to. Save some time
                    extra.goldfish_target_position = Vec2::new(p_transform.translation.x - 5.0, p_transform.translation.y);
                    
                    let opposite : f32 = e_transform.translation.x - p_transform.translation.x - 5.0;
                    let adjacent : f32 = e_transform.translation.y - p_transform.translation.y;
                    extra.goldfish_angle = (opposite / adjacent).atan(); // Use some trig to calculate the angle of attack

                    extra.goldfish_movement.x = e_velocity.acceleration * extra.goldfish_angle.sin().abs(); // X: Angular Movement
                    extra.goldfish_movement.y = e_velocity.acceleration * extra.goldfish_angle.cos(); // Y: Angular Movement

                    if e_transform.translation.y > p_transform.translation.y {
                        extra.goldfish_movement.y = extra.goldfish_movement.y * -1.0;
                    }

                    e_velocity.current_velocity.x = 25.0;

                    let diff = e_transform.translation - p_transform.translation; // | Shamelessly stole this code from stack overflow my brain is dead
                    let angle = diff.y.atan2(diff.x); // <----------------------------| https://stackoverflow.com/questions/65370874/bevy-rotation-in-2d

                    extra.goldfish_angle_offset = angle;

                    extra.calculations_finished = true;
                }
                e_texture.index = 4;
                if e_velocity.current_velocity.x >= 0.0 {
                    e_velocity.current_velocity.x -= 25.0 * time.delta_seconds();

                    e_transform.translation.x += e_velocity.current_velocity.x * time.delta_seconds();
                    
                    let current_rotation : f32 = e_transform.rotation.to_euler(EulerRot::XYZ).2.clone();

                    e_transform.rotate(Quat::from_axis_angle(Vec3::new(0., 0., 1.), (extra.goldfish_angle_offset - current_rotation) / (15.00 / (time.delta_seconds() * 60.0)).clamp(1.0, 100.0)));

                } else {
                    enemy.current_state = EnemyStates::Attacking;
                    e_texture.index = 5;
                }
            },

            EnemyStates::Attacking => {
                //println!("Goldfish Movement: {}", extra.goldfish_movement);
                e_transform.translation.x -= extra.goldfish_movement.x * time.delta_seconds();
                e_transform.translation.y += extra.goldfish_movement.y * time.delta_seconds();

                if (e_transform.translation.x.abs() > 270.0) || (e_transform.translation.y.abs() > 140.0) {
                    e_transform.translation.x = 300.0;
                    e_transform.translation.y = rand::thread_rng().gen_range(-200.0..200.0);
                    a_tools.ticks_iii = 150.0;
                    enemy.current_state = EnemyStates::Returning;
                }
            },

            EnemyStates::Returning => {
                let current_rotation = (e_transform.rotation.to_euler(EulerRot::XYZ)).2.clone();
                e_transform.rotation = Quat::from_rotation_z(0.0);
                e_texture.index = 0;
                if a_tools.ticks_iii > 0.0 {
                    a_tools.tick(time.delta_seconds());
                    e_transform.translation.x += ((enemy.origin.x - e_transform.translation.x) / (20.0)) * (time.delta_seconds() * 60.0);
                    e_transform.translation.y += ((enemy.origin.y - e_transform.translation.y) / (10.0)) * (time.delta_seconds() * 60.0);
                } else {
                    a_tools.ticks_i = 20.0;
                    a_tools.generic_counter_i = 3;
                    a_tools.generic_counter_ii = rand::thread_rng().gen_range(2..6);
                    e_texture.index = 0;
                    enemy.current_state = EnemyStates::Idle;
                }
            },

            EnemyStates::Dead => return,

        }
        }
    }
    }
}


pub fn debug_wave_spawn(
    mut global_query: Query<&mut EnemySpawningQueue, With<GlobalEnt>>
) {
    for (mut spawining_queue) in global_query.iter_mut()
    {
        spawining_queue.queue.push(EnemySpawningInfo{ origin: Vec2::new(100.0, 0.0), e_id: EnemyIds::Debug});
        for _i in 0..25 {spawining_queue.queue.push(EnemySpawningInfo {origin: Vec2::new(rand::thread_rng().gen_range(0.0..150.0), rand::thread_rng().gen_range(-100.0..100.0)),e_id: EnemyIds::Goldfish,});}
    }
}