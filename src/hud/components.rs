use bevy::{prelude::*, reflect::Array};

#[derive(Component)]
pub struct GameUi {

}

#[derive(Component)]
pub struct UiComponent;

#[derive(Component)]
pub struct HealthUi {
    
}

#[derive(Component)]
pub struct MoneyUiCharacter {
    pub index : u32,
}

#[derive(Component)]
pub struct MoneyUi {
    pub characters : [u8 ; 6],
}