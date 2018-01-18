use std::collections::VecDeque;
use std::collections::HashMap;

use piston_window::{Button, Key, ButtonState, MouseButton, Motion};

static KEY_MAP: [(&'static str, Key); 6] =
    [
        ("MOVE_UP", Key::W),
        ("MOVE_DOWN", Key::S),
        ("MOVE_LEFT", Key::A),
        ("MOVE_RIGHT", Key::D),
        ("PLUS", Key::NumPadPlus),
        ("MINUS", Key::NumPadMinus),
    ];


/// TODO handle mouse better
#[derive(Debug)]
pub struct InputController {
    events: HashMap<&'static str, ButtonState>,
    left_click: Option<ButtonState>,
    right_click: Option<ButtonState>,
    pub mouse: (i32, i32),
}

impl InputController {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            left_click: None,
            right_click: None,
            mouse: (0, 0),
        }
    }

    pub fn button_event(&mut self, button: Button, state: ButtonState) {
        match button {
            Button::Keyboard(event_key) => {
                for mapping in KEY_MAP.iter() {
                    let (name, key) = *mapping;
                    if event_key == key {
                        self.events.insert(name, state);
                    }
                }
            }
            Button::Mouse(event_mouse) => {
                match event_mouse {
                    MouseButton::Left => {
                        self.left_click = Some(state);
                    },
                    MouseButton::Right => {
                        self.right_click = Some(state);
                    },

                    _ => {

                    }
                }
            }
            _ => {},
        }
    }

    pub fn mouse_motion(&mut self, motion: Motion) {
        match motion {
            Motion::MouseCursor(x, y) => {
                self.mouse = (x as i32, y as i32);
            },
            _ => {

            }
        }
    }

    pub fn is_left_clicked(&self) -> bool {
        match self.left_click {
            Some(state) => {
                match state {
                    ButtonState::Press => {
                        return true;
                    },
                    _ => {
                        return false;
                    }
                }
            },
            _ => {
                return false;
            }
        }
    }

    pub fn is_pressed(&self, name: &str) -> bool {
        match self.events.get(name) {
            Some(state) => {
                match *state {
                    ButtonState::Press => {
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
        },
        _ => {
            return false; // TODO default hashmap to released so we can detect invalid key checks
            }
        }
    }
}
