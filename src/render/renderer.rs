use crate::git;
use crate::render::window;
use crate::render::Render;
use crate::render::ascii_table::*;

use ncurses;

pub struct Renderer {
    main_window: window::Window,
    buffer: Vec<String>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

impl Render for Renderer {
    fn render(&mut self) {
        let mut git_status: Vec<String> = git::run_status_command();
        self.main_window.buffer.append(&mut git_status);

        self.main_window.render();
    }
}

impl Renderer {
    pub fn new(height: i32, width: i32) -> Renderer {
        ncurses::initscr();
        ncurses::raw();

        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::noecho();

        // curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        Renderer {
            main_window: window::Window::new(0, 0, height, width),
            buffer: Vec::with_capacity(height as usize),
        }
    }
}

fn create_win(start_y: i32, start_x: i32, height: i32, width: i32) -> ncurses::WINDOW {
    let win = ncurses::newwin(height, width, start_y, start_x);
    ncurses::box_(win, 0, 0);

    ncurses::wrefresh(win);
    win
}

fn destroy_win(win: ncurses::WINDOW) {
    let ch = ' ' as ncurses::chtype;
    ncurses::wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    ncurses::wrefresh(win);

    ncurses::delwin(win);
}
