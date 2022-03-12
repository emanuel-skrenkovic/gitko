use crate::git;
use crate::ascii_table::*;
use crate::render::{Component, Window};

pub struct CommitDiffWindow {
    commit_hash: String,
}

impl CommitDiffWindow {
    pub fn new(commit_hash: &str) -> CommitDiffWindow {
        CommitDiffWindow { commit_hash: commit_hash.to_owned() }
    }

    fn move_screen_up(&mut self, window: &mut Window<CommitDiffWindow>) -> bool {
        window.move_screen_up(1); // TODO: fix move above screen crash
        true
    }

    fn move_screen_down(&mut self, window: &mut Window<CommitDiffWindow>) -> bool {
        window.move_screen_down(1);
        true
    }
}

impl Component<CommitDiffWindow> for CommitDiffWindow {
    fn on_start(&mut self, window: &mut Window<CommitDiffWindow>) {
        window.data = git::diff_commit(&self.commit_hash);
    }

    fn register_handlers(&self, window: &mut Window<CommitDiffWindow>) {
        window.register_handler(KEY_J_LOWER, CommitDiffWindow::move_screen_down);
        window.register_handler(KEY_K_LOWER, CommitDiffWindow::move_screen_up);
    }
}
