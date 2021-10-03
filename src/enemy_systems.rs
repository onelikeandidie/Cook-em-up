use std::f32::consts::PI;

use bevy::{core::Time, math::{Vec2, Vec3}, prelude::{Bundle, Commands, Entity, IntoSystem, Plugin, Query, Res, SpriteSheetBundle, Transform, With}, sprite::TextureAtlasSprite};

use crate::{gun_systems::{Gun, GunCollection, GunCooldown}, laser_systems::{FromEnemy, LaserBundle}, util::{Health, HitBox, Materials, Speed}};

//#region Components
pub struct AI;

pub struct AIState {
    pub movement: AIMoveStates,
    pub state_step: f32
}
impl Default for AIState {
    fn default() -> Self {
        Self { 
            movement: AIMoveStates::Entering, 
            state_step: 0. 
        }
    }
}

#[derive(std::cmp::PartialEq)]
pub enum AIMoveStates {
    Entering,
    Hovering
}

pub struct AICircle {
    pub x_origin: f32,
    pub y_origin: f32,
    pub x_radius: f32,
    pub y_radius: f32,
}

pub struct AIHorizontal;

pub struct AIEntrance {
    pub direction: EntranceDirections
}

pub enum EntranceDirections {
    Left,
    Up,
    Right,
}


#[derive(Bundle)]
pub struct EnemyBundle {
    pub ai: AI,
    pub speed: Speed,
    pub state: AIState,
    pub weapon: GunCollection,
    pub health: Health,
    pub hitbox: HitBox,

    #[bundle]
    pub sprite: SpriteSheetBundle
}
impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            ai: AI,
            speed: Speed(10., 10.),
            state: Default::default(),
            weapon: GunCollection {
                guns: Box::new([
                    Gun {
                        cooldown: GunCooldown(0., 1.),
                        offset: Vec3::new(0., 0., 0.),
                        initial_speed: Speed(0., -350.),
                        ..Default::default()
                    }
                ])
            },
            health: Health(1., 1.),
            hitbox: HitBox {
                rect: Vec3::new(50., 50., 1.)
            },
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

//#endregion

fn enemy_update (
    time: Res<Time>,
    query: Query<(Entity, &mut AIState, With<AI>)>
) {
    query.for_each_mut(|(entity, mut state, _)| {
        state.state_step += time.delta().as_secs_f32();
    });
}

fn enemy_entrance_circle_movement (
    query: Query<(&mut Transform, &AIState, &AIEntrance, &AICircle, &Speed, With<AI>)>
) {
    query.for_each_mut(|(
            mut transform, 
            state, 
            entrance, 
            circle, 
            speed, _
        )| {
            if state.movement == AIMoveStates::Entering {
                let max_distance_x = state.state_step * speed.0;
                let max_distance_y = state.state_step * speed.1;
                // Compute angles
                let tx = speed.0 * state.state_step % 360. / PI;
                let ty = speed.1 * state.state_step % 360. / PI;

                // Calculate circle position?
                let x_dst = circle.x_radius * tx.cos() + circle.x_origin;
                let y_dst = circle.y_radius * ty.sin() + circle.y_origin;

                let current_x = transform.translation.x;
                let current_y = transform.translation.y;

                // Calculate Distance
                let dx = current_x - x_dst;
                let dy = current_y - y_dst;
                let distance_x = (dx * dx).sqrt();
                let distance_y = (dy * dy).sqrt();
                let distance_ratio_x = if distance_x == 0. {
                    0. 
                } else  {
                    max_distance_x / distance_x
                };
                let distance_ratio_y = if distance_y == 0. {
                    0. 
                } else  {
                    max_distance_y / distance_y
                };

                let mut x = current_x - dx * distance_ratio_x;
                x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
                let mut y = current_y - dy * distance_ratio_y;
                y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

                // Apply maths
                transform.translation.x = x;
                transform.translation.y = y;

                //match entrance.direction {
                //    EntranceDirections::Left => {
                //    },
                //    EntranceDirections::Up => {},
                //    EntranceDirections::Right => {},
                //}
            }
    });
}

fn enemy_shoot (
    mut commands: Commands,
    materials: Res<Materials>,
    query: Query<(Entity, &Transform, &mut Gun, With<AI>)>,
    query2: Query<(Entity, &Transform, &mut GunCollection, With<AI>)>
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
                            index: 2,
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
                }).insert(FromEnemy);
            gun.cooldown.0 = gun.cooldown.1;
        };
        
    query.for_each_mut(|(entity, transform, mut gun, _)| {
        if gun.cooldown.0 == 0. {
            shoot_guns(&transform, &mut gun);
        }
    });

    query2.for_each_mut(|(entity, transform, mut gun_collection, _)| {
        let mut guns = &mut *gun_collection.guns;
        for gun in guns {
            if gun.cooldown.0 == 0. {
                shoot_guns(&transform, gun);
            } 
        }
    });
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_system(enemy_shoot.system())
            .add_system(enemy_update.system())
            .add_system(enemy_entrance_circle_movement.system());
    }
}