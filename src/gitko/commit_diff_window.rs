use crate::git;
use crate::searchable::{SearchableComponent, register_search_handlers};
use gitko_render::{Component, KeyHandlers, Line, Window, Part};

use gitko_common::ascii_table::*;

pub struct CommitDiffWindow {
    commit_hash: String,
    term: String
}

impl CommitDiffWindow {
    pub fn new(commit_hash: &str) -> CommitDiffWindow {
        CommitDiffWindow {
            commit_hash: commit_hash.to_owned(),
            term: "".to_owned()
        }
    }

    fn move_screen_up(&mut self, window: &mut Window) -> bool {
        window.move_screen_up(1);
        true
    }

    fn move_screen_down(&mut self, window: &mut Window) -> bool {
        window.move_screen_down(1);
        true
    }

    fn jump_screen_up(&mut self, window: &mut Window) -> bool {
        for _ in 0..20 {
            self.move_screen_up(window);
        }

        true
    }

    fn jump_screen_down(&mut self, window: &mut Window) -> bool {
        for _ in 0..20 {
            self.move_screen_down(window);
        }

        true
    }
}

fn map_line(line: String) -> Line {
    if line.starts_with('+') {
        Line::new(vec![
            Part::painted(
                &line,
                (0, 255, 0),
                (0, 0, 0)
            //     ncurses::COLOR_GREEN,
            //     ncurses::COLOR_BLACK
            )
        ])
    } else if line.starts_with('-') {
        Line::new(vec![
            Part::painted(
                &line,
                (255, 0, 0),
                (0, 0, 0)
                // ncurses::COLOR_RED,
                // ncurses::COLOR_BLACK
            )
        ])
    } else if line.starts_with("@@") {
        Line::new(vec![
            Part::painted(
                &line,
                (0, 255, 255),
                (0, 0, 0)
                // ncurses::COLOR_CYAN,
                // ncurses::COLOR_BLACK
            )
        ])
    } else {
        Line::from_string(line, None)
    }
}

impl Component<CommitDiffWindow> for CommitDiffWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.show_cursor(false);

        window.set_lines(
            git::diff_commit(&self.commit_hash)
                .iter()
                .map(|l| map_line(l.to_owned()))
                .collect()
        );
    }

    fn on_exit(&mut self, window: &mut Window) {
        window.show_cursor(true);
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<CommitDiffWindow>) {
        handlers.insert(KEY_J_LOWER, CommitDiffWindow::move_screen_down);
        handlers.insert(KEY_K_LOWER, CommitDiffWindow::move_screen_up);

        handlers.insert(4, CommitDiffWindow::jump_screen_down);
        handlers.insert(21, CommitDiffWindow::jump_screen_up);

        register_search_handlers(handlers);
    }
}

impl SearchableComponent<CommitDiffWindow> for CommitDiffWindow {
    fn term(&self) -> String {
        self.term.clone()
    }

    fn set_term(&mut self, term: String) {
        self.term = term;
    }
}
