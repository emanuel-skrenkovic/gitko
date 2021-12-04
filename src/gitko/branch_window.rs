use crate::render::display::Display;
use crate::render::window::ScreenSize;
use crate::render::window::Window;
use crate::render::ascii_table::*;
use crate::git::commands as git;

pub struct BranchWindow {
    data: Vec<String>,
    display: Display
}

impl BranchWindow {
    pub fn new(size: ScreenSize) -> BranchWindow {
        BranchWindow {
            data: vec![],
            display: Display::new(size)
        }
    }
}

impl Window for BranchWindow {
    fn on_keypress(&mut self, c: i32) -> bool {
        match c {
            KEY_LF => {
                let line = self.display.get_cursor_line_data();
                if !line.starts_with("*") {
                    git::checkout_branch(line.trim());
                }
            }
            _ => {}
        }

        true
    }

    fn on_activate(&mut self) {
        self.data = git::branch();

        self.display.queue_write_buffer(&self.data);
    }

    fn data(&self) -> &Vec<String> { &self.data }

    fn start_position(&self) -> usize { 0 }

    fn set_start_position(&mut self, _new_position: usize) { }

    fn display (&self) -> &Display { &self.display }

    fn display_mut (&mut self) -> &mut Display { &mut self.display }
}
