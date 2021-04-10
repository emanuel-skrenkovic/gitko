use crate::render::window;
use crate::render::Render;

pub struct Renderer {
    main_window: window::Window,
}

impl Render for Renderer {
    fn render(&mut self) {
        // TODO: every window should render its own content
        // (maybe implement gitrunner trait or something)

        // in renderer only ncurses stuff should be handled
        self.main_window.render()
    }
}

impl Renderer {
    pub fn new(height: i32, width: i32) -> Renderer {
        Renderer {
            main_window: window::Window::new(height, width),
        }
    }
}
