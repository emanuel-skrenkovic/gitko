use gitko_ncurses_render::{CursesScreenFactory};
use gitko_render::{Renderer, ScreenSize, Position, ScreenFactory};

use crate::gitko::main_window::MainWindow;

mod git;
mod gitko;
mod searchable;

static mut MAX_WIDTH: i32   = 0;
static mut MAX_HEIGHT: i32  = 0;
static HIGHLIGHT_COLOR: i16 = 69;

#[allow(dead_code)]
fn max_width() -> i32 {
    unsafe { MAX_WIDTH }
}

#[allow(dead_code)]
fn max_height() -> i32 {
    unsafe { MAX_HEIGHT }
}

fn screen() -> Box<dyn ScreenFactory> {
    Box::new(CursesScreenFactory::new())
}

fn main() {
    init_ncurses();

    Renderer::new(
        &mut MainWindow::new(),
        ScreenSize::max(),
        Position::default(),
        screen()
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

    ncurses::init_color(HIGHLIGHT_COLOR, 150, 150, 150);
    ncurses::init_pair(HIGHLIGHT_COLOR, ncurses::COLOR_WHITE, HIGHLIGHT_COLOR);
}
