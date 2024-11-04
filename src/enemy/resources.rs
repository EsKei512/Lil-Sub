use bevy::prelude::*;

#[derive(Debug)]
pub enum EnemyStates {
    Paused,
    Spawining,
    Idle,
    PreparingToAttack,
    Attacking,
    Returning,
    Dead,
}