#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate piston_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate cgmath;
extern crate specs;

use piston_window::{RenderArgs, UpdateArgs, EventLoop, clear, text,
                    Button, Event, Context, Input, Loop, Key, PistonWindow, Size, WindowSettings};
use opengl_graphics::{GlGraphics, OpenGL};

pub mod components;
pub mod render;
pub mod input;
pub mod states;
pub mod systems;

use states::State;
use states::game::Game;

fn main() {
    info!("Starting...");

    // Renderer TODO configurable
    let opengl = OpenGL::V3_2;

    // Create an Glutin window. Resizes to fit window size
    let mut window: PistonWindow = WindowSettings::new(
        "Game",
        [1024, 786]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    info!("Window successfully started");

    window.set_ups(120);
    window.set_max_fps(60);

    // Make state
    let mut state = Game::new(window);

    info!("State initialized. Starting game loop...");
    while let Some(e) = state.get_next_event() {
        match e {
            Event::Loop(loop_event) => {
                match loop_event {

                    Loop::Update(args) => {
                        state.simulate(args);
                    }

                    Loop::Idle(args) => {
                        // Do background tasks
                    }

                    Loop::Render(args) => {
                        state.draw(args);
                    }

                    Loop::AfterRender(args) => {

                    }

                    _ => {
                        warn!("Unhandled loop event!");
                        println!("Unhandled loop event");
                    }
                }
            },

            Event::Input(input) => {
                match input {
                    Input::Button(args) => {
                        state.input_event(args);
                    }

                    Input::Move(motion) => {
                        state.mouse_motion(motion);
                    }

                    Input::Text(args) => {
                        // TODO
                    }

                    Input::Resize(x, y) => {

                    }

                    Input::Focus(is_focused) => {

                    }

                    Input::Cursor(is_cursor_in) => {

                    }

                    Input::Close(args) => {

                    }

                    _ => {
                        warn!("Unhandled input event!");
                        println!("Unhandled input event");
                    }
                }
            },

            _ => { },
        }
    }
}
