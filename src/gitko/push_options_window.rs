use crate::git;
use crate::max_height;
use crate::ascii_table::*;
use crate::gitko::output_window::OutputWindow;
use crate::render::{Renderer, Line, KeyHandlers, Component, Window, ScreenSize, Position};

pub struct PushOptionsWindow { }

impl PushOptionsWindow {
    fn git_push(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line().trim().to_owned();

        let args = if !line.is_empty() {
            Some(vec![line.as_str()])
        } else {
            None
        };

        window.clear();

        let output = git::push(args);
        if output.is_empty() { return false }

        let output_window_height = output.len() as i32 + 1;
        Renderer::new(
            &mut OutputWindow { output },
            ScreenSize { lines: output_window_height , cols: window.width() },
            Position { x: 0, y: max_height() - output_window_height }
        ).render();

        false
    }
}

impl Component<PushOptionsWindow> for PushOptionsWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.lines = vec!["", "--force-with-lease"]
            .iter()
            .map(|s| Line::from_string(s.to_string()))
            .collect();
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<PushOptionsWindow>) {
        handlers.insert(KEY_LF, PushOptionsWindow::git_push);
    }
}
