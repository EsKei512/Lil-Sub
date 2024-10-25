use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub is_active: bool,
}

#[derive(Component)]
pub struct PlayerChild;

#[derive(Component)]
pub struct PlayerBullet {
    pub float_horizontal_acceleration : f32,
}