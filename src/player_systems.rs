use bevy::{core::Time, input::Input, math::Vec3, prelude::{Bundle, Commands, IntoSystem, KeyCode, Plugin, Query, Res, SpriteSheetBundle, SystemStage, Transform, With}, sprite::TextureAtlasSprite};

use crate::{gun_systems::{Gun, GunCollection, GunCooldown}, laser_systems::{FromPlayer, LaserBundle}, util::{Health, Materials, Speed, TIME_STEP, WinSize}};

pub struct Player;

pub struct PlayerState {
    movement: PlayerMoveStates,
    state_step: f32
}
impl Default for PlayerState {
    fn default() -> Self {
        Self { 
            movement: PlayerMoveStates::Idle, 
            state_step: 0. 
        }
    }
}

pub enum PlayerMoveStates {
    Idle,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub player_speed: Speed,
    pub player_state: PlayerState,
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
            player_state: Default::default(),
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
                    translation: Vec3::new(window.half_w, 75. / 2. + 5., 10.),
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
    mut query: Query<(With<Player>, &mut Transform, &Speed, &mut PlayerState)>,
    ws: Res<WinSize>,
    time: Res<Time>
) {
    query.for_each_mut(
        |(_, mut transform, speed, mut state)| {
            let mut p = &mut transform.translation;

            let ydir: f32 = if ws.padding_bottom + 50. < p.y && keyboard_input.pressed(KeyCode::Down) {
                state.movement = PlayerMoveStates::MoveDown;
                -1.
            } else if p.y < ws.h - (ws.padding_top +  50.) && keyboard_input.pressed(KeyCode::Up) {
                state.movement = PlayerMoveStates::MoveUp;
                1.
            } else {
                0.
            };
            p.y += ydir * speed.1 * time.delta().as_secs_f32();

            let xdir: f32 = if ws.padding_left + 50. < p.x && keyboard_input.pressed(KeyCode::Left) {
                state.movement = PlayerMoveStates::MoveLeft;
                -1.
            } else if p.x < ws.w - (ws.padding_right +  50.) && keyboard_input.pressed(KeyCode::Right) {
                state.movement = PlayerMoveStates::MoveRight;
                1.
            } else {
                state.movement = PlayerMoveStates::Idle;
                0.
            };
            p.x += xdir * speed.0 * time.delta().as_secs_f32();
        }
    );
}

fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    assets: Res<Materials>,
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
                        texture_atlas: assets.projectile_atlas.clone(),
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
                    speed: gun.initial_speed.clone(),
                    ..Default::default()
                }).insert(FromPlayer);
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

fn player_state_update (
    time: Res<Time>,
    mut query: Query<(With<Player>, &mut PlayerState)>,
) {
    let t = time.delta().as_secs_f32() * 10.;
    query.for_each_mut(
        |(_, mut state)| {
            match state.movement {
                PlayerMoveStates::Idle => {
                    if state.state_step > 0. {
                        state.state_step -= t;
                        if state.state_step < 0. {
                            state.state_step = 0.;
                        }
                    } else if state.state_step < 0. {
                        state.state_step += t;
                        if state.state_step > 0. {
                            state.state_step = 0.;
                        }
                    }
                },
                PlayerMoveStates::MoveLeft => {
                    if state.state_step > -2. {
                        state.state_step -= t;
                    }
                },
                PlayerMoveStates::MoveRight => {
                    if state.state_step < 2. {
                        state.state_step += t;
                    }
                },
                _ => {}
            }
        }
    );
}

fn player_sprite_update (
    mut query: Query<(With<Player>, &mut TextureAtlasSprite, &PlayerState )>
) {
    query.for_each_mut(
        |(_, mut sprite, state)| {
            sprite.index = (state.state_step.round().clamp(-2., 2.) + 2.) as u32;
        }
    )
}

//#endregion

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_startup_stage("game_setup_actors",SystemStage::single(player_spawn.system()))
            .add_system(player_movement.system())
            .add_system(player_shoot.system())
            .add_system(player_sprite_update.system())
            .add_system(player_state_update.system());
        }
}