use bevy::prelude::*;

#[derive(Resource)]
struct GameButtons {
    main_goal: bool,
    bonus: u32,
}