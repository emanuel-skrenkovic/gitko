use crate::render::ascii_table::*;
use crate::render::window::ColorRule;
use crate::render::window::Window;
use crate::render::Point;
use crate::render::color;

fn added_change_color_rule(line: &str) -> bool {
    line.starts_with("+")
}

fn removed_change_color_rule(line: &str) -> bool {
    line.starts_with("-")
}

pub fn on_activate(win: &mut Window) {
    win.apply_color_rules(vec![
        ColorRule {
            foreground: color::RED,
            background: color::BLACK,
            rule: removed_change_color_rule,
        },
        ColorRule {
            foreground: color::GREEN,
            background: color::BLACK,
            rule: added_change_color_rule,
        },
    ]);

    // TODO: make cursor invisible. Currently stays invisible after exit
    // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

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
            // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
            win.marked_for_delete = true;
        }

        _ => {}
    }
}
