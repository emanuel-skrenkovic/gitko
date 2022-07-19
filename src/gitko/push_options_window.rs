use crate::screen;
use crate::git;
use crate::max_height;
use crate::gitko::output_window::OutputWindow;
use gitko_render::{Renderer, Line, KeyHandlers, Component, Window, ScreenSize, Position};

use gitko_common::ascii_table::{KEY_LF};

pub struct PushOptionsWindow { }

impl PushOptionsWindow {
    fn git_push(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line().trim().to_owned();

        let args = if line.is_empty() {
            None
        } else {
            Some(vec![line.as_str()])
        };

        window.clear();

        let output = git::push(args);
        if output.is_empty() { return false }

        let output_window_height = output.len() as i32 + 1;
        Renderer::new(
            &mut OutputWindow { output },
            ScreenSize { lines: output_window_height, cols: window.width() },
            Position { x: 0, y: max_height() - output_window_height },
            screen()
        ).render();

        false
    }
}

impl Component<PushOptionsWindow> for PushOptionsWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.set_lines(
            vec!["", "--force-with-lease"]
                .iter()
                .map(|s| Line::plain(s))
                .collect()
        );
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<PushOptionsWindow>) {
        handlers.insert(KEY_LF, PushOptionsWindow::git_push);
    }
}
