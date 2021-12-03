use crate::render::ascii_table::*;
use crate::render::display::Display;

pub type Position = (i32, i32);

pub struct ScreenSize {
    pub lines: i32,
    pub cols: i32
}

impl ScreenSize {
    pub fn max() -> ScreenSize {
        ScreenSize {
            lines: 0,
            cols: 0
        }
    }
}

pub trait Window {
    fn on_keypress(&mut self, _c: i32) -> bool { true }
    fn on_activate(&mut self) { }

    fn cursor_position(&self) -> Position {
        self.display().cursor_position()
    }

    fn data(&self) -> &Vec<String>;
    fn start_position(&self) -> usize;
    fn set_start_position(&mut self, new_position: usize);

    // TODO
    fn move_cursor_down(&mut self) {
        let delta = self.display_mut().try_move_cursor_down();

        if delta > 0 {
            self.set_start_position(self.start_position() + delta as usize);

            self.display().queue_write_buffer(
                &self.data()[self.start_position()..].to_vec());
        }
    }

    fn move_cursor_up(&mut self) {
        let delta = self.display_mut().try_move_cursor_up();
        let delta_abs = delta.abs();

        if delta < 0 && self.start_position() as i32 - delta_abs >=0 {
            self.set_start_position(self.start_position() - delta_abs as usize);

            self.display().queue_write_buffer(
                &self.data()[self.start_position()..].to_vec());
        }
    }

    fn display(&self) -> &Display;
    fn display_mut(&mut self) -> &mut Display;

    fn close(&self) {
        self.display().close();
    }

    fn clear(&self) {
        self.display().clear();
    }

    fn refresh(&mut self) {
        self.clear();
        self.on_activate();
    }

    fn render_child<T>(&mut self, mut child: T) where T : Window {
        // TODO: seems to work for now. Might be busted.
        // Take a closer look.
        child.render();
        child.close();

        self.clear();
        self.on_activate();
    }

    fn render(&mut self) {
        self.on_activate();

        let mut c: i32 = 0;
        while c != KEY_Q_LOWER {
            // TODO: two updates per keypress for now.
            // Need to understand better.
            self.display().refresh();

            match c {
                KEY_J_LOWER => { self.move_cursor_down(); }
                KEY_K_LOWER => { self.move_cursor_up(); }
                KEY_Q_LOWER => { self.close(); }
                _ => {}
            }
            
            let cont = self.on_keypress(c);
            if !cont { break; }

            self.display().refresh();

            c = self.display().listen_input();
        }
    }
}
