use crate::git::commands as git;
use crate::render::ascii_table::*;
use crate::render::window::Window;

pub fn on_activate(_win: &mut Window) {}

pub fn on_key_press(win: &mut Window, c: i32) {
    match c {
        KEY_LF => {
            if !win.value_buffer.is_empty() {
                git::commit(&win.value_buffer[1]);
            }
        }

        KEY_DEL => {
            if win.cursor.x == 0 {
                return;
            }

            win.value_buffer[1].pop();
            win.move_cursor_left();
        }

        _ => {
            if c == KEY_NULL {
                return;
            }

            win.value_buffer[1].push_str(ascii_to_char(c));
            win.move_cursor_right();
        }
    }
}
