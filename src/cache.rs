use crate::entry::{Entries, Entry, EntryFlag, EntryState};
use crate::graphic::Graphic;
use crate::state::State;
use macroquad::color::Color;
use macroquad::prelude::ImageFormat;
use macroquad::texture::{Texture2D, load_texture};
use std::collections::HashMap;
use std::hash::Hash;

// This is for heavy (memory & computation) values.
// It also assumes that `capacity` isn't that big (less than 100).
pub struct LRU<K, V> {
    capacity: usize,
    data: HashMap<K, V>,

    // order[0] is the least-recently-used key,
    // and order.last() is the most-recently-used key.
    // "use" means `get`, `insert` and `contains_key`.
    order: Vec<K>,
}

impl<K: Clone + Eq + Hash + PartialEq, V> LRU<K, V> {
    pub fn with_capacity(capacity: usize) -> LRU<K, V> {
        LRU {
            capacity,
            data: HashMap::new(),
            order: vec![],
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.order.sort_by_key(|k| if k == key { 1 } else { 0 });
        self.data.get(key)
    }

    pub fn contains_key(&mut self, key: &K) -> bool {
        self.order.sort_by_key(|k| if k == key { 1 } else { 0 });
        self.data.contains_key(key)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.order.sort_by_key(|k| if k == &key { 1 } else { 0 });

        if let Some(last) = self.order.last() && last == &key {
            // nop
        }

        else {
            self.order.push(key.clone());
            self.data.insert(key, value);
        }

        if self.order.len() > self.capacity {
            let least_recently_used_key = self.order.remove(0);
            self.data.remove(&least_recently_used_key);
        }
    }
}

pub struct RenderCache {
    pub canvas: LRU<(String, usize, EntryState), Vec<Graphic>>,
    pub scroll_bar_colors: LRU<String, Vec<Color>>,
}

impl RenderCache {
    pub fn new() -> RenderCache {
        RenderCache {
            canvas: LRU::with_capacity(128),
            scroll_bar_colors: LRU::with_capacity(128),
        }
    }
}

impl State {
    pub async fn update_cache(&mut self, entries: &Entries, textures: &mut TextureCache) {
        if !entries.is_empty() {
            let canvas_key = (self.curr_entries_id.clone(), self.cursor, self.entry_state);

            if !self.cache.canvas.contains_key(&canvas_key) {
                // TODO: render error message
                let mut canvas = (entries.render_canvas)(&entries[self.cursor], self.entry_state).unwrap();

                for graphic in canvas.iter_mut() {
                    match graphic {
                        Graphic::ImageFile { path, x, y, w, h } => {
                            textures.register(path).await;
                            *graphic = Graphic::Image { path: path.to_string(), x: *x, y: *y, w: *w, h: *h };
                        },
                        _ => {},
                    }
                }

                self.cache.canvas.insert(canvas_key, canvas);
            }
        }

        if !self.cache.scroll_bar_colors.contains_key(&self.curr_entries_id) {
            self.cache.scroll_bar_colors.insert(self.curr_entries_id.clone(), calc_scroll_bar_colors(&entries.entries));
        }
    }
}

pub struct TextureCache {
    data: LRU<String, Texture2D>,
}

impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache { data: LRU::with_capacity(64) }
    }

    pub async fn get(&mut self, path: &String) -> &Texture2D {
        if self.data.get(path).is_none() {
            self.register(path).await;
        }

        self.data.get(path).unwrap()
    }

    pub async fn register(&mut self, path: &str) -> String {
        match load_texture(path).await {
            Ok(data) => { self.data.insert(path.to_string(), data); },
            Err(_) => {
                if !self.data.contains_key(&String::from("?")) {
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
    let d = entries.len() as f32 / 200.0;

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
