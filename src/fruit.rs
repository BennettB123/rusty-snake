use crate::global_state::GlobalState;

use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

pub struct Fruit {
    pub x: i32,
    pub y: i32,
    curr_color: Color,
}

impl Fruit {
    const COLORS: [Color; 6] = [YELLOW, SKYBLUE, PURPLE, ORANGE, RED, PINK];

    pub fn new() -> Self {
        Fruit {
            x: rand::gen_range(0, storage::get::<GlobalState>().num_columns - 1),
            y: rand::gen_range(0, storage::get::<GlobalState>().num_rows - 1),
            curr_color: Fruit::get_random_color(),
        }
    }

    pub fn teleport(&mut self) {
        self.x = rand::gen_range(0, storage::get::<GlobalState>().num_columns - 1);
        self.y = rand::gen_range(0, storage::get::<GlobalState>().num_rows - 1);
        self.curr_color = Fruit::get_random_color();
    }

    pub fn draw(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let draw_x: f32 =
            (screen_width / storage::get::<GlobalState>().num_columns as f32) * self.x as f32;
        let draw_y: f32 =
            (screen_height / storage::get::<GlobalState>().num_rows as f32) * self.y as f32;

        let draw_width: f32 = screen_width / storage::get::<GlobalState>().num_columns as f32;
        let draw_height: f32 = screen_height / storage::get::<GlobalState>().num_rows as f32;

        draw_rectangle(draw_x, draw_y, draw_width, draw_height, self.curr_color);
    }

    fn get_random_color() -> Color {
        Fruit::COLORS[rand::gen_range(0, Fruit::COLORS.len() - 1)]
    }
}
