use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

use crate::global_state::GlobalState;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_opposite(&self, other: &Direction) -> bool {
        match self {
            Direction::Up => other == &Direction::Down,
            Direction::Down => other == &Direction::Up,
            Direction::Left => other == &Direction::Right,
            Direction::Right => other == &Direction::Left,
        }
    }
}

pub struct Snake {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Snake {
    const COLOR: Color = LIME;

    pub fn new() -> Self {
        Snake {
            x: 0,
            y: 0,
            direction: Direction::Right,
        }
    }

    pub fn change_direction(&mut self, new_dir: Direction) {
        if !new_dir.is_opposite(&self.direction) {
            self.direction = new_dir;
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y -= 1;
                if self.y < 0 {
                    self.y = storage::get::<GlobalState>().num_rows - 1;
                }
            }
            Direction::Down => {
                self.y += 1;
                if self.y >= storage::get::<GlobalState>().num_rows {
                    self.y = 0;
                }
            }
            Direction::Left => {
                self.x -= 1;
                if self.x < 0 {
                    self.x = storage::get::<GlobalState>().num_columns - 1;
                }
            }
            Direction::Right => {
                self.x += 1;
                if self.x >= storage::get::<GlobalState>().num_columns {
                    self.x = 0;
                }
            }
        }
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

        draw_rectangle(draw_x, draw_y, draw_width, draw_height, Snake::COLOR);
    }
}
