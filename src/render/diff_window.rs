use crate::render::ascii_table::*;
use crate::render::color;
use crate::render::window::ColorRule;
use crate::render::window::Style;
use crate::render::window::Window;
use crate::render::Point;

fn added_change_color_rule(line: &str) -> bool {
    line.starts_with('+')
}

fn removed_change_color_rule(line: &str) -> bool {
    line.starts_with('-')
}

pub fn on_activate(win: &mut Window) {
    win.style(Style {
        color_rules: vec![
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
        ],
    });

    // TODO: make cursor invisible. Currently stays invisible after exit
    // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

pub fn on_key_press(win: &mut Window, c: i32) {
    match c {
        // ctrl + d
        KEY_EOT => {
            win.move_cursor(Point {
                x: 0,
                y: win.height - 1,
            });
            win.move_cursor_down_n(10);
        }

        // ctrl + u
        KEY_NAK => {
            win.move_cursor(Point { x: 0, y: 0 });
            win.move_cursor_up_n(10);
        }

        KEY_J_LOWER => {
            if win.cursor.y == 0 {
                win.move_cursor(Point {
                    x: 0,
                    y: win.height,
                });
            }
        }

        KEY_K_LOWER => {
            win.move_cursor(Point { x: 0, y: -1 });
        }

        KEY_Q_LOWER => {
            // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
            win.marked_for_delete = true;
        }

        _ => {}
    }
}
