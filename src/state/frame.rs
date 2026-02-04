use super::State;
use crate::action::Action;
use crate::entry::{Entries, EntryState};
use crate::input::Input;
use crate::transform::check_contain;
use macroquad::input::KeyCode;

impl State {
    pub async fn frame(&mut self, entries: &Entries, mut input: Input) -> Action {
        let original_cursor = self.cursor;

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

            if self.show_extra_content {
                self.show_extra_content = false;
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
            input.pressed_keys.insert(KeyCode::Down);
        }

        else if self.scrolling_with_arrow_keys > 12 {
            input.pressed_keys.insert(KeyCode::Up);
        }

        let is_shift_down = input.pressed_keys.contains(&KeyCode::LeftShift) || input.pressed_keys.contains(&KeyCode::RightShift);
        let is_ctrl_down = input.pressed_keys.contains(&KeyCode::LeftControl) || input.pressed_keys.contains(&KeyCode::RightControl);
        let is_alt_down = input.pressed_keys.contains(&KeyCode::LeftAlt) || input.pressed_keys.contains(&KeyCode::RightAlt);
        let side_bar_start = if self.wide_side_bar { 600.0 } else { 900.0 };

        if !entries.is_empty() {
            let mut scroll_speed = 1;

            if input.mouse_wheel.1 < 0.0 && input.mouse_pos.0 >= side_bar_start {
                input.pressed_keys.insert(KeyCode::Down);
                scroll_speed = (entries.len() / 32).max(1);
            }

            else if input.mouse_wheel.1 > 0.0 && input.mouse_pos.0 >= side_bar_start {
                input.pressed_keys.insert(KeyCode::Up);
                scroll_speed = (entries.len() / 32).max(1);
            }

            if input.pressed_keys.contains(&KeyCode::Down) {
                if is_ctrl_down {
                    // Ctrl+Shift+Down: jump to next category 2
                    if is_shift_down {
                        let curr_cat = &entries[self.cursor].category2;

                        for _ in 0..entries.len() {
                            self.cursor = (self.cursor + 1) % entries.len();

                            if &entries[self.cursor].category2 != curr_cat {
                                break;
                            }
                        }
                    }

                    // Ctrl+Down: jump to next category 1
                    else {
                        let curr_cat = &entries[self.cursor].category1;

                        for _ in 0..entries.len() {
                            self.cursor = (self.cursor + 1) % entries.len();

                            if &entries[self.cursor].category1 != curr_cat {
                                break;
                            }
                        }
                    }
                }

                // Down: jump to next entry
                else {
                    self.cursor = (self.cursor + scroll_speed) % entries.len();
                }
            }

            else if input.pressed_keys.contains(&KeyCode::Up) {
                if is_ctrl_down {
                    // Ctrl+Shift+Up: jump to prev category 2
                    if is_shift_down {
                        let curr_cat = &entries[self.cursor].category2;

                        for _ in 0..entries.len() {
                            self.cursor = (self.cursor + entries.len() - 1) % entries.len();

                            if &entries[self.cursor].category2 != curr_cat {
                                break;
                            }
                        }
                    }

                    // Ctrl+Down: jump to prev category 1
                    else {
                        let curr_cat = &entries[self.cursor].category1;

                        for _ in 0..entries.len() {
                            self.cursor = (self.cursor + entries.len() - 1) % entries.len();

                            if &entries[self.cursor].category1 != curr_cat {
                                break;
                            }
                        }
                    }
                }

                // Down: jump to prev entry
                else {
                    self.cursor = (self.cursor + entries.len() - scroll_speed) % entries.len();
                }
            }

            else if input.pressed_keys.contains(&KeyCode::Space) {
                // Alt+Space: jump to prev entry with the same flag
                if is_alt_down {
                    let curr_flag = &entries[self.cursor].flag;

                    for _ in 0..entries.len() {
                        self.cursor = (self.cursor + entries.len() - 1) % entries.len();

                        if &entries[self.cursor].flag == curr_flag {
                            break;
                        }
                    }
                }

                // Space: jump to next entry with the same flag
                else {
                    let curr_flag = &entries[self.cursor].flag;

                    for _ in 0..entries.len() {
                        self.cursor = (self.cursor + 1) % entries.len();

                        if &entries[self.cursor].flag == curr_flag {
                            break;
                        }
                    }
                }
            }

            let n = if is_shift_down || is_ctrl_down || is_alt_down {
                None
            } else if input.pressed_keys.contains(&KeyCode::Key1) {
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

        if !is_shift_down && !is_ctrl_down && !is_alt_down {
            if input.pressed_keys.contains(&KeyCode::Left) {
                self.wide_side_bar = true;
            }

            else if input.pressed_keys.contains(&KeyCode::Right) {
                self.wide_side_bar = false;
            }

            if input.pressed_keys.contains(&KeyCode::H) {
                self.show_help = !self.show_help;

                if self.show_help {
                    self.show_extra_content = false;
                }
            }

            if input.pressed_keys.contains(&KeyCode::C) {
                if !self.show_help {
                    self.show_extra_content = !self.show_extra_content;

                    if self.show_extra_content && (entries.is_empty() || entries[self.cursor].extra_content.is_none()) {
                        self.show_popup("There's no extra content to display.");
                        self.show_extra_content = false;
                    }
                }
            }

            // TODO: do I have to reset camera after this?
            if input.pressed_keys.contains(&KeyCode::M) {
                self.entry_state = self.entry_state.next();
            }

            for (key, key_code, transition) in [
                ("J", KeyCode::J, &entries.transition),
                ("K", KeyCode::K, &entries.get(self.cursor).map(|entry| entry.transition1.clone()).unwrap_or(None)),
                ("L", KeyCode::L, &entries.get(self.cursor).map(|entry| entry.transition2.clone()).unwrap_or(None)),
            ] {
                if input.pressed_keys.contains(&key_code) {
                    if let Some(transition) = transition {
                        self.curr_entries_id = transition.id.to_string();
                        self.reset_entries_state();
                        return Action::Transit(transition.id.to_string());
                    }

                    else {
                        self.show_popup(&format!("There's no transition mapped to {key} key."));
                    }
                }
            }
        }

        let mut camera_move_speed = 1.0;

        if input.mouse_pos.0 < side_bar_start {
            if input.mouse_wheel.0 < 0.0 {
                input.down_keys.insert(KeyCode::A);
                camera_move_speed = 3.0;
            }

            if input.mouse_wheel.0 > 0.0 {
                input.down_keys.insert(KeyCode::D);
                camera_move_speed = 3.0;
            }

            if input.mouse_wheel.1 < 0.0 {
                input.down_keys.insert(KeyCode::W);
                camera_move_speed = 3.0;
            }

            if input.mouse_wheel.1 > 0.0 {
                input.down_keys.insert(KeyCode::S);
                camera_move_speed = 3.0;
            }
        }

        let (camera_move_speed, zoom_faster) = if input.down_keys.contains(&KeyCode::LeftShift) || input.down_keys.contains(&KeyCode::RightShift) {
            (40.0 / self.camera_zoom * camera_move_speed, true)
        } else {
            (10.0 / self.camera_zoom * camera_move_speed, false)
        };

        if input.down_keys.contains(&KeyCode::W) {
            self.camera_pos.1 -= camera_move_speed;
        }

        if input.down_keys.contains(&KeyCode::A) {
            self.camera_pos.0 -= camera_move_speed;
        }

        if input.down_keys.contains(&KeyCode::S) {
            self.camera_pos.1 += camera_move_speed;
        }

        if input.down_keys.contains(&KeyCode::D) {
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
        self.entry_state = EntryState::None;
        self.show_extra_content = false;
        self.camera_pos = (450.0, 300.0);
        self.camera_zoom = 1.0;
    }
}
