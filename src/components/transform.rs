use specs::DenseVecStorage;
use specs::Component;
use cgmath::Vector2;

#[derive(Debug)]
pub struct TransformComponent {
    pub position: Vector2<i32>,
    pub rotation: f64,
}

impl TransformComponent {
    pub fn get_position(&self) -> Vector2<i32> {
        return self.position;
    }

    pub fn get_rotation_deg(&self) -> f64 {
        return self.rotation;
    }

    pub fn set_position(&mut self, p: Vector2<i32>) {
        self.position = p;
    }

    pub fn add_position(&mut self, p: Vector2<i32>) {
        self.position += p;
    }
}

impl Component for TransformComponent {
    type Storage = DenseVecStorage<Self>;
}