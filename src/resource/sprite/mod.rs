use super::texture::GlTexture;

pub mod uv_sprite;
pub trait Sprite {
    fn fill_sprite<'a, 'b: 'a>(&'b self, out: &mut [f32]) -> &'a GlTexture;
}
