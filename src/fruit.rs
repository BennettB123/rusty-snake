use ::rand::Rng;

use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

use crate::cell::Cell;
use crate::global_state::GlobalState;

pub struct Fruit {
    pub location: Cell,
    curr_color: Color,
}

impl Fruit {
    const COLORS: [Color; 6] = [YELLOW, SKYBLUE, PURPLE, ORANGE, RED, PINK];

    pub fn new() -> Self {
        Fruit {
            location: Cell {
                x: ::rand::thread_rng().gen_range(0..storage::get::<GlobalState>().num_columns - 1),
                y: ::rand::thread_rng().gen_range(0..storage::get::<GlobalState>().num_rows - 1),
            },
            curr_color: Fruit::get_random_color(),
        }
    }

    pub fn teleport(&mut self, illegal_cells: Vec<Cell>) {
        let num_cols = storage::get::<GlobalState>().num_columns;
        let num_rows = storage::get::<GlobalState>().num_rows;

        let mut possible_cells: Vec<Cell> = vec![];
        for x in 0..num_cols {
            for y in 0..num_rows {
                possible_cells.push(Cell::new(x, y));
            }
        }

        for bad_spot in illegal_cells {
            possible_cells.retain(|c| c != &bad_spot);
        }

        let idx = ::rand::thread_rng().gen_range(0..possible_cells.len() - 1);
        self.location = *possible_cells.get(idx).unwrap();
        self.curr_color = Fruit::get_random_color();
    }

    pub fn draw(&self) {
        self.location.draw(self.curr_color);
    }

    fn get_random_color() -> Color {
        Fruit::COLORS[::rand::thread_rng().gen_range(0..Fruit::COLORS.len() - 1)]
    }
}
