use crate::git;
use crate::ascii_table::*;
use crate::render::{Component, KeyHandlers, Line, Renderer, ScreenSize, Window, Position, WriteableWindow};
use crate::gitko::commit_diff_window::CommitDiffWindow;

pub struct LogWindow {
    term: String,
    found_at: Option<usize>
}

impl LogWindow {
    pub fn new() -> LogWindow {
        LogWindow { term: "".to_owned(), found_at: None }
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

    fn search_logs(&mut self, window: &mut Window) -> bool {
        self.clear_search();
        let mut search_window = SearchWindow::new();

        Renderer::new(
            &mut search_window,
            ScreenSize { lines: 2, cols: window.width() },
            Position { x: 0, y: window.height() - 2 }
        ).render();

        self.term = search_window.text;
        self.jump_to_term_forward(window);

        true
    }

    fn next_search_result(&mut self, window: &mut Window) -> bool {
        self.jump_to_term_forward(window);
        true
    }

    fn prev_search_result(&mut self, window: &mut Window) -> bool {
        self.jump_to_term_backward(window);
        true
    }

    fn clear_search(&mut self) {
        self.term     = "".to_owned();
        self.found_at = None;
    }

    fn jump_to_term_forward(&mut self, window: &mut Window) {
        if self.term.is_empty() {
            return
        }

        let start = if let Some(last_found_at) = self.found_at {
            last_found_at + 1
        } else {
            0
        };
        let end = window.lines.len();
        let search_data = window.data()[start..end].to_vec();

        self.found_at = match search_data.iter().position(|l| l.contains(&self.term)) {
            Some(position) => {
                let new_position = start + position;

                window.set_cursor(Position{ x: 0, y: new_position as i32 });
                Some(new_position)
            },
            None => None

        };
    }

    fn jump_to_term_backward(&mut self, window: &mut Window) {
        if self.term.is_empty() {
            return
        }

        let data = window.data();

        let end = if let Some(last_found_at) = self.found_at {
            last_found_at
        } else {
            data.len()
        };

        let search_data = data[0..end].to_vec();

        self.found_at = match search_data.iter().rposition(|l| l.contains(&self.term)) {
            Some(position) => {
                window.set_cursor(Position{ x: 0, y: position as i32 });
                Some(position)
            },
            None => None
        };
    }
}

impl Component<LogWindow> for LogWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.lines = git::log(None)
            .iter()
            .map(|l| Line::from_string(l.to_owned()))
            .collect();
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<LogWindow>) {
        handlers.insert(KEY_LF, LogWindow::get_commit_log);
        handlers.insert(KEY_N_LOWER, LogWindow::next_search_result);
        handlers.insert(KEY_N_UPPER, LogWindow::prev_search_result);
        handlers.insert(KEY_FORWARD_SLASH, LogWindow::search_logs);
    }
}

#[derive(Clone)]
struct SearchWindow {
    text: String
}

impl SearchWindow {
    pub fn new() -> SearchWindow {
        SearchWindow { text: "".to_owned() }
    }
}

impl Component<SearchWindow> for SearchWindow {
    fn on_render(&mut self, window: &mut Window) -> bool {
        window.as_writeable_mut().listen();

        self.text = window.get_cursor_line().trim().to_owned();

        false
    }
}
