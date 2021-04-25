use std::path::Path;

use crate::git;
use crate::render::ascii_table::*;
use crate::render::commit_window;
use crate::render::diff_window;
use crate::render::window::Window;
use crate::render::Point;
use crate::render::Render;

pub fn on_activate(win: &mut Window) {
    let git_status: Vec<String> = git::run_status_command();

    let mut new_changes: Vec<String> = git_status
        .iter()
        .cloned()
        .filter(|c| c.starts_with("??"))
        .collect();

    let mut unstaged_changes: Vec<String> = git_status
        .iter()
        .cloned()
        .filter(|c| c.starts_with(" M"))
        .collect();

    let mut staged_changes: Vec<String> = git_status
        .iter()
        .cloned()
        .filter(|c| c.starts_with("M ") || c.starts_with("A "))
        .collect();

    let mut status: Vec<String> = vec![];

    status.push("New:".to_string());
    status.append(&mut new_changes);

    status.push("".to_string());

    status.push("Modified:".to_string());
    status.append(&mut unstaged_changes);

    status.push("".to_string());

    status.push("Staged:".to_string());
    status.append(&mut staged_changes);

    win.buffer = status;
}

pub fn on_key_press(win: &mut Window, c: i32) {
    match c {
        // cursor movement
        KEY_J_LOWER => {
            win.move_cursor_down();
        }

        KEY_K_LOWER => {
            win.move_cursor_up();
        }

        KEY_T_LOWER => {
            let path = &win.get_cursor_line()[3..];
            git::run_add_command(&path);
        }

        KEY_U_LOWER => {
            let path = &win.get_cursor_line()[3..];
            git::unstage_file(&path);
        }

        KEY_Q_LOWER => {
            win.delete = true;
        }

        KEY_C_LOWER => {
            win.buffer = vec![];
            win.queue_update();

            let child: &mut Window = win.spawn_child(
                Point { x: 0, y: 0 },
                vec!["Commit message below:".to_string(), "".to_string()],
                commit_window::on_activate,
                commit_window::on_key_press,
            );

            child.move_cursor_down();

            child.render();
        }

        KEY_W_LOWER => {
            let path = &win.get_cursor_line()[3..];

            if path.is_empty() || !Path::new(path).exists() {
                return;
            }

            let diff_lines = git::run_diff_command(&path);

            win.write_at(&diff_lines, win.cursor.y as usize);
            win.queue_update();

            let copy = diff_lines.clone();

            let child: &mut Window = win.spawn_child(
                Point {
                    y: win.cursor.y + 1,
                    x: 5,
                },
                diff_lines,
                diff_window::on_activate,
                diff_window::on_key_press,
            );

            child.value_buffer = copy;
            child.render();
        }
        _ => {}
    }
}
