use super::State;
use crate::action::Action;
use crate::entry::{Entries, EntryState};
use crate::input::Input;
use crate::transform::check_contain;
use macroquad::input::KeyCode;

impl State {
    pub async fn frame(&mut self, entries: &Entries, input: &Input) -> Action {
        let original_cursor = self.cursor;
        let mut scroll_up = false;
        let mut scroll_down = false;

        if let Some((life, _)) = &mut self.popup {
            *life -= 1;

            if *life == 0 {
                self.popup = None;
            }
        }

        if input.released_keys.contains(&KeyCode::Escape) {
            if self.show_help {
                self.show_help = false;
                return Action::None;
            }

            return Action::Quit;
        }

        if input.down_keys.contains(&KeyCode::Down) {
            self.scrolling_with_arrow_keys = (self.scrolling_with_arrow_keys - 1).min(-1);
        }

        else if input.down_keys.contains(&KeyCode::Up) {
            self.scrolling_with_arrow_keys = (self.scrolling_with_arrow_keys + 1).max(1);
        }

        else {
            self.scrolling_with_arrow_keys = 0;
        }

        if self.scrolling_with_arrow_keys < -12 {
            scroll_down = true;
        }

        else if self.scrolling_with_arrow_keys > 12 {
            scroll_up = true;
        }

        #[cfg(target_os="macos")]
        let is_ctrl_down = input.down_keys.contains(&KeyCode::LeftSuper) || input.down_keys.contains(&KeyCode::RightSuper);
        #[cfg(not(target_os="macos"))]
        let is_ctrl_down = input.down_keys.contains(&KeyCode::LeftControl) || input.down_keys.contains(&KeyCode::RightControl);

        let is_shift_down = input.down_keys.contains(&KeyCode::LeftShift) || input.down_keys.contains(&KeyCode::RightShift);
        let is_alt_down = input.down_keys.contains(&KeyCode::LeftAlt) || input.down_keys.contains(&KeyCode::RightAlt);
        let side_bar_start = if self.wide_side_bar { 600.0 } else { 900.0 };

        if !is_shift_down && !is_ctrl_down && !is_alt_down {
            if !entries.is_empty() {
                let mut scroll_speed = 1;

                if input.mouse_wheel.1 < 0.0 && input.mouse_pos.0 >= side_bar_start {
                    scroll_down = true;
                    scroll_speed = (entries.len() / 32).max(1);
                }

                else if input.mouse_wheel.1 > 0.0 && input.mouse_pos.0 >= side_bar_start {
                    scroll_up = true;
                    scroll_speed = (entries.len() / 32).max(1);
                }

                if input.pressed_keys.contains(&KeyCode::Down) || scroll_down {
                    self.cursor = (self.cursor + scroll_speed) % entries.len();
                }

                else if input.pressed_keys.contains(&KeyCode::Up) || scroll_up {
                    self.cursor = (self.cursor + entries.len() - scroll_speed) % entries.len();
                }

                let n = if input.pressed_keys.contains(&KeyCode::Key1) {
                    Some(0)
                } else if input.pressed_keys.contains(&KeyCode::Key2) {
                    Some(1)
                } else if input.pressed_keys.contains(&KeyCode::Key3) {
                    Some(2)
                } else if input.pressed_keys.contains(&KeyCode::Key4) {
                    Some(3)
                } else if input.pressed_keys.contains(&KeyCode::Key5) {
                    Some(4)
                } else if input.pressed_keys.contains(&KeyCode::Key6) {
                    Some(5)
                } else if input.pressed_keys.contains(&KeyCode::Key7) {
                    Some(6)
                } else if input.pressed_keys.contains(&KeyCode::Key8) {
                    Some(7)
                } else if input.pressed_keys.contains(&KeyCode::Key9) {
                    Some(8)
                } else {
                    None
                };

                if let Some(n) = n {
                    self.cursor = n * (entries.len() - 1) / 8;
                }
            }

            if input.pressed_keys.contains(&KeyCode::Left) {
                self.wide_side_bar = true;
            }

            else if input.pressed_keys.contains(&KeyCode::Right) {
                self.wide_side_bar = false;
            }

            if input.pressed_keys.contains(&KeyCode::H) {
                self.show_help = !self.show_help;
            }

            if input.pressed_keys.contains(&KeyCode::M) {
                self.entry_state.0 = (self.entry_state.0 + 1) % entries.entry_state_count;
                // self.reset_entry_state();
            }
        }

        if is_shift_down {
            for (key, key_code, transition) in [
                ("Up", KeyCode::Up, &entries.transition),
                ("Left", KeyCode::Left, &entries.get(self.cursor).map(|entry| entry.transition1.clone()).unwrap_or(None)),
                ("Right", KeyCode::Right, &entries.get(self.cursor).map(|entry| entry.transition2.clone()).unwrap_or(None)),
            ] {
                if input.pressed_keys.contains(&key_code) {
                    if let Some(transition) = transition {
                        self.curr_entries_id = transition.id.to_string();
                        self.reset_entries_state();
                        return Action::Transit(transition.id.to_string());
                    }

                    else {
                        self.show_popup(&format!(
                            "There's no transition mapped to {}+{key} key.",
                            if cfg!(target_os="macos") { "Shift" } else { "Ctrl" },
                        ));
                    }
                }
            }
        }

        let mut camera_move_speed = 1.0;
        let mut scroll_up = false;
        let mut scroll_down = false;
        let mut scroll_left = false;
        let mut scroll_right = false;

        if input.mouse_pos.0 < side_bar_start {
            if input.mouse_wheel.0 < 0.0 {
                scroll_left = true;
                camera_move_speed = 3.0;
            }

            if input.mouse_wheel.0 > 0.0 {
                scroll_right = true;
                camera_move_speed = 3.0;
            }

            if input.mouse_wheel.1 < 0.0 {
                scroll_up = true;
                camera_move_speed = 3.0;
            }

            if input.mouse_wheel.1 > 0.0 {
                scroll_down = true;
                camera_move_speed = 3.0;
            }
        }

        let (camera_move_speed, zoom_faster) = if is_shift_down {
            (40.0 / self.camera_zoom * camera_move_speed, true)
        } else {
            (10.0 / self.camera_zoom * camera_move_speed, false)
        };

        if input.down_keys.contains(&KeyCode::W) || scroll_up {
            self.camera_pos.1 -= camera_move_speed;
        }

        if input.down_keys.contains(&KeyCode::A) || scroll_left {
            self.camera_pos.0 -= camera_move_speed;
        }

        if input.down_keys.contains(&KeyCode::S) || scroll_down {
            self.camera_pos.1 += camera_move_speed;
        }

        if input.down_keys.contains(&KeyCode::D) || scroll_right {
            self.camera_pos.0 += camera_move_speed;
        }

        if input.down_keys.contains(&KeyCode::Z) {
            if zoom_faster {
                self.camera_zoom = (self.camera_zoom * 1.2).min(8.0);
            }

            else {
                self.camera_zoom = (self.camera_zoom * 1.05).min(8.0);
            }
        }

        if input.down_keys.contains(&KeyCode::X) {
            if zoom_faster {
                self.camera_zoom = (self.camera_zoom * 0.8333).max(0.1);
            }

            else {
                self.camera_zoom = (self.camera_zoom * 0.9523).max(0.1);
            }
        }

        if self.wide_side_bar && check_contain(
            [584.0, 344.0, 32.0, 32.0],
            input.mouse_pos,
        ) && input.mouse_pressed[0] {
            self.wide_side_bar = false;
        }

        else if !self.wide_side_bar && check_contain(
            [884.0, 344.0, 32.0, 32.0],
            input.mouse_pos,
        ) && input.mouse_pressed[0] {
            self.wide_side_bar = true;
        }

        else if let Some(i) = self.hovered_entry && input.mouse_pressed[0] {
            self.cursor = i;
        }

        if self.cursor != original_cursor {
            self.reset_entry_state();
        }

        Action::None
    }

    fn show_popup(&mut self, message: &str) {
        self.popup = Some((120, message.to_string()));
    }

    fn reset_entries_state(&mut self) {
        self.cursor = 0;
        self.hovered_entry = None;
        self.reset_entry_state();
    }

    fn reset_entry_state(&mut self) {
        self.entry_state = EntryState(0);
        self.camera_pos = (450.0, 300.0);
        self.camera_zoom = 1.0;
    }
}
