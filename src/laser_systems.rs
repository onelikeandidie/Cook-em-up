use bevy::{ecs::bundle::Bundle, math::Vec2, prelude::{Commands, Entity, IntoSystem, Plugin, Query, Res, SpriteSheetBundle, Transform, With}};

use crate::{enemy_systems::AI, util::{Health, HitBox, Speed, TIME_STEP, WinSize}};

pub struct Laser;

pub struct FromPlayer;
pub struct FromEnemy;

pub struct Damage(pub f32, pub f32);

//#region Bundles
#[derive(Bundle)]
pub struct LaserBundle {
    pub damage: Damage,
    pub speed: Speed,
    pub laser: Laser,

    #[bundle]
    pub sprite: SpriteSheetBundle
}
impl Default for LaserBundle {
    fn default() -> Self {
        Self {
            damage: Damage(1., 1.),
            speed: Speed(0., 500.),
            laser: Laser,
            sprite: SpriteSheetBundle::default()
        }
    }
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
        if 0. > translation.y || translation.y > win_size.h {
            commands.entity(laser_entity).despawn();
        }
    });
}

fn laser_hit(
    mut commands: Commands,
    mut query: Query<(&mut Health, &HitBox, &Transform, With<AI>)>,
    mut laser_query: Query<(Entity, &Transform, &Damage, With<Laser>, Option<&FromPlayer>, Option<&FromEnemy>)>
) {
    query.for_each_mut(|(mut health, hitbox, transform, _)|{
        laser_query.for_each_mut(|(
            laser_entity,
            laser_transform,
            damage,
            _,
            from_player,
            from_enemy
        )| {
            if let Some(from_player) = from_player {
                if (hitbox.contains(&transform.translation,&laser_transform.translation)) {
                    health.0 -= damage.0;
                    commands.entity(laser_entity).despawn();
                }
            }
        });
    });
}

//#endregion

pub struct LaserSystemsPlugin;

impl Plugin for LaserSystemsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_system(laser_movement.system())
            .add_system(laser_hit.system())
            .add_system(laser_disappear.system());
        }
}