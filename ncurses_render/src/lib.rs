use std::convert::TryInto;

use gitko_render::{Line, DrawScreen, ScreenSize, Position, Style, ScreenFactory};

/*
const GREEN_TEXT: i16 = 1;
const RED_TEXT: i16   = 2;
const BLUE_TEXT: i16  = 3;
*/
static HIGHLIGHT_COLOR: i16 = 69;

// TODO: move elsewhere
pub fn clamp(num: i32, min: i32, max: i32) -> i32 {
    if num < min {
        return min;
    } else if num > max {
        return max;
    }

    num
}

// TODO: think about removing and adding functionality to Component trait
pub struct CursesWindow {
    pub lines: Vec<Line>,

    height: i32,
    width: i32,

    position: Position,
    pub cursor_position: Position,
    cursor_hidden: bool,
    curses_window: ncurses::WINDOW
}

impl CursesWindow {
    pub fn new(size: ScreenSize, position: Position) -> CursesWindow {
        let curses_window = ncurses::newwin(size.lines,
                                            size.cols,
                                            position.y,
                                            position.x);

        let mut y: i32 = 0;
        let mut x: i32 = 0;
        ncurses::getmaxyx(curses_window, &mut y, &mut x);

        ncurses::wmove(curses_window, 0, 0);
        ncurses::wrefresh(curses_window);

        CursesWindow {
            lines: vec![],

            height: y,
            width: x,

            position: Position::default(),
            cursor_position: Position::default(),
            cursor_hidden: false,
            curses_window
        }
    }
}

impl DrawScreen for CursesWindow {
    fn set_data(&mut self, lines: Vec<Line>) {
        self.lines = lines;
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn resize(&mut self, new_size: ScreenSize) {
        ncurses::wresize(self.curses_window, new_size.lines, new_size.cols);
        self.height = new_size.lines;
        self.width = new_size.cols;
    }

    fn show_cursor(&mut self, show: bool) {
        if show {
            ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
            self.cursor_hidden = false;
        } else {
            ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
            self.cursor_hidden = true;
        }
    }

    fn get_cursor_line(&self) -> String {
        // Move the cursor to the beginning of the line
        // to get all the characters.
        let move_cursor = self.cursor_position.x != 0;
        if move_cursor {
            ncurses::wmove(self.curses_window, self.cursor_position.y, 0);
        }

        let length = self.width();
        let mut output = String::with_capacity(length.try_into().unwrap());
        ncurses::winnstr(
            self.curses_window,
            &mut output,
            length);

        // Move the cursor back to its original position.
        if move_cursor {
            ncurses::wmove(
                self.curses_window,
                self.cursor_position.y,
                self.cursor_position.x);
        }

        output
    }

    fn queue_update(&mut self) {
        ncurses::werase(self.curses_window);

        for (i, line) in self.lines.iter().enumerate() {
            self.position.x = 0;
            self.position.y = i as i32;

            for part in &line.parts {
                for style in &part.styles {
                    match style {
                        Style::Underlined => {
                            ncurses::wattron(
                                self.curses_window,
                                ncurses::A_UNDERLINE()
                            );
                        },
                        Style::Bold => {
                            ncurses::wattron(
                                self.curses_window,
                                ncurses::A_BOLD()
                            );
                        },
                        Style::Painted (_, _) => { },
                        Style::Plain => { }
                    }
                }

                ncurses::mvwaddstr(
                    self.curses_window,
                    self.position.y,
                    self.position.x,
                    &part.value
                );
                self.position.x += part.value.len() as i32;

                for style in &part.styles {
                    match style {
                        Style::Underlined => {
                            ncurses::wattroff(
                                self.curses_window,
                                ncurses::A_UNDERLINE()
                            );
                        },
                        Style::Bold => {
                            ncurses::wattroff(
                                self.curses_window,
                                ncurses::A_BOLD()
                            );
                        },
                        Style::Painted (_, _) => { },
                        Style::Plain => { }
                    }
                }
            }
        }

        ncurses::wnoutrefresh(self.curses_window);
    }

    fn refresh(&mut self) {
        ncurses::wmove(self.curses_window,
                       self.cursor_position.y,
                       self.cursor_position.x);

        for i in 0..self.height {
            if !self.cursor_hidden && self.cursor_position.y == i as i32 {
                ncurses::wchgat(
                    self.curses_window,
                    -1,
                    ncurses::COLOR_PAIR(HIGHLIGHT_COLOR),
                    HIGHLIGHT_COLOR
                );
            }
        }

        ncurses::doupdate();
    }

    fn clear(&mut self) {
        ncurses::wmove(self.curses_window, 0, 0);
        ncurses::wclear(self.curses_window);
        ncurses::doupdate();
        ncurses::wrefresh(self.curses_window);
    }

    // Returns the delta between the attempted cursor
    // position move and actual end position.
    // This is the value which the data needs to be
    // scrolled by.
    // TODO: pretty confusing, need better way.
    fn move_cursor(&mut self, position: Position) -> (i32, Position) {
        // TODO: optimize by not doing anything when
        // trying to go beyond edges (unless scrolling).
        let y = clamp(position.y, 0, self.height - 1);
        let x = clamp(position.x, 0, self.width - 1);

        let delta = position.y - y;

        ncurses::wmove(self.curses_window, y, x);
        let new_position = Position { x, y };
        self.cursor_position = Position { x, y };

        (delta, new_position)
    }

    fn set_cursor(&mut self, position: Position) {
        self.move_cursor(position);
    }

    fn listen_input(&self) -> i32 {
        ncurses::wgetch(self.curses_window)
    }
}

impl Drop for CursesWindow {
    fn drop(&mut self) {
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
        ncurses::endwin();
    }
}

#[derive(Clone)]
pub struct CursesScreenFactory {
}

impl CursesScreenFactory {
    pub fn new() -> CursesScreenFactory{
        CursesScreenFactory { }
    }
}

impl ScreenFactory for CursesScreenFactory {
    fn create(&self, size: ScreenSize, position: Position) -> Box<dyn DrawScreen> {
        Box::new(CursesWindow::new(size, position))
    }
}
