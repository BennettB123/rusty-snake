use std::process::exit;

use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

mod cell;
mod fruit;
mod global_state;
mod input_buffer;
mod snake;

use fruit::Fruit;
use global_state::GlobalState;
use input_buffer::InputBuffer;
use snake::{Direction, Snake};

const GLOBAL_STATE: GlobalState = GlobalState {
    game_name: "Rusty Snake",
    screen_width: 1500,
    screen_height: 1500,
    num_columns: 15,
    num_rows: 15,
    draw_grid_padding: 0.005,
};

const SNAKE_SPEED: f32 = 10.0; // moves per second

// Movement Keys
const UP_KEYS: [KeyCode; 3] = [KeyCode::Up, KeyCode::W, KeyCode::K];
const DOWN_KEYS: [KeyCode; 3] = [KeyCode::Down, KeyCode::S, KeyCode::J];
const LEFT_KEYS: [KeyCode; 3] = [KeyCode::Left, KeyCode::A, KeyCode::H];
const RIGHT_KEYS: [KeyCode; 3] = [KeyCode::Right, KeyCode::D, KeyCode::L];

#[macroquad::main(get_window_conf())]
async fn main() {
    // store this in macroquads global storage so it can be used across modules
    storage::store(GLOBAL_STATE);

    let mut input_buffer = InputBuffer::new(5);
    let mut snake = Snake::new();
    let mut snake_move_progress: f32 = 0.0; // keep's snake's movement constant regardless of FPS.
                                            // snake moves when this gets to 1.0

    let mut fruit = Fruit::new();

    while !exit_requested() {
        clear_background(BLACK);

        // get last_key_pressed
        match get_last_key_pressed() {
            None => (),
            Some(key) => input_buffer.add_key(key),
        }

        // limit snake movement based on frame rate
        snake_move_progress = snake_move_progress + (get_frame_time() * SNAKE_SPEED);
        if snake_move_progress >= 1.0 {
            // consume user input
            match input_buffer.get_key() {
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

            snake.update();
            snake_move_progress = 1.0 - snake_move_progress;

            // check for collisions
            if snake.did_eat_self() {
                exit(0); // TODO: losing screen + restart
            }

            if did_snake_eat_fruit(&snake, &fruit) {
                snake.grow();
                fruit.teleport();
            }
        }

        // draw all elements
        draw_grid_lines();
        fruit.draw();
        snake.draw();

        next_frame().await;
    }
}

fn get_window_conf() -> Conf {
    Conf {
        window_title: GLOBAL_STATE.game_name.to_owned(),
        window_width: GLOBAL_STATE.screen_width,
        window_height: GLOBAL_STATE.screen_height,
        window_resizable: true,
        ..Conf::default()
    }
}

fn exit_requested() -> bool {
    is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q)
}

fn did_snake_eat_fruit(snake: &Snake, fruit: &Fruit) -> bool {
    let snake_head = snake.get_head_location();
    snake_head.x == fruit.location.x && snake_head.y == fruit.location.y
}

fn draw_grid_lines() {}
