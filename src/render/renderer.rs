use crate::git;
use crate::render::window;
use crate::render::Render;

use ncurses;

// TODO: ASCII table?
const KEY_Q_LOWER: i32 = 113;
const KEY_H_LOWER: i32 = 104;
const KEY_J_LOWER: i32 = 106;
const KEY_K_LOWER: i32 = 107;
const KEY_L_LOWER: i32 = 108;
const KEY_W_LOWER: i32 = 119;
const KEY_ZERO: i32 = 48;
const KEY_DOLLAR: i32 = 36;

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
        self.buffer.append(&mut git_status);

        let mut max_x = 0;
        let mut max_y = 0;

        ncurses::getmaxyx(ncurses::stdscr(), &mut max_y, &mut max_x);

        let mut start_x = 0;
        let mut start_y = 0;

        let curses_window = create_win(0, 0, max_y, max_x);
        ncurses::wrefresh(curses_window);

        let mut c = ncurses::getch();
        while c != KEY_Q_LOWER {
            ncurses::wclear(curses_window);

            for (_, line) in self.buffer.iter().enumerate() {
                ncurses::waddstr(curses_window, line);
                ncurses::waddch(curses_window, 10);
            }

            match c {
                // cursor movement
                KEY_H_LOWER => {
                    start_x = if start_x == 0 { start_x } else { start_x - 1 };
                    ncurses::wmove(curses_window, start_y, start_x);
                }
                KEY_J_LOWER => {
                    start_y = if start_y == max_y {
                        start_y
                    } else {
                        start_y + 1
                    };
                    ncurses::wmove(curses_window, start_y, start_x);
                }
                KEY_K_LOWER => {
                    start_y = if start_y == 0 { 0 } else { start_y - 1 };
                    ncurses::wmove(curses_window, start_y, start_x);
                }
                KEY_L_LOWER => {
                    start_x = if start_x == max_x {
                        start_x
                    } else {
                        start_x + 1
                    };
                    ncurses::wmove(curses_window, start_y, start_x);
                }

                KEY_ZERO => {
                    start_x = 0;
                    ncurses::wmove(curses_window, start_y, start_x);
                }

                KEY_DOLLAR => {
                    let line_number: usize = start_y as usize;
                    let path_line = &self.buffer[line_number];

                    start_x = path_line.chars().count() as i32;
                    ncurses::wmove(curses_window, start_y, start_x);
                }

                KEY_W_LOWER => {
                    let line_number = start_y as usize;
                    let path_line = &self.buffer[line_number];
                    let path = &path_line[3..];

                    let mut diff_lines = git::run_diff_command(&path);

                    let mut new_buffer: Vec<String> = vec![];

                    let mut before: Vec<String> = self.buffer[0..line_number + 1].to_vec();
                    new_buffer.append(&mut before);

                    new_buffer.append(&mut diff_lines);

                    let mut after: Vec<String> = self.buffer[line_number + 1..].to_vec();
                    new_buffer.append(&mut after);

                    self.buffer = new_buffer;
                }
                _ => {}
            }

            ncurses::wrefresh(curses_window);
            c = ncurses::getch();
        }
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
