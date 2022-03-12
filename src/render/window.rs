use std::collections::HashMap;

use crate::render::ascii_table::*;
use crate::render::display::Display;

pub type Position = (i32, i32);

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
