use bevy::{core::Time, math::Vec3, prelude::{Commands, HorizontalAlign, IntoSystem, Plugin, Query, Res, ResMut, SystemStage, Transform, VerticalAlign, With}, text::{Text, Text2dBundle, TextAlignment, TextSection, TextStyle}};

use crate::util::{Materials, WinSize};

pub struct GameState {
    pub distance: Distance,
}

pub struct Distance (pub f32, pub f32);

pub struct DistanceText;

fn setup_gamestate (
    mut commands: Commands,
) {
    commands.insert_resource(GameState {
        distance: Distance(0., 1000.),
    });
}

fn spawn_ui (
    mut commands: Commands,
    assets: Res<Materials>,
    win_size: Res<WinSize>,
    game_state: Res<GameState>,
) {
    let mut sections: Vec<TextSection> = Vec::new();
    sections.push(TextSection {
        value: "0.0".to_owned(),
        style: TextStyle {
            font: assets.font.clone(),
            font_size: 28.,
            ..Default::default()
        },
        ..Default::default()
    });
    sections.push(TextSection {
        value: [
            "/".to_owned(), 
            game_state.distance.1.round().to_string(), 
            "m".to_owned()].concat(),
        style: TextStyle {
            font: assets.font.clone(),
            font_size: 16.,
            ..Default::default()
        },
        ..Default::default()
    });
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: sections,
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Right,
                },
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(15., 15., 69.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DistanceText);
}

fn update_distance(
    time: Res<Time>, 
    mut game_state: ResMut<GameState>
) {
    let mut d = &mut game_state.distance;
    if d.0 < d.1 {
        d.0 += time.delta().as_secs_f32() * 10.;
        if d.0 > d.1 {
            d.0 = d.1;
        }
    }
}

fn update_ui (
    mut game_state: ResMut<GameState>,
    query: Query<(&mut Text, With<DistanceText>)>,
) {
    query.for_each_mut(
        |(mut text, _)| {
            text.sections.get_mut(0).unwrap().value = 
                game_state.distance.0.round().to_string();
        }
    );
}

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_startup_system(setup_gamestate.system())
            .add_startup_stage("game_setup_ui", SystemStage::single(spawn_ui.system()))
            .add_system(update_distance.system())
            .add_system(update_ui.system());
        }
}