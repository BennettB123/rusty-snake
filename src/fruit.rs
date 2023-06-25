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
        let cell_width: f32 = screen_width() / storage::get::<GlobalState>().num_columns as f32;
        let cell_height: f32 = screen_height() / storage::get::<GlobalState>().num_rows as f32;
        let padding_w = storage::get::<GlobalState>().draw_grid_padding * screen_width();
        let padding_h = storage::get::<GlobalState>().draw_grid_padding * screen_height();

        let draw_x: f32 = cell_width * self.x as f32 + padding_w;
        let draw_y: f32 = cell_height * self.y as f32 + padding_h;
        let draw_w: f32 = cell_width - (padding_w * 2.0);
        let draw_h: f32 = cell_height - (padding_h * 2.0);

        draw_rectangle(draw_x, draw_y, draw_w, draw_h, self.curr_color);
    }

    fn get_random_color() -> Color {
        Fruit::COLORS[rand::gen_range(0, Fruit::COLORS.len() - 1)]
    }
}
