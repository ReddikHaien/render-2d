use crate::resource::texture::GlTexture;

use super::Sprite;

#[derive(Default)]
pub struct UvSprite{
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    texture: GlTexture
}

impl UvSprite{
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32, texture: GlTexture) -> Self{
        Self{
            min_x,
            max_x,
            max_y, 
            min_y,
            texture
        }
    }
}

impl Sprite for UvSprite{
    fn fill_sprite<'a, 'b: 'a>(&'b self, out: &mut [f32]) -> &'a GlTexture {
        out[0] = self.min_x;
        out[1] = self.min_y;
        out[2] = self.max_x;
        out[3] = self.max_y;

        &self.texture
    }
}