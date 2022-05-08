use crate::git;
use crate::ascii_table::*;
use crate::render::{Component, KeyHandlers, Line, Window};

pub struct CommitOptionsWindow {

}

impl CommitOptionsWindow {
    fn git_commit(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line().trim().to_owned();

        let args = if !line.is_empty() {
            Some(vec![line.as_str()])
        } else {
            None
        };

        git::commit(args);
        false
    }
}

impl Component<CommitOptionsWindow> for CommitOptionsWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.lines = vec!["", "--amend"]
            .iter()
            .map(|s| Line::from_string(s.to_string()))
            .collect();
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<CommitOptionsWindow>) {
        handlers.insert(KEY_LF, CommitOptionsWindow::git_commit);
    }
}
