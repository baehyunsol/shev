use macroquad::color::Color;
use macroquad::input::KeyCode;
use crate::action::Action;
use crate::cache::{RenderCache, TextureCache};
use crate::config::Config;
use crate::entry::{Entry, EntryFlag, EntryState};
use crate::graphic::Graphic;
use crate::input::Input;
use crate::transform;

pub struct State {
    pub title: Option<String>,
    pub entries: Vec<Entry>,
    pub entry_top_bar_message: fn(&Entry, EntryState) -> Option<String>,
    pub entry_canvas: fn(&Entry, EntryState) -> Result<Vec<Graphic>, String>,
    pub(crate) entry_state: EntryState,
    pub(crate) cursor: usize,
    pub(crate) wide_side_bar: bool,
    pub(crate) show_help: bool,
    pub(crate) canvas_offset: (f32, f32),

    /// It's calculated when `entries` are first loaded and cached forever.
    /// -> It's immutable because `entries` is immutable.
    pub(crate) scroll_bar_colors: Vec<Color>,

    /// I don't want to call `entry_top_bar_message()` and `entry_canvas()` every frame,
    /// so they're cached. They are called only if `Entry` or `EntryState` changes.
    pub(crate) cache: RenderCache,
}

impl State {
    pub fn new(
        title: Option<String>,
        entries: Vec<Entry>,
        entry_top_bar_message: fn(&Entry, EntryState) -> Option<String>,
        entry_canvas: fn(&Entry, EntryState) -> Result<Vec<Graphic>, String>,
    ) -> State {
        let scroll_bar_colors = calc_scroll_bar_colors(&entries);
        State {
            title,
            entries,
            entry_state: EntryState::None,
            entry_top_bar_message,
            entry_canvas,
            scroll_bar_colors,
            ..State::default()
        }
    }

    pub(crate) async fn frame(&mut self, input: &Input, textures: &mut TextureCache) -> Action {
        if input.released_keys.contains(&KeyCode::Escape) {
            if self.show_help {
                self.show_help = false;
                return Action::None;
            }

            return Action::Quit;
        }

        if !self.entries.is_empty() {
            if input.released_keys.contains(&KeyCode::Down) {
                self.reset_entry_state();

                if input.down_keys.contains(&KeyCode::LeftControl) || input.down_keys.contains(&KeyCode::RightControl) {
                    // Ctrl+Shift+Down: jump to next category 2
                    if input.down_keys.contains(&KeyCode::LeftShift) || input.down_keys.contains(&KeyCode::RightShift) {
                        let curr_cat = &self.entries[self.cursor].category2;

                        for _ in 0..self.entries.len() {
                            self.cursor = (self.cursor + 1) % self.entries.len();

                            if &self.entries[self.cursor].category2 != curr_cat {
                                break;
                            }
                        }
                    }

                    // Ctrl+Down: jump to next category 1
                    else {
                        let curr_cat = &self.entries[self.cursor].category1;

                        for _ in 0..self.entries.len() {
                            self.cursor = (self.cursor + 1) % self.entries.len();

                            if &self.entries[self.cursor].category1 != curr_cat {
                                break;
                            }
                        }
                    }
                }

                // Down: jump to next entry
                else {
                    self.cursor = (self.cursor + 1) % self.entries.len();
                }
            }

            else if input.released_keys.contains(&KeyCode::Up) {
                self.reset_entry_state();

                if input.down_keys.contains(&KeyCode::LeftControl) || input.down_keys.contains(&KeyCode::RightControl) {
                    // Ctrl+Shift+Up: jump to prev category 2
                    if input.down_keys.contains(&KeyCode::LeftShift) || input.down_keys.contains(&KeyCode::RightShift) {
                        let curr_cat = &self.entries[self.cursor].category2;

                        for _ in 0..self.entries.len() {
                            self.cursor = (self.cursor + self.entries.len() - 1) % self.entries.len();

                            if &self.entries[self.cursor].category2 != curr_cat {
                                break;
                            }
                        }
                    }

                    // Ctrl+Down: jump to prev category 1
                    else {
                        let curr_cat = &self.entries[self.cursor].category1;

                        for _ in 0..self.entries.len() {
                            self.cursor = (self.cursor + self.entries.len() - 1) % self.entries.len();

                            if &self.entries[self.cursor].category1 != curr_cat {
                                break;
                            }
                        }
                    }
                }

                // Down: jump to prev entry
                else {
                    self.cursor = (self.cursor + self.entries.len() - 1) % self.entries.len();
                }
            }

            else if input.released_keys.contains(&KeyCode::Space) {
                self.reset_entry_state();

                // Alt+Space: jump to prev entry with the same flag
                if input.down_keys.contains(&KeyCode::LeftAlt) || input.down_keys.contains(&KeyCode::RightAlt) {
                    let curr_flag = &self.entries[self.cursor].flag;

                    for _ in 0..self.entries.len() {
                        self.cursor = (self.cursor + self.entries.len() - 1) % self.entries.len();

                        if &self.entries[self.cursor].flag == curr_flag {
                            break;
                        }
                    }
                }

                // Space: jump to next entry with the same flag
                else {
                    let curr_flag = &self.entries[self.cursor].flag;

                    for _ in 0..self.entries.len() {
                        self.cursor = (self.cursor + 1) % self.entries.len();

                        if &self.entries[self.cursor].flag == curr_flag {
                            break;
                        }
                    }
                }
            }

            let n = if input.released_keys.contains(&KeyCode::Key1) {
                Some(0)
            } else if input.released_keys.contains(&KeyCode::Key2) {
                Some(1)
            } else if input.released_keys.contains(&KeyCode::Key3) {
                Some(2)
            } else if input.released_keys.contains(&KeyCode::Key4) {
                Some(3)
            } else if input.released_keys.contains(&KeyCode::Key5) {
                Some(4)
            } else if input.released_keys.contains(&KeyCode::Key6) {
                Some(5)
            } else if input.released_keys.contains(&KeyCode::Key7) {
                Some(6)
            } else if input.released_keys.contains(&KeyCode::Key8) {
                Some(7)
            } else if input.released_keys.contains(&KeyCode::Key9) {
                Some(8)
            } else {
                None
            };
    
            if let Some(n) = n {
                self.cursor = n * (self.entries.len() - 1) / 8;
            }
        }

        if input.released_keys.contains(&KeyCode::Left) {
            self.wide_side_bar = true;
        }

        if input.released_keys.contains(&KeyCode::Right) {
            self.wide_side_bar = false;
        }

        if input.released_keys.contains(&KeyCode::H) {
            self.show_help = !self.show_help;
        }

        let canvas_move_speed = if input.down_keys.contains(&KeyCode::LeftShift) || input.down_keys.contains(&KeyCode::RightShift) {
            40.0
        } else {
            10.0
        };

        if input.down_keys.contains(&KeyCode::W) {
            self.canvas_offset.1 -= canvas_move_speed;
        }

        if input.down_keys.contains(&KeyCode::A) {
            self.canvas_offset.0 -= canvas_move_speed;
        }

        if input.down_keys.contains(&KeyCode::S) {
            self.canvas_offset.1 += canvas_move_speed;
        }

        if input.down_keys.contains(&KeyCode::D) {
            self.canvas_offset.0 += canvas_move_speed;
        }

        self.update_cache(textures).await;
        Action::None
    }

    /// It thinks that the screen is always 1080x720.
    /// There's another function out there that fits the graphics
    /// to the actual screen size.
    pub(crate) fn render(&self, config: &Config) -> Vec<Graphic> {
        let mut graphics = vec![];

        self.render_canvas(config, &mut graphics);
        self.render_top_bar(config, &mut graphics);
        self.render_side_bar(config, &mut graphics);

        if self.show_help {
            self.render_help(&mut graphics);
        }

        graphics
    }

    fn render_top_bar(&self, config: &Config, graphics: &mut Vec<Graphic>) {
        // bg
        graphics.push(Graphic::Rect {
            x: 0.0,
            y: 0.0,
            w: if self.wide_side_bar { 600.0 } else { 900.0 },
            h: 120.0,
            radius: None,
            thickness: None,
            color: config.top_bar_bg,
        });

        let mut lines = vec![];

        if let Some(title) = &self.title {
            lines.push(title.to_string());
        }

        if !self.entries.is_empty() {
            if let Some(top_bar_title) = &self.entries[self.cursor].top_bar_title {
                lines.push(top_bar_title.to_string());
            }
        }

        if let Some(entry_top_bar_message) = &self.cache.entry_top_bar_message {
            lines.push(entry_top_bar_message.to_string());
        }

        let mut curr_y = 30.0;
        let line_max_len = if self.wide_side_bar { 54 } else { 96 };
        let center = if self.wide_side_bar { 300.0 } else { 450.0 };

        for line in lines.iter() {
            let truncated_line = if line.chars().count() > (line_max_len + 4) {
                format!("{}...", line.chars().take(line_max_len).collect::<String>())
            } else {
                line.to_string()
            };
            let mut curr_x = center - truncated_line.chars().count() as f32 * 4.0;

            for ch in truncated_line.chars() {
                if ch != ' ' {
                    graphics.push(Graphic::Char {
                        ch,
                        x: curr_x,
                        y: curr_y,
                        size: 21,
                        color: config.top_bar_font,
                    });
                }

                curr_x += 8.0;
            }

            curr_y += 20.0;
        }

        let mut curr_x = if self.wide_side_bar { 540.0 } else { 840.0 };

        for ch in "Help: h".chars() {
            graphics.push(Graphic::Char {
                ch,
                x: curr_x,
                y: 115.0,
                size: 16,
                color: config.top_bar_font,
            });
            curr_x += 7.0;
        }
    }

    fn render_side_bar(&self, config: &Config, graphics: &mut Vec<Graphic>) {
        let (x, w) = if self.wide_side_bar { (600.0, 480.0) } else { (900.0, 180.0) };
        let title_max_len = if self.wide_side_bar { 48 } else { 13 };

        // bg
        graphics.push(Graphic::Rect {
            x,
            y: 0.0,
            w,
            h: 720.0,
            radius: None,
            thickness: None,
            color: config.side_bar_bg,
        });

        let mut list_start = self.cursor.max(16) - 16;
        let list_end = (list_start + 33).min(self.entries.len());

        if list_end < list_start + 33 {
            list_start = list_end.max(33) - 33;
        }

        let mut curr_y = 20.0;

        for i in list_start..list_end {
            let truncated_title = if self.entries[i].side_bar_title.chars().count() > (title_max_len + 4) {
                format!("{}...", self.entries[i].side_bar_title.chars().take(title_max_len).collect::<String>())
            } else {
                self.entries[i].side_bar_title.to_string()
            };
            let title = format!(
                "{} {}. {}",
                if i == self.cursor { ">>" } else { "  " },
                i + 1,
                truncated_title,
            );
            let mut curr_x = x + 6.4;

            for ch in title.chars() {
                if ch != ' ' {
                    graphics.push(Graphic::Char {
                        ch,
                        x: curr_x,
                        y: curr_y,
                        size: 16,
                        color: config.side_bar_font,
                    });
                }

                curr_x += 6.4;
            }

            match self.entries[i].flag {
                EntryFlag::None => {},
                _ => {
                    let color = match self.entries[i].flag {
                        EntryFlag::Red => Color { r: 0.75, g: 0.25, b: 0.25, a: 1.0 },
                        EntryFlag::Green => Color { r: 0.25, g: 0.75, b: 0.25, a: 1.0 },
                        EntryFlag::Blue => Color { r: 0.25, g: 0.25, b: 0.75, a: 1.0 },
                        EntryFlag::None => unreachable!(),
                    };

                    graphics.push(Graphic::Ellipse {
                        x: curr_x + 8.0,
                        y: curr_y - 5.0,
                        rx: 5.0,
                        ry: 5.0,
                        color,
                        thickness: None,
                    });
                },
            }

            curr_y += 20.0;
        }

        if self.wide_side_bar && self.entries.len() > 33 {
            graphics.push(Graphic::Rect {
                x: 1050.0,
                y: 20.0,
                w: 10.0,
                h: 640.0,
                radius: None,
                thickness: None,
                color: Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0,
                },
            });

            for (i, color) in self.scroll_bar_colors.iter().enumerate() {
                graphics.push(Graphic::Rect {
                    x: 1055.0,
                    y: 20.0 + (i * 5) as f32,
                    w: 4.0,
                    h: 5.0,
                    radius: None,
                    thickness: None,
                    color: *color,
                });
            }

            graphics.push(Graphic::Ellipse {
                x: 1055.0,
                y: 20.0 + (self.cursor * 640 / self.entries.len()) as f32,
                rx: 8.0,
                ry: 8.0,
                thickness: None,
                color: config.side_bar_font,
            });
        }

        let counter = if self.entries.is_empty() { String::from("0 / 0") } else { format!("{} / {}", self.cursor + 1, self.entries.len()) };
        let mut curr_x = 1065.0 - 6.4 * counter.chars().count() as f32;

        for ch in counter.chars() {
            graphics.push(Graphic::Char {
                ch,
                x: curr_x,
                y: 680.0,
                size: 16,
                color: config.side_bar_font,
            });
            curr_x += 6.4;
        }
    }

    fn render_canvas(&self, config: &Config, graphics: &mut Vec<Graphic>) {
        // The canvas has 900x600 resolution.
        let mut canvas = self.cache.canvas.clone();
        transform::move_rel(&mut canvas, 0.0, 120.0);
        transform::move_rel(&mut canvas, -self.canvas_offset.0, -self.canvas_offset.1);
        graphics.extend(canvas);
    }

    fn render_help(&self, graphics: &mut Vec<Graphic>) {
        graphics.push(Graphic::Rect {
            x: 30.0,
            y: 30.0,
            w: 1020.0,
            h: 660.0,
            radius: Some(12.0),
            thickness: None,
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        });
        graphics.push(Graphic::Rect {
            x: 40.0,
            y: 40.0,
            w: 1000.0,
            h: 640.0,
            radius: Some(12.0),
            thickness: None,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        });
        let mut curr_y = 72.0;

        for line in [
            "Esc: Quit",
            "Left/Right: Toggle side-bar",
            "Up/Down: Jump to prev/next entry",

            // TODO: skip these lines if there's no category-1 or category-2
            "Ctrl + Up/Down: Jump to prev/next category-1",
            "Ctrl + Shift + Up/Down: Jump to prev/next category-2",

            "Space: Jump to next entry with the same flag",
            "Alt + Space: Jump to prev entry with the same flag",
            "0~9: Quick jump",
            "W/A/S/D: Move canvas",
            "Shift + W/A/S/D: Move canvas faster",
        ] {
            let mut curr_x = 72.0;

            for ch in line.chars() {
                if ch != ' ' {
                    graphics.push(Graphic::Char {
                        ch,
                        x: curr_x,
                        y: curr_y,
                        size: 18,
                        color: Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        },
                    });
                }

                curr_x += 8.0;
            }

            curr_y += 20.0;
        }
    }

    fn reset_entry_state(&mut self) {
        self.entry_state = EntryState::None;
        self.canvas_offset = (0.0, 0.0);
    }
}

impl Default for State {
    fn default() -> State {
        State {
            title: None,
            entries: vec![],
            entry_state: EntryState::None,
            entry_top_bar_message: |_, _| None,
            entry_canvas: |_, _| Ok(vec![]),
            cursor: 0,
            wide_side_bar: false,
            show_help: false,
            canvas_offset: (0.0, 0.0),
            scroll_bar_colors: calc_scroll_bar_colors(&[]),
            cache: RenderCache::new(),
        }
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
