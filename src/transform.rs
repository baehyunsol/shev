use crate::graphic::Graphic;
use crate::input::Input;

pub fn fit_graphics_to_screen(
    graphics: &mut Vec<Graphic>,
    canvas_width: f32,
    canvas_height: f32,
    screen_width: f32,
    screen_height: f32,
) {
    let (
        x_offset,
        y_offset,
        scale,
    ) = if canvas_width * screen_height > canvas_height * screen_width {
        let scale = screen_width / canvas_width;
        (0.0, (screen_height - canvas_height * scale) / 2.0, scale)
    } else {
        let scale = screen_height / canvas_height;
        ((screen_width - canvas_width * scale) / 2.0, 0.0, scale)
    };

    for graphic in graphics.iter_mut() {
        match graphic {
            Graphic::Rect { x, y, w, h, radius, thickness, .. } => {
                *x = *x * scale + x_offset;
                *y = *y * scale + y_offset;
                *w *= scale;
                *h *= scale;

                if let Some(radius) = radius {
                    *radius *= scale;
                }

                if let Some(thickness) = thickness {
                    *thickness *= scale;
                }
            },
            Graphic::Ellipse { x, y, rx, ry, thickness, .. } => {
                *x = *x * scale + x_offset;
                *y = *y * scale + y_offset;
                *rx *= scale;
                *ry *= scale;

                if let Some(thickness) = thickness {
                    *thickness *= scale;
                }
            },
            Graphic::Triangle { p1: (x1, y1), p2: (x2, y2), p3: (x3, y3), .. } => {
                *x1 = *x1 * scale + x_offset;
                *x2 = *x2 * scale + x_offset;
                *x3 = *x3 * scale + x_offset;
                *y1 = *y1 * scale + y_offset;
                *y2 = *y2 * scale + y_offset;
                *y3 = *y3 * scale + y_offset;
            },
            Graphic::Char { x, y, size, .. } => {
                *x = *x * scale + x_offset;
                *y = *y * scale + y_offset;
                *size *= scale;
            },
            Graphic::ImageFile { x, y, w, h, .. } | Graphic::Image { x, y, w, h, .. } => {
                *x = *x * scale + x_offset;
                *y = *y * scale + y_offset;
                *w *= scale;
                *h *= scale;
            },
        }
    }
}

pub fn fit_input_to_screen(
    input: &mut Input,
    canvas_width: f32,
    canvas_height: f32,
    screen_width: f32,
    screen_height: f32,
) {
    let (
        x_offset,
        y_offset,
        scale,
    ) = if canvas_width * screen_height > canvas_height * screen_width {
        let scale = screen_width / canvas_width;
        (0.0, (screen_height - canvas_height * scale) / 2.0, scale)
    } else {
        let scale = screen_height / canvas_height;
        ((screen_width - canvas_width * scale) / 2.0, 0.0, scale)
    };

    input.mouse_pos.0 = (input.mouse_pos.0 - x_offset) / scale;
    input.mouse_pos.1 = (input.mouse_pos.1 - y_offset) / scale;
}

pub fn move_rel(graphics: &mut Vec<Graphic>, x_offset: f32, y_offset: f32) {
    for graphic in graphics.iter_mut() {
        match graphic {
            Graphic::Rect { x, y, .. } |
            Graphic::Ellipse { x, y, .. } |
            Graphic::Char { x, y, .. } |
            Graphic::ImageFile { x, y, .. } |
            Graphic::Image { x, y, .. } => {
                *x += x_offset;
                *y += y_offset;
            },
            Graphic::Triangle { p1: (x1, y1), p2: (x2, y2), p3: (x3, y3), .. } => {
                *x1 += x_offset;
                *x2 += x_offset;
                *x3 += x_offset;
                *y1 += y_offset;
                *y2 += y_offset;
                *y3 += y_offset;
            },
        }
    }
}

pub fn scale(graphics: &mut Vec<Graphic>, scale: f32) {
    for graphic in graphics.iter_mut() {
        match graphic {
            Graphic::Rect { x, y, w, h, radius, thickness, .. } => {
                *x *= scale;
                *y *= scale;
                *w *= scale;
                *h *= scale;

                if let Some(radius) = radius {
                    *radius *= scale;
                }

                if let Some(thickness) = thickness {
                    *thickness *= scale;
                }
            },
            Graphic::Ellipse { x, y, rx, ry, thickness, .. } => {
                *x *= scale;
                *y *= scale;
                *rx *= scale;
                *ry *= scale;

                if let Some(thickness) = thickness {
                    *thickness *= scale;
                }
            },
            Graphic::Triangle { p1: (x1, y1), p2: (x2, y2), p3: (x3, y3), .. } => {
                *x1 *= scale;
                *x2 *= scale;
                *x3 *= scale;
                *y1 *= scale;
                *y2 *= scale;
                *y3 *= scale;
            },
            Graphic::Char { x, y, size, .. } => {
                *x *= scale;
                *y *= scale;
                *size *= scale;
            },
            Graphic::ImageFile { x, y, w, h, .. } | Graphic::Image { x, y, w, h, .. } => {
                *x *= scale;
                *y *= scale;
                *w *= scale;
                *h *= scale;
            },
        }
    }
}

pub fn check_contain(rect: [f32; 4], point: (f32, f32)) -> bool {
    let (p_x, p_y) = point;
    let [r_x, r_y, r_w, r_h] = rect;

    r_x <= p_x && p_x < r_x + r_w && r_y <= p_y && p_y < r_y + r_h
}
