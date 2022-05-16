use crate::ascii_table::*;
use crate::render::{Bold, Underlined, Component, KeyHandlers, Line, Window};

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
        let mut lines: Vec<Line> = vec![
            Line::new(vec![
                Box::new(
                    Bold::new(Underlined::new("Command output:"))
                )
           ])
        ];

        lines.append(&mut self.output
                     .iter()
                     .map(|s| Line::from_string(s.to_owned()))
                     .collect());

        window.lines = lines;
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<OutputWindow>) {
        handlers.insert(KEY_LF, OutputWindow::close);
        handlers.insert(KEY_ETB, OutputWindow::close);
    }
}
