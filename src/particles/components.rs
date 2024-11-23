use bevy::prelude::*;

use super::resources::ParticleBehaviorIDs;
use super::resources::ParticleSpawnerShapes;

#[derive(Component)]
pub struct Particle {
    pub particle_graphics: Handle<Image>,
    pub particle_graphics_dimensions: Vec2,
    pub particle_behavior: ParticleBehaviorIDs,
    pub argument_one : f32,
    pub argument_two : f32,
    pub time_scale : f32,
}
#[derive(Component)]
pub struct ParticleSpawner {
    pub current_tick : f32,
    pub tick_max : f32,
    pub spawning_shape : ParticleSpawnerShapes,
    pub spawning_shape_dimensions : Vec2,
    pub particle_to_spawn : Particle,
}