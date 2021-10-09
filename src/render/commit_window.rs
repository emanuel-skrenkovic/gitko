use crate::git::commands as git;
use crate::render::ascii_table::*;
use crate::render::window::Window;

pub fn on_activate(win: &mut Window) {
    win.move_cursor_down();
}

pub fn on_key_press(win: &mut Window, c: i32) {
    match c {
        KEY_LF => {
            if !win.is_empty() {
                git::commit(&win.line_at(1));

                let notification_message = format!("Commited with message: {}", &win.line_at(1));
                win.update_value_at(3, notification_message);
            }
        }

        KEY_DEL => {
            if win.cursor.x == 0 {
                return;
            }

            win.value_at(1).pop();
            win.move_cursor_left();
        }

        _ => {
            if c == KEY_NULL {
                return;
            }

            let mut new_value = win.value_at(1);
            new_value.push_str(ascii_to_char(c));

            win.update_value_at(1, new_value);
            win.move_cursor_right();
        }
    }
}
