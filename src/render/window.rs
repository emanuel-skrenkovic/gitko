use crate::num;
use crate::render::ascii_table::*;

use std::convert::TryInto;

pub type Position = (i32, i32);

pub struct ScreenSize {
    lines: i32,
    cols: i32
}

impl ScreenSize {
    pub fn max() -> ScreenSize {
        ScreenSize {
            lines: 0,
            cols: 0
        }
    }
}

pub trait BaseWindow {
    fn on_keypress(&mut self, c: i32);
    fn on_activate(&mut self);

    fn cursor_position(&self) -> Position;

    // TODO: scrolling functionality

    // I think it needs to be here, rather than the Window struct impl.
    // Window struct should not know about the data storage, and just
    // display what it is given.

    fn move_cursor_down(&mut self);

    fn move_cursor_up(&mut self);

    fn move_cursor(&mut self, position: Position);

    fn window(&self) -> ncurses::WINDOW;

    fn close(&self);
    fn clear(&self);

    fn render_child<T>(&mut self, mut child: T) where T : BaseWindow {
        // TODO: seems to work for now. Might be busted.
        // Take a closer look.
        child.render();
        child.close();
        self.clear();
        self.on_activate();
    }

    fn render(&mut self) {
        let win = self.window();
        ncurses::wmove(win, 0, 0);

        self.on_activate();

        let mut c: i32 = 0;
        while c != KEY_Q_LOWER {
            // TODO: two updates per keypress for now.
            // Need to understand better.
            ncurses::wmove(win, self.cursor_position().0, self.cursor_position().1);
            ncurses::doupdate();

            // TODO: move cursor here. on_keypress
            // is for custom functionality.

            match c {
                KEY_J_LOWER => { self.move_cursor_down(); }
                KEY_K_LOWER => { self.move_cursor_up(); }
                KEY_Q_LOWER => { self.close(); }
                _ => {}
            }
            
            self.on_keypress(c);
            ncurses::doupdate();
            ncurses::wmove(win, self.cursor_position().0, self.cursor_position().1);
            c = ncurses::wgetch(win);
        }
    }
}

pub struct Window {
    lines: i32,
    cols: i32,
    cursor_position: Position,
    pub curses_window: ncurses::WINDOW
}

impl Window {
    pub fn new(size: ScreenSize) -> Window {
        ncurses::start_color();

        let curses_window = ncurses::newwin(size.lines, size.cols, 0, 0);

        let mut y: i32 = 0;
        let mut x: i32 = 0;
        ncurses::getmaxyx(curses_window, &mut y, &mut x);

        ncurses::wrefresh(curses_window);

        Window {
            lines: y,
            cols: x,
            cursor_position: (0, 0),
            curses_window
        }
    }

    pub fn close(&self) {
        ncurses::wclear(self.curses_window);
        ncurses::delwin(self.curses_window);
    }

    // region Display

    pub fn lines(&self) -> i32 {
        self.lines
    }

    pub fn cols(&self) -> i32 {
        self.cols
    }

    pub fn queue_write(&self, data: &String, position: Position) {
        // https://linux.die.net/man/3/waddstr
        ncurses::mvwaddstr(
            self.curses_window,
            position.0,
            position.1,
            &data);
        ncurses::wnoutrefresh(self.curses_window);
    }

    pub fn queue_write_buffer(&self, data: &Vec<String>) {
        ncurses::wclear(self.curses_window);

        for (i, line) in data.iter().enumerate() {
            ncurses::mvwaddstr(self.curses_window, i as i32, 0, &line);
        }

        ncurses::wnoutrefresh(self.curses_window);
    }

    fn refresh(&self) {
        ncurses::doupdate();
    }

    pub fn clear(&self) {
        ncurses::wclear(self.curses_window);
        ncurses::doupdate();
    }

    // endregion

    // region Cursor

    pub fn cursor_position(&self) -> Position {
        self.cursor_position
    }

    pub fn try_move_cursor_down(&mut self) -> i32 {
        self.move_cursor(
            (self.cursor_position.0 + 1, self.cursor_position.1))
    }

    pub fn try_move_cursor_up(&mut self) -> i32 {
        self.move_cursor(
            (self.cursor_position.0 - 1, self.cursor_position.1))
    }

    // Returns the delta between the attempted cursor
    // position move and actual end position.
    // This is the value which the data needs to be
    // scrolled by.
    // TODO: pretty confusing, need better way.
    pub fn move_cursor(&mut self, position: Position) -> i32 {
        // TODO: optimize by not doing anything when
        // trying to go beyong edges (unless scrolling).
        let y = num::clamp(position.0, 0, self.lines() - 1);
        let x = num::clamp(position.1, 0, self.cols() - 1);

        let delta = position.0 - y;
        
        ncurses::wmove(self.curses_window, y, x);
        self.cursor_position = (y, x);

        delta
    }

    // endregion

    // region Data

    pub fn get_cursor_line_data(&self) -> String {
        let length = self.cols();

        let mut output: String = String::with_capacity(
            length.try_into().unwrap());
        ncurses::winnstr(
            self.curses_window,
            &mut output,
            length);

        output
    }

    // endregion
}

impl Drop for Window {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
