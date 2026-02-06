use crate::cache::TextureCache;
use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::{draw_circle, draw_rectangle, draw_triangle};
use macroquad::text::{Font, TextParams, draw_text_ex};
use macroquad::texture::{DrawTextureParams, draw_texture_ex};

mod text_box;

pub use text_box::TextBox;

#[derive(Clone, Debug)]
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
    Triangle {
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
        color: Color,
    },

    /// NOTE: (x, y) is bottom-left of the character.
    /// TODO: I'm not sure whether (x, y) is bottom-left of the character...
    Char {
        ch: char,
        x: f32,
        y: f32,
        size: f32,
        color: Color,
    },

    /// NOTE: If you're using shev as a framework, use this to render an image. The engine will
    ///       cache the image.
    ///
    /// NOTE: If you're developing shev, make sure to convert this to `Graphic::Image { .. }`
    ///       before calling `render()`.
    ImageFile {
        path: String,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    },

    /// NOTE: If you're using shev as a framework, you don't need this.
    Image {
        path: String,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    }
}

impl Graphic {
    pub fn get_rect(&self) -> [f32; 4] {
        match self {
            Graphic::Rect { x, y, w, h, .. } |
            Graphic::ImageFile { x, y, w, h, .. } |
            Graphic::Image { x, y, w, h, .. } => [*x, *y, *w, *h],
            Graphic::Ellipse { x, y, rx, ry, .. } => [*x - *rx, *y - *ry, *rx * 2.0, *ry * 2.0],
            Graphic::Triangle { p1: (x1, y1), p2: (x2, y2), p3: (x3, y3), .. } => {
                let x_min = (*x1).min(*x2).min(*x3);
                let x_max = (*x1).max(*x2).max(*x3);
                let y_min = (*y1).min(*y2).min(*y3);
                let y_max = (*y1).max(*y2).max(*y3);
                [x_min, y_min, x_max - x_min, y_max - y_min]
            },
            Graphic::Char { x, y, size, .. } => [*x, *y, *x + *size, *y + *size],
        }
    }
}

pub async fn render(graphics: &[Graphic], font: &Font, textures: &mut TextureCache, (screen_width, screen_height): (f32, f32)) {
    for graphic in graphics.iter() {
        let [x, y, w, h] = graphic.get_rect();

        if x > screen_width || y > screen_height || x + w < 0.0 || y + h < 0.0 {
            continue;
        }

        match graphic {
            Graphic::Rect { x, y, w, h, radius: None, thickness: None, color } => {
                draw_rectangle(*x, *y, *w, *h, *color);
            },
            Graphic::Rect { x, y, w, h, radius: Some(r), thickness: None, color } => {
                draw_rectangle(
                    *x + *r,
                    *y + *r,
                    *w - 2.0 * *r,
                    *h - 2.0 * *r,
                    *color,
                );
                draw_rectangle(
                    *x + *r,
                    *y,
                    *w - 2.0 * *r,
                    *r,
                    *color,
                );
                draw_rectangle(
                    *x + *r,
                    *y + *h - *r,
                    *w - 2.0 * *r,
                    *r,
                    *color,
                );
                draw_rectangle(
                    *x,
                    *y + *r,
                    *r,
                    *h - 2.0 * *r,
                    *color,
                );
                draw_rectangle(
                    *x + *w - *r,
                    *y + *r,
                    *r,
                    *h - 2.0 * *r,
                    *color,
                );

                for (angles, (c_x, c_y)) in [
                    ([0.0f32, 0.2618, 0.5236, 0.7854, 1.0472, 1.3090], (*x + *w - *r, *y + *h - *r)),
                    ([1.5708, 1.8326, 2.0944, 2.3562, 2.6180, 2.8798], (*x + *r, *y + *h - *r)),
                    ([3.1416, 3.4034, 3.6652, 3.9270, 4.1888, 4.4506], (*x + *r, *y + *r)),
                    ([4.7124, 4.9742, 5.2360, 5.4978, 5.7596, 6.0214], (*x + *w - *r, *y + *r)),
                ] {
                    for angle in angles.into_iter() {
                        let (a1_x, a1_y) = (angle.cos(), angle.sin());
                        let (a2_x, a2_y) = ((angle + 0.2618).cos(), (angle + 0.2618).sin());

                        draw_triangle(
                            Vec2::new(c_x + a1_x * *r, c_y + a1_y * *r),
                            Vec2::new(c_x + a2_x * *r, c_y + a2_y * *r),
                            Vec2::new(c_x, c_y),
                            *color,
                        );
                    }
                }
            },
            Graphic::Rect { .. } => todo!(),
            Graphic::Ellipse { x, y, rx, ry, thickness: None, color } if rx == ry => {
                draw_circle(*x, *y, *rx, *color);
            },
            Graphic::Ellipse { .. } => todo!(),
            Graphic::Triangle { p1: (x1, y1), p2: (x2, y2), p3: (x3, y3), color } => {
                draw_triangle(
                    Vec2::new(*x1, *y1),
                    Vec2::new(*x2, *y2),
                    Vec2::new(*x3, *y3),
                    *color,
                );
            },
            Graphic::Char { ch, x, y, size, color } => {
                draw_text_ex(
                    &std::iter::once(*ch).collect::<String>(),
                    *x,
                    *y,
                    TextParams {
                        font: Some(font),
                        font_size: size.round() as u16,
                        color: *color,
                        ..Default::default()
                    },
                );
            },
            Graphic::ImageFile { .. } => panic!("It should've been converted to `Graphic::Image`: {graphic:?}"),
            Graphic::Image { path, x, y, w, h } => {
                draw_texture_ex(
                    textures.get(path).await,
                    *x,
                    *y,
                    // TODO: what's it for?
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(*w, *h)),
                        ..Default::default()
                    },
                );
            },
        }
    }
}

pub fn hide_off_screen(graphics: &mut Vec<Graphic>, screen_width: f32, screen_height: f32) {
    graphics.push(Graphic::Rect {
        x: -400.0,
        y: -400.0,
        w: screen_width + 800.0,
        h: 400.0,
        radius: None,
        thickness: None,
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
    });
    graphics.push(Graphic::Rect {
        x: -400.0,
        y: screen_height,
        w: screen_width + 800.0,
        h: 400.0,
        radius: None,
        thickness: None,
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
    });
    graphics.push(Graphic::Rect {
        x: -400.0,
        y: -400.0,
        w: 400.0,
        h: screen_height + 800.0,
        radius: None,
        thickness: None,
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
    });
    graphics.push(Graphic::Rect {
        x: screen_width,
        y: -400.0,
        w: 400.0,
        h: screen_height + 800.0,
        radius: None,
        thickness: None,
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
    });
}
