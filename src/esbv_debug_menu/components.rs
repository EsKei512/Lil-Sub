use bevy::{prelude::*, reflect::Array};

use super::resources::*;
use serde_json::Value;

#[derive(Component)]
pub struct DebugMenu {
    pub current_menu_index : MenuIndices,
    pub current_curor_index: i32,
}

#[derive(Component)]
pub struct DebugMenuChild;

#[derive(Component)]
pub struct DebugMenuWindow {
    pub sprite_id: DebugWindowIDs,
    pub size : Vec2,
}

#[derive(Component)]
pub struct DebugTextOrigin {
    pub label     : String, // ------ The text the string is to display
    pub font_data : Value, // ------- The font data from a json file
    pub font_file : Handle<Image>, // The font file used
    pub text_id   : u64, // Unique ID to help with parent-child relationships
    pub origin    : Vec3,
    pub index     : i32, // The line the string is on. Allows highlighting when hovered.
}
#[derive(Component)]
pub struct DebugTextCharacter {
    pub sprite_frame : i32,
    pub origin  : Vec3,
    pub index   : i32,
    pub text_id : u64,
}