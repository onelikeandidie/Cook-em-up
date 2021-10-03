use bevy::{math::{Vec2, Vec3}, prelude::Handle, sprite::{ColorMaterial, TextureAtlas}, text::Font};


pub const TIME_STEP: f32 = 1. / 60.;
pub const SPRITE_SCALE: f32 = 2.;

//#region Resources
pub struct Materials {
    pub player_materials: Handle<ColorMaterial>,
    pub laser_materials: Handle<ColorMaterial>,
    pub player_atlas: Handle<TextureAtlas>,
    pub projectile_atlas: Handle<TextureAtlas>,
    pub enemy_atlas: Handle<TextureAtlas>,
    pub font: Handle<Font>
}

#[derive(Clone, Copy)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
    pub half_w: f32,
    pub half_h: f32,
    pub padding_top: f32,
    pub padding_right: f32,
    pub padding_bottom: f32,
    pub padding_left: f32,
}

//#endregion

//#region Components
#[derive(Clone, Copy)]
pub struct Speed(pub f32, pub f32);
impl Default for Speed {
    fn default() -> Self {
        Self (0., 0.)
    }
}

pub struct Health(pub f32, pub f32);

pub struct HitBox {
    pub rect: Vec3,
}
impl HitBox {
    pub fn contains (&self, origin: &Vec3, point: &Vec3) -> bool {
        let rect = *origin;
        let outrect = *origin - self.rect;
        outrect.x < point.x && point.x < rect.x && outrect.y < point.y && point.y < rect.y
    }
}
//#endregion