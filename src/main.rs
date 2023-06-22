use macroquad::{
    color, input, prelude::get_frame_time, prelude::Color, shapes, telemetry::frame, window,
};

const SCREEN_WIDTH: i32 = 750;
const SCREEN_HEIGHT: i32 = 750;
const NUM_COLUMNS: i32 = 25;
const NUM_ROWS: i32 = 25;

const MAX_FPS: f32 = 30.0;
const SNAKE_SPEED: f32 = 1.0; // number of snake moves per second

#[macroquad::main(get_window_conf())]
async fn main() {
    let mut my_snake = Snake::new();
    let mut frame_count: u128 = 0;

    while !input::is_key_down(input::KeyCode::Escape) {
        window::clear_background(color::BLACK);

        // update game state
        if frame_count % (MAX_FPS / SNAKE_SPEED) as u128 == 0 {
            my_snake.update();
        }

        // draw game state
        my_snake.draw();

        window::next_frame().await;
        frame_count += 1;

        limit_frame_rate();
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

// TODO: This limiter is rough. It results in some frames being very short and others being very long.
//       Smooth this out!
fn limit_frame_rate() {
    let minimum_frame_time = 1. / MAX_FPS;
    let frame_time = get_frame_time();
    println!();
    println!("frame_time: {}", frame_time);
    println!("FPS: {}", macroquad::time::get_fps());

    if frame_time < minimum_frame_time {
        let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
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
