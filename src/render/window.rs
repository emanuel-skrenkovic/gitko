use crate::git;
use crate::git::GitRunner;
use crate::render::Render;

use ncurses::*;

pub struct Window {
    position_y: i32,
    position_x: i32,

    height: i32,
    width: i32,

    curses_window: WINDOW,

    pub buffer: Vec<String>,
    pub children: Vec<Window>,
}

impl Window {
    pub fn new(position_y: i32, position_x: i32, height: i32, width: i32) -> Window {
        let curses_window = newwin(height, width, position_y, position_x);
        box_(curses_window, 0, 0);
        wrefresh(curses_window);

        Window {
            position_y: position_y,
            position_x: position_x,

            height: height,
            width: width,

            curses_window: curses_window,

            buffer: vec![],
            children: vec![],
        }
    }
}

impl Render for Window {
    fn render(&mut self) {
        self.run_git_command();

        for (i, line) in self.buffer.iter().enumerate() {
            mvaddstr(i as i32, 0, line);
        }

        if !self.children.is_empty() {
            display_children(&self.children);
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

fn display_children(windows: &Vec<Window>) {
    for (_, window) in windows.iter().enumerate() {
        if !window.children.is_empty() {
            display_children(&window.children);
        }

        // window.render()
    }
}
