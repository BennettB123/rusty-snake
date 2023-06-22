use macroquad::{prelude::Color, window, shapes, color, input};
use rand::prelude::*;

const WIDTH: i32 = 1500;
const HEIGHT: i32 = 1500;

#[macroquad::main(get_window_conf())]
async fn main() {
    let mut my_snake = Snake::new();

    while !input::is_key_down(input::KeyCode::Escape) {
        window::clear_background(color::BLACK);

        // update game state
        my_snake.update();

        // draw game state
        my_snake.draw();

        window::next_frame().await
    }
}

fn get_window_conf() -> window::Conf {
    window::Conf {
        window_title: "Rusty Snake".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        high_dpi: true,
        window_resizable: true,
        ..window::Conf::default()
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Snake {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Snake {
    const COLOR: Color = color::LIME;

    fn new() -> Self {
        Snake {
            x: 0,
            y: 0,
            direction: Direction::Right,
        }
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn draw(&self) {
        shapes::draw_rectangle(self.x as f32, self.y as f32, 50 as f32, 50 as f32, Snake::COLOR);
    }
}

