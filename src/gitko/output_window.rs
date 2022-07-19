use gitko_render::{Component, KeyHandlers, Line, Window, Part, Style};

use gitko_common::ascii_table::*;

pub struct OutputWindow {
    pub output: Vec<String>
}

impl OutputWindow {
    fn close(&mut self, _window: &mut Window) -> bool {
        false
    }
}

impl Component<OutputWindow> for OutputWindow {
    fn on_start(&mut self, window: &mut Window) {
        // TODO: should not see ncurses her
        // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        window.show_cursor(false);

        let mut lines: Vec<Line> = vec![
            Line::new(vec![
                Part::new("Command output:", Some(vec![Style::Bold, Style::Underlined]))
            ])
        ];

        lines.append(&mut self.output
                     .iter()
                     .map(|s| Line::from_string(s.to_owned(), None))
                     .collect());

        window.set_lines(lines);
    }

    fn on_exit(&mut self, window: &mut Window) {
        window.show_cursor(true);
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<OutputWindow>) {
        handlers.insert(KEY_LF, OutputWindow::close);
        handlers.insert(KEY_ETB, OutputWindow::close);
    }
}
