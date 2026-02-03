use crate::cache::RenderCache;
use crate::entry::EntryState;

mod frame;
mod render;

pub struct State {
    pub(crate) curr_entries_id: String,
    pub(crate) cursor: usize,
    pub(crate) entry_state: EntryState,
    pub(crate) wide_side_bar: bool,
    pub(crate) show_help: bool,
    pub(crate) show_extra_content: bool,
    pub(crate) camera_pos: (f32, f32),
    pub(crate) camera_zoom: f32,
    pub(crate) popup: Option<(u32, String)>,

    /// I don't want to call `entry_top_bar_message()` and `entry_canvas()` every frame,
    /// so they're cached. They are called only if `Entry` or `EntryState` changes.
    pub(crate) cache: RenderCache,
}
