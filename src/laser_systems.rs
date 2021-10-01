use bevy::{ecs::bundle::Bundle, prelude::{Commands, Entity, IntoSystem, Plugin, Query, Res, SpriteSheetBundle, Transform, With}};

use crate::util::{Speed, TIME_STEP, WinSize};

pub struct Laser;

pub struct Damage(pub f32, pub f32);

//#region Bundles
#[derive(Bundle)]
pub struct LaserBundle {
    pub damage: f32,
    pub speed: Speed,
    pub laser: Laser,

    #[bundle]
    pub sprite: SpriteSheetBundle
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

pub struct LaserSystemsPlugin;

impl Plugin for LaserSystemsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_system(laser_movement.system())
            .add_system(laser_disappear.system());
        }
}