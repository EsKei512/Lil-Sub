use bevy::prelude::*;

use crate::resources::*;

use std::collections::HashMap;

pub const DEBUG_ENABLED : bool = true; // Should probably disable this before publishing.....

pub enum MenuIndices {
    CLOSED,
    HOME,
    ABOUT,
    QUIT_AYS,
}

#[derive(Debug)]
pub enum DebugWindowIDs {
    UpLeftCorner,
    UpRightCorner,
    DownLeftCorner,
    DownRightCorner,
    UpBorder,
    LeftBorder,
    DownBorder,
    RightBorder,
    Fill,
}

pub const MENU_OFFSET : Vec2 = Vec2::new(5.0 - (GAME_RES_X as f32 / 2.0), -5.0 + (GAME_RES_Y as f32 / 2.0));

pub const TILE_SIZE : Vec2 = Vec2::new(6.0, 8.0);