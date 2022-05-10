use crate::git;
use crate::ascii_table::*;
use crate::render::{Line, KeyHandlers, Component, Window};

pub struct PushOptionsWindow { }

impl PushOptionsWindow {
    fn git_push(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line().trim().to_owned();

        let args = if !line.is_empty() {
            Some(vec![line.as_str()])
        } else {
            None
        };

        git::push(args);

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
