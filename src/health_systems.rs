use bevy::prelude::{Commands, Entity, IntoSystem, Plugin, Query};

use crate::util::Health;

fn health_update(
    mut commands: Commands,
    query: Query<(Entity, &Health)>
) {
    query.for_each_mut(|(entity, health)| {
        if health.0 <= 0. {
            commands.entity(entity).despawn();
        }
    });
}

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_system(health_update.system());
    }
}