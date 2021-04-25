use crate::render::ascii_table::*;
use crate::render::window::Window;

pub fn on_activate(_win: &mut Window) {}

pub fn on_key_press(win: &mut Window, c: i32) {
    match c {
        KEY_LF => {
            win.buffer.push("NOT IMPLEMENTED".to_string());
        }

        KEY_DEL => {
            if win.cursor.x == 0 {
                return;
            }

            win.buffer[1].pop();
            win.move_cursor_left();
        }

        _ => {
            if c == KEY_NULL {
                return;
            }

            win.buffer[1].push_str(ascii_to_char(c));
            win.move_cursor_right();
        }
    }
}
