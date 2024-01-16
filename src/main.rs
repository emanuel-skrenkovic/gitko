use gitko_render::{Renderer, ScreenSize, Position, ScreenFactory};

mod git;
mod gitko;
mod searchable;

fn main() {
    #[allow(dead_code)]
    #[cfg(feature = "ncurses")] {
        use gitko_ncurses_render::{init, exit};
        use crate::gitko::main_window::MainWindow;

        init();

        Renderer::new(
            &mut MainWindow::new(),
            ScreenSize::max(),
            Position::default(),
            screen()
        ).render();

        exit();
    }

    #[allow(dead_code)]
    #[cfg(feature = "crossterm")] {
        use gitko_crossterm::{init, exit};
        use crate::gitko::main_window::MainWindow;

        init();

        Renderer::new(
            &mut MainWindow::new(),
            ScreenSize::max(),
            Position::default(),
            screen()
        ).render();

        exit();
    }
}

fn max_width() -> i32 {
    #[allow(dead_code)]
    #[cfg(feature = "ncurses")] {
        use gitko_ncurses_render::MAX_WIDTH;
        unsafe { return MAX_WIDTH }
    }
    #[allow(dead_code)]
    #[cfg(feature = "crossterm")] {
        use gitko_crossterm::MAX_WIDTH;
        unsafe { MAX_WIDTH }
    }
}

#[allow(dead_code)]
fn max_height() -> i32 {
    #[allow(dead_code)]
    #[cfg(feature = "ncurses")] {
        use gitko_ncurses_render::MAX_HEIGHT;
        unsafe { return MAX_HEIGHT }
    }

    #[allow(dead_code)]
    #[cfg(feature = "crossterm")] {
        use gitko_crossterm::MAX_HEIGHT;
        unsafe { MAX_HEIGHT }
    }
}

fn screen() -> ScreenFactory {
    #[allow(dead_code)]
    #[cfg(feature = "ncurses")] {
        use gitko_ncurses_render::screen_factory;
        return screen_factory
    }

    #[allow(dead_code)]
    #[cfg(feature = "crossterm")] {
        use gitko_crossterm::screen_factory;
        screen_factory
    }
}
