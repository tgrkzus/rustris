use specs::{System, SystemData, ReadStorage, WriteStorage, Fetch, FetchMut, Join};

use cgmath::Vector2;

use input::InputController;
use render::camera::Camera;

/// Handles the player controls. When in the normal game state
/// Different systems will be used for different states but this will handle the
/// majority of controls like camera movement, construction, orders etc. TODO figure out controls
pub struct GameControllerSystem;

impl<'a> System<'a> for GameControllerSystem {
    type SystemData = (Fetch<'a, InputController>,
                       FetchMut<'a, Camera>);

    fn run(&mut self, (mut input_controller, mut camera): Self::SystemData) {
        let mut offset: Vector2<i32> = Vector2::new(0, 0);
        if input_controller.is_pressed("MOVE_UP") {
            offset.y += 3;
        }
        if input_controller.is_pressed("MOVE_DOWN") {
            offset.y -= 3;
        }
        if input_controller.is_pressed("MOVE_LEFT") {
            offset.x += 3;
        }
        if input_controller.is_pressed("MOVE_RIGHT") {
            offset.x -= 3;
        }

        if input_controller.is_pressed("PLUS") {
            camera.add_zoom(0.01);
        }

        if input_controller.is_pressed("MINUS") {
            camera.add_zoom(-0.01);
        }

        camera.add_offset(offset);
    }
}
