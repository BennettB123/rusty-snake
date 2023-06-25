use crate::cell::Cell;
use crate::global_state::GlobalState;

use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

pub struct Fruit {
    pub location: Cell,
    curr_color: Color,
}

impl Fruit {
    const COLORS: [Color; 6] = [YELLOW, SKYBLUE, PURPLE, ORANGE, RED, PINK];

    pub fn new() -> Self {
        Fruit {
            location: Cell {
                x: rand::gen_range(0, storage::get::<GlobalState>().num_columns - 1),
                y: rand::gen_range(0, storage::get::<GlobalState>().num_rows - 1),
            },
            curr_color: Fruit::get_random_color(),
        }
    }

    pub fn teleport(&mut self) {
        self.location.x = rand::gen_range(0, storage::get::<GlobalState>().num_columns - 1);
        self.location.y = rand::gen_range(0, storage::get::<GlobalState>().num_rows - 1);
        self.curr_color = Fruit::get_random_color();
    }

    pub fn draw(&self) {
        self.location.draw(self.curr_color);
    }

    fn get_random_color() -> Color {
        Fruit::COLORS[rand::gen_range(0, Fruit::COLORS.len() - 1)]
    }
}
