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
    App::new()                                // Plugins
        .add_plugins((                             
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PlayerPlugin
        ))

        .insert_resource(Msaa::Off)

        .add_systems(Startup, ( // Startup Systems
            setup_camera,
        ))

        .add_systems(Update, (  // Update systems
            fit_canvas,
        ))
        
        .run();
}
