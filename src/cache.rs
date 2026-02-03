use crate::entry::{Entries, Entry, EntryFlag, EntryState};
use crate::graphic::Graphic;
use crate::state::State;
use macroquad::color::Color;
use macroquad::prelude::ImageFormat;
use macroquad::texture::{Texture2D, load_texture};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) struct RenderCache {
    // If `cursor` or `entry_state` changes, `canvas` has to be re-drawn.
    // If `entries_id` changes, `scroll_bar_colors` has to be re-calculated.
    pub cursor: usize,
    pub entry_state: EntryState,
    pub canvas: Vec<Graphic>,
    pub entries_id: String,
    pub scroll_bar_colors: Vec<Color>,
}

impl RenderCache {
    pub fn new(entries_id: String, entries: &Entries) -> RenderCache {
        RenderCache {
            // let's make sure that the initial frame will update the cache
            cursor: usize::MAX,
            entry_state: EntryState::Red,
            canvas: vec![],
            entries_id: entries_id.to_string(),
            scroll_bar_colors: calc_scroll_bar_colors(&entries.entries),
        }
    }
}

impl State {
    pub async fn update_cache(&mut self, entries: &Entries, textures: &mut TextureCache) {
        if (self.cursor, self.entry_state) != (self.cache.cursor, self.cache.entry_state) {
            self.cache = RenderCache {
                cursor: self.cursor,
                entry_state: self.entry_state,
                canvas: if entries.is_empty() {
                    vec![]
                } else {
                    // TODO: render error message
                    (entries.render_canvas)(&entries[self.cursor], self.entry_state).unwrap()
                },
                ..self.cache.clone()
            };

            for graphic in self.cache.canvas.iter_mut() {
                match graphic {
                    Graphic::ImageFile { path, x, y, w, h } => {
                        let texture_id = textures.register(path).await;
                        *graphic = Graphic::Image { texture_id, x: *x, y: *y, w: *w, h: *h };
                    },
                    _ => {},
                }
            }
        }

        if self.curr_entries_id != self.cache.entries_id {
            self.cache = RenderCache {
                entries_id: self.curr_entries_id.clone(),
                scroll_bar_colors: calc_scroll_bar_colors(&entries.entries),
                ..self.cache.clone()
            };
        }
    }
}

pub struct TextureCache {
    data: HashMap<String, Texture2D>,
}

impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache { data: HashMap::new() }
    }

    pub fn get(&self, id: &str) -> Option<&Texture2D> {
        self.data.get(id)
    }

    pub async fn register(&mut self, path: &str) -> String {
        match load_texture(path).await {
            Ok(data) => { self.data.insert(path.to_string(), data); },
            Err(_) => {
                if !self.data.contains_key("?") {
                    let x = Texture2D::from_file_with_format(include_bytes!("../resources/x.png"), Some(ImageFormat::Png));
                    self.data.insert(String::from("?"), x);
                }

                return String::from("?");
            },
        }

        path.to_string()
    }
}

fn calc_scroll_bar_colors(entries: &[Entry]) -> Vec<Color> {
    let mut colors = vec![Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 }; 128];
    let d = entries.len() as f32 / 300.0;

    for (i, entry) in entries.iter().enumerate() {
        let j = i * 128 / entries.len();

        match entry.flag {
            EntryFlag::Red => {
                if j > 1 { colors[j - 2].r += d * 0.25; }
                if j > 0 { colors[j - 1].r += d * 0.5; }
                colors[j].r += d;
                if j < 127 { colors[j + 1].r += d * 0.5; }
                if j < 126 { colors[j + 2].r += d * 0.25; }
            },
            EntryFlag::Green => {
                if j > 1 { colors[j - 2].g += d * 0.25; }
                if j > 0 { colors[j - 1].g += d * 0.5; }
                colors[j].g += d;
                if j < 127 { colors[j + 1].g += d * 0.5; }
                if j < 126 { colors[j + 2].g += d * 0.25; }
            },
            EntryFlag::Blue => {
                if j > 1 { colors[j - 2].b += d * 0.25; }
                if j > 0 { colors[j - 1].b += d * 0.5; }
                colors[j].b += d;
                if j < 127 { colors[j + 1].b += d * 0.5; }
                if j < 126 { colors[j + 2].b += d * 0.25; }
            },
            EntryFlag::None => {},
        }
    }

    colors
}
