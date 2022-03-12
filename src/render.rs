use std::convert::TryInto;
use std::collections::HashMap;

use crate::num;
use crate::ascii_table::*;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn default() -> Position {
        Position { x: 0, y: 0 }
    }
}

pub struct ScreenSize {
    pub lines: i32,
    pub cols: i32
}

impl ScreenSize {
    pub fn max() -> ScreenSize {
        ScreenSize { lines: 0, cols: 0 }
    }
}

pub struct Renderer<T: Component<T>> {
    window: Window<T>,
    component: T
}

impl<T: Component<T>> Renderer<T> {
    pub fn new(component: T, size: ScreenSize, position: Position) -> Renderer<T> {
        Renderer {
            window: Window::new(size, position),
            component
        }
    }

    pub fn render(&mut self) {
        let component = &mut self.component;
        component.register_handlers(&mut self.window);
        component.on_start(&mut self.window);

        self.refresh();

        let mut c: i32 = 0;
        loop {
            if !self.on_keypress(c) { break; }

            self.refresh();
            if !self.component.on_render(&mut self.window) { break; }

            c = self.window.listen_input();
        }
    }

    pub fn draw(&mut self) {
        let component = &mut self.component;
        let window = &mut self.window;

        component.on_start(window);
        window.queue_write(component.data());
        window.display.draw();
    }

    fn on_keypress(&mut self, c: i32) -> bool {
        if let Some(handler) = self.window.key_handlers.get(&c) {
            return handler(&mut self.component, &mut self.window)
        } else {
            match c {
                KEY_J_LOWER => { self.window.move_cursor_down(self.component.data()); }
                KEY_K_LOWER => { self.window.move_cursor_up(self.component.data()); }
                KEY_Q_LOWER => { return false }
                _ => {}
            }
        }

        true
    }

    fn refresh(&mut self) {
        self.window.queue_write(self.component.data());
        self.window.refresh();
    }
}

pub trait Component<T: Component<T>> {
    fn on_start(&mut self, _window: &mut Window<T>) { }
    fn on_render(&mut self, _window: &mut Window<T>) -> bool { true }

    fn data(&self) -> &[String];
    fn register_handlers(&self, _window: &mut Window<T>) { }
}

// TODO: think about removing and adding functionality to Component trait
pub struct Window<T: Component<T>> {
    pub key_handlers: HashMap<i32, fn(&mut T, &mut Self) -> bool>,
    pub display: Display,

    screen_start: usize
}

impl<T> Window<T> where T: Component<T> {
    pub fn new(size: ScreenSize, position: Position) -> Window<T> {
        Window {
            key_handlers: HashMap::new(),
            display: Display::new(size, position),
            screen_start: 0
        }
    }

    pub fn register_handler(&mut self, key: i32, handler: fn(&mut T, &mut Self) -> bool) {
        self.key_handlers.insert(key, handler);
    }

    pub fn refresh(&self) {
        self.display.refresh();
    }

    pub fn queue_write(&self, data: &[String]) {
        self.display.queue_write_buffer(&data[self.screen_start..])
    }

    pub fn listen_input(&self) -> i32 {
        self.display.listen_input()
    }

    pub fn resize(&mut self, new_size: ScreenSize) {
        self.display.resize(new_size);
    }

    pub fn get_cursor_line(&self) -> String {
        self.display.get_cursor_line_data()
    }

    pub fn move_cursor_down(&mut self, data: &[String]) {
        let delta = self.display.try_move_cursor_down();

        let next_position = self.screen_start + delta as usize;
        let next_end = next_position + self.display.lines() as usize;

        if delta > 0 && next_end < data.len() {
            self.move_screen_down(data, delta as usize);
        }
    }

    pub fn move_cursor_up(&mut self, data: &[String]) {
        let delta = self.display.try_move_cursor_up();
        let delta_abs = delta.abs();

        if delta < 0 && self.screen_start as i32 - delta_abs >=0 {
            self.move_screen_up(data, delta_abs as usize);

        }
    }

    pub fn move_screen_down(&mut self, data: &[String], delta: usize) {
        if self.screen_start + delta >= data.len() { return; }

        self.screen_start += delta;
        self.display.queue_write_buffer(&data[self.screen_start..]);
    }

    pub fn move_screen_up(&mut self, data: &[String], delta: usize) {
        if (self.screen_start as i32 - delta as i32 ) < 0 { return; }

        self.screen_start -= delta;
        self.display.queue_write_buffer(&data[self.screen_start..]);
    }

    pub fn height(&self) -> i32 {
        self.display.lines()
    }

    pub fn width(&self) -> i32 {
        self.display.cols()
    }
}

const GREEN_TEXT: i16 = 1;
const RED_TEXT: i16   = 2;
const BLUE_TEXT: i16  = 3;

pub struct Display {
    lines: i32,
    cols: i32,
    cursor_position: Position,
    curses_window: ncurses::WINDOW
}

impl Display {
    pub fn new(size: ScreenSize, position: Position) -> Display {
        let curses_window = ncurses::newwin(size.lines,
                                            size.cols,
                                            position.y,
                                            position.x);

        let mut y: i32 = 0;
        let mut x: i32 = 0;
        ncurses::getmaxyx(curses_window, &mut y, &mut x);

        ncurses::wmove(curses_window, 0, 0);
        ncurses::wrefresh(curses_window);

        Display {
            lines: y,
            cols: x,
            cursor_position: Position::default(),
            curses_window
        }
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

    pub fn resize(&mut self, size: ScreenSize) {
        ncurses::wresize(self.curses_window, size.lines, size.cols);
        self.lines = size.lines;
        self.cols = size.cols;
    }

    pub fn queue_write_buffer(&self, data: &[String]) {
        ncurses::werase(self.curses_window);

        for (i, line) in data.iter().enumerate() {
            self.write_line(line, Position { x: 0, y: i as i32 });
        }

        ncurses::wnoutrefresh(self.curses_window);
    }

    fn write_line(&self, line: &str, position: Position) {
        // Ugly, but more control.
        let color: Option<i16> =
            if line.starts_with("+++") || line.starts_with("---") {
                None
            } else if line.starts_with('+') {
                Some(GREEN_TEXT)
            } else if line.starts_with('-') {
                Some(RED_TEXT)
            } else if line.starts_with("@@") {
                Some(BLUE_TEXT)
            } else {
                None
            };

        let color_on = color.is_some();
        if color_on {
            ncurses::wattron(
                self.curses_window,
                ncurses::COLOR_PAIR(color.unwrap()));
        }

        // https://linux.die.net/man/3/waddstr
        ncurses::mvwaddstr(
            self.curses_window,
            position.y,
            position.x,
            line);

        if color_on {
            ncurses::wattroff(
                self.curses_window,
                ncurses::COLOR_PAIR(color.unwrap()));
        }
    }

    pub fn refresh(&self) {
        let cursor = self.cursor_position();
        ncurses::wmove(self.curses_window, cursor.y, cursor.x);
        ncurses::doupdate();
    }

    pub fn draw(&self) {
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        ncurses::doupdate();
    }

    pub fn clear(&self) {
        ncurses::werase(self.curses_window);
        ncurses::doupdate();
    }

    // endregion

    // region Cursor

    pub fn cursor_position(&self) -> Position {
        self.cursor_position
    }

    pub fn try_move_cursor_down(&mut self) -> i32 {
        self.move_cursor(
            Position { x: self.cursor_position.x, y: self.cursor_position.y + 1 })
    }

    pub fn try_move_cursor_up(&mut self) -> i32 {
        self.move_cursor(
            Position { x: self.cursor_position.x, y: self.cursor_position.y - 1 })
    }

    // Returns the delta between the attempted cursor
    // position move and actual end position.
    // This is the value which the data needs to be
    // scrolled by.
    // TODO: pretty confusing, need better way.
    pub fn move_cursor(&mut self, position: Position) -> i32 {
        // TODO: optimize by not doing anything when
        // trying to go beyond edges (unless scrolling).
        let y = num::clamp(position.y, 0, self.lines() - 1);
        let x = num::clamp(position.x, 0, self.cols() - 1);

        let delta = position.y - y;

        ncurses::wmove(self.curses_window, y, x);
        self.cursor_position = Position { x, y };

        delta
    }

    // endregion

    // region Data

    pub fn get_cursor_line_data(&self) -> String {
        let length = self.cols();

        // Move the cursor to the beginning of the line
        // to get all the characters.
        ncurses::wmove(self.curses_window, self.cursor_position.y, 0);

        let mut output: String = String::with_capacity(
            length.try_into().unwrap());
        ncurses::winnstr(
            self.curses_window,
            &mut output,
            length);

        // Move the cursor back to its original position.
        ncurses::wmove(
            self.curses_window,
            self.cursor_position.y,
            self.cursor_position.x);

        output
    }

    // endregion
}

impl Drop for Display {
    fn drop(&mut self) {
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
        ncurses::endwin();
    }
}

pub trait WriteableDisplay {
    fn as_writeable(&self) -> &dyn WriteableDisplay
        where Self : Sized
    {
        self
    }

    fn as_writeable_mut(&mut self) -> &mut dyn WriteableDisplay
        where Self : Sized
    {
        self
    }

    fn listen(&mut self);
}

impl WriteableDisplay for Display {
    fn listen(&mut self) {
        loop {
            let c = ncurses::wgetch(self.curses_window);
            match c {
                KEY_DEL => {
                    let cursor = self.cursor_position;
                    self.move_cursor(Position { x: cursor.x - 1, y: cursor.y });

                    ncurses::wdelch(self.curses_window);
                }
                KEY_ETB => {
                    self.clear();
                    break;
                }
                KEY_LF => { break; }
                _ => {
                    ncurses::waddch(self.curses_window, c as u32);

                    let cursor = self.cursor_position;
                    self.move_cursor(Position { x: cursor.x + 1, y: cursor.y });

                }
            }
        }
    }
}
