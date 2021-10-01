use bevy::{input::Input, math::Vec3, prelude::{Bundle, Commands, IntoSystem, KeyCode, Plugin, Query, Res, SpriteSheetBundle, SystemStage, Transform, With}, sprite::TextureAtlasSprite};

use crate::{gun_systems::{Gun, GunCollection, GunCooldown}, laser_systems::LaserBundle, util::{Health, Materials, Speed, TIME_STEP, WinSize}};

pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub player_speed: Speed,
    pub weapon: GunCollection,
    pub health: Health,

    #[bundle]
    pub sprite: SpriteSheetBundle
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            player_speed: Speed(500., 200.),
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
            sprite: SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3::new(2.,2., 1.),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}

//#region Player Setup systems
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_startup_stage("game_setup_actors",SystemStage::single(player_spawn.system()))
            .add_system(player_movement.system())
            .add_system(player_shoot.system());
        }
}