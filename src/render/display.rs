use std::convert::TryInto;

use crate::num;
use crate::render::window::Position;
use crate::render::window::ScreenSize;
use crate::render::writeable_display::WriteableDisplay;

use crate::render::ascii_table::*;

pub struct Display {
    lines: i32,
    cols: i32,
    cursor_position: Position,
    curses_window: ncurses::WINDOW
}

impl Display {
    pub fn new(size: ScreenSize) -> Display {
        ncurses::start_color();

        let curses_window = ncurses::newwin(size.lines, size.cols, 0, 0);

        let mut y: i32 = 0;
        let mut x: i32 = 0;
        ncurses::getmaxyx(curses_window, &mut y, &mut x);

        ncurses::wmove(curses_window, 0, 0);
        ncurses::wrefresh(curses_window);

        Display {
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

    pub fn listen_input(&self) -> i32 {
        ncurses::wgetch(self.curses_window)
    }

    // region Display

    pub fn lines(&self) -> i32 {
        self.lines
    }

    pub fn cols(&self) -> i32 {
        self.cols
    }

    pub fn queue_write(&self, data: &str, position: Position) {
        // https://linux.die.net/man/3/waddstr
        ncurses::mvwaddstr(
            self.curses_window,
            position.0,
            position.1,
            data);
        ncurses::wnoutrefresh(self.curses_window);
    }

    pub fn queue_write_buffer(&self, data: &[String]) {
        ncurses::wclear(self.curses_window);

        for (i, line) in data.iter().enumerate() {
            ncurses::mvwaddstr(self.curses_window, i as i32, 0, line);
        }

        ncurses::wnoutrefresh(self.curses_window);
    }

    pub fn refresh(&self) {
        let cursor = self.cursor_position();
        ncurses::wmove(self.curses_window, cursor.0, cursor.1);
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
        // trying to go beyond edges (unless scrolling).
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

        // Move the cursor to the beginning of the line
        // to get all the characters.
        ncurses::wmove(self.curses_window, self.cursor_position.0, 0);

        let mut output: String = String::with_capacity(
            length.try_into().unwrap());
        ncurses::winnstr(
            self.curses_window,
            &mut output,
            length);

        // Move the cursor back to its original position.
        ncurses::wmove(
            self.curses_window,
            self.cursor_position.0,
            self.cursor_position.1);

        output
    }

    // endregion
}

impl Drop for Display {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

impl WriteableDisplay for Display {
    fn listen(&mut self) {
        loop {
            let c = ncurses::wgetch(self.curses_window);
            match c {
                KEY_DEL => {
                    let cursor = self.cursor_position;
                    self.move_cursor((cursor.0, cursor.1 - 1));

                    ncurses::wdelch(self.curses_window);
                }
                KEY_ETB => {
                    // TODO: delete everything and then break out.
                    self.move_cursor((0, 0));
                    self.clear();
                    self.refresh();

                    break;
                }
                KEY_LF => { break; }
                _ => {
                    ncurses::waddch(self.curses_window, c as u32);

                    let cursor = self.cursor_position;
                    self.move_cursor((cursor.0, cursor.1 + 1));
                }
            }
        }
    }
}
