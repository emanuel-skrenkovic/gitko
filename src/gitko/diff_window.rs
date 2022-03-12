
use crate::ascii_table::*;
use crate::render::{Component, Window};
use crate::git;

pub struct DiffWindow {
    path: String
}

impl DiffWindow {
    pub fn new(path: &str) -> DiffWindow {
        DiffWindow { path: path.to_string() }
    }

    fn move_screen_up(&mut self, window: &mut Window<DiffWindow>) -> bool {
        window.move_screen_up(1); // TODO: fix move above screen crash
        true
    }

    fn move_screen_down(&mut self, window: &mut Window<DiffWindow>) -> bool {
        window.move_screen_down(1);
        true
    }
}

impl Component<DiffWindow> for DiffWindow {
    fn on_start(&mut self, window: &mut Window<DiffWindow>) {
        window.data = git::diff_file(&self.path);
    }

    fn register_handlers(&self, window: &mut Window<DiffWindow>) {
        window.register_handler(KEY_J_LOWER, DiffWindow::move_screen_down);
        window.register_handler(KEY_K_LOWER, DiffWindow::move_screen_up);
    }
}
