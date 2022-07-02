use std::{fs::File, io::{BufReader, BufRead}};

use crate::git;
use crate::git::{FileState};
use crate::searchable::{SearchableComponent, register_search_handlers};
use crate::ascii_table::*;
use crate::render::{KeyHandlers, Colored, Component, Line, Window};

pub struct DiffWindow {
    path: String,
    file_state: FileState,
    term: String
}

impl DiffWindow {
    pub fn new(path: &str, file_state: FileState) -> DiffWindow {
        DiffWindow {
            path: path.to_string(),
            file_state,
            term: "".to_owned()
        }
    }

    fn move_screen_up(&mut self, window: &mut Window) -> bool {
        window.move_screen_up(1); // TODO: fix move above screen crash
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
            Box::new(
                Colored::new(
                    line,
                    ncurses::COLOR_GREEN,
                    ncurses::COLOR_BLACK
                )
            )
        ])
    } else if line.starts_with('-') {
        Line::new(vec![
            Box::new(
                Colored::new(
                    line,
                    ncurses::COLOR_RED,
                    ncurses::COLOR_BLACK
                )
            )
        ])
    } else if line.starts_with("@@") {
        Line::new(vec![
            Box::new(
                Colored::new(
                    line,
                    ncurses::COLOR_CYAN,
                    ncurses::COLOR_BLACK
                )
            )
        ])
    } else {
        Line::from_string(line)
    }
}

impl Component<DiffWindow> for DiffWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.show_cursor(false);

        match self.file_state {
            FileState::Untracked => {
                // Assume the path is a file path
                // Component above should parse directories into file paths.
                let file = File::open(&self.path).expect("Could not find file");
                let lines: Vec<String> = BufReader::new(file)
                    .lines()
                    .map(|l| l.expect("Could not parse line"))
                    .collect();

                window.lines = lines
                    .iter()
                    .map(|l| map_line(l.to_owned()))
                    .collect();
            },
            _ => {
                window.lines = git::diff_file(&self.path)
                    .iter()
                    .map(|l| map_line(l.to_owned()))
                    .collect();
            }
        }
    }

    fn on_exit(&mut self, window: &mut Window) {
        window.show_cursor(true);
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<DiffWindow>) {
        handlers.insert(KEY_J_LOWER, DiffWindow::move_screen_down);
        handlers.insert(KEY_K_LOWER, DiffWindow::move_screen_up);

        handlers.insert(4, DiffWindow::jump_screen_down);
        handlers.insert(21, DiffWindow::jump_screen_up);

        register_search_handlers(handlers);
    }
}

impl SearchableComponent<DiffWindow> for DiffWindow {
    fn term(&self) -> String {
        self.term.clone()
    }

    fn set_term(&mut self, term: String) {
        self.term = term;
    }
}
