use super::State;
use crate::config::Config;
use crate::entry::{Entries, EntryFlag};
use crate::graphic::{Graphic, TextBox};
use crate::input::Input;
use crate::transform::{check_contain, move_rel, scale};
use macroquad::color::Color;

impl State {
    /// It thinks that the screen is always 1080x720.
    /// There's another function out there that fits the graphics
    /// to the actual screen size.
    pub fn render(&mut self, input: &Input, entries: &Entries, config: &Config) -> Vec<Graphic> {
        let mut graphics = vec![];

        self.render_canvas(&mut graphics);
        self.render_top_bar(config, entries, &mut graphics);
        self.render_side_bar(input, config, entries, &mut graphics);

        if self.show_help {
            self.render_help(entries, &mut graphics);
        }

        if let Some(_) = &self.popup {
            self.render_popup(&mut graphics);
        }

        graphics
    }

    fn render_top_bar(&self, config: &Config, entries: &Entries, graphics: &mut Vec<Graphic>) {
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

        // Vec<(line_content, entry_flag, small_font)>
        let mut lines: Vec<(String, EntryFlag, bool)> = vec![];

        if let Some(title) = &entries.title {
            lines.push((title.to_string(), EntryFlag::None, false));
        }

        if !entries.is_empty() {
            let entry = &entries[self.cursor];
            lines.push((entry.name.to_string(), entry.flag, false));
        }

        lines.push((format!(
            "{}{}{}H: Help",
            if let Some(t) = &entries.transition { format!("Ctrl+Up: {}, ", t.description.as_ref().unwrap_or(&t.id)) } else { String::new() },
            if let Some(Some(t)) = &entries.get(self.cursor).map(|e| &e.transition1) { format!("Ctrl+Left: {}, ", t.description.as_ref().unwrap_or(&t.id)) } else { String::new() },
            if let Some(Some(t)) = &entries.get(self.cursor).map(|e| &e.transition2) { format!("Ctrl+Right: {}, ", t.description.as_ref().unwrap_or(&t.id)) } else { String::new() },
        ), EntryFlag::None, true));
        let (font_size, mut curr_y, line_height) = match lines.len() {
            0 | 1 => (21.0, 60.0, 0.0),
            2 => (21.0, 45.0, 40.0),
            3 => (21.0, 30.0, 33.0),
            4 => (21.0, 24.0, 28.0),
            5 => (16.0, 24.0, 21.0),
            _ => {
                lines = lines[..5].to_vec();
                (16.0, 24.0, 21.0)
            },
        };
        let center = if self.wide_side_bar { 300.0 } else { 450.0 };

        for (line, entry_flag, small_font) in lines.into_iter() {
            let font_size = if small_font { font_size * 0.75 } else { font_size };
            let line_max_len = if self.wide_side_bar { (900.0 / font_size) as usize } else { (1350.0 / font_size) as usize };
            let truncated_line = if line.chars().count() > (line_max_len + 4) {
                format!("{}...", line.chars().take(line_max_len).collect::<String>())
            } else {
                line.to_string()
            };
            let mut curr_x = center - truncated_line.chars().count() as f32 * font_size * 0.275;

            for ch in truncated_line.chars() {
                if ch != ' ' {
                    graphics.push(Graphic::Char {
                        ch,
                        x: curr_x,
                        y: curr_y,
                        size: font_size,
                        color: config.top_bar_font,
                    });
                }

                curr_x += font_size * 0.55;
            }

            match entry_flag {
                EntryFlag::None => {},
                _ => {
                    let color = match entry_flag {
                        EntryFlag::Red => Color { r: 0.75, g: 0.25, b: 0.25, a: 1.0 },
                        EntryFlag::Green => Color { r: 0.25, g: 0.75, b: 0.25, a: 1.0 },
                        EntryFlag::Blue => Color { r: 0.25, g: 0.25, b: 0.75, a: 1.0 },
                        EntryFlag::None => unreachable!(),
                    };

                    graphics.push(Graphic::Ellipse {
                        x: curr_x + 10.0,
                        y: curr_y - 6.25,
                        rx: 6.25,
                        ry: 6.25,
                        color,
                        thickness: None,
                    });
                },
            }

            curr_y += line_height;
        }
    }

    fn render_side_bar(&mut self, input: &Input, config: &Config, entries: &Entries, graphics: &mut Vec<Graphic>) {
        let (x, w) = if self.wide_side_bar { (600.0, 480.0) } else { (900.0, 180.0) };
        let title_max_len = if self.wide_side_bar { 36 } else { 8 };

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

        let mut list_start = self.cursor.max(18) - 18;
        let list_end = (list_start + 37).min(entries.len());

        if list_end < list_start + 37 {
            list_start = list_end.max(37) - 37;
        }

        let mut curr_y = 20.0;
        let mut is_hovering_on_something = false;

        for i in list_start..list_end {
            let truncated_title = if entries[i].name.chars().count() > (title_max_len + 4) {
                format!("{}...", entries[i].name.chars().take(title_max_len).collect::<String>())
            } else {
                entries[i].name.to_string()
            };
            let bullet = match (i == self.cursor, Some(i) == self.hovered_entry) {
                (true, false) => ">>",
                (false, true) => " *",
                (true, true) => "*>",
                (false, false) => "  ",
            };
            let title = format!(
                "{bullet} {}. {}",
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
                        size: 15.0,
                        color: config.side_bar_font,
                    });
                }

                curr_x += 8.0;
            }

            if check_contain([x + 5.0, curr_y - 17.6, w - 40.0, 17.6], input.mouse_pos) {
                is_hovering_on_something = true;
                self.hovered_entry = Some(i);
            }

            match entries[i].flag {
                EntryFlag::None => {},
                _ => {
                    let color = match entries[i].flag {
                        EntryFlag::Red => Color { r: 0.75, g: 0.25, b: 0.25, a: 1.0 },
                        EntryFlag::Green => Color { r: 0.25, g: 0.75, b: 0.25, a: 1.0 },
                        EntryFlag::Blue => Color { r: 0.25, g: 0.25, b: 0.75, a: 1.0 },
                        EntryFlag::None => unreachable!(),
                    };

                    graphics.push(Graphic::Ellipse {
                        x: curr_x + 7.0,
                        y: curr_y - 5.0,
                        rx: 5.0,
                        ry: 5.0,
                        color,
                        thickness: None,
                    });
                },
            }

            curr_y += 17.6;
        }

        if !is_hovering_on_something {
            self.hovered_entry = None;
        }

        if self.wide_side_bar && entries.len() > 33 {
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

            for (i, color) in self.curr_scroll_bar_colors().iter().enumerate() {
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
                y: 20.0 + (self.cursor * 640 / (entries.len() - 1)) as f32,
                rx: 8.0,
                ry: 8.0,
                thickness: None,
                color: config.side_bar_font,
            });
        }

        let counter = if entries.is_empty() { String::from("0 / 0") } else { format!("{} / {}", self.cursor + 1, entries.len()) };
        let mut curr_x = 1065.0 - 8.8 * counter.chars().count() as f32;

        for ch in counter.chars() {
            graphics.push(Graphic::Char {
                ch,
                x: curr_x,
                y: 680.0,
                size: 16.0,
                color: config.side_bar_font,
            });
            curr_x += 8.8;
        }

        // "expand" button
        if self.wide_side_bar {
            let alpha = if check_contain([584.0, 344.0, 32.0, 32.0], input.mouse_pos) {
                1.0
            } else {
                0.5
            };

            graphics.push(Graphic::Rect {
                x: 584.0,
                y: 344.0,
                w: 32.0,
                h: 32.0,
                radius: None,
                thickness: None,
                color: Color {
                    r: 0.7,
                    g: 0.7,
                    b: 0.7,
                    a: alpha,
                },
            });
            graphics.push(Graphic::Triangle {
                p1: (590.0, 350.0),
                p2: (590.0, 370.0),
                p3: (610.0, 360.0),
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: alpha,
                },
            });
        }

        else {
            let alpha = if check_contain([884.0, 344.0, 32.0, 32.0], input.mouse_pos) {
                1.0
            } else {
                0.5
            };

            graphics.push(Graphic::Rect {
                x: 884.0,
                y: 344.0,
                w: 32.0,
                h: 32.0,
                radius: None,
                thickness: None,
                color: Color {
                    r: 0.7,
                    g: 0.7,
                    b: 0.7,
                    a: alpha,
                },
            });
            graphics.push(Graphic::Triangle {
                p1: (910.0, 350.0),
                p2: (910.0, 370.0),
                p3: (890.0, 360.0),
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: alpha,
                },
            });
        }
    }

    fn render_canvas(&mut self, graphics: &mut Vec<Graphic>) {
        // The canvas has 900x600 resolution.
        let mut canvas = self.curr_canvas().unwrap_or(&vec![]).clone();
        scale(&mut canvas, self.camera_zoom);

        // The camera position is mapped to (450, 420) of the screen.
        // -> canvas has rect (0, 120, 900, 600) and its center is (450, 420).
        move_rel(&mut canvas, 450.0 - self.camera_pos.0 * self.camera_zoom, 420.0 - self.camera_pos.1 * self.camera_zoom);
        graphics.extend(canvas);
    }

    fn render_help(&mut self, entries: &Entries, graphics: &mut Vec<Graphic>) {
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

        // TODO: check these globally, and keep them in variables
        //       that's for 2 reasons:
        //       1. (very small) performance gain
        //       2. even if an entries have no state, we have to show help message if other entrieses have states
        let has_entry = !entries.is_empty();
        let has_entry_state = entries.entry_state_count > 1;
        let has_something_on_canvas = !self.curr_canvas().unwrap_or(&vec![]).is_empty();
        let has_transition = entries.transition.is_some() || entries.iter().any(|entry| entry.transition1.is_some() || entry.transition2.is_some());

        let mut lines = vec![
            (String::from("Esc: Quit"), true),
            (String::from("Left/Right: Toggle side-bar"), true),
            (String::from("Up/Down: Jump to prev/next entry"), has_entry),
            (String::from("1~9: Quick jump"), has_entry),
            (String::from("W/A/S/D: Move camera"), has_something_on_canvas),
            (String::from("Shift + W/A/S/D: Move camera faster"), has_something_on_canvas),
            (String::from("Z/X: Zoom In/Out"), has_something_on_canvas),
            (String::from("Shift + Z/X: Zoom In/Out faster"), has_something_on_canvas),
            (String::from("H: See help message"), true),
            (String::from("M: Change entry state"), has_entry_state),
            (String::from("Ctrl + Up/Left/Right: Transit to another entries"), has_transition),
        ];

        for (i, filter) in entries.filters.iter().enumerate() {
            lines.push((format!("Ctrl + {}: {}", i + 1, filter.name), true));
        }

        let help_message = lines.into_iter().filter(|(_, show)| *show).map(|(s, _)| s).collect::<Vec<_>>().join("\n");
        graphics.extend(TextBox::new(
            &help_message,
            18.0,
            Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            [72.0, 72.0, 936.0, 400.0],
        ).render());
    }

    fn render_popup(&self, graphics: &mut Vec<Graphic>) {
        if let Some((life, message)) = &self.popup {
            let center = if self.wide_side_bar { 300.0 } else { 450.0 };
            let mut curr_x = center - message.len() as f32 * 4.4;

            graphics.push(Graphic::Rect {
                x: curr_x - 20.0,
                y: 600.0,
                w: message.len() as f32 * 8.8 + 40.0,
                h: 80.0,
                radius: None,
                thickness: None,
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: *life.min(&60) as f32 / 60.0,
                },
            });
            graphics.push(Graphic::Rect {
                x: curr_x - 16.0,
                y: 604.0,
                w: message.len() as f32 * 8.8 + 32.0,
                h: 72.0,
                radius: None,
                thickness: None,
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: *life.min(&60) as f32 / 60.0,
                },
            });

            for ch in message.chars() {
                graphics.push(Graphic::Char {
                    ch,
                    x: curr_x,
                    y: 645.0,
                    size: 16.0,
                    color: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: *life.min(&60) as f32 / 60.0,
                    },
                });
                curr_x += 8.8;
            }
        }
    }
}
