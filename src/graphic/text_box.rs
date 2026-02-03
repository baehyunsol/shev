use super::Graphic;
use macroquad::color::Color;

pub struct TextBox {
    s: String,
    font_size: f32,
    color: ColorMap,
    rect: [f32; 4],
    padding: [f32; 4],
}

enum ColorMap {
    Simple(Color),
    Each(Vec<Color>),
}

impl TextBox {
    pub fn new(
        s: &str,
        font_size: f32,
        color: Color,
        rect: [f32; 4],
    ) -> TextBox {
        TextBox {
            s: s.to_string(),
            font_size,
            color: ColorMap::Simple(color),
            rect,
            padding: [0.0; 4],
        }
    }

    pub fn with_color_map(&mut self, color_map: Vec<Color>) -> &mut Self {
        self.color = ColorMap::Each(color_map);
        self
    }

    pub fn render(&self) -> Vec<Graphic> {
        let [x, y, w, h] = self.rect;
        let [top, bottom, left, right] = self.padding;

        text_box(
            &self.s,
            self.font_size,
            &self.color,
            [x + left, y + top, w - left - right, h - top - bottom],
        )
    }
}

fn text_box(
    s: &str,
    font_size: f32,
    color: &ColorMap,
    rect: [f32; 4],
) -> Vec<Graphic> {
    let [x, y, w, h] = rect;
    let mut result = vec![];
    let max_x = (w / (font_size * 0.55) - 1.0).max(4.0) as usize - 4;
    let max_y = (h / (font_size * 1.1) - 1.0).max(1.0) as usize - 1;
    let (s, colors) = break_lines_and_apply_colors(s, color, max_x, max_y);
    let mut curr_y = y;
    let mut curr_x = x;

    for (ch, color) in s.chars().zip(colors.iter()) {
        if ch == '\n' {
            curr_x = x;
            curr_y += font_size * 1.1;
            continue;
        }

        result.push(Graphic::Char {
            ch,
            x: curr_x,
            y: curr_y,
            size: font_size,
            color: *color,
        });
        curr_x += font_size * 0.55;
    }

    result
}

fn break_lines_and_apply_colors(s: &str, color: &ColorMap, max_x: usize, max_y: usize) -> (String, Vec<Color>) {
    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut chars = vec![];
    let mut colors = vec![];
    let mut line_broken = false;

    for (i, ch) in s.chars().enumerate() {
        let color = match color {
            ColorMap::Simple(c) => *c,
            ColorMap::Each(cs) => cs[i],
        };

        if ch == '\n' {
            curr_x = 0;
            curr_y += 1;
            line_broken = false;
            chars.push(ch);
            colors.push(color);

            if curr_y > max_y {
                break;
            }
        }

        else {
            if line_broken {
                continue;
            }

            if curr_x > max_x {
                for _ in 0..3 {
                    chars.push('.');
                    colors.push(color);
                }

                line_broken = true;
                continue;
            }

            chars.push(ch);
            colors.push(color);
            curr_x += 1;
        }
    }

    (chars.into_iter().collect(), colors)
}
