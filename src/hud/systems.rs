use bevy::{ecs::query, prelude::*};
use rand::seq::index;

use crate::resources::{GAME_RES_X, GAME_RES_Y};
use crate::components::{GlobalEnt, PlayerStats};

use super::components::*;

pub fn spawn_ui (
    mut commands : Commands,
    asset_server : Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let ui_entity : Entity = commands.spawn((
        SpriteBundle {
            texture : asset_server.load("sprites/gameplay/ui/hud/health_and_money.png"),
            transform : Transform::from_xyz(36.0 - (GAME_RES_X as f32 / 2.0), -14.0 + (GAME_RES_Y as f32 / 2.0), 25.0),
            ..default()
        },
        GameUi {

        },
        UiComponent,
        MoneyUi {
            characters : [0, 0, 0, 0, 0, 0],
        }
    )).id();

    let hp_entity : Entity = commands.spawn((
        SpriteBundle {
            texture : asset_server.load(""),
            transform : Transform::from_xyz(0.0, 0.0, 26.0),
            ..default()
        },
        HealthUi {

        },
        UiComponent
    )).id();

    for i in 0u32..6 { // The text that displays the money
        
        println!("{}", i);

        commands.spawn((
            SpriteBundle {
                transform : Transform::from_xyz(15.0 - (GAME_RES_X as f32 / 2.0) + (5.0 * (i + 1) as f32), -19.0 + (GAME_RES_Y as f32 / 2.0), 800.0),
                texture : asset_server.load("sprites/gameplay/ui/hud/money_text.png"),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(9), 12, 1, None, None)),
                index: 0,
            },
            MoneyUiCharacter {
                index : i
            },
            UiComponent
        ));

    }

    commands.entity(ui_entity).add_child(hp_entity);

}

pub fn universal_ui_logic (

) {
    
}

pub fn hp_ui_logic (

) {
    
}

pub fn money_ui_logic (
    mut money_text_query : Query<(& MoneyUiCharacter, &mut TextureAtlas), With<MoneyUiCharacter>>,
    mut money_query : Query<&mut MoneyUi, With<UiComponent>>,
    global_query : Query<&PlayerStats, With<GlobalEnt>>,
) {
    for player_stats in global_query.iter()
    {

    for mut money in money_query.iter_mut() {
        
        let mut money_to_calculate = player_stats.money.clone();

        for i in 1u32..7 { // This finds what character we need to display per character in the money UI
            let loop_index = 6 - i;

            if (money_to_calculate < 10u32.pow(loop_index)) && (i != 6)  { // If it's too small, display the tiny zero

                money.characters[(i - 1) as usize] = 11;

            } else {

                let money_reference : f32  = money_to_calculate.clone() as f32;

                let letter_value : u8 = (money_reference / 10u32.pow(loop_index) as f32).floor() as u8; // Get the letter value by doing some division
                
                money.characters[(i - 1) as usize] = letter_value;

                money_to_calculate -= 10u32.pow(loop_index) * letter_value as u32; // Prepare the value for the next loop by removing the character we just saw

            }
        }
    
    for (character, mut texture) in money_text_query.iter_mut() {

        texture.index = money.characters[character.index as usize] as usize;
        
    }}}
}