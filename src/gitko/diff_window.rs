use crate::render::window::ScreenSize;
use crate::render::window::Position;
use crate::render::window::Window;
use crate::render::window::BaseWindow;

use crate::git::commands as git;

pub struct DiffWindow {
    path: String,
    data: Vec<String>,
    window: Window
}

impl DiffWindow {
    pub fn new(size: ScreenSize, path: &str) -> DiffWindow {
        DiffWindow {
            path: path.to_string(),
            data: vec![],
            window: Window::new(size)
        }
    }
}

impl BaseWindow for DiffWindow {
    fn on_keypress(&mut self, c: i32) {
        
    }

    fn on_activate(&mut self) {
        self.data = git::diff_file(&self.path);

        for (i, line) in self.data.iter().enumerate() {
            self.window.queue_write(&line.to_string(), (i as i32, 0));
        }
    }

    // TODO: Passthrough methods are evil!
    // Think of a better way.

    fn window(&self) -> ncurses::WINDOW { 
        self.window.curses_window
    }

    fn cursor_position(&self) -> Position {
        self.window.cursor_position()
    }

    fn move_cursor_down(&mut self) {
        self.window.move_cursor_down();
    }

    fn move_cursor_up(&mut self) {
        self.window.move_cursor_up();
    }

    fn move_cursor(&mut self, position: Position) {
        self.window.move_cursor(position);
    }

    fn close(&self) {
        self.window.close();
    }

    fn clear(&self) {
        self.window.clear();
    }
}
