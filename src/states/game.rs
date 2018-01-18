use cgmath::Vector2;

use specs::{World, DispatcherBuilder, RunNow, Dispatcher};
use piston_window::{EventLoop, clear, text, Button, Event, Context,
                    Input, Loop, Key, PistonWindow, Size, WindowSettings, Motion};
use piston_window::{UpdateArgs, RenderArgs, ButtonArgs, Flip, G2dTexture, TextureSettings};
use opengl_graphics::{GlGraphics, OpenGL};
use piston_window;

use components::render::SpriteComponent;
use components::transform::TransformComponent;
use components::pickable::PickableComponent;

use systems::render::SpriteRenderSystem;
use systems::game_controller::GameControllerSystem;
use systems::picking::PickingSystem;

use render::{Renderer, RenderCommandQueue};
use render::camera::Camera;
use render::texture::Texture;
use input::InputController;

use states::State;

pub struct Game<'a> {
    world: World,
    sim_dispatch: Dispatcher<'a, 'a>,
    render_dispatch: Dispatcher<'a, 'a>,
    renderer: Renderer,
}

impl<'a> Game<'a> {
    pub fn new(mut window: PistonWindow) -> Self {
        let mut world = World::new();

        // Resources
        world.add_resource(RenderCommandQueue::new());
        world.add_resource(InputController::new());
        world.add_resource(Camera::new());

        // Register
        world.register::<SpriteComponent>();
        world.register::<TransformComponent>();
        world.register::<PickableComponent>();

        // Rendering has to be thread local and seperate from simulation
        let mut render_dispatch = DispatcherBuilder::new()
            .add_thread_local(SpriteRenderSystem)
            .build();

        let mut sim_dispatch = DispatcherBuilder::new()
            .add(GameControllerSystem, "game_controller_system", &[])
            .add(PickingSystem, "picking_system", &[])
            .build();

        world
            .create_entity()
            .with(TransformComponent { position: Vector2::new(0, 0), rotation: 0.0 })
            .with(SpriteComponent {
                texture: Texture::new(
                    piston_window::Texture::from_path(
                        &mut window.factory,
                        "assets/worker.png",
                        Flip::None,
                        &TextureSettings::new(),
                    ).unwrap(),
                    Vector2::new(0.25, 0.25),
                )
            })
            .with(PickableComponent { size: Vector2::new(50, 50) })
            .build();

        world
            .create_entity()
            .with(TransformComponent { position: Vector2::new(500, 500), rotation: 45.0 })
            .with(SpriteComponent {
                texture: Texture::new(
                    piston_window::Texture::from_path(
                        &mut window.factory,
                        "assets/worker.png",
                        Flip::None,
                        &TextureSettings::new(),
                    ).unwrap(),
                    Vector2::new(0.25, 0.25),
                )
            })
            .build();

        Self {
            world,
            sim_dispatch,
            render_dispatch,
            renderer: Renderer::new(window),
        }
    }
}

impl<'a> State for Game<'a> {
    fn simulate(&mut self, args: UpdateArgs) {
        self.sim_dispatch.dispatch(&self.world.res);
    }

    fn draw(&mut self, args: RenderArgs) {
        {
            let mut camera = self.world.write_resource::<Camera>();
            self.renderer.update(&(*camera), args);
            self.renderer.clear();
        }

        // Populate render queue
        self.render_dispatch.dispatch(&mut self.world.res);

        // Consume render queue and actually draw
        self.renderer.draw(&mut (*self.world.write_resource::<RenderCommandQueue>()));
        self.renderer.flush();
    }

    fn input_event(&mut self, args: ButtonArgs) {
        {
            let mut input_controller = self.world.write_resource::<InputController>();
            input_controller.button_event(args.button, args.state);
        }
    }

    fn mouse_motion(&mut self, motion: Motion) {
        {
            let mut input_controller = self.world.write_resource::<InputController>();
            input_controller.mouse_motion(motion);
        }
    }

    fn get_next_event(&mut self) -> Option<Event> {
        self.renderer.window.next()
    }
}
