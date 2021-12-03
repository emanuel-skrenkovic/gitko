use crate::gitko::main_window::MainWindow;
use crate::render::window::Window;
use crate::render::window::ScreenSize;

mod git;
mod num;
mod render;
mod gitko;

fn main() {
    init_ncurses();

    let mut main_window = MainWindow::new(ScreenSize::max());
    main_window.render();
}

fn init_ncurses() {
    ncurses::initscr();
    ncurses::cbreak();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();   
}
