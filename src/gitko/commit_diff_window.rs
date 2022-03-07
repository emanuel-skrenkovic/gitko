use crate::render::window::{Component, Window};

use crate::render::ascii_table::*;
use crate::git::commands as git;

pub struct CommitDiffWindow {
    commit_hash: String,
    data: Vec<String>,
}

impl CommitDiffWindow {
    pub fn new(commit_hash: &str) -> CommitDiffWindow {
        CommitDiffWindow {
            commit_hash: commit_hash.to_owned(),
            data: vec![]
        }
    }

    fn move_screen_up(&mut self, window: &mut Window<CommitDiffWindow>) -> bool {
        window.move_screen_up(&self.data, 1); // TODO: fix move above screen crash
        true
    }

    fn move_screen_down(&mut self, window: &mut Window<CommitDiffWindow>) -> bool {
        window.move_screen_down(&self.data, 1);
        true
    }
}

impl Component<CommitDiffWindow> for CommitDiffWindow {
    fn on_start(&mut self) {
        self.data = git::diff_commit(&self.commit_hash);
    }

    fn data(&self) -> &[String] {
        &self.data
    }

    fn register_handlers(&self, window: &mut Window<CommitDiffWindow>) {
        window.register_handler(KEY_J_LOWER, CommitDiffWindow::move_screen_down);
        window.register_handler(KEY_K_LOWER, CommitDiffWindow::move_screen_up);
    }
}
