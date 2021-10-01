use bevy::{prelude::Handle, sprite::{ColorMaterial, TextureAtlas}};


pub const TIME_STEP: f32 = 1. / 60.;
pub const SPRITE_SCALE: f32 = 2.;

//#region Resources
pub struct Materials {
    pub player_materials: Handle<ColorMaterial>,
    pub laser_materials: Handle<ColorMaterial>,
    pub player_atlas: Handle<TextureAtlas>,
    pub projectile_atlas: Handle<TextureAtlas>,
}

pub struct WinSize {
    pub w: f32,
    pub h: f32
}

//#endregion

//#region Components
pub struct Speed(pub f32, pub f32);
impl Default for Speed {
    fn default() -> Self {
        Self (0., 0.)
    }
}

pub struct Health(pub f32, pub f32);

//#endregion