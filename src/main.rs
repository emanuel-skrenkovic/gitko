mod git;
mod num;
mod render;

use crate::render::window2::MainWindow;
use crate::render::window2::BaseWindow;
use crate::render::window2::ScreenSize;

fn main() {
    init_curses();

    let mut main_window = MainWindow::new(ScreenSize::max());
    main_window.render();
}

fn init_curses() {
    ncurses::initscr();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();   
}
