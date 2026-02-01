use crate::config::GuiConfig;
use crate::entry::Entry;
use crate::graphic::Graphic;

pub struct State<Data> {
    pub entries: Vec<Entry<Data>>,
    pub cursor: usize,
}

impl<T> State<T> {
    pub fn render(&self, config: &GuiConfig) -> Vec<Graphic> {
        let mut graphics = vec![];

        // top-bar bg
        graphics.push(Graphic::Rect {
            x: 0.0,
            y: 0.0,
            w: 1080.0,
            h: 120.0,
            radius: None,
            thickness: None,
            color: config.top_bar_bg,
        });

        self.render_side_bar(config, &mut graphics);

        graphics
    }

    fn render_side_bar(&self, config: &GuiConfig, graphics: &mut Vec<Graphic>) {
        // bg
        graphics.push(Graphic::Rect {
            x: 900.0,
            y: 0.0,
            w: 180.0,
            h: 720.0,
            radius: None,
            thickness: None,
            color: config.side_bar_bg,
        });
        // render side-bar
        let mut list_start = self.cursor.max(12) - 12;
        let list_end = (list_start + 25).min(self.entries.len());

        if list_end < list_start + 25 {
            list_start = list_end.max(25) - 25;
        }

        let mut curr_y = 60.0;

        for i in list_start..list_end {
            let truncated_title = if self.entries[i].title.chars().count() > 24 {
                format!("{}...", self.entries[i].title.chars().take(20).collect::<String>())
            } else {
                self.entries[i].title.to_string()
            };
            let title = format!(
                "{} {}. {}",
                if i == self.cursor { ">>" } else { "  " },
                i + 1,
                truncated_title,
            );
            let mut curr_x = 915.0;

            for ch in title.chars() {
                if ch != ' ' {
                    graphics.push(Graphic::Char {
                        ch,
                        x: curr_x,
                        y: curr_y,
                        size: 14,
                        color: config.side_bar_font,
                    });
                }

                curr_x += 6.5;
            }

            curr_y += 20.0;
        }
    }
}
