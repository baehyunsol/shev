use macroquad::prelude::{
    Conf as WindowConfig,
    next_frame,
    load_ttf_font_from_bytes,
    screen_height,
    screen_width,
};
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

mod action;
mod cache;
mod config;
mod entry;
mod graphic;
mod input;
mod state;
mod transform;

use action::Action;
use cache::{RenderCache, TextureCache};
pub use macroquad::color::Color;
pub use config::Config;
pub use entry::{Entries, Entry, EntryFlag, EntryState, Transition};
pub use graphic::{Graphic, TextBox};
use graphic::hide_off_screen;
use input::get_input;
use state::State;
use transform::{fit_graphics_to_screen, fit_input_to_screen};

pub fn run(
    conf: Config,
    entries_map: HashMap<String, Entries>,
    initial_entries_id: String,
) {
    let window_config = WindowConfig {
        window_width: conf.window_width,
        window_height: conf.window_height,
        window_resizable: conf.window_resizable,
        window_title: conf.window_title.clone(),
        fullscreen: conf.fullscreen,
        ..Default::default()
    };

    macroquad::Window::from_config(window_config, run_inner(conf, entries_map, initial_entries_id));
}

async fn run_inner(
    conf: Config,
    entries_map: HashMap<String, Entries>,
    initial_entries_id: String,
) {
    let empty_entries = Entries::default();
    let mut texture_cache = TextureCache::new();
    let mut entries = if entries_map.is_empty() {
        &empty_entries
    } else {
        entries_map.get(&initial_entries_id).unwrap()
    };
    let mut state = State {
        curr_entries_id: initial_entries_id.to_string(),
        cursor: 0,
        entry_state: EntryState::None,
        wide_side_bar: false,
        show_help: false,
        show_extra_content: false,
        camera_pos: (450.0, 300.0),
        camera_zoom: 1.0,
        popup: None,
        cache: RenderCache::new(initial_entries_id, entries),
    };
    let font = load_ttf_font_from_bytes(include_bytes!("../resources/SpaceMono-Regular.ttf")).unwrap();

    loop {
        state.update_cache(entries, &mut texture_cache).await;
        let frame_started_at = Instant::now();
        let (s_w, s_h) = (screen_width(), screen_height());

        let mut graphics = state.render(entries, &conf);
        hide_off_screen(&mut graphics, 1080.0, 720.0);
        fit_graphics_to_screen(&mut graphics, 1080.0, 720.0, s_w, s_h);
        graphic::render(&graphics, &font, &mut texture_cache, (s_w, s_h));

        let mut input = get_input();
        fit_input_to_screen(&mut input, 1080.0, 720.0, s_w, s_h);

        match state.frame(&entries, input).await {
            Action::None => {},
            Action::Transit(id) => {
                entries = entries_map.get(&id).unwrap();
                state.curr_entries_id = id.to_string();
            },
            Action::Quit => {
                break;
            },
        }

        next_frame().await;
        let elapsed_time = Instant::now().duration_since(frame_started_at).as_millis() as u64;

        if elapsed_time < 25 {
            thread::sleep(Duration::from_millis(25 - elapsed_time));
        }
    }
}
