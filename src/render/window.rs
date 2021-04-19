use crate::git;
use crate::render::ascii_table::*;
use crate::render::Point;
use crate::render::Render;

use ncurses;

pub struct Window {
    height: i32,
    width: i32,

    cursor: Point,

    curses_window: ncurses::WINDOW,

    pub buffer: Vec<String>,
    pub children: Vec<Window>,

    pub delete: bool,
}

impl Window {
    pub fn new(position: Point, height: i32, width: i32) -> Window {
        let curses_window = ncurses::newwin(height, width, position.y, position.x);

        ncurses::box_(curses_window, 0, 0);
        ncurses::wrefresh(curses_window);

        Window {
            height: height,
            width: width,

            cursor: Point { x: 0, y: 0 },

            curses_window: curses_window,

            buffer: vec![],
            children: vec![],

            delete: false,
        }
    }

    pub fn get_cursor_line(&self) -> &String {
        let line_number = self.cursor.y as usize;
        &self.buffer[line_number]
    }

    pub fn spawn_child(&mut self, position: Point, buffer: Vec<String>) -> &mut Window {
        let height = buffer.len() as i32;
        let width = buffer.iter().map(|x| x.len()).max().unwrap_or_default() as i32;

        let mut child_window = Window::new(position, height, width);
        child_window.buffer = buffer;

        self.children.push(child_window);

        self.children.last_mut().unwrap()
    }

    pub fn write_buffer(&self) {
        ncurses::wclear(self.curses_window);

        for (_, line) in self.buffer.iter().enumerate() {
            ncurses::waddstr(self.curses_window, line);
            ncurses::waddch(self.curses_window, KEY_LF as u32);
        }

        ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);
    }

    pub fn update(&mut self) {
        self.write_buffer();

        if !self.children.is_empty() {
            // remove all the empty lines that were placed where the sub window is
            self.buffer.retain(|l| !l.is_empty());

            // remove all the children marked for deletion
            for (_, child) in self
                .children
                .iter()
                .filter(|&child| child.delete)
                .enumerate()
            {
                child.close(); // frees the resources used by ncurses
            }

            // remove the deleted windows from children vec
            self.children.retain(|c| !c.delete);
        }

        ncurses::wnoutrefresh(self.curses_window);
    }

    pub fn write_at(&mut self, buffer: &Vec<String>, position: usize) {
        let mut new_buffer: Vec<String> = Vec::with_capacity(self.height as usize);

        let mut before: Vec<String> = if position == 0 {
            vec![self.buffer[0].clone()]
        } else {
            self.buffer[0..position + 1].to_vec()
        };
        let after = &self.buffer[position + 1..];

        new_buffer.append(&mut before);

        for _ in 0..buffer.len() {
            new_buffer.push("".to_string());
        }

        for (_, line) in after.iter().enumerate() {
            new_buffer.push(line.to_string());
        }

        self.buffer = new_buffer;
    }

    pub fn close(&self) {
        ncurses::delwin(self.curses_window);
    }
}

impl Render for Window {
    fn render(&mut self) {
        ncurses::wmove(self.curses_window, 0, 0);
        ncurses::refresh();

        let mut c: i32 = 0;
        while c != KEY_Q_LOWER {
            self.update();
            ncurses::doupdate();

            match c {
                // cursor movement
                /*
                KEY_H_LOWER => {
                    self.cursor.x = if self.cursor.x == 0 {
                        self.cursor.x
                    } else {
                        self.cursor.x - 1
                    };
                    ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);
                }
                */
                KEY_J_LOWER => {
                    self.cursor.y = if self.cursor.y == self.height {
                        self.cursor.y
                    } else {
                        self.cursor.y + 1
                    };
                    ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);
                }
                KEY_K_LOWER => {
                    self.cursor.y = if self.cursor.y == 0 {
                        0
                    } else {
                        self.cursor.y - 1
                    };
                    ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);
                }
                /*
                KEY_L_LOWER => {
                    self.cursor.x = if self.cursor.x == self.width {
                        self.cursor.x
                    } else {
                        self.cursor.x + 1
                    };
                    ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);
                }
                */

                KEY_ZERO => {
                    self.cursor.x = 0;
                    ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);
                }

                KEY_DOLLAR => {
                    let path_line = self.get_cursor_line();
                    self.cursor.x = path_line.chars().count() as i32;

                    ncurses::wmove(self.curses_window, self.cursor.y, self.cursor.x);
                }

                KEY_T_LOWER => {
                    let path = &self.get_cursor_line()[3..];
                    git::run_add_command(&path);
                }

                KEY_U_LOWER => {
                    let path = &self.get_cursor_line()[3..];
                    git::unstage_file(&path);
                }

                KEY_C_LOWER => {
                    self.buffer.retain(|l| !l.is_empty());
                }

                KEY_Q_LOWER => {
                    self.delete = true;
                }

                KEY_W_LOWER => {
                    let line_number = self.cursor.y as usize;

                    let path = &self.get_cursor_line()[3..];
                    let diff_lines = git::run_diff_command(&path);

                    self.write_at(&diff_lines, line_number);
                    self.update();

                    let child_position = Point {
                        y: self.cursor.y + 1,
                        x: 5,
                    };
                    let child: &mut Window = self.spawn_child(child_position, diff_lines);

                    child.render();
                }
                _ => {}
            }

            self.update();

            if !self.children.is_empty() {
                display_children(&mut self.children);
            }

            ncurses::doupdate();

            c = ncurses::wgetch(self.curses_window);
        }
    }
}

impl git::GitRunner for Window {
    fn run_git_command(&mut self) {
        // TODO: need to somehow capture curses command to know
        // which git command to send
        self.buffer = git::run_status_command()
    }
}

fn display_children(windows: &mut Vec<Window>) {
    for (_, window) in windows.iter_mut().enumerate() {
        if !window.children.is_empty() {
            display_children(&mut window.children);
        }

        window.render()
    }
}
