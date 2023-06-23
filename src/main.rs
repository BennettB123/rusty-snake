use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

mod global_state;
use global_state::GlobalState;

mod snake;
use snake::{Direction, Snake};

const GLOBAL_STATE: GlobalState = GlobalState {
    game_name: "Rusty Snake",
    screen_width: 1500,
    screen_height: 1500,
    num_columns: 25,
    num_rows: 25,
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

    ///// CREATE GAME STATE /////
    let mut snake = Snake::new();
    let mut snake_move_progress: f32 = 0.0;

    while !exit_requested() {
        clear_background(BLACK);

        ///// UPDATE GAME STATE /////
        // consume user input
        let last_key = get_last_key_pressed();
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
