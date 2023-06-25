use macroquad::input::KeyCode;
use std::collections::VecDeque;

// TODO: use max_size. Do we want to remove oldest, or newest???
pub struct InputBuffer {
    inputs: VecDeque<KeyCode>,
    max_size: usize,
}

impl InputBuffer {
    pub fn new(max_size: usize) -> Self {
        InputBuffer {
            inputs: VecDeque::new(),
            max_size,
        }
    }

    pub fn add_key(&mut self, key: KeyCode) {
        self.inputs.push_front(key);
    }

    pub fn get_key(&mut self) -> Option<KeyCode> {
        self.inputs.pop_back()
    }
}
