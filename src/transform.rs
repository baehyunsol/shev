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
            Graphic::Char { x, y, size, .. } => {
                *x = *x * scale + x_offset;
                *y = *y * scale + y_offset;
                *size = (*size as f32 * scale).round() as u16;
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
        }
    }
}
