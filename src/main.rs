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
use bevy_kira_audio::prelude::*;

pub mod components;
use components::*;
pub mod resources;
use resources::*;
pub mod systems;
use systems::*;

pub mod esbv_debug_menu;
use esbv_debug_menu::EsBvDebugPlugin;
pub mod player;
use player::PlayerPlugin;
pub mod enemy;
use enemy::EnemyPlugin;
pub mod particles;
use particles::ParticlesPlugin;
pub mod hud;
use hud::HudPlugin;
pub mod collectables;
use collectables::CollectablesPlugin;

fn main() {
    App::new()                                // Plugins
        .add_plugins((                             
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            AudioPlugin,
            PlayerPlugin,
            EnemyPlugin,
            ParticlesPlugin,
            EsBvDebugPlugin,
            HudPlugin,
            CollectablesPlugin,
        ))

        .insert_resource(Msaa::Off)

        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))

        .add_systems(Startup, ( // Startup Systems
            initialize_game,
            setup_camera.after(initialize_game),
        ))

        .add_systems(Update, (  // Update systems
            fit_canvas,
        ))
        
        .run();
}
