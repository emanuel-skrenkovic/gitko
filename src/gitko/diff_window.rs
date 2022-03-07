use crate::render::window::{Component, Window};

use crate::render::ascii_table::*;
use crate::git::commands as git;

pub struct DiffWindow {
    path: String,
    data: Vec<String>,
}

impl DiffWindow {
    pub fn new(path: &str) -> DiffWindow {
        DiffWindow {
            path: path.to_string(),
            data: vec![],
        }
    }

    fn load_data(&mut self) {
        self.data = git::diff_file(&self.path);
    }

    fn move_screen_up(&mut self, window: &mut Window<DiffWindow>) -> bool {
        window.move_screen_up(&self.data, 1); // TODO: fix move above screen crash
        true
    }

    fn move_screen_down(&mut self, window: &mut Window<DiffWindow>) -> bool {
        window.move_screen_down(&self.data, 1);
        true
    }
}

impl Component<DiffWindow> for DiffWindow {
    fn on_start(&mut self) {
        self.load_data();
    }

    fn data(&self) -> &[String] {
        &self.data
    }

    fn register_handlers(&self, window: &mut Window<DiffWindow>) {
        window.register_handler(KEY_J_LOWER, DiffWindow::move_screen_down);
        window.register_handler(KEY_K_LOWER, DiffWindow::move_screen_up);
    }
}
