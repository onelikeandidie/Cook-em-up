#![allow(unused)] // Because unused for cleaner IDE

mod player;
mod gun_systems;
mod laser_systems;
mod util;
mod assets_config;

use assets_config::{LASER_SPRITE, PLAYER_SPRITE, PLAYER_SPRITESHEET, PROJECTILE_SPRITESHEET};
use bevy::math::{Vec2};
use bevy::prelude::{App, AssetServer, Assets, ClearColor, Color, Commands, Handle, IntoSystem, OrthographicCameraBundle, Res, ResMut, Texture};
use bevy::DefaultPlugins;
use bevy::sprite::{ColorMaterial, TextureAtlas};
use bevy::window::{WindowDescriptor, WindowMode, Windows};
use gun_systems::GunSystemsPlugin;
use laser_systems::{LaserSystemsPlugin};
use player::{PlayerPlugin};
use util::{Materials, WinSize};

//#region Components
pub struct AI;

//#endregion
//#region Startup Systems
fn setup(
    mut commands: Commands, 
    windows: ResMut<Windows>
) {
    let window = windows.get_primary().unwrap();
    // Add a camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create resources
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height()
    });
}

fn load_assets(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut atlas: ResMut<Assets<TextureAtlas>>
) {
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
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup.system())
        .add_startup_system(load_assets.system())
        .add_plugin(PlayerPlugin)
        .add_plugin(GunSystemsPlugin)
        .add_plugin(LaserSystemsPlugin)
        .run(); // Start the app
}
//#endregion