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
    fn on_keypress(&mut self, c: i32);
    fn on_activate(&mut self);

    fn cursor_position(&self) -> Position;

    // TODO: scrolling functionality

    // I think it needs to be here, rather than the Window struct impl.
    // Window struct should not know about the data storage, and just
    // display what it is given.

    // // TODO
    // fn data() -> Vec<String>;
    // fn start_position() -> usize;
    // fn set_start_position(&mut self);

    // TODO
    fn move_cursor_down(&mut self);
    fn move_cursor_up(&mut self);

    fn move_cursor(&mut self, position: Position);

    fn display(&self) -> &Display;

    fn close(&self);
    fn clear(&self);

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
            
            self.on_keypress(c);
            self.display().refresh();

            c = self.display().listen_input();
        }
    }
}
