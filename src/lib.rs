use macroquad::prelude::{Conf as WindowConfig, next_frame, screen_height, screen_width};
use std::{thread, time};

mod action;
mod cache;
mod config;
mod entry;
mod graphic;
mod input;
mod state;
mod transform;

use action::Action;
use cache::TextureCache;
pub use macroquad::color::Color;
pub use config::Config;
pub use entry::{Entry, EntryFlag, EntryState};
pub use graphic::Graphic;
use input::get_input;
pub use state::State;
use transform::{fit_graphics_to_screen, fit_input_to_screen};

pub fn run(conf: Config, state: State) {
    let window_config = WindowConfig {
        window_width: conf.window_width,
        window_height: conf.window_height,
        window_resizable: conf.window_resizable,
        window_title: conf.window_title.clone(),
        fullscreen: conf.fullscreen,
        ..Default::default()
    };

    macroquad::Window::from_config(window_config, run_inner(conf, state));
}

async fn run_inner(conf: Config, mut state: State) {
    let mut texture_cache = TextureCache::new();

    loop {
        let (s_w, s_h) = (screen_width(), screen_height());

        let mut input = get_input();
        fit_input_to_screen(&mut input, 1080.0, 720.0, s_w, s_h);

        match state.frame(&input, &mut texture_cache).await {
            Action::None => {},
            Action::Quit => {
                break;
            },
        }

        let mut graphics = state.render(&conf);
        fit_graphics_to_screen(&mut graphics, 1080.0, 720.0, s_w, s_h);
        graphic::render(&graphics, &texture_cache);

        next_frame().await;
        thread::sleep(time::Duration::from_millis(25));
    }
}
