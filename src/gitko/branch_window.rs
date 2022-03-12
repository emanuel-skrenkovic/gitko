use crate::git;
use crate::ascii_table::*;
use crate::render::{Component, KeyHandlers, Renderer, ScreenSize, Window, Position};

use crate::gitko::prompt_window::PromptWindow;

pub struct BranchWindow { }

impl BranchWindow {
    pub fn new() -> BranchWindow {
        BranchWindow { }
    }

    fn open_delete_branch_prompt(
        &mut self, window:
        &mut Window) -> bool {
        let line = window.get_cursor_line();

        if !line.starts_with('*') {
            let branch = line.trim();
            let prompt = PromptWindow::new(
                &format!("Are you sure you want to delete branch '{}'? y/n", branch),
                || { git::delete_branch(branch); },
                || { });

            Renderer::new(
                prompt,
                ScreenSize { lines: 1, cols: 0 }, // TODO
                Position { x: 0, y: window.height() - 1 }
            ).render();

            self.on_start(window);
        }

        true
    }

    fn checkout_branch(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line();
        if !line.starts_with('*') {
            git::checkout_branch(line.trim());
        }

        self.on_start(window);

        true
    }
}

impl Component<BranchWindow> for BranchWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.data = git::branch();
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<BranchWindow>) {
        handlers.insert(KEY_LF, BranchWindow::checkout_branch);
        handlers.insert(KEY_D_LOWER, BranchWindow::open_delete_branch_prompt);
    }
}
