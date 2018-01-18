use specs::DenseVecStorage;
use specs::Component;

use render::texture::Texture;

pub struct SpriteComponent {
    pub texture: Texture,
}

impl Component for SpriteComponent {
    type Storage = DenseVecStorage<Self>;
}

impl SpriteComponent {
    /// Returns a copy of the texture
    pub fn texture(&self) -> Texture {
        self.texture.clone()
    }
}