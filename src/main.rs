use crate::render::{Renderer, ScreenSize, Position};
use crate::gitko::main_window::MainWindow;

mod git;
mod num;
mod render;
mod ascii_table;
mod gitko;
mod searchable;

static mut MAX_WIDTH: i32 = 0;
static mut MAX_HEIGHT: i32 = 0;

#[allow(dead_code)]
fn max_width() -> i32 {
    unsafe { MAX_WIDTH }
}

#[allow(dead_code)]
fn max_height() -> i32 {
    unsafe { MAX_HEIGHT }
}

fn main() {
    init_ncurses();

    Renderer::new(
        &mut MainWindow::new(),
        ScreenSize::max(),
        Position::default()
    ).render();
}

fn init_ncurses() {
    let base_window = ncurses::initscr();

    unsafe {
        ncurses::getmaxyx(base_window, &mut MAX_HEIGHT, &mut MAX_WIDTH);
    }

    ncurses::cbreak();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::start_color();

    ncurses::init_pair(1, ncurses::COLOR_GREEN, ncurses::COLOR_BLACK);
    ncurses::init_pair(2, ncurses::COLOR_RED, ncurses::COLOR_BLACK);
    ncurses::init_pair(3, ncurses::COLOR_CYAN, ncurses::COLOR_BLACK);
}
