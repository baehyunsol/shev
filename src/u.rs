use crate::config::GuiConfig;
use crate::entry::{Entry, EntryFlag};
use crate::state::State;
use macroquad::color::Color;
use macroquad::prelude::Conf as WindowConfig;

pub fn window_config() -> WindowConfig {
    WindowConfig {
        window_width: 1080,
        window_height: 720,
        window_resizable: true,
        window_title: "Gui".to_string(),
        fullscreen: false,
        ..Default::default()
    }
}

pub fn gui_config() -> GuiConfig {
    GuiConfig {
        top_bar_bg: Color {
            r: 0.2,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        },
        side_bar_bg: Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        },
        side_bar_font: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
    }
}

pub fn init_state() -> State<()> {
    State {
        entries: vec!["t1", "hi, my name is sol", "hahaha"].into_iter().map(
            |title| Entry { title: title.to_string(), data: (), category1: None, category2: None, flag: EntryFlag::Green }
        ).collect(),
        cursor: 0,
    }
}
