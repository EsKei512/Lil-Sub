use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::components::*;
use super::resources::*;

use crate::systems::parse_json;

pub fn spawn_debug_menu (
    mut commands : Commands,
) {
    commands.spawn(DebugMenu {
            current_menu_index: MenuIndices::CLOSED, 
            current_curor_index: 0
        });
}

pub fn debug_menu_logic (
    mut debug_menu_query : Query<(&mut DebugMenu), With<DebugMenu>>,
    mut debug_menu_child_query : Query<Entity, With<DebugMenuChild>>,
    mut debug_window_query : Query<&mut DebugMenuWindow, With<DebugMenuWindow>>,
    mut commands : Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut db_menu) in debug_menu_query.iter_mut() {


    match db_menu.current_menu_index {
        MenuIndices::CLOSED => {
            
            for (entity) in debug_menu_child_query.iter() { // If closed, erase all debug menu entities
                commands.entity(entity).despawn();
            }

            if keyboard_input.just_pressed(KeyCode::F12) { // Check and open the debug menu if needed
                db_menu.current_menu_index = MenuIndices::HOME;
                db_menu.current_curor_index = 0;
            }

        },
        MenuIndices::HOME => {

            let mut window_count : u8 = 0;

            for (db_window) in debug_window_query.iter() { // Count how many window entities
                window_count = window_count + 1;
            }
            if window_count < 8 { // If windows are not around, spawn new ones
                
                for i in 0u8..9 { // Spawn window. Each loop spawns a different component (corner, border, fill)

                    let texture : Handle<Image> = match i {
                        0 => asset_server.load("debug/corner_ul.png"),
                        1 => asset_server.load("debug/corner_ur.png"),
                        2 => asset_server.load("debug/corner_dl.png"),
                        3 => asset_server.load("debug/corner_dr.png"),
                        4 => asset_server.load("debug/border_u.png"),
                        5 => asset_server.load("debug/border_l.png"),
                        6 => asset_server.load("debug/border_d.png"),
                        7 => asset_server.load("debug/border_r.png"),
                        _ => asset_server.load("debug/fill.png"),
                    };
    
                    commands.spawn((
                        DebugMenuWindow {
                            sprite_id: match i { // This determines the behavior of the window section
                                0 => DebugWindowIDs::UpLeftCorner,
                                1 => DebugWindowIDs::UpRightCorner,
                                2 => DebugWindowIDs::DownLeftCorner,
                                3 => DebugWindowIDs::DownRightCorner,
                                4 => DebugWindowIDs::UpBorder,
                                5 => DebugWindowIDs::LeftBorder,
                                6 => DebugWindowIDs::DownBorder,
                                7 => DebugWindowIDs::RightBorder,
                                _ => DebugWindowIDs::Fill,
                            },
                            size: Vec2::new(22.0, 10.0),
                        },
                        SpriteBundle {
                            transform: Transform::from_xyz(0.0, 0.0, 998.0),
                            texture : texture,
                            ..default()
                        },
                        DebugMenuChild,
                    ));
                }
                
            }

            if keyboard_input.just_pressed(KeyCode::F12) { // Code to close the debug menu if requested
                db_menu.current_menu_index = MenuIndices::CLOSED;
                db_menu.current_curor_index = 0;
            }

        },
        MenuIndices::ABOUT => todo!(),
        MenuIndices::QUIT_AYS => todo!(),
    }

    
}}

pub fn debug_menu_window_logic (
    mut debug_menu_window_query : Query<(&DebugMenuWindow, &mut Transform), With<DebugMenuWindow>>,
) {
    for (db_window, mut db_transform) in debug_menu_window_query.iter_mut() {

    match db_window.sprite_id {
        DebugWindowIDs::UpLeftCorner => {
            db_transform.translation.x = 0.0 + MENU_OFFSET.x;
            db_transform.translation.y = 0.0 + MENU_OFFSET.y;
            db_transform.scale = Vec3::ONE;
        },
        DebugWindowIDs::UpRightCorner => {
            db_transform.translation.x = (db_window.size.x * TILE_SIZE.x) + MENU_OFFSET.x;
            db_transform.translation.y = 0.0 + MENU_OFFSET.y;
            db_transform.scale = Vec3::ONE;
        },
        DebugWindowIDs::DownLeftCorner => {
            db_transform.translation.x = 0.0 + MENU_OFFSET.x;
            db_transform.translation.y = (-db_window.size.y * TILE_SIZE.y) + MENU_OFFSET.y;
            db_transform.scale = Vec3::ONE;
        },
        DebugWindowIDs::DownRightCorner => {
            db_transform.translation.x = (db_window.size.x * TILE_SIZE.x) + MENU_OFFSET.x;
            db_transform.translation.y = (-db_window.size.y * TILE_SIZE.y) + MENU_OFFSET.y;
            db_transform.scale = Vec3::ONE;
        },
        DebugWindowIDs::UpBorder => {
            db_transform.translation.x = ((db_window.size.x * TILE_SIZE.x) / 2.0) + MENU_OFFSET.x;
            db_transform.translation.y = 0.0 + MENU_OFFSET.y;
            db_transform.scale = Vec3::new(db_window.size.x - 1.0, 1.0, 1.0);
        },
        DebugWindowIDs::LeftBorder => {
            db_transform.translation.x = 0.0 + MENU_OFFSET.x;
            db_transform.translation.y = ((-db_window.size.y * TILE_SIZE.y) / 2.0) + MENU_OFFSET.y;
            db_transform.scale = Vec3::new(1.0, db_window.size.y - 1.0, 1.0);
        },
        DebugWindowIDs::DownBorder => {
            db_transform.translation.x = ((db_window.size.x * TILE_SIZE.x) / 2.0) + MENU_OFFSET.x;
            db_transform.translation.y = (-db_window.size.y * TILE_SIZE.y) + MENU_OFFSET.y;
            db_transform.scale = Vec3::new(db_window.size.x - 1.0, 1.0, 1.0);
        },
        DebugWindowIDs::RightBorder => {
            db_transform.translation.x = (db_window.size.x * TILE_SIZE.x) + MENU_OFFSET.x;
            db_transform.translation.y = ((-db_window.size.y * TILE_SIZE.y) / 2.0) + MENU_OFFSET.y;
            db_transform.scale = Vec3::new(1.0, db_window.size.y - 1.0, 1.0);
        },
        DebugWindowIDs::Fill => {
            db_transform.translation.x = ((db_window.size.x * TILE_SIZE.x) / 2.0) + MENU_OFFSET.x;
            db_transform.translation.y = (-(db_window.size.y * TILE_SIZE.y) / 2.0) + MENU_OFFSET.y;;
            db_transform.scale = Vec3::new(db_window.size.x - 1.0, db_window.size.y - 1.0, 1.0);;
        },
    }

}}