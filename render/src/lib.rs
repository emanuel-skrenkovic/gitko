#![allow(dead_code)]

use std::cmp::{Ordering};
use std::collections::HashMap;

use gitko_common::ascii_table::*;

pub type KeyHandlers<T> = HashMap<i32, fn(&mut T, &mut Window) -> bool>;
pub type ScreenFactory = fn(ScreenSize, Position) -> Box<dyn DrawScreen>;

pub struct Renderer<'a, T: Component<T>>  {
    key_handlers: KeyHandlers<T>,
    window: Window,
    component: &'a mut T
}

impl<'a, T: Component<T>> Renderer<'a, T> {
    pub fn new(
        component: &'a mut T,
        size: ScreenSize,
        position: Position,
        screen_factory: ScreenFactory) -> Renderer<'a, T> {
        Renderer {
            key_handlers: KeyHandlers::new(),
            window: Window::new(size, position, screen_factory),
            component
        }
    }

    pub fn render(&mut self) {
        self.window.clear();

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

        self.component.on_exit(&mut self.window);
    }

    pub fn draw(&mut self) {
        self.component.on_start(&mut self.window);
        self.refresh();
   }

    fn on_keypress(&mut self, c: i32) -> bool {
        if let Some(handler) = self.key_handlers.get(&c) {
            return handler(self.component, &mut self.window)
        } else {
            match c {
                KEY_J_LOWER => self.window.move_cursor_down(),
                KEY_K_LOWER => self.window.move_cursor_up(),
                KEY_Q_LOWER => return false,
                4 => { // 4 == EOT end of transmission - a very dirty hack to get ctrl + d
                    for _ in 0..20 {
                        self.window.move_cursor_down();
                    }
                },
                21 => { // 21 == NAK negative acknowledge - a very dirty hack to get ctrl + u
                    for _ in 0..20 {
                        self.window.move_cursor_up();
                    }
                }
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
    fn on_exit(&mut self, _window: &mut Window) { }

    fn register_handlers(&self, _handlers: &mut KeyHandlers<T>) { }
}

pub struct Window {
    pub lines: Vec<Line>,
    screen_start: usize,

    position: Position,
    pub cursor_position: Position,

    cursor_hidden: bool,
    screen: Box<dyn DrawScreen>
}

impl Window {
    pub fn new(size: ScreenSize, position: Position, screen_factory: ScreenFactory) -> Window {
        Window {
            lines: vec![],
            screen_start: 0,

            position: Position::default(),
            cursor_position: Position::default(),
            cursor_hidden: false,
            screen: screen_factory(size, position)
        }
    }

    pub fn set_lines(&mut self, lines: Vec<Line>) {
        self.lines = lines;
    }

    pub fn lines(&self) -> Vec<Line> {
        self.lines.clone()
    }

    fn refresh(&mut self) {
        self.screen.refresh();
    }

    fn queue_update(&mut self) {
        let lines  = self.lines.len();
        let height = self.screen.height() as usize;

        let start = self.screen_start;
        let end   = height + self.screen_start;
        let end   = if end < lines { end } else { lines };

        let data  = self.lines[start..end].to_vec();

        self.screen.set_data(data);
        self.screen.queue_update();
    }

    // TODO: think about listening for input outside of rendering methods
    fn listen_input(&self) -> i32 {
        self.screen.listen_input()
    }

    // TODO: think about listening for input outside of rendering methods
    pub fn listen(&mut self) {
        self.screen.listen()
    }

    fn resize(&mut self, new_size: ScreenSize) {
        self.screen.resize(new_size)
    }

    fn data(&self) -> Vec<String> {
        self.lines
            .iter()
            .map(|l| l.value())
            .collect()
    }

    pub fn show_cursor(&mut self, show: bool) {
        self.screen.show_cursor(show);
    }

    pub fn get_cursor_line(&self) -> String {
        self.screen.get_cursor_line()
    }

    fn set_cursor(&mut self, position: Position) {
        let current_position = self.cursor_position.y + self.screen_start as i32;

        match position.y.cmp(&current_position) {
            Ordering::Less => {
                let diff = current_position - position.y;

                for _ in 0..diff {
                    self.move_cursor_up();
                }
            }
            Ordering::Greater => {
                let diff = position.y - current_position;

                for _ in 0..diff {
                    self.move_cursor_down();
                }
            }
            _ => {
                self.move_cursor_down();
            }
        }
    }

    fn move_cursor_down(&mut self) {
        let delta = self.try_move_cursor_down();

        let next_position = self.screen_start + delta as usize;
        let next_end = next_position + self.height() as usize;

        if delta > 0 && next_end < self.lines.len() {
            self.move_screen_down(delta as usize);
        }
    }

    fn move_cursor_up(&mut self) {
        let delta = self.try_move_cursor_up();
        let delta_abs = delta.abs();

        if delta < 0 && self.screen_start as i32 - delta_abs >=0 {
            self.move_screen_up(delta_abs as usize);
        }
    }

    pub fn move_screen_down(&mut self, delta: usize) {
        if self.screen_start + delta >= self.lines.len() { return }

        self.screen_start += delta;
        self.queue_update();
    }

    pub fn move_screen_up(&mut self, delta: usize) {
        if (self.screen_start as i32 - delta as i32 ) < 0 { return; }

        self.screen_start -= delta;
        self.queue_update();
    }

    pub fn move_next(&mut self, term: &str) {
        if term.is_empty() { return }

        let start = self.cursor_position.y as usize + self.screen_start + 1;

        let next = self.lines
            .iter()
            .skip(start)
            .map(|l| l.value())
            .position(|l| l.contains(term));

        if let Some(position) = next {
            self.set_cursor(
                Position { x: 0, y: (start + position) as i32 }
            );
        }
    }

    pub fn move_prev(&mut self, term: &str) {
        if term.is_empty() { return }

        let end = self.cursor_position.y as usize;

        let prev = self.lines
            .iter()
            .take(self.screen_start + end)
            .map(|l| l.value())
            .rposition(|l| l.contains(term));

        if let Some(position) = prev {
            self.set_cursor(
                Position { x: 0, y: position as i32 }
            );
        }
    }

    fn try_move_cursor_down(&mut self) -> i32 {
        let new_position = Position {
            x: self.cursor_position.x,
            y: self.cursor_position.y + 1
        };
        let (delta, resulting_position) = self.screen.move_cursor(new_position);
        self.cursor_position = resulting_position;

        delta
    }

    fn try_move_cursor_up(&mut self) -> i32 {
        let new_position = Position {
            x: self.cursor_position.x,
            y: self.cursor_position.y - 1
        };
        let (delta, resulting_position) = self.screen.move_cursor(new_position);
        self.cursor_position = resulting_position;

        delta
    }

    pub fn height(&self) -> i32 {
        self.screen.height()
    }

    pub fn width(&self) -> i32 {
        self.screen.width()
    }

    pub fn clear(&mut self) {
        self.screen.clear();
    }
}

pub trait DrawScreen {
    fn set_data(&mut self, lines: Vec<Line>);

    fn height(&self) -> i32;
    fn width(&self) -> i32;

    fn resize(&mut self, new_size: ScreenSize);

    fn show_cursor(&mut self, show: bool);
    fn get_cursor_line(&self) -> String;

    fn queue_update(&mut self);
    fn refresh(&mut self);
    fn clear(&mut self);

    // Returns the delta between the attempted cursor
    // position move and actual end position.
    // This is the value which the data needs to be
    // scrolled by.
    // TODO: pretty confusing, need better way.
    fn move_cursor(&mut self, position: Position) -> (i32, Position);
    fn set_cursor(&mut self, position: Position);

    fn listen_input(&self) -> i32;
    fn listen(&mut self);
}

pub type RGB = (u8, u8, u8);

#[derive(Clone)]
pub enum Style {
    Underlined,
    Bold,
    Painted(RGB, RGB),
    Plain
}

#[derive(Clone)]
pub struct Part {
    pub value: String,
    pub styles: Vec<Style>
}

impl Part {
    pub fn new(value: &str, styles: Option<Vec<Style>>) -> Part {
        if let Some(s) = styles {
            return Part { value: value.to_owned(), styles: s }
        }

        Part { value: value.to_owned(), styles: vec![Style::Plain] }
    }

    pub fn plain(value: &str) -> Part {
        Part::new(value, Some(vec![Style::Plain]))
    }

    pub fn bold(value: &str) -> Part {
        Part::new(value, Some(vec![Style::Bold]))
    }

    pub fn underlined(value : &str) -> Part {
        Part::new(value, Some(vec![Style::Underlined]))
    }

    pub fn painted(value: &str, foreground: RGB, background: RGB) -> Part {
        Part::new(value, Some(vec![Style::Painted(foreground, background)]))
    }
}

#[derive(Clone)]
pub struct Line {
    pub parts: Vec<Part>
}

impl Line {
    pub fn new(parts: Vec<Part>) -> Line {
        Line { parts }
    }

    pub fn plain(value: &str) -> Line {
        Line::from_str(value, None)
    }

    pub fn empty() -> Line {
        Line::plain("")
    }

    pub fn from_string(from: String, styles: Option<Vec<Style>>) -> Line {
        if let Some(s) = styles {
            let parts = vec![Part { value: from, styles: s }];
            return Line::new(parts)
        }

        let parts = vec![Part { value: from, styles: vec![Style::Plain] }];
        Line::new(parts)
    }

    pub fn from_str(from: &str, styles: Option<Vec<Style>>) -> Line {
        Line::from_string(from.to_owned(), styles)
    }

    pub fn value(&self) -> String {
        self.parts
            .iter()
            .map(|p| p.value.clone())
            .fold(String::new(), |agg, val| agg + &val)
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn move_left(&mut self, by: i32) {
        self.x -= by;
    }

    pub fn move_right(&mut self, by: i32) {
        self.x += by;
    }

    pub fn move_up(&mut self, by: i32) {
        self.y += by;
    }

    pub fn move_down(&mut self, by: i32) {
        self.y -= by;
    }


    pub fn left(&self, by: i32) -> Position {
        Position {
            x: self.x - by,
            y: self.y
        }
    }

    pub fn right(&self, by: i32) -> Position {
        Position {
            x: self.x + by,
            y: self.y
        }
    }

    pub fn up(&self, by: i32) -> Position {
        Position {
            x: self.x,
            y: self.y + by
        }
    }

    pub fn down(&self, by: i32) -> Position {
        Position {
            x: self.x,
            y: self.y - by
        }
    }
}

#[derive(PartialEq)]
pub struct ScreenSize {
    pub lines: i32,
    pub cols: i32
}

impl ScreenSize {
    pub fn max() -> ScreenSize {
        ScreenSize { lines: 0, cols: 0 }
    }
}
