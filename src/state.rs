use crate::cache::RenderCache;
use crate::entry::EntryState;
use crate::graphic::Graphic;
use macroquad::color::Color;

mod frame;
mod render;

pub struct State {
    pub curr_entries_id: String,
    pub cursor: usize,
    pub entry_state: EntryState,
    pub wide_side_bar: bool,
    pub hovered_entry: Option<usize>,
    pub show_help: bool,
    pub show_extra_content: bool,
    pub camera_pos: (f32, f32),
    pub camera_zoom: f32,
    pub popup: Option<(u32, String)>,

    // If you hold Up or Down key for a long time,
    // that's the same as pressing the key every frame.
    pub scrolling_with_arrow_keys: i32,

    /// I don't want to call `entry_top_bar_message()` and `entry_canvas()` every frame,
    /// so they're cached. They are called only if `Entry` or `EntryState` changes.
    pub cache: RenderCache,
}

impl State {
    pub fn curr_scroll_bar_colors(&mut self) -> &Vec<Color> {
        // It must be here because `self.update_cache()` is called every frame.
        self.cache.scroll_bar_colors.get(&self.curr_entries_id).unwrap()
    }

    pub fn curr_canvas(&mut self) -> Option<&Vec<Graphic>> {
        // It must be here because `self.update_cache()` is called every frame.
        self.cache.canvas.get(&(self.curr_entries_id.to_string(), self.cursor, self.entry_state))
    }
}
