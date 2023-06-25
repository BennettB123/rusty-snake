use macroquad::input::KeyCode;
use std::collections::VecDeque;

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
        if self.inputs.len() >= self.max_size {
            self.inputs.pop_back();
        }
        self.inputs.push_front(key);
    }

    pub fn get_key(&mut self) -> Option<KeyCode> {
        self.inputs.pop_back()
    }
}
