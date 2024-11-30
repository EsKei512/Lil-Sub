use bevy::prelude::*;

use crate::Complex2dMovement;

use super::resources::ParticleBehaviorIDs;
use super::resources::ParticleSpawnerShapes;

#[derive(Component, Clone)]
pub struct Particle {
    pub particle_behavior: ParticleBehaviorIDs, // An enum which determines the behavior of the particle

    pub argument_one   : f32, // A generic argument to be used by the particle
    pub argument_two   : f32, // A generic argument to be used by the particle

    pub time_scale : f32, // A multiplier for the particles for fine-tuning

    pub lifetime : f32,

    pub time_before_animation_advance : f32,
    pub animation_random_offset : f32,

    pub movement : Complex2dMovement,
    pub initial_force : f32,
    pub rotation_offset : f32,
}
#[derive(Component)]
pub struct ParticleSpawner {
    pub particle_graphics: Handle<Image>, // The graphics used by the particle
    pub particle_graphics_dimensions: UVec2, // The sprite sheet info for the particle
    pub particle_graphics_texture_size: u32,

    pub current_tick : f32,
    pub tick_max     : f32,
    pub tick_min     : f32,

    pub spawning_shape : ParticleSpawnerShapes,
    pub spawning_shape_dimensions : Vec2,

    pub spawner_position : Vec3,

    pub particle_to_spawn : Particle,

    pub limited_particles  : bool,
    pub how_many_particles : u32,
}

#[derive(Component)]
pub struct ParticleSpawnerIdentifier;

#[derive(Component)]
pub struct ParticleIdentifier;