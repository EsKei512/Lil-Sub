use bevy::prelude::*;
use bevy::transform;

use super::resources::ParticleBehaviorIDs;
use super::resources::ParticleSpawnerShapes;

use crate::components::*;

use rand::Rng;

use super::components::*;

pub fn particle_spawner_logic(
    mut spawner_query : Query<(Entity, &mut ParticleSpawner), (With<ParticleSpawnerIdentifier>)>,
    mut commands : Commands,
    time : Res<Time>,
    asset_server : Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, mut spawner) in spawner_query.iter_mut()
    {
    
        spawner.current_tick -= time.delta_seconds();


        if spawner.current_tick > 0.0 {continue;} // Break out of the loop if we don't need to spawn a particle yet


        if spawner.limited_particles == true && spawner.how_many_particles > 0 {

            spawner.how_many_particles -= 1;
            spawner.current_tick = rand::thread_rng().gen_range(spawner.tick_min..spawner.tick_max);

            match spawner.particle_to_spawn.particle_behavior { // Satanic code
                ParticleBehaviorIDs::Burst => {
                    spawner.particle_to_spawn.rotation_offset = rand::thread_rng().gen_range(0.0..spawner.particle_to_spawn.argument_one) + spawner.particle_to_spawn.argument_two;
                    spawner.particle_to_spawn.movement.current_velocity = Vec3::new(spawner.particle_to_spawn.initial_force * spawner.particle_to_spawn.rotation_offset.cos(), spawner.particle_to_spawn.initial_force * spawner.particle_to_spawn.rotation_offset.sin(), 0.0);
                },
            }

            commands.spawn((
                spawner.particle_to_spawn.clone(), // Particle data
                SpriteBundle {
                    transform : Transform::from_xyz(spawner.spawner_position.x, spawner.spawner_position.y, 6.0),
                    texture: spawner.particle_graphics.clone(),
                    ..default()
                },
                AnimationTools {
                    ticks_i: spawner.particle_to_spawn.time_before_animation_advance,
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(spawner.particle_graphics_texture_size), spawner.particle_graphics_dimensions.x, spawner.particle_graphics_dimensions.y, None, None)),
                    index: 0,
                },
                ParticleIdentifier,
            ));

        }
        else if spawner.limited_particles == false {

        }
        else { // Erase self if nothing left to do.
            //commands.entity(entity).despawn();
        }

    }
}

pub fn particle_logic(
    mut particle_query : Query<(Entity, &mut Particle, &mut TextureAtlas, &mut Transform, &mut AnimationTools), (With<ParticleIdentifier>)>,
    time : Res<Time>,
    mut commands : Commands,
) {
    for (entity, mut particle, mut p_texture, mut p_transform, mut a_tools) in particle_query.iter_mut()
    {

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn(); 
        }

        if a_tools.ticks_i <= 0.0 {
            p_texture.index += 1;
            a_tools.ticks_i = particle.time_before_animation_advance;
        }

        match particle.particle_behavior {
            ParticleBehaviorIDs::Burst => {

                //particle.movement.current_velocity.x += (rand::thread_rng().gen_range(0.0..particle.argument_one) + particle.argument_two).cos() * particle.movement.acceleration;
                //particle.movement.current_velocity.x -= (particle.rotation_offset).cos() * particle.movement.natural_deceleration * time.delta_seconds();

                //particle.movement.current_velocity.y += (rand::thread_rng().gen_range(0.0..particle.argument_one) + particle.argument_two).sin() * particle.movement.acceleration;
                if particle.movement.current_velocity.x > 0.0 {
                    particle.movement.current_velocity.x -= ((particle.rotation_offset).cos() * particle.movement.natural_deceleration * time.delta_seconds()).clamp(0.0, 10.0);
                }
                else {
                    particle.movement.current_velocity.x -= ((particle.rotation_offset).cos() * particle.movement.natural_deceleration * time.delta_seconds()).clamp(-10.0, 0.0);
                }


                if particle.movement.current_velocity.y > 0.0 {
                    particle.movement.current_velocity.y -= ((particle.rotation_offset).sin() * particle.movement.natural_deceleration * time.delta_seconds()).clamp(0.0, 10.0);
                }
                else {
                    particle.movement.current_velocity.y -= ((particle.rotation_offset).sin() * particle.movement.natural_deceleration * time.delta_seconds()).clamp(-10.0, 0.0);
                }
                //particle.movement.current_velocity.y -= (particle.rotation_offset).sin() * particle.movement.natural_deceleration * time.delta_seconds();

                p_transform.translation += particle.movement.current_velocity * time.delta_seconds();
            },
        }

        a_tools.ticks_i -= time.delta_seconds();
        particle.lifetime -= time.delta_seconds();

    }
}