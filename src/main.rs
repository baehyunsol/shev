use macroquad::prelude::{next_frame, screen_height, screen_width};
use std::{thread, time};

pub mod config;
pub mod entry;
pub mod graphic;
pub mod input;
mod state;
mod transform;
mod u;

use input::get_input;
use transform::{fit_graphics_to_screen, fit_input_to_screen};
use u::{gui_config, init_state, window_config};

#[macroquad::main(window_config)]
async fn main() {
    let gc = gui_config();
    let mut state = init_state();

    loop {
        let (s_w, s_h) = (screen_width(), screen_height());

        let mut input = get_input();
        fit_input_to_screen(&mut input, s_w, s_h);

        let mut graphics = state.render(&gc);
        fit_graphics_to_screen(&mut graphics, s_w, s_h);
        graphic::render(&graphics);

        next_frame().await;
        thread::sleep(time::Duration::from_millis(40));
    }
}
