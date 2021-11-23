use crate::render::window::Window;
use crate::render::display::Display;
use crate::render::window::ScreenSize;

use crate::git::commands as git;

pub struct DiffWindow {
    path: String,
    data_start: usize, 
    data: Vec<String>,
    display: Display
}

impl DiffWindow {
    pub fn new(size: ScreenSize, path: &str) -> DiffWindow {
        DiffWindow {
            path: path.to_string(),
            data_start: 0,
            data: vec![],
            display: Display::new(size)
        }
    }
}

impl Window for DiffWindow {
    fn on_keypress(&mut self, _c: i32) {
        
    }

    fn on_activate(&mut self) {
        self.data = git::diff_file(&self.path);

        for (i, line) in self.data.iter().enumerate() {
            self.display.queue_write(&line.to_string(), (i as i32, 0));
        }
    }

    // TODO: Passthrough methods are evil!
    // Think of a better way.

    fn display(&self) -> &Display { &self.display }
    fn display_mut(&mut self) -> &mut Display { &mut self.display }

    fn data(&self) -> &Vec<String> {
        &self.data
    }

    fn start_position(&self) -> usize {
        self.data_start
    }

    fn set_start_position(&mut self, new_position: usize) {
        self.data_start = new_position;
    }

    fn close(&self) {
        self.display.close();
    }

    fn clear(&self) {
        self.display.clear();
    }
}
