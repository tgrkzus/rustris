pub mod utility;
pub mod camera;
pub mod texture;

use std::collections::VecDeque;

use opengl_graphics::{ GlGraphics, OpenGL };
use piston_window::{RenderArgs, Viewport, PistonWindow, G2dTexture, Event, OpenGLWindow,
                    Context};
use specs::World;

use cgmath::{Vector2, Matrix4};

use render::camera::Camera;
use render::texture::Texture;

pub struct Renderer {
    pub window: PistonWindow,
    viewport: Viewport,
    view_trans: Vector2<f64>,
    view_scale: Vector2<f64>
}

impl Renderer {
    pub fn new(window: PistonWindow) -> Self {
        Self {
            window,
            viewport: Viewport {
                rect: [0, 0, 0, 0],
                draw_size: [0, 0],
                window_size: [0, 0],
            },
            view_trans: Vector2::new(0.0, 0.0),
            view_scale: Vector2::new(1.0, 1.0),
        }
    }

    /// Update's the graphics context
    pub fn update(&mut self, camera: &Camera, args: RenderArgs) {
        self.viewport = args.viewport();
        self.view_trans = Vector2::new(camera.offset.x as f64, camera.offset.y as f64);
        self.view_scale = Vector2::new(camera.zoom as f64, camera.zoom as f64);
    }

    pub fn clear(&mut self) {
        self.window.g2d.draw(
            &mut self.window.encoder,
            &self.window.output_color,
            &self.window.output_stencil,
            self.viewport,
            |c, gl| {
                use piston_window::*;
                use graphics::*;

                // Clear the screen.
                clear([0.0, 0.5, 0.8, 1.0], gl);
            }
        );
    }

    pub fn draw(&mut self, queue: &mut RenderCommandQueue) {
        self.window.window.make_current();
        for cmd in queue.queue.iter() {
            match *cmd {
                RenderCommand::DrawSprite(ref tex, pos, rot) => {
                    self.draw_sprite(tex.get_texture(),
                                     pos,
                                     tex.scale,
                                     rot);
                }
            }
        }
        queue.queue.clear()
    }

    fn draw_sprite(&mut self,
                   tex: &G2dTexture,
                   position: Vector2<i32>,
                   scale: Vector2<f64>,
                   rot: f64) {

        let v_trans = self.view_trans;
        let v_scale = self.view_scale;
        // TODO fix garbage
        self.window.g2d.draw(
            &mut self.window.encoder,
            &self.window.output_color,
            &self.window.output_stencil,
            self.viewport,
            |c, gl| {
                use piston_window::*;
                use graphics::*;

                let transform = c.transform

                    .trans(v_trans.x, v_trans.y) // Camera trans
                    .scale(v_scale.x, v_scale.y) // Camera zoom

                    .trans(position.x as f64, position.y as f64) // Local
                    .scale(scale.x, scale.y)

                    .rot_rad(rot.to_radians());

                image(tex, transform, gl);
            }
        );
    }

    pub fn flush(&mut self) {
        self.window.encoder.flush(&mut self.window.device);
    }
}

pub enum RenderCommand {
    DrawSprite(Texture, Vector2<i32>, f64),
}

pub struct RenderCommandQueue {
    pub queue: VecDeque<RenderCommand>,
}

impl RenderCommandQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, command: RenderCommand) {
        self.queue.push_back(command);
    }
}