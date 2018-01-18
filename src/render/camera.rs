use cgmath::{Vector2, Ortho};
use piston_window::Viewport;

/// Orthographic camera
pub struct Camera {
    pub offset: Vector2<i32>, // Offset. Note that this is the center of the camera
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            offset: Vector2::new(0, 0),
            zoom: 1.0,
        }
    }

    pub fn set_offset(&mut self, p: Vector2<i32>) {
        self.offset = p;
    }

    pub fn add_offset(&mut self, p: Vector2<i32>) {
        self.offset += p;
    }

    pub fn set_zoom(&mut self, z: f32) {
        self.zoom = z;

        self.check_zoom_range();
    }

    pub fn add_zoom(&mut self, z: f32) {
        self.zoom += z;

        self.check_zoom_range();
    }

    fn check_zoom_range(&mut self) {
        if self.zoom < 0.25 {
            self.zoom = 0.25;
        }
        if self.zoom > 4.0 {
            self.zoom = 4.0;
        }
    }

    pub fn screen_to_world(&self, p: Vector2<i32>) -> Vector2<i32> {
        Vector2::new(((p - self.offset).x as f32 / self.zoom) as i32,
                     ((p - self.offset).y as f32 / self.zoom) as i32)
    }
}