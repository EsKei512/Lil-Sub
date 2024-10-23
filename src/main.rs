use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
    window::WindowResized,
};

pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod systems;
use systems::*;

pub mod player;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, fit_canvas)
        .insert_resource(Msaa::Off)
        .run();
}
