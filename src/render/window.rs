use crate::num;
use crate::render::ascii_table::*;
use crate::render::Point;
use crate::render::Render;

use std::convert::TryInto;

use ncurses;

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

    pub on_activate: fn(win: &mut Window),
    pub on_key_press: fn(win: &mut Window, c: i32),

    pub color_rules: Vec<ColorRule>,

    value_buffer: Vec<String>,
    buffer: Vec<String>,

    children: Vec<Window>,

    curses_window: ncurses::WINDOW,
}

impl Window {
    pub fn new(
        position: Point,
        height: i32,
        width: i32,
        on_activate: fn(win: &mut Window),
        on_key_press: fn(win: &mut Window, c: i32),
    ) -> Window {
        let curses_window = ncurses::newwin(height, width, position.y, position.x);

        ncurses::start_color();

        ncurses::wmove(curses_window, 0, 0);
        ncurses::wrefresh(curses_window);

        Window {
            start: 0,
            height: height,
            width: width,

            cursor: Point { x: 0, y: 0 },

            marked_for_delete: false,

            color_rules: vec![],

            value_buffer: vec![],
            buffer: vec![],

            on_activate: on_activate,
            on_key_press: on_key_press,

            children: vec![],

            curses_window: curses_window,
        }
    }

    pub fn apply_color_rules(&mut self, color_rules: Vec<ColorRule>) {
        self.color_rules = color_rules;

        for (i, color_rule) in self.color_rules.iter().enumerate() {
            ncurses::init_pair((i + 1) as i16, color_rule.foreground, color_rule.background);
        }
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
        let position = Point {
            x: self.cursor.x,
            y: self.cursor.y - 1,
        };

        self.move_cursor(position);
    }

    pub fn move_cursor_down(&mut self) {
        let position = Point {
            x: self.cursor.x,
            y: self.cursor.y + 1,
        };

        self.move_cursor(position);
    }

    pub fn move_cursor_left(&mut self) {
        let position = Point {
            x: self.cursor.x - 1,
            y: self.cursor.y,
        };

        self.move_cursor(position);
    }

    pub fn move_cursor_right(&mut self) {
        let position = Point {
            x: self.cursor.x + 1,
            y: self.cursor.y,
        };

        self.move_cursor(position);
    }

    pub fn move_cursor(&mut self, position: Point) {
        let max: i32 = self.get_value().len() as i32;
        let above = position.y >= self.height && position.y <= max;

        if above {
            self.start += 1;
        }

        let below = position.y < 0 && self.start > 0;

        if below {
            self.start -= 1;
        }

        // TODO: works better without this if for some ungodly reason.
        if position.y < max {
            self.cursor = Point {
                x: num::clamp(position.x, 0, self.width),
                y: num::clamp(position.y, 0, self.height),
            };

            ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);

            if above || below {
                self.set_buffer_to_position();
            }
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
        position: Point,
        buffer: Vec<String>,
        on_activate: fn(win: &mut Window),
        on_key_press: fn(win: &mut Window, c: i32),
    ) -> &mut Window {
        let mut max_height = 0;
        let mut max_width = 0;

        // TODO: read about what this actually does
        ncurses::getmaxyx(self.curses_window, &mut max_height, &mut max_width);

        let height = buffer.len() as i32;
        let width = max_width - position.x;

        let mut child_window = Window::new(position, height, width, on_activate, on_key_press);
        child_window.set_value(buffer);

        self.children.push(child_window);

        self.children.last_mut().unwrap()
    }

    pub fn set_buffer_to_position(&mut self) {
        let max: i32 = self.get_value().len() as i32;
        let end = num::clamp(self.height + self.start as i32, self.height, max) as usize;
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
        let has_colors = !self.color_rules.is_empty();

        if has_colors {
            for (i, color_rule) in self.color_rules.iter().enumerate() {
                if (color_rule.rule)(line) {
                    ncurses::wattron(self.curses_window, ncurses::COLOR_PAIR((i + 1) as i16));
                }
            }
        }

        ncurses::waddstr(self.curses_window, line);
        ncurses::waddch(self.curses_window, KEY_LF as u32);

        if has_colors {
            for (i, color_rule) in self.color_rules.iter().enumerate() {
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

    pub fn write_at(&mut self, buffer: &Vec<String>, position: usize) {
        let mut before: Vec<String> = self.buffer[0..position + 1].to_vec();
        let mut middle: Vec<String> = vec!["".to_string(); buffer.len()].to_vec();
        let mut after: Vec<String> = self.buffer[position + 1..].to_vec();

        let mut new_buffer: Vec<String> = Vec::with_capacity(self.height as usize);
        new_buffer.append(&mut before);
        new_buffer.append(&mut middle);
        new_buffer.append(&mut after);

        self.buffer = new_buffer;
    }

    pub fn close(&self) {
        ncurses::delwin(self.curses_window);
    }
}

impl Render for Window {
    fn render(&mut self) {
        ncurses::wmove(self.curses_window, 0, 0);

        (self.on_activate)(self);

        self.refresh();

        let mut c: i32 = 0;
        while c != KEY_Q_LOWER {
            (self.on_key_press)(self, c);

            self.refresh();

            c = ncurses::wgetch(self.curses_window);
        }
    }
}
