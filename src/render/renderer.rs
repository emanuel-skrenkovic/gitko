use crate::render::main_window;
use crate::render::window;
use crate::render::Point;
use crate::render::Render;

use ncurses;

pub struct Renderer {
    main_window: window::Window,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

impl Render for Renderer {
    fn render(&mut self) {
        // do initial refresh on entire ncurses
        ncurses::refresh();

        self.main_window.render();
    }
}

impl Renderer {
    pub fn new(height: i32, width: i32) -> Renderer {
        ncurses::initscr();
        ncurses::raw();

        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::noecho();

        Renderer {
            main_window: window::Window::new(
                Point { y: 0, x: 0 },
                height,
                width,
                main_window::on_activate,
                main_window::on_key_press,
            ),
        }
    }
}
