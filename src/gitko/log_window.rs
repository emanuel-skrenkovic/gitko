use crate::git;
use crate::ascii_table::*;
use crate::render::{Component, KeyHandlers, Renderer, ScreenSize, Window, Position, WriteableWindow};
use crate::gitko::commit_diff_window::CommitDiffWindow;

pub struct LogWindow { }

impl LogWindow {
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
        let mut search_window = SearchWindow::new();

        Renderer::new(
            &mut search_window,
            ScreenSize { lines: 2, cols: window.width() },
            Position { x: 0, y: window.height() - 2 }
        ).render();

        let term = search_window.term;

        // TODO: search data and jump cursor to line

        true
    }
}

impl Component<LogWindow> for LogWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.data = git::log(None);
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<LogWindow>) {
        handlers.insert(KEY_LF, LogWindow::get_commit_log);
        handlers.insert(KEY_FORWARD_SLASH, LogWindow::search_logs);
    }
}

#[derive(Clone)]
struct SearchWindow {
    term: String
}

impl SearchWindow {
    pub fn new() -> SearchWindow {
        SearchWindow { term: "".to_owned() }
    }
}

impl Component<SearchWindow> for SearchWindow {
    fn on_render(&mut self, window: &mut Window) -> bool {
        window.as_writeable_mut().listen();

        self.term = window.get_cursor_line().trim().to_owned();

        false
    }
}
