use crate::render::window::ScreenSize;
use crate::render::window::Position;
use crate::render::display::Display;
use crate::render::window::Window;

use crate::git::commands as git;

pub struct DiffWindow {
    path: String,
    data_start: i32, 
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
    fn on_keypress(&mut self, c: i32) {
        
    }

    fn on_activate(&mut self) {
        self.data = git::diff_file(&self.path);

        for (i, line) in self.data.iter().enumerate() {
            self.display.queue_write(&line.to_string(), (i as i32, 0));
        }
    }

    // TODO: Passthrough methods are evil!
    // Think of a better way.

    fn window(&self) -> ncurses::WINDOW { 
        self.display.curses_window
    }

    fn cursor_position(&self) -> Position {
        self.display.cursor_position()
    }

    fn move_cursor_down(&mut self) {
        let delta = self.display.try_move_cursor_down();

        if delta > 0 {
            self.data_start += delta;

            self.display.queue_write_buffer(
                &self.data[(self.data_start as usize)..].to_vec());
        }
    }

    fn move_cursor_up(&mut self) {
        let delta = self.display.try_move_cursor_up();
        let delta_abs = delta.abs();

        if delta < 0 && (self.data_start - delta_abs >= 0) {
           self.data_start -= delta_abs;

            self.display.queue_write_buffer(
                &self.data[(self.data_start as usize)..].to_vec());
        }
    }

    fn move_cursor(&mut self, position: Position) {
        self.display.move_cursor(position);
    }

    fn close(&self) {
        self.display.close();
    }

    fn clear(&self) {
        self.display.clear();
    }
}
