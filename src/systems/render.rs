use specs::{System, SystemData, ReadStorage, FetchMut, Join};

use components::render::SpriteComponent;
use components::transform::TransformComponent;

use render::{RenderCommandQueue, RenderCommand};

pub struct SpriteRenderSystem;

impl<'a> System<'a> for SpriteRenderSystem {
    type SystemData = (ReadStorage<'a, SpriteComponent>,
                       ReadStorage<'a, TransformComponent>,
                       FetchMut<'a, RenderCommandQueue>);

    fn run(&mut self, (sprites, transforms, mut queue): Self::SystemData) {
        for (sprite, transform) in (&sprites, &transforms).join() {
            queue.add(
                RenderCommand::DrawSprite(
                    sprite.texture(),
                    transform.get_position(),
                    transform.get_rotation_deg()
                ));
        }
    }
}
