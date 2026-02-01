use crate::graphic::Graphic;
use crate::input::Input;

pub fn fit_graphics_to_screen(
    graphics: &mut Vec<Graphic>,
    screen_width: f32,
    screen_height: f32,
) {
    // internal dimension: (1080, 720)
    let (
        x_offset,
        y_offset,
        scale,
    ) = if 1080.0 * screen_height > 720.0 * screen_width {
        let scale = screen_width / 1080.0;
        (0.0, (screen_height - 720.0 * scale) / 2.0, scale)
    } else {
        let scale = screen_height / 720.0;
        ((screen_width - 1080.0 * scale) / 2.0, 0.0, scale)
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
        }
    }
}

pub fn fit_input_to_screen(
    input: &mut Input,
    screen_width: f32,
    screen_height: f32,
) {
    todo!()
}
