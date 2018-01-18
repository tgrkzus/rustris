use specs::VecStorage;
use specs::Component;

use cgmath::Vector2;

pub struct PickableComponent {
    pub size: Vector2<i32>,
}

impl Component for PickableComponent {
    type Storage = VecStorage<Self>;
}