use macroquad::{
    color, input,
    input::KeyCode,
    prelude::get_frame_time,
    prelude::Color,
    shapes,
    window::{self, screen_height, screen_width},
};

const SCREEN_WIDTH: i32 = 1500;
const SCREEN_HEIGHT: i32 = 1500;
const NUM_COLUMNS: i32 = 25;
const NUM_ROWS: i32 = 25;

const SNAKE_SPEED: f32 = 10.0; // moves per second

// Movement Keys
const UP_KEYS: [KeyCode; 3] = [KeyCode::Up, KeyCode::W, KeyCode::K];
const DOWN_KEYS: [KeyCode; 3] = [KeyCode::Down, KeyCode::S, KeyCode::J];
const LEFT_KEYS: [KeyCode; 3] = [KeyCode::Left, KeyCode::A, KeyCode::H];
const RIGHT_KEYS: [KeyCode; 3] = [KeyCode::Right, KeyCode::D, KeyCode::L];

#[macroquad::main(get_window_conf())]
async fn main() {
    ///// CREATE GAME STATE /////
    let mut snake = Snake::new();
    let mut snake_move_progress: f32 = 0.0;

    while !exit_requested() {
        window::clear_background(color::BLACK);

        ///// UPDATE GAME STATE /////
        // consume user input
        let last_key = input::get_last_key_pressed();
        match last_key {
            None => (),
            Some(key) => {
                if UP_KEYS.contains(&key) {
                    snake.change_direction(Direction::Up);
                } else if DOWN_KEYS.contains(&key) {
                    snake.change_direction(Direction::Down);
                } else if LEFT_KEYS.contains(&key) {
                    snake.change_direction(Direction::Left);
                } else if RIGHT_KEYS.contains(&key) {
                    snake.change_direction(Direction::Right);
                }
            }
        }

        // limit snake movement based on frame rate
        snake_move_progress = snake_move_progress + (get_frame_time() * SNAKE_SPEED);
        if snake_move_progress >= 1.0 {
            snake.update();
            snake_move_progress = 1.0 - snake_move_progress;
        }

        ///// DRAW GAME STATE /////
        snake.draw();

        window::next_frame().await;
    }
}

fn get_window_conf() -> window::Conf {
    window::Conf {
        window_title: "Rusty Snake".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: true,
        ..window::Conf::default()
    }
}

fn exit_requested() -> bool {
    input::is_key_down(input::KeyCode::Escape) || input::is_key_down(input::KeyCode::Q)
}

#[derive(PartialEq)]
enum Direction {
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

    fn change_direction(&mut self, new_dir: Direction) {
        if !new_dir.is_opposite(&self.direction) {
            self.direction = new_dir;
        }
    }

    // TODO: replace the `if`s in here with a clamp fn
    fn update(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y -= 1;
                if self.y < 0 {
                    self.y = NUM_ROWS - 1;
                }
            }
            Direction::Down => {
                self.y += 1;
                if self.y >= NUM_ROWS {
                    self.y = 0;
                }
            }
            Direction::Left => {
                self.x -= 1;
                if self.x < 0 {
                    self.x = NUM_COLUMNS - 1;
                }
            }
            Direction::Right => {
                self.x += 1;
                if self.x >= NUM_COLUMNS {
                    self.x = 0;
                }
            }
        }
    }

    fn draw(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let draw_x: f32 = (screen_width / NUM_COLUMNS as f32) * self.x as f32;
        let draw_y: f32 = (screen_height / NUM_ROWS as f32) * self.y as f32;
        let draw_width: f32 = screen_width / NUM_COLUMNS as f32;
        let draw_height: f32 = screen_height / NUM_ROWS as f32;

        shapes::draw_rectangle(draw_x, draw_y, draw_width, draw_height, Snake::COLOR);
    }
}
