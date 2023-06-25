use macroquad::experimental::collections::storage;
use macroquad::prelude::*;
use std::collections::VecDeque;

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
        let cell_width: f32 = screen_width() / storage::get::<GlobalState>().num_columns as f32;
        let cell_height: f32 = screen_height() / storage::get::<GlobalState>().num_rows as f32;
        let padding_w = storage::get::<GlobalState>().draw_grid_padding * screen_width();
        let padding_h = storage::get::<GlobalState>().draw_grid_padding * screen_height();

        let draw_x: f32 = cell_width * self.x as f32 + padding_w;
        let draw_y: f32 = cell_height * self.y as f32 + padding_h;
        let draw_w: f32 = cell_width - (padding_w * 2.0);
        let draw_h: f32 = cell_height - (padding_h * 2.0);

        draw_rectangle(draw_x, draw_y, draw_w, draw_h, Snake::COLOR);
    }
}

pub struct Snake {
    links: VecDeque<SnakeLink>,
    direction: Direction,
    needs_to_grow: bool,
}

impl Snake {
    const COLOR: Color = GREEN;

    pub fn new() -> Self {
        Snake {
            links: VecDeque::from([SnakeLink::new(0, 0)]),
            direction: Direction::Right,
            needs_to_grow: false,
        }
    }

    pub fn get_head_link(&self) -> SnakeLink {
        self.links[0]
    }

    pub fn change_direction(&mut self, new_dir: Direction) {
        if !new_dir.is_opposite(&self.direction) {
            self.direction = new_dir;
        }
    }

    pub fn grow(&mut self) {
        self.needs_to_grow = true;
    }

    // moves the snake by pushing the new head location to the font of self.links
    //   and popping the tail off the back. Do not pop the tail off the back if
    //   the snake needs to grow
    pub fn update(&mut self) {
        // find new head location
        let mut new_head = SnakeLink::new(self.links[0].x, self.links[0].y);
        match self.direction {
            Direction::Up => {
                new_head.y -= 1;
                if new_head.y < 0 {
                    new_head.y = storage::get::<GlobalState>().num_rows - 1;
                }
            }
            Direction::Down => {
                new_head.y += 1;
                if new_head.y >= storage::get::<GlobalState>().num_rows {
                    new_head.y = 0;
                }
            }
            Direction::Left => {
                new_head.x -= 1;
                if new_head.x < 0 {
                    new_head.x = storage::get::<GlobalState>().num_columns - 1;
                }
            }
            Direction::Right => {
                new_head.x += 1;
                if new_head.x >= storage::get::<GlobalState>().num_columns {
                    new_head.x = 0;
                }
            }
        }

        self.links.push_front(new_head);

        // pop tail off if we aren't growing
        if !self.needs_to_grow {
            self.links.pop_back();
        }
        self.needs_to_grow = false;
    }

    pub fn did_eat_self(&self) -> bool {
        let head = self.links[0];

        for link in self.links.iter().skip(1) {
            if head.x == link.x && head.y == link.y {
                return true;
            }
        }

        false
    }

    pub fn draw(&self) {
        for link in &self.links {
            link.draw();
        }
    }
}
