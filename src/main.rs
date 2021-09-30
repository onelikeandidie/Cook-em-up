#![allow(unused)] // Because unused for cleaner IDE

use bevy::input::Input;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{App, AssetServer, Assets, Bundle, ClearColor, Color, Commands, Entity, Handle, HandleUntyped, IntoSystem, KeyCode, OrthographicCameraBundle, Query, Res, ResMut, SpriteBundle, SpriteSheetBundle, SystemStage, Texture, Transform, With};
use bevy::DefaultPlugins;
use bevy::core::Time;
use bevy::sprite::{ColorMaterial, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite};
use bevy::window::{WindowDescriptor, WindowMode, Windows};

const PLAYER_SPRITE: &str = "players/player_1_idle.png";
const LASER_SPRITE: &str = "projectiles/laser.png";

const TIME_STEP: f32 = 1. / 60.;

//#region Resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser_materials: Handle<ColorMaterial>,
    player_atlas: Handle<TextureAtlas>,
    projectile_atlas: Handle<TextureAtlas>,
}

pub struct WinSize {
    w: f32,
    h: f32
}

//#endregion
//#region Components
pub struct Player;
pub struct AI;
pub struct Laser;

/** First is current cooldown, second is reset cooldown */
pub struct GunCooldown(f32, f32);
impl Default for GunCooldown {
    fn default() -> Self {
        Self (1., 1.)
    }
}

pub struct Gun {
    damage: f32,
    cooldown: GunCooldown,
    offset: Vec3,
}
impl Default for Gun {
    fn default() -> Self {
        Self {
            damage: 1.,
            cooldown: GunCooldown::default(),
            offset: Vec3::ZERO
        }
    }
}

pub struct GunCollection {
    guns: Box<[Gun]>
}
impl Default for GunCollection {
    fn default() -> Self {
        Self {
            guns: Box::new([Gun::default()])
        }
    }
}

pub struct Speed(f32, f32);
impl Default for Speed {
    fn default() -> Self {
        Self (0., 0.)
    }
}

pub struct Health(f32, f32);

//#endregion
//#region Bundles
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    player_speed: Speed,
    weapon: GunCollection,
    health: Health,

    #[bundle]
    sprite: SpriteSheetBundle
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            player_speed: Speed(500., 300.),
            weapon: GunCollection {
                guns: Box::new([
                    Gun {
                        cooldown: GunCooldown(0., 0.33),
                        offset: Vec3::new(20., 0., 0.),
                        ..Default::default()
                    }, Gun {
                        cooldown: GunCooldown(0., 0.33),
                        offset: Vec3::new(-20., 0., 0.),
                        ..Default::default()
                    }
                ])
            },
            health: Health(100., 100.),
            sprite: SpriteSheetBundle::default()
        }
    }
}

#[derive(Bundle)]
struct LaserBundle {
    damage: f32,
    speed: Speed,
    laser: Laser,

    #[bundle]
    sprite: SpriteSheetBundle
}
impl Default for LaserBundle {
    fn default() -> Self {
        Self {
            damage: 1.,
            speed: Speed(0., 500.),
            laser: Laser,
            sprite: SpriteSheetBundle::default()
        }
    }
}

//#endregion
//#region Startup Systems
fn setup(
    mut commands: Commands, 
    mut windows: ResMut<Windows>
) {
    let mut window = windows.get_primary().unwrap();
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
    mut textures: ResMut<Assets<Texture>>,
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
        player_atlas: load_atlas("players/player_1.png", Vec2::new(32., 32.), 5, 1),
        projectile_atlas: load_atlas("projectiles/projectiles.png", Vec2::new(8., 8.), 4, 4),
    });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, window: Res<WinSize>) {
    // Spawn a sprite
    commands
        .spawn_bundle(PlayerBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: materials.player_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 2,
                    ..Default::default() 
                },
                transform: Transform {
                    translation: Vec3::new(0., -window.h / 2. + 75. / 2. + 5., 10.),
                    scale: Vec3::new(2., 2., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
}

//#endregion
//#region Player Update Systems
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(With<Player>, &mut Transform, &Speed)>,
) {
    if let Ok((_, mut transform, speed)) = query.single_mut() {
        let xdir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };

        transform.translation.x += xdir * speed.0 * TIME_STEP;

        let ydir = if keyboard_input.pressed(KeyCode::Down) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Up) {
            1.
        } else {
            0.
        };

        transform.translation.y += ydir * speed.1 * TIME_STEP;
    }
}

fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Transform, &mut Gun, With<Player>)>,
    mut query2: Query<(&Transform, &mut GunCollection, With<Player>)>
) {
    let mut shoot_guns = 
        |transform: &Transform, gun: &mut Gun| {
            let x = transform.translation.x;
            let y = transform.translation.y;
            let off_x = gun.offset.x;
            let off_y = gun.offset.y;
            commands
                .spawn_bundle(LaserBundle {
                    sprite: SpriteSheetBundle {
                        texture_atlas: materials.projectile_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: 0,
                            ..Default::default() 
                        },
                        transform: Transform {
                            translation: Vec3::new(x + off_x, y + off_y, 0.),
                            scale: Vec3::new(2., 2., 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                });
            gun.cooldown.0 = gun.cooldown.1;
        };

    query.for_each_mut(|(player_transform, mut gun, _)| {
        if keyboard_input.pressed(KeyCode::X) && gun.cooldown.0 == 0. {
            shoot_guns(player_transform, &mut gun);
        }
    });

    query2.for_each_mut(|(player_transform, mut gun_collection, _)| {
        if keyboard_input.pressed(KeyCode::X) {
            let mut guns = &mut *gun_collection.guns;
            for gun in guns {
                if gun.cooldown.0 == 0. {
                    shoot_guns(player_transform, gun);
                } 
            }
        }
    });
}

//#endregion
//#region Gun Systems
fn gun_cooldown(
    time: Res<Time>,
    mut query: Query<&mut Gun>,
    mut query2: Query<&mut GunCollection>
) {
    let mut update_cooldowns = 
        |gun: &mut Gun| {
            if gun.cooldown.0 > 0. {
                gun.cooldown.0 -= time.delta().as_secs_f32();
                if gun.cooldown.0 < 0. {
                    gun.cooldown.0 = 0.
                }
            }
        };
    query.for_each_mut(|(mut gun)| {
        update_cooldowns(&mut gun);
    });
    query2.for_each_mut(|(mut gun_collection)| {
        let mut guns = &mut *gun_collection.guns;
        for gun in guns {
            update_cooldowns(gun);
        }
    });
}

//#endregion
//#region Laser Systems
fn laser_movement (
    mut query: Query<(Entity, &mut Transform, &Speed, With<Laser>)>
) {
    query.for_each_mut(|(laser_entity, mut transform, speed, _)| {
        let translation = &mut transform.translation;
        if speed.0 != 0. {
            translation.x += speed.0 * TIME_STEP;
        }
        if speed.1 != 0. {
            translation.y += speed.1 * TIME_STEP;
        }
    });
}

fn laser_disappear(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Transform, With<Laser>)>
) {
    query.for_each_mut(|(laser_entity, mut transform, _)| {
        let translation = &transform.translation;
        if translation.y > win_size.h {
            commands.entity(laser_entity).despawn();
        }
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
        .add_startup_stage("game_setup_actors",SystemStage::single(player_spawn.system()))
        .add_system(player_movement.system())
        .add_system(player_shoot.system())
        .add_system(laser_movement.system())
        .add_system(laser_disappear.system())
        .add_system(gun_cooldown.system())
        .run(); // Start the app
}
//#endregion