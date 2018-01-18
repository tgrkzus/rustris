pub mod game;

use piston_window::{UpdateArgs, RenderArgs, ButtonArgs, Event, Motion};

pub trait State {
    fn simulate(&mut self, args: UpdateArgs);
    fn draw(&mut self, args: RenderArgs);

    fn input_event(&mut self, args: ButtonArgs);
    fn mouse_motion(&mut self, motion: Motion);

    fn get_next_event(&mut self) -> Option<Event>;
}