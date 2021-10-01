use bevy::{core::Time, math::Vec3, prelude::{IntoSystem, Plugin, Query, Res}};

/** First is current cooldown, second is reset cooldown */
pub struct GunCooldown(pub f32, pub f32);
impl Default for GunCooldown {
    fn default() -> Self {
        Self (1., 1.)
    }
}

pub struct Gun {
    pub damage: f32,
    pub cooldown: GunCooldown,
    pub offset: Vec3,
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
    pub guns: Box<[Gun]>
}
impl Default for GunCollection {
    fn default() -> Self {
        Self {
            guns: Box::new([Gun::default()])
        }
    }
}

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


pub struct GunSystemsPlugin;

impl Plugin for GunSystemsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_system(gun_cooldown.system());
        }
}