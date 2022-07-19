use crate::git;
use crate::screen;
use crate::{max_height};
use crate::gitko::output_window::OutputWindow;
use crate::gitko::commit_diff_window::CommitDiffWindow;
use crate::searchable::{SearchableComponent, register_search_handlers};
use gitko_render::{Component, KeyHandlers, Line,Renderer, ScreenSize, Window, Position, Part};

use gitko_common::ascii_table::{KEY_LF, KEY_N_LOWER, KEY_N_UPPER, KEY_R_UPPER};

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
            Position::default(),
            screen()
        ).render();

        true
    }

    fn open_reset_options(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line().trim().to_owned();

        if let Some(commit_hash) = parse_commit_hash(&line) {
            Renderer::new(
                &mut ResetOptionsWindow { commit_hash: commit_hash.to_owned() },
                ScreenSize { lines: 5, cols: window.width() },
                Position { x: 0, y: window.height() - 5 },
                screen()
            ).render();
        }

        self.on_start(window);

        true
    }
}

fn map_line(line: &str) -> Line {
    let mut parts: Vec<Part> = vec![];

    let mut chars = line.chars();
    let star = chars.position(|c| c == '*');
    if let Some(star_position) = star {
        let start = chars.position(|c| c != ' ');

        if let Some(start_position) = start {
            let hash_start = star_position + start_position + 1;

            parts.push(Part::plain(&line[0..hash_start]));

            let hash_length = 7;
            parts.push(
                Part::painted(
                    &line[hash_start..hash_start + hash_length],
                    (255, 255, 0),
                    (0, 0, 0)
                )
            );

            parts.push(Part::plain(&line[hash_start + hash_length..]));
        }
    } else {
        parts.push(Part::plain(line));
    }

    Line::new(parts)
}

impl Component<LogWindow> for LogWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.set_lines(
            git::log(None)
                .iter()
                .map(|l| map_line(l))
                .collect()
        );
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<LogWindow>) {
        handlers.insert(KEY_LF, LogWindow::get_commit_log);
        handlers.insert(KEY_N_LOWER, LogWindow::next_search_result);
        handlers.insert(KEY_N_UPPER, LogWindow::prev_search_result);
        handlers.insert(KEY_R_UPPER, LogWindow::open_reset_options);
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


struct ResetOptionsWindow {
    commit_hash: String
}

impl ResetOptionsWindow {
    fn git_reset(&mut self, window: &mut Window) -> bool {
        let reset_mode = window.get_cursor_line().trim().to_owned();
        if !reset_mode.starts_with("--") { return true }

        window.clear();

        let output = git::reset(&self.commit_hash, &reset_mode);
        if output.is_empty() { return false }

        let output_window_height = output.len() as i32 + 1;
        Renderer::new(
            &mut OutputWindow { output },
            ScreenSize { lines: output_window_height , cols: window.width() },
            Position { x: 0, y: max_height() - output_window_height },
            screen()
        ).render();

        false
    }
}

impl Component<ResetOptionsWindow> for ResetOptionsWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.set_lines(
            vec!["Git reset modes:", "--soft", "--mixed", "--hard", "--merge", "--keep"]
                .iter()
                .map(|s| Line::plain(s))
                .collect()
        );
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<ResetOptionsWindow>) {
        handlers.insert(KEY_LF, ResetOptionsWindow::git_reset);
    }
}

fn parse_commit_hash(line: &str) -> Option<&str> {
    let mut chars = line.chars();

    let star = chars.position(|c| c == '*');
    if let Some(star_position) = star {
        let start = chars.position(|c| c != ' ');

        if let Some(start_position) = start {
            let hash_start = star_position + start_position + 1;
            let hash_length = 7;

            return Some(&line[hash_start..hash_start + hash_length])
        }
    }

    None
}
