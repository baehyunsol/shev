use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use macroquad::text::{TextParams, draw_text_ex};

pub enum Graphic {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: Option<f32>,
        thickness: Option<f32>,
        color: Color,
    },
    Ellipse {
        x: f32,
        y: f32,
        rx: f32,
        ry: f32,
        thickness: Option<f32>,
        color: Color,
    },
    Char {
        ch: char,
        x: f32,
        y: f32,
        size: u16,
        color: Color,
    },
}

pub fn render(graphics: &[Graphic]) {
    for graphic in graphics.iter() {
        match graphic {
            Graphic::Rect { x, y, w, h, radius: None, thickness: None, color } => {
                draw_rectangle(*x, *y, *w, *h, *color);
            },
            Graphic::Rect { .. } => todo!(),
            Graphic::Ellipse { .. } => todo!(),
            Graphic::Char { ch, x, y, size, color } => {
                draw_text_ex(
                    &std::iter::once(*ch).collect::<String>(),
                    *x,
                    *y,
                    TextParams {
                        font_size: *size,
                        color: *color,
                        ..Default::default()
                    },
                );
            },
        }
    }
}
