use macroquad::{color, input, prelude::get_frame_time, prelude::Color, shapes, window};

const SCREEN_WIDTH: i32 = 1500;
const SCREEN_HEIGHT: i32 = 1500;
const NUM_COLUMNS: i32 = 10;
const NUM_ROWS: i32 = 10;

const SNAKE_SPEED: f32 = 5.0; // moves per second

#[macroquad::main(get_window_conf())]
async fn main() {
    ///// CREATE GAME STATE /////
    let mut snake = Snake::new();
    let mut snake_move_progress: f32 = 0.0;

    while !exit_requested() {
        window::clear_background(color::BLACK);

        ///// UPDATE GAME STATE /////

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
        high_dpi: true,
        window_resizable: true,
        ..window::Conf::default()
    }
}

fn exit_requested() -> bool {
    input::is_key_down(input::KeyCode::Escape) || input::is_key_down(input::KeyCode::Q)
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        self.direction = new_dir;
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y += 1;
                if self.y >= NUM_ROWS {
                    self.y = 0;
                }
            }
            Direction::Down => {
                self.y -= 1;
                if self.y < 0 {
                    self.y = NUM_ROWS - 1;
                }
            }
            Direction::Left => {
                self.x -= 1;
                if self.x < 0 {
                    self.y = NUM_COLUMNS - 1;
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
        let draw_x: f32 = (SCREEN_WIDTH as f32 / NUM_COLUMNS as f32) * self.x as f32;
        let draw_y: f32 = (SCREEN_HEIGHT as f32 / NUM_ROWS as f32) * self.y as f32;
        let draw_width: f32 = SCREEN_WIDTH as f32 / NUM_COLUMNS as f32;
        let draw_height: f32 = SCREEN_HEIGHT as f32 / NUM_ROWS as f32;

        shapes::draw_rectangle(draw_x, draw_y, draw_width, draw_height, Snake::COLOR);
    }
}
