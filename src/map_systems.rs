use bevy::{math::Vec3, prelude::{Commands, IntoSystem, Plugin, Res, ResMut, SpriteSheetBundle, SystemStage, Transform}, sprite::TextureAtlasSprite};

use crate::{enemy_systems::{AICircle, AIEntrance, EnemyBundle, EntranceDirections}, game_systems::GameState, util::{Materials, WinSize}};

pub struct MapState {
    pub last_spawn: f32,
}

fn map_setup (
    mut commands: Commands
) {
    commands
        .insert_resource(MapState {
            last_spawn: 0.,
        })
}

fn enemy_spawn (
    mut commands: Commands,
    assets: Res<Materials>,
    game_state: Res<GameState>,
    mut map_state: ResMut<MapState>,
    win_size: Res<WinSize>
) {
    if map_state.last_spawn + 20. < game_state.distance.0 {
        map_state.last_spawn = game_state.distance.0 + 20.;
        commands
            .spawn_bundle(EnemyBundle {
                sprite: SpriteSheetBundle {
                    texture_atlas: assets.enemy_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..Default::default() 
                    },
                    transform: Transform {
                        translation: Vec3::new(win_size.w + 100., win_size.h - 75., 10.),
                        scale: Vec3::new(4., 4., 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(AIEntrance { 
                direction: EntranceDirections::Left 
            })
            .insert(AICircle {
                 x_origin: win_size.half_w, 
                 y_origin: win_size.half_h + 200., 
                 x_radius: 100., y_radius: 100. 
            });
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_startup_system(map_setup.system())
            .add_system(enemy_spawn.system());
    }
}