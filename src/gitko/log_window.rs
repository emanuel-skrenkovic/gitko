use crate::git::commands as git;
use crate::render::window::Window;
use crate::render::display::Display;
use crate::render::window::ScreenSize;

use crate::render::ascii_table::*;

use crate::gitko::commit_diff_window::CommitDiffWindow;

pub struct LogWindow {
    data: Vec<String>,
    display: Display,
    data_start: usize
}

impl LogWindow {
    pub fn new(size: ScreenSize) -> LogWindow {
        LogWindow {
            data: vec![],
            display: Display::new(size),
            data_start: 0
        }
    }
}

impl Window for LogWindow {
    fn on_activate(&mut self) {
        self.data = git::log(None);
        self.display.queue_write_buffer(&self.data);
    }

    fn on_keypress(&mut self, c: i32) -> bool {
        if c == KEY_LF {
                let line = self.display.get_cursor_line_data();
                let trimmed_line = line
                    .trim_matches(|c| c == '|' || c == '\\' || c == '*' || c == ' ');
                let commit_hash = &trimmed_line[0..7];

                self.render_child(CommitDiffWindow::new(ScreenSize::max(), commit_hash));
        }

        true
    }

    fn data(&self) -> &Vec<String> { &self.data }

    fn start_position(&self) -> usize { self.data_start }
    fn set_start_position(&mut self, new_position: usize) {
        self.data_start = new_position;
    }

    fn display(&self) -> &Display { &self.display }
    fn display_mut(&mut self) -> &mut Display { &mut self.display }
}
