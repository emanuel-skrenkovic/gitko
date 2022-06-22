use crate::git;
use crate::ascii_table::*;
use crate::gitko::commit_diff_window::CommitDiffWindow;
use crate::searchable::{SearchableComponent, register_search_handlers};
use crate::render::{Colored, Component, KeyHandlers, Line,Renderer, ScreenSize, Window, Position, Widget};

pub struct LogWindow {
    term: String,
}

impl LogWindow {
    pub fn new() -> LogWindow {
        LogWindow { term: "".to_owned() }
    }

    fn get_commit_log(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line();
        let trimmed_line = line
            .trim_matches(|c| c == '|' || c == '\\' || c == '*' || c == ' ');

        if trimmed_line.is_empty() {
            return true;
        }

        let commit_hash = &trimmed_line[0..7];

        Renderer::new(
            &mut CommitDiffWindow::new(commit_hash),
            ScreenSize::max(),
            Position::default()
        ).render();

        true
    }
}

fn map_line(line: &String) -> Line {
    let mut parts: Vec<Box<dyn Widget>> = vec![];

    let mut chars = line.chars();
    let star = chars.position(|c| c == '*');
    if let Some(star_position) = star {
        let start = chars.position(|c| c != ' ');

        if let Some(start_position) = start {
            let hash_start = star_position + start_position + 1;

            parts.push(Box::new(
                line[0..hash_start].to_owned()
            ));

            let hash_length = 7;
            parts.push(Box::new(
                Colored::new(
                    line[hash_start..hash_start + hash_length].to_owned(),
                    ncurses::COLOR_YELLOW,
                    ncurses::COLOR_BLACK
                )
            ));

            parts.push(Box::new(
                line[hash_start + hash_length..].to_owned()
            ))
        }
    } else {
        parts.push(Box::new(line.to_owned()))
    }

    Line::new(parts)
}

impl Component<LogWindow> for LogWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.lines = git::log(None)
            .iter()
            .map(map_line)
            .collect();
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<LogWindow>) {
        handlers.insert(KEY_LF, LogWindow::get_commit_log);
        handlers.insert(KEY_N_LOWER, LogWindow::next_search_result);
        handlers.insert(KEY_N_UPPER, LogWindow::prev_search_result);
        register_search_handlers(handlers);
    }
}

impl SearchableComponent<LogWindow> for LogWindow {
    fn term(&self) -> String {
        self.term.clone()
    }

    fn set_term(&mut self, term: String) {
        self.term = term;
    }
}
