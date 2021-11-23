use crate::git::commands as git;
use crate::render::window::Window;
use crate::render::display::Display;
use crate::render::window::ScreenSize;

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
    fn on_keypress(&mut self, _c: i32) {
    }

    fn on_activate(&mut self) {
        self.data = git::log(None);
        self.display.queue_write_buffer(&self.data);
    }

    fn data(&self) -> &Vec<String> { &self.data }

    fn start_position(&self) -> usize { self.data_start }
    fn set_start_position(&mut self, new_position: usize) {
        self.data_start = new_position;
    }

    fn display(&self) -> &Display { &self.display }
    fn display_mut(&mut self) -> &mut Display { &mut self.display }
}
