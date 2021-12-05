use crate::render::window::Window;
use crate::render::display::Display;
use crate::render::window::ScreenSize;

use crate::render::ascii_table::*;

pub struct PromptWindow {
    data: Vec<String>,
    display: Display,
    result: bool
}

impl PromptWindow {
    pub fn new(size: ScreenSize, message: &str) -> PromptWindow {
        PromptWindow {
            data: vec![message.to_string()],
            display: Display::new(size),
            result: false
        }
    }

    pub fn get_result(&self) -> bool {
        self.result
    }
}

impl Window for &mut PromptWindow {
    fn on_activate(&mut self) {
        self.display.queue_write_buffer(&self.data);
    }

    fn on_keypress(&mut self, c: i32) -> bool {
        match c {
            KEY_Y_LOWER => {
                self.result = true;
                false
            }
            KEY_N_LOWER => {
                self.result = false;
                false
            }
            _ => { true }
        }
    }

    fn data(&self) -> &Vec<String> { &self.data }

    fn start_position(&self) -> usize { 0 }

    fn set_start_position(&mut self, _new_position: usize) { }

    fn display (&self) -> &Display { &self.display }

    fn display_mut (&mut self) -> &mut Display { &mut self.display }
}
