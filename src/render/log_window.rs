use crate::render::ascii_table::*;
use crate::render::window::Window;
use crate::render::Point;

pub fn on_activate(_win: &mut Window) {}

pub fn on_key_press(win: &mut Window, c: i32) {
    match c {
        KEY_J_LOWER => {
            win.move_cursor(Point {
                x: 0,
                y: win.height,
            });
            win.move_cursor_down();
        }

        KEY_K_LOWER => {
            win.move_cursor(Point { x: 0, y: 0 });
            win.move_cursor_up();
        }

        KEY_Q_LOWER => {
            win.marked_for_delete = true;
        }

        _ => {}
    }
}
