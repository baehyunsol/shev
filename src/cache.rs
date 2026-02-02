use crate::entry::EntryState;
use crate::graphic::Graphic;
use crate::state::State;
use macroquad::texture::{Texture2D, load_texture};
use std::collections::HashMap;

pub struct RenderCache {
    // keys
    pub cursor: usize,
    pub entry_state: EntryState,

    // values
    pub entry_top_bar_message: Option<String>,
    pub canvas: Vec<Graphic>,
}

impl RenderCache {
    pub fn new() -> RenderCache {
        RenderCache {
            // let's make sure that the initial frame will update the cache
            cursor: usize::MAX,
            entry_state: EntryState::Red,
            entry_top_bar_message: None,
            canvas: vec![],
        }
    }
}

impl State {
    pub async fn update_cache(&mut self, textures: &mut TextureCache) {
        if (self.cursor, self.entry_state) == (self.cache.cursor, self.cache.entry_state) {
            return;
        }

        self.cache = RenderCache {
            cursor: self.cursor,
            entry_state: self.entry_state,
            entry_top_bar_message: if self.entries.is_empty() {
                None
            } else {
                (self.entry_top_bar_message)(&self.entries[self.cursor], self.entry_state)
            },
            canvas: if self.entries.is_empty() {
                vec![]
            } else {
                // TODO: render error message
                (self.entry_canvas)(&self.entries[self.cursor], self.entry_state).unwrap()
            },
        };

        for graphic in self.cache.canvas.iter_mut() {
            match graphic {
                Graphic::ImageFile { path, x, y, w, h } => {
                    let texture_id = textures.register(path).await.unwrap();
                    *graphic = Graphic::Image { texture_id, x: *x, y: *y, w: *w, h: *h };
                },
                _ => {},
            }
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

    pub async fn register(&mut self, path: &str) -> Result<String, String> {
        self.data.insert(
            path.to_string(),
            load_texture(path).await.map_err(|e| format!("{e:?}"))?,
        );
        Ok(path.to_string())
    }
}
