use macroquad::{prelude::Color, window, shapes, color, input};
use rand::prelude::*;

const WIDTH: i32 = 1500;
const HEIGHT: i32 = 1500;

#[macroquad::main(get_window_conf())]
async fn main() {
    while !input::is_key_down(input::KeyCode::Escape) {
        window::clear_background(color::BLACK);

        window::next_frame().await
    }
}

fn get_window_conf() -> window::Conf {
    window::Conf {
        window_title: "Rusty-snake".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        high_dpi: true,
        ..window::Conf::default()
    }
}

