use bevy::prelude::*;

// Particle behaviors
#[derive(Clone)]
pub enum ParticleBehaviorIDs {
    // Particles burst out in a circular shape
    // ARG 1 - The particles spread 
    // ARG 2 - The particles direction offset
    // ARG 3 - The speed at which the particles shoot out 
    Burst,
}

// The shape of the particle spawner
pub enum ParticleSpawnerShapes {
    Point,
    Square,
    Circle,
}