use crate::num;
use crate::render::ascii_table::*;
use crate::render::Point;
use crate::render::Render;

use std::convert::TryInto;

pub struct Style {
    pub color_rules: Vec<ColorRule>,
}

pub struct ColorRule {
    pub foreground: i16,
    pub background: i16,
    pub rule: fn(&str) -> bool,
}

pub struct Window {
    pub start: usize,
    pub height: i32,
    pub width: i32,

    pub cursor: Point,

    pub marked_for_delete: bool,

    pub style: Option<Style>,

    on_activate_fn: fn(win: &mut Window),
    on_key_press_fn: fn(win: &mut Window, c: i32),

    value_buffer: Vec<String>,
    buffer: Vec<String>,

    children: Vec<Window>,

    curses_window: ncurses::WINDOW,
}

impl Window {
    pub fn new(
        height: i32,
        width: i32,
        on_activate: fn(win: &mut Window),
        on_key_press: fn(win: &mut Window, c: i32),
    ) -> Window {
        ncurses::start_color();

        let curses_window = ncurses::newwin(height, width, 0, 0);
        ncurses::wrefresh(curses_window);

        Window {
            start: 0,
            height,
            width,

            cursor: Point { x: 0, y: 0 },

            marked_for_delete: false,

            style: None,

            value_buffer: vec![],
            buffer: vec![], // TODO: rename to display_buffer or something similar

            on_activate_fn: on_activate,
            on_key_press_fn: on_key_press,

            children: vec![],

            curses_window,
        }
    }

    pub fn position(&mut self, coords: Point) -> &mut Self {
        ncurses::mvderwin(self.curses_window, coords.y, coords.x);
        // ncurses::mvwin(self.curses_window, coords.y, coords.x);
        // ncurses::wrefresh(self.curses_window);
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Self {
        self.style = Some(style);

        let style = self.style.as_ref().unwrap();

        for (i, color_rule) in style.color_rules.iter().enumerate() {
            ncurses::init_pair((i + 1) as i16, color_rule.foreground, color_rule.background);
        }

        self
    }

    pub fn set_value(&mut self, value: Vec<String>) {
        self.value_buffer = value;
    }

    pub fn get_value(&self) -> &Vec<String> {
        &self.value_buffer
    }

    pub fn is_empty(&self) -> bool {
        self.get_value().is_empty()
    }

    pub fn update_value_at(&mut self, index: usize, new_value: String) {
        assert!(index < self.value_buffer.len());
        self.value_buffer[index] = new_value;
    }

    pub fn value_at(&self, index: usize) -> String {
        assert!(index < self.get_value().len());
        self.buffer[index].clone()
    }

    pub fn line_at(&self, line_number: usize) -> String {
        assert!(line_number < self.height.try_into().unwrap());
        self.buffer[line_number].clone()
    }

    pub fn move_cursor_up(&mut self) {
        self.move_cursor(Point {
            x: self.cursor.x,
            y: self.cursor.y - 1
        });
    }

    pub fn move_cursor_up_n(&mut self, n: u32) {
        self.move_cursor(Point {
            x: self.cursor.x,
            y: self.cursor.y - n as i32
        });
    }

    pub fn move_cursor_down(&mut self) {
        self.move_cursor(Point {
            x: self.cursor.x,
            y: self.cursor.y + self.start as i32 + 1,
        });
    }

    pub fn move_cursor_down_n(&mut self, n: u32) {
        self.move_cursor(Point {
            x: self.cursor.x,
            y: self.cursor.y + n as i32
        });
    }

    pub fn move_cursor_left(&mut self) {
        self.move_cursor(Point {
            x: self.cursor.x - 1,
            y: self.cursor.y,
        });
    }

    pub fn move_cursor_right(&mut self) {
        self.move_cursor(Point {
            x: self.cursor.x + 1,
            y: self.cursor.y,
        });
    }

    pub fn move_cursor(&mut self, position: Point) {
        let new_end = position.y as usize;

        if position.y < self.start as i32 { // move up
            let diff = (self.start as i32 - (self.start as i32 - position.y)).abs();
            self.start = if (self.start as i32) - diff >= 0 { self.start - (diff as usize) } else { 0 };

            self.set_buffer_to_position();
        } else if position.y > self.start as i32 && new_end > self.height as usize - 1 { // move down
            self.start = (self.start as i32 +
                         (position.y - self.height - 1).abs()) as usize;
            self.set_buffer_to_position();
        } else {
            self.cursor = Point {
                x: position.x,
                y: position.y
            };
        }
    }

    pub fn get_cursor_line(&self) -> &str {
        let line_number = self.cursor.y as usize;

        // TODO: terrible, not sure if even needed
        if line_number > self.buffer.len() {
            &""
        } else {
            &self.buffer[line_number]
        }
    }

    pub fn spawn_child(
        &mut self,
        buffer: Vec<String>,
        on_activate: fn(win: &mut Window),
        on_key_press: fn(win: &mut Window, c: i32),
    ) -> &mut Window {
        let mut max_height = 0;
        let mut max_width = 0;

        // TODO: read about what this actually does
        ncurses::getmaxyx(self.curses_window, &mut max_height, &mut max_width);

        let height = buffer.len() as i32;
        let width = max_width;

        let mut child_window = Window::new(height, width, on_activate, on_key_press);
        child_window.set_value(buffer);
        self.children.push(child_window);

        self.children.last_mut().unwrap()
    }

    pub fn set_buffer_to_position(&mut self) {
        let end = num::clamp(self.height + self.start as i32,
                             self.height,
                             self.get_value().len() as i32) as usize;

        self.buffer = self.get_value()[self.start..end].to_vec();
    }

    pub fn clear_buffer(&mut self) {
        self.buffer = Vec::with_capacity(self.height as usize);
    }

    pub fn write_buffer(&mut self) {
        ncurses::wclear(self.curses_window);

        for line in self.buffer.iter() {
            self.write_line(line);
        }

        // Needs to be here because of the clear above.
        ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);
    }

    pub fn write_line(&self, line: &str) {
        let is_styled = self.style.is_some();

        if is_styled {
            let style = self.style.as_ref().unwrap();
            for (i, color_rule) in style.color_rules.iter().enumerate() {
                if (color_rule.rule)(line) {
                    ncurses::wattron(self.curses_window, ncurses::COLOR_PAIR((i + 1) as i16));
                }
            }
        }

        ncurses::waddstr(self.curses_window, line);
        ncurses::waddch(self.curses_window, KEY_LF as u32);

        if is_styled {
            let style = self.style.as_ref().unwrap();
            for (i, color_rule) in style.color_rules.iter().enumerate() {
                if (color_rule.rule)(line) {
                    ncurses::wattroff(self.curses_window, ncurses::COLOR_PAIR((i + 1) as i16));
                }
            }
        }
    }

    pub fn queue_update(&mut self) {
        if !self.children.is_empty() {
            for child in self
                .children
                .iter()
                .filter(|&child| child.marked_for_delete)
            {
                child.close(); // frees the resources used by ncurses
            }

            // remove the deleted windows from children vec
            self.children.retain(|c| !c.marked_for_delete);
        }

        self.write_buffer();

        ncurses::wnoutrefresh(self.curses_window);
    }

    pub fn refresh(&mut self) {
        self.set_buffer_to_position();
        self.queue_update();
        ncurses::doupdate();
    }

    pub fn write_at(&mut self, buffer: &[String], position: usize) {
        let mut new_buffer: Vec<String> = Vec::with_capacity(self.height as usize);

        let mut before: Vec<String> = self.buffer[0..position + 1].to_vec();
        new_buffer.append(&mut before);

        let mut middle: Vec<String> = vec!["".to_string(); buffer.len()].to_vec();
        new_buffer.append(&mut middle);

        let mut after: Vec<String> = self.buffer[position + 1..].to_vec();
        new_buffer.append(&mut after);

        self.buffer = new_buffer;
    }

    pub fn close(&self) {
        ncurses::delwin(self.curses_window);
    }

    pub fn on_activate(&mut self) {
        (self.on_activate_fn)(self);
    }

    pub fn on_key_press(&mut self, c: i32) {
        (self.on_key_press_fn)(self, c);
    }
}

impl Render for Window {
    fn render(&mut self) {
        ncurses::wmove(self.curses_window, 0, 0);

        self.on_activate();

        self.refresh();

        let mut c: i32 = 0;
        while c != KEY_Q_LOWER {
            self.on_key_press(c);

            self.refresh();

            c = ncurses::wgetch(self.curses_window);
        }
    }
}
