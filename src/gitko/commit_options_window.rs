use crate::screen;
use crate::git;
use crate::max_height;
use crate::gitko::output_window::OutputWindow;
use gitko_render::{Renderer, ScreenSize, Position, Component, KeyHandlers, Line, Window};

use gitko_common::ascii_table::{KEY_LF};

pub struct CommitOptionsWindow { }

impl CommitOptionsWindow {
    fn git_commit(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line().trim().to_owned();

        let args = if line.is_empty() {
            None
        } else {
            Some(vec![line.as_str()])
        };

        let output = git::commit(args);
        let output_window_height = output.len() as i32 + 1;

        if output.is_empty() { return false }

        Renderer::new(
            &mut OutputWindow{ output },
            ScreenSize { lines: output_window_height, cols: window.width() },
            Position { x: 0, y: max_height() - output_window_height },
            screen()
        ).render();

        false
    }
}

impl Component<CommitOptionsWindow> for CommitOptionsWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.set_lines(
            vec!["", "--amend"]
                .iter()
                .map(|s| Line::plain(s))
                .collect()
            );
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<CommitOptionsWindow>) {
        handlers.insert(KEY_LF, CommitOptionsWindow::git_commit);
    }
}
