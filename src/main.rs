#![allow(unused)] // Because unused for cleaner IDE

mod player_systems;
mod enemy_systems;
mod gun_systems;
mod laser_systems;
mod util;
mod assets_config;
mod game_systems;
mod map_systems;
mod collision_systems;
mod health_systems;

use assets_config::{ENEMY_SPRITESHEET_1, FONT_TTF, LASER_SPRITE, PLAYER_SPRITE, PLAYER_SPRITESHEET, PROJECTILE_SPRITESHEET};
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::math::{Vec2};
use bevy::prelude::{App, AssetServer, Assets, ClearColor, Color, Commands, Handle, IntoSystem, OrthographicCameraBundle, Res, ResMut, Texture, Transform};
use bevy::DefaultPlugins;
use bevy::render::camera::{Camera, DepthCalculation, OrthographicProjection};
use bevy::render::render_graph::base::camera::CAMERA_2D;
use bevy::sprite::{ColorMaterial, TextureAtlas};
use bevy::text::Font;
use bevy::window::{WindowDescriptor, WindowMode, Windows};
use enemy_systems::EnemyPlugin;
use game_systems::GameSystemsPlugin;
use gun_systems::GunSystemsPlugin;
use health_systems::HealthPlugin;
use laser_systems::{LaserSystemsPlugin};
use map_systems::MapPlugin;
use player_systems::{PlayerPlugin};
use util::{Materials, WinSize};

//#region Startup Systems
fn setup(
    mut commands: Commands, 
    windows: ResMut<Windows>
) {
    let window = windows.get_primary().unwrap();

    let win_size = WinSize {
        w: window.width(),
        h: window.height(),
        half_w: window.width() / 2.,
        half_h: window.height() / 2.,
        padding_top: 10.,
        padding_right: 10.,
        padding_bottom: 25.,
        padding_left: 10.,
    };

    // Create resources
    commands.insert_resource(win_size.clone());

    // Add a camera
    let far = 1000.0;
    commands.spawn_bundle(OrthographicCameraBundle {
            camera: Camera {
                name: Some(CAMERA_2D.to_string()),
                ..Default::default()
            },
            orthographic_projection: OrthographicProjection {
                far,
                depth_calculation: DepthCalculation::ZDifference,
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_xyz(win_size.half_w.clone(), win_size.half_h.clone(), far - 0.1),
            global_transform: Default::default(),
        }
    );
}

fn load_assets(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut atlas: ResMut<Assets<TextureAtlas>>,
    mut fonts: ResMut<Assets<Font>>,
) {
    // Closure for loading atlas (spritesheets) and choppin them
    let mut load_atlas = 
        |path: &str, tile_size: Vec2, columns: usize, rows: usize| -> Handle<TextureAtlas> {
            let texture_handle: Handle<Texture> = asset_server.load(path).into();
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                tile_size,
                columns, rows
            );
            atlas.add(texture_atlas)
        };

    // Load assets
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser_materials: materials.add(asset_server.load(LASER_SPRITE).into()),
        player_atlas: load_atlas(PLAYER_SPRITESHEET, Vec2::new(32., 32.), 5, 1),
        projectile_atlas: load_atlas(PROJECTILE_SPRITESHEET, Vec2::new(8., 8.), 4, 4),
        enemy_atlas: load_atlas(ENEMY_SPRITESHEET_1, Vec2::new(16., 16.), 1, 1),
        font: asset_server.load(FONT_TTF).into()
    });
}

//#endregion
//#region Main
fn main() {
    App::build() // Create the Application
        .insert_resource(WindowDescriptor {
            title: "Cook'em Up".to_owned(),
            width: 600.0, 
            height: 800.0,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::hex("000000").unwrap()))
        .add_plugins(DefaultPlugins)
        // Adds a system that prints diagnostics to the console
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        //.add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup.system())
        .add_startup_system(load_assets.system())
        .add_plugin(GameSystemsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GunSystemsPlugin)
        .add_plugin(LaserSystemsPlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(MapPlugin)
        .run(); // Start the app
}
//#endregion