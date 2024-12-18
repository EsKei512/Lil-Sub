use bevy::{
    ecs::query, prelude::*, render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    }, sprite::MaterialMesh2dBundle, window::WindowResized
};

use super::resources::*;

use super::components::*;

use serde_json::Value;

use std::fs::{self, read_to_string, File};
use std::io::{self, Read};

pub fn setup_camera (
    mut commands: Commands, 
    mut images: ResMut<Assets<Image>>,
) {
    let canvas_size = Extent3d {
        width: GAME_RES_X,
        height: GAME_RES_Y,
        ..default()
    };

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        InGameCamera,
        PIXEL_PERFECT_RENDERING,
    ));

    // spawn the canvas
    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        PIXEL_IMPERFECT_RENDERING,
    ));

    // the "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2dBundle::default(), OuterCamera, PIXEL_IMPERFECT_RENDERING));
}

pub fn fit_canvas( // TODO: Fix the scale at certain resolutions
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width  / GAME_RES_X as f32;
        let v_scale = event.height / GAME_RES_Y as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}

pub fn initialize_game (
    mut commands: Commands,
    asset_server : Res<AssetServer>,
) {
    commands.spawn((
        GlobalEnt,
        EnemySpawningQueue {
            queue: Vec::new(),
        },
        GameSettings {
            ..default()
        },
        PlayerStats {
            hp: 5,
            money: 0,
        },
    ));

    // Debug Background
    commands.spawn((
        SpriteBundle{
            texture : asset_server.load("sprites/testing/manbg.png"),
            transform : Transform::from_xyz(0.0, 0.0, -999.0),
            ..default()
        },
        DebugBackground,
    ));
}

pub fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn parse_json(path: &str) -> Value {
    match read_to_string(path) {
        Ok(jval) => {
            serde_json::from_str(&jval).unwrap()
        },
        Err(error) => panic!("Error. Invalid file: {:?}", error),
    }
}