use crate::git;
use crate::{screen, max_width};
use gitko_render::{Component, KeyHandlers, Line, Renderer, ScreenSize, Window, Position};

use gitko_common::ascii_table::{KEY_D_LOWER, KEY_LF, KEY_N_LOWER};

use crate::gitko::text_window::TextWindow;
use crate::gitko::input_window::InputWindow;
use crate::gitko::prompt_window::PromptWindow;

pub struct BranchWindow { }

impl BranchWindow {
    fn open_delete_branch_prompt(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line();
        if line.trim().is_empty() { return true }

        if !line.starts_with('*') {
            let branch = line.trim();
            let mut prompt = PromptWindow::new(
                &format!("Are you sure you want to delete branch '{}'? y/n", branch),
                || { git::delete_branch(branch); },
                || { /* Do nothing on no. */ }
            );

            Renderer::new(
                &mut prompt,
                ScreenSize { lines: 1, cols: 0 }, // TODO
                Position { x: 0, y: window.height() - 1 },
                screen()
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

    fn create_branch(&mut self, window: &mut Window) -> bool {
        Renderer::new(
            &mut TextWindow {
                lines: vec!["Enter new branch name:"]
            },
            ScreenSize { lines: 5, cols: max_width() }, // TODO
            Position { x: 0, y: window.height() - 2 },
            screen()
        ).draw();

        let mut input_window = InputWindow::new();
        Renderer::new(
            &mut input_window,
            ScreenSize { lines: 2, cols: max_width() },
            Position { x: 0, y: window.height() - 1 },
            screen()
        ).render();

        git::create_branch(&input_window.text);

        self.on_start(window);

        true
    }
}

impl Component<BranchWindow> for BranchWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.set_lines(
            git::branch()
                .iter()
                .map(|l| Line::from_string(l.clone(), None))
                .collect()
        );
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<BranchWindow>) {
        handlers.insert(KEY_D_LOWER, BranchWindow::open_delete_branch_prompt);
        handlers.insert(KEY_LF, BranchWindow::checkout_branch);
        handlers.insert(KEY_N_LOWER, BranchWindow::create_branch);
    }
}
