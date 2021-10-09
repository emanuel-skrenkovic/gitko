use crate::render::main_window;
use crate::render::window::Window;
use crate::render::Point;
use crate::render::Render;

pub struct Renderer {
    main_window: Window,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

impl Render for Renderer {
    fn render(&mut self){
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

        let mut main_window = Window::new(
            height,
            width,
            main_window::on_activate,
            main_window::on_key_press,
        );
        main_window.position(Point { x: 0, y: 0 });

        Renderer { main_window }
    }
}
