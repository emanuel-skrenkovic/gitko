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

pub type KeyHandlers<T> = HashMap<i32, fn(&mut T, &mut Window) -> bool>;

pub struct Renderer<'a, T: Component<T>>  {
    key_handlers: KeyHandlers<T>,
    window: Window,
    component: &'a mut T
}

impl<'a, T: Component<T>> Renderer<'a, T> {
    pub fn new(component: &'a mut T, size: ScreenSize, position: Position) -> Renderer<'a, T> {
        Renderer {
            key_handlers: KeyHandlers::new(),
            window: Window::new(size, position),
            component
        }
    }

    pub fn render(&mut self) {
        let component = &mut self.component;
        component.register_handlers(&mut self.key_handlers);
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
        self.component.on_start(&mut self.window);
        self.window.queue_update();
    }

    fn on_keypress(&mut self, c: i32) -> bool {
        if let Some(handler) = self.key_handlers.get(&c) {
            return handler(&mut self.component, &mut self.window)
        } else {
            match c {
                KEY_J_LOWER => { self.window.move_cursor_down(); }
                KEY_K_LOWER => { self.window.move_cursor_up(); }
                KEY_Q_LOWER => { return false }
                _ => {}
            }
        }

        true
    }

    fn refresh(&mut self) {
        self.window.queue_update();
        self.window.refresh();
    }
}

pub trait Component<T: Component<T>> {
    fn on_start(&mut self, _window: &mut Window) { }
    fn on_render(&mut self, _window: &mut Window) -> bool { true }

    fn register_handlers(&self, _handlers: &mut KeyHandlers<T>) { }
}

const GREEN_TEXT: i16 = 1;
const RED_TEXT: i16   = 2;
const BLUE_TEXT: i16  = 3;

// TODO: think about removing and adding functionality to Component trait
pub struct Window {
    pub data: Vec<String>,
    screen_start: usize,

    height: i32,
    width: i32,

    cursor_position: Position,
    curses_window: ncurses::WINDOW
}

impl Window {
    pub fn new(size: ScreenSize, position: Position) -> Window {
        let curses_window = ncurses::newwin(size.lines,
                                            size.cols,
                                            position.y,
                                            position.x);

        let mut y: i32 = 0;
        let mut x: i32 = 0;
        ncurses::getmaxyx(curses_window, &mut y, &mut x);

        ncurses::wmove(curses_window, 0, 0);
        ncurses::wrefresh(curses_window);

        Window {
            data: vec![],
            screen_start: 0,

            height: y,
            width: x,

            cursor_position: Position::default(),
            curses_window
        }
    }

    pub fn refresh(&self) {
        ncurses::wmove(self.curses_window,
                       self.cursor_position.y,
                       self.cursor_position.x);
        ncurses::doupdate();
    }

    pub fn queue_update(&self) {
        ncurses::werase(self.curses_window);

        for (i, line) in self.data[self.screen_start..].iter().enumerate() {
            self.write_line(
                line,
                Position { x: 0, y: i as i32 }
            );
        }

        ncurses::wnoutrefresh(self.curses_window);
    }

    pub fn listen_input(&self) -> i32 {
        ncurses::wgetch(self.curses_window)

    }

    pub fn resize(&mut self, new_size: ScreenSize) {
        ncurses::wresize(self.curses_window, new_size.lines, new_size.cols);
        self.height = new_size.lines;
        self.width = new_size.cols;
    }

    pub fn get_cursor_line(&self) -> String {
        // Move the cursor to the beginning of the line
        // to get all the characters.
        let move_cursor = self.cursor_position.x != 0;
        if move_cursor {
            ncurses::wmove(self.curses_window, self.cursor_position.y, 0);
        }

        let length = self.width();
        let mut output = String::with_capacity(length.try_into().unwrap());
        ncurses::winnstr(
            self.curses_window,
            &mut output,
            length);

        // Move the cursor back to its original position.
        if move_cursor {
            ncurses::wmove(
                self.curses_window,
                self.cursor_position.y,
                self.cursor_position.x);
        }

        output
    }

    pub fn move_cursor_down(&mut self) {
        let delta = self.try_move_cursor_down();

        let next_position = self.screen_start + delta as usize;
        let next_end = next_position + self.height as usize;

        if delta > 0 && next_end < self.data.len() {
            self.move_screen_down(delta as usize);
        }
    }

    pub fn move_cursor_up(&mut self) {
        let delta = self.try_move_cursor_up();
        let delta_abs = delta.abs();

        if delta < 0 && self.screen_start as i32 - delta_abs >=0 {
            self.move_screen_up(delta_abs as usize);
        }
    }

    pub fn move_screen_down(&mut self, delta: usize) {
        if self.screen_start + delta >= self.data.len() { return; }

        self.screen_start += delta;
        self.queue_update();
    }

    pub fn move_screen_up(&mut self, delta: usize) {
        if (self.screen_start as i32 - delta as i32 ) < 0 { return; }

        self.screen_start -= delta;
        self.queue_update();
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
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

    fn try_move_cursor_down(&mut self) -> i32 {
        self.move_cursor(Position {
            x: self.cursor_position.x,
            y: self.cursor_position.y + 1
        })
    }

    fn try_move_cursor_up(&mut self) -> i32 {
        self.move_cursor(Position {
            x: self.cursor_position.x,
            y: self.cursor_position.y - 1
        })
    }

    // Returns the delta between the attempted cursor
    // position move and actual end position.
    // This is the value which the data needs to be
    // scrolled by.
    // TODO: pretty confusing, need better way.
    fn move_cursor(&mut self, position: Position) -> i32 {
        // TODO: optimize by not doing anything when
        // trying to go beyond edges (unless scrolling).
        let y = num::clamp(position.y, 0, self.height - 1);
        let x = num::clamp(position.x, 0, self.width - 1);

        let delta = position.y - y;

        ncurses::wmove(self.curses_window, y, x);
        self.cursor_position = Position { x, y };

        delta
    }

    pub fn clear(&self) {
        ncurses::werase(self.curses_window);
        ncurses::doupdate();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
        ncurses::endwin();
    }
}

pub trait WriteableWindow {
    fn as_writeable(&self) -> &dyn WriteableWindow
        where Self : Sized
    {
        self
    }

    fn as_writeable_mut(&mut self) -> &mut dyn WriteableWindow
        where Self : Sized
    {
        self
    }

    fn listen(&mut self);
}

impl WriteableWindow for Window {
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
