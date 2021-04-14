use crate::git;
use crate::git::GitRunner;
use crate::render::Render;
use crate::render::ascii_table::*;

use ncurses;

pub struct Window {
    position_y: i32,
    position_x: i32,

    height: i32,
    width:  i32,

    curses_window: ncurses::WINDOW,

    pub buffer:   Vec<String>,
    pub children: Vec<Window>,
}

impl Window {
    pub fn new(position_y: i32, position_x: i32, height: i32, width: i32) -> Window {
        let curses_window = ncurses::newwin(height, width, position_y, position_x);
        ncurses::box_(curses_window, 0, 0);
        ncurses::wrefresh(curses_window);

        Window {
            position_y: position_y,
            position_x: position_x,

            height: height,
            width:  width,

            curses_window: curses_window,

            buffer:   vec![],
            children: vec![],
        }
    }

    pub fn spawn_child(&mut self, position_y: i32, position_x: i32, height: i32, width: i32) -> &mut Window {
        let child_window = Window::new(position_y, position_x, height, width);
        self.children.push(child_window);

        self.children.last_mut().unwrap()
    }
}

impl Render for Window {
    fn render(&mut self) {
        let mut start_x = 0;
        let mut start_y = 0;

        ncurses::wrefresh(self.curses_window);

        let mut c = ncurses::getch();
        while c != KEY_Q_LOWER {
            ncurses::wclear(self.curses_window);

            for (_, line) in self.buffer.iter().enumerate() {
                ncurses::waddstr(self.curses_window, line);
                ncurses::waddch(self.curses_window, 10);
            }

            match c {
                // cursor movement
                KEY_H_LOWER => {
                    start_x = if start_x == 0 { start_x } else { start_x - 1 };
                    ncurses::wmove(self.curses_window, start_y, start_x);
                }
                KEY_J_LOWER => {
                    start_y = if start_y == self.height {
                        start_y
                    } else {
                        start_y + 1
                    };
                    ncurses::wmove(self.curses_window, start_y, start_x);
                }
                KEY_K_LOWER => {
                    start_y = if start_y == 0 { 0 } else { start_y - 1 };
                    ncurses::wmove(self.curses_window, start_y, start_x);
                }
                KEY_L_LOWER => {
                    start_x = if start_x == self.width {
                        start_x
                    } else {
                        start_x + 1
                    };
                    ncurses::wmove(self.curses_window, start_y, start_x);
                }

                KEY_ZERO => {
                    start_x = 0;
                    ncurses::wmove(self.curses_window, start_y, start_x);
                }

                KEY_DOLLAR => {
                    let line_number: usize = start_y as usize;
                    let path_line = &self.buffer[line_number];

                    start_x = path_line.chars().count() as i32;
                    ncurses::wmove(self.curses_window, start_y, start_x);
                }

                KEY_W_LOWER => {
                    let line_number = start_y as usize;
                    let path_line = &self.buffer[line_number];
                    let path = &path_line[3..];

                    let mut diff_lines = git::run_diff_command(&path);

                    let mut child: &mut Window = self.spawn_child(
                        start_y + 1,
                        0,
                        (diff_lines.len() + 1) as i32,
                        self.width);

                    child.buffer = diff_lines;
                    child.render();
                }
                _ => {}
            }

            ncurses::wrefresh(self.curses_window);
            c = ncurses::getch();
        }


        // if !self.children.is_empty() {
            // display_children(&self.children);
        //}
    }
}

impl git::GitRunner for Window {
    fn run_git_command(&mut self) {
        // TODO: need to somehow capture curses command to know
        // which git command to send
        self.buffer = git::run_status_command()
    }
}

fn display_children(windows: &Vec<Window>) {
    for (_, window) in windows.iter().enumerate() {
        if !window.children.is_empty() {
            display_children(&window.children);
        }

        // window.render()
    }
}
