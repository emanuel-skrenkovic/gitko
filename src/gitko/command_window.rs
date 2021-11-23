use crate::render::window::Window;
use crate::render::display::Display;
use crate::render::window::ScreenSize;
use crate::render::ascii_table::*;

pub struct CommandWindow {
    data: Vec<String>,
    display: Display
}

impl CommandWindow {
    pub fn new(size: ScreenSize) -> CommandWindow {
        CommandWindow {
            data: vec![String::new()],
            display: Display::new(size)
        }
    }
}

impl Window for CommandWindow {
    fn on_keypress(&mut self, c: i32) {
        if c == KEY_NULL {
            return;
        }

        // let mut line = &mut self.data[0];
        // line.push_str(ascii_to_char(c));
        let char = ascii_to_char(c);
        &mut self.data[0].push_str(char);

        let cursor = self.cursor_position();
        self.display.move_cursor((cursor.0, cursor.1 + 1));

        self.refresh();

        // TODO: get display as writeable - allows
        // for regular typing in display.
    }

    fn on_activate(&mut self) {
        self.display.queue_write_buffer(&self.data);
    }

    fn data(&self) -> &Vec<String> { &self.data }

    fn start_position(&self) -> usize { 0 }
    fn set_start_position(&mut self, _new_position: usize) { }

    fn display(&self) -> &Display { &self.display }
    fn display_mut(&mut self) -> &mut Display { &mut self.display }
}
