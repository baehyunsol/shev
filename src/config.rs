use macroquad::color::Color;

pub struct Config {
    pub window_width: i32,
    pub window_height: i32,
    pub window_resizable: bool,
    pub window_title: String,
    pub fullscreen: bool,

    pub top_bar_bg: Color,
    pub top_bar_font: Color,
    pub side_bar_bg: Color,
    pub side_bar_font: Color,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            window_width: 1080,
            window_height: 720,
            window_resizable: true,
            window_title: "Gui".to_string(),
            fullscreen: false,
            top_bar_bg: Color {
                r: 0.2,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            top_bar_font: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            side_bar_bg: Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 1.0,
            },
            side_bar_font: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        }
    }
}
