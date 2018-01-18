use piston_window::G2dTexture;
use cgmath::Vector2;

#[derive(Clone)]
pub struct Texture {
    pub texture: G2dTexture,
    pub scale: Vector2<f64>,
}

impl Texture {
    pub fn new(texture: G2dTexture, scale: Vector2<f64>) -> Self {
        Self { texture, scale }
    }

    pub fn get_texture(&self) -> &G2dTexture {
        return &self.texture;
    }
}