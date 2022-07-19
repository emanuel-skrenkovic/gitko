use gitko_crossterm::{init, exit, CrosstermScreenFactory as DrawScreenFactory, MAX_HEIGHT, MAX_WIDTH};
use gitko_render::{Renderer, ScreenSize, Position, ScreenFactory};

use crate::gitko::main_window::MainWindow;

mod git;
mod gitko;
mod searchable;

#[allow(dead_code)]
fn max_width() -> i32 {
    unsafe { MAX_WIDTH }
}

#[allow(dead_code)]
fn max_height() -> i32 {
    unsafe { MAX_HEIGHT }
}

fn screen() -> Box<dyn ScreenFactory> {
    Box::new(DrawScreenFactory::new())
}

fn main() {
    init();

    Renderer::new(
        &mut MainWindow::new(),
        ScreenSize::max(),
        Position::default(),
        screen()
    ).render();

    exit();
}


