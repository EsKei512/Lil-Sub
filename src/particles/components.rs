use bevy::prelude::*;

use super::resources::ParticleIds;

#[derive(Component)]
pub struct Particle {
    pub particle_id: ParticleIds,
}