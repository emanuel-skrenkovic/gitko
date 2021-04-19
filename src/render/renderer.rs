use crate::git;
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
        let git_status: Vec<String> = git::run_status_command();

        let unstaged_changes: Vec<String> = git_status
            .iter()
            .cloned()
            .filter(|c| c.starts_with(" M"))
            .collect();

        let unstaged_length: i32 = unstaged_changes.len() as i32;

        let staged_changes: Vec<String> = git_status
            .iter()
            .cloned()
            .filter(|c| c.starts_with(" A"))
            .collect();

        let _unstaged_window = self
            .main_window
            .spawn_child(Point { x: 0, y: 0 }, unstaged_changes);

        let _staged_window = self.main_window.spawn_child(
            Point {
                x: 0,
                y: unstaged_length + 1,
            },
            staged_changes,
        );

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
            main_window: window::Window::new(Point { y: 0, x: 0 }, height, width),
        }
    }
}
