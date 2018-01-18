use specs::{System, SystemData, ReadStorage, WriteStorage, Fetch, FetchMut, Join};

use cgmath::Vector2;

use components::pickable::PickableComponent;
use components::transform::TransformComponent;

use input::InputController;
use render::camera::Camera;

pub struct PickingSystem;

impl<'a> System<'a> for PickingSystem {
    type SystemData = (
        ReadStorage<'a, PickableComponent>,
        ReadStorage<'a, TransformComponent>,
        Fetch<'a, InputController>,
        Fetch<'a, Camera>,
    );

    fn run(&mut self, (pickables, transforms, input_controller, camera): Self::SystemData) {

        for (pickable, transform,) in (&pickables, &transforms,).join() {
            let hit = camera.screen_to_world(
                Vector2::new(input_controller.mouse.0, input_controller.mouse.1));

            let tl = transform.position;
            let tr = tl + Vector2::new(pickable.size.x, 0);
            let bl = tl + Vector2::new(0, pickable.size.y);
            let br = tl + Vector2::new(pickable.size.x, pickable.size.y);

            if hit.x >= tl.x && hit.y >= tl.y && hit.x <= br.x && hit.y <= br.y {
                println!("Hit {}, {}", hit.x, hit.y);
            }
        }
    }
}
