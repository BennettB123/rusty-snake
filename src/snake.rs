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

#[derive(Clone, Copy)]
pub struct SnakeLink {
    pub x: i32,
    pub y: i32,
}

impl SnakeLink {
    fn new(x: i32, y: i32) -> SnakeLink {
        SnakeLink { x, y }
    }

    fn draw(&self) {
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

pub struct Snake {
    links: Vec<SnakeLink>,
    direction: Direction,
}

impl Snake {
    const COLOR: Color = GREEN;

    pub fn new() -> Self {
        Snake {
            links: vec![SnakeLink::new(0, 0)],
            direction: Direction::Right,
        }
    }

    pub fn get_head(&self) -> SnakeLink {
        self.links[0]
    }

    pub fn change_direction(&mut self, new_dir: Direction) {
        if !new_dir.is_opposite(&self.direction) {
            self.direction = new_dir;
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => {
                self.links[0].y -= 1;
                if self.links[0].y < 0 {
                    self.links[0].y = storage::get::<GlobalState>().num_rows - 1;
                }
            }
            Direction::Down => {
                self.links[0].y += 1;
                if self.links[0].y >= storage::get::<GlobalState>().num_rows {
                    self.links[0].y = 0;
                }
            }
            Direction::Left => {
                self.links[0].x -= 1;
                if self.links[0].x < 0 {
                    self.links[0].x = storage::get::<GlobalState>().num_columns - 1;
                }
            }
            Direction::Right => {
                self.links[0].x += 1;
                if self.links[0].x >= storage::get::<GlobalState>().num_columns {
                    self.links[0].x = 0;
                }
            }
        }
    }

    pub fn draw(&self) {
        for link in &self.links {
            link.draw();
        }
    }
}
