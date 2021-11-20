use crate::num;
use crate::render::ascii_table::*;

use std::convert::TryInto;

pub type Position = (i32, i32);

pub struct ScreenSize {
    lines: i32,
    cols: i32
}

impl ScreenSize {
    pub fn max() -> ScreenSize {
        ScreenSize {
            lines: 0,
            cols: 0
        }
    }
}

pub trait BaseWindow {
    fn on_keypress(&mut self, c: i32);
    fn on_activate(&mut self);

    fn cursor_position(&self) -> Position;
    fn move_cursor_down(&mut self);
    fn move_cursor_up(&mut self);
    fn move_cursor(&mut self, position: Position);

    fn window(&self) -> ncurses::WINDOW;

    fn render(&mut self) {
        let win = self.window();
        ncurses::wmove(win, 0, 0);

        self.on_activate();

        let mut c: i32 = 0;
        while c != KEY_Q_LOWER {
            // TODO: two updates per keypress for now.
            // Need to understand better.
            ncurses::wmove(win, self.cursor_position().0, self.cursor_position().1);
            ncurses::doupdate();

            // TODO: move cursor here. on_keypress
            // is for custom functionality.

            match c {
                KEY_J_LOWER => { self.move_cursor_down(); }
                KEY_K_LOWER => { self.move_cursor_up(); }
                _ => {}
            }
            
            self.on_keypress(c);
            ncurses::doupdate();
            ncurses::wmove(win, self.cursor_position().0, self.cursor_position().1);
            c = ncurses::wgetch(win);
        }
    }
}

pub struct Window2 {
    lines: i32,
    cols: i32,
    cursor_position: Position,
    pub curses_window: ncurses::WINDOW
}

impl Window2 {
    pub fn new(size: ScreenSize) -> Window2 {
        ncurses::start_color();

        let curses_window = ncurses::newwin(size.lines, size.cols, 0, 0);

        let mut y: i32 = 0;
        let mut x: i32 = 0;
        ncurses::getmaxyx(curses_window, &mut y, &mut x);

        ncurses::wrefresh(curses_window);

        Window2 {
            lines: y,
            cols: x,
            cursor_position: (0, 0),
            curses_window
        }
    }

    // region Display

    pub fn lines(&self) -> i32 {
        self.lines
    }

    pub fn cols(&self) -> i32 {
        self.cols
    }

    pub fn queue_write(&self, data: String, position: Position) {
        // https://linux.die.net/man/3/waddstr
        ncurses::mvwaddstr(
            self.curses_window,
            position.0,
            position.1,
            &data);
        ncurses::wnoutrefresh(self.curses_window);
    }

    fn refresh(&self) {
        ncurses::doupdate();
    }

    // endregion

    // region Cursor

    fn cursor_position(&self) -> Position {
        self.cursor_position
    }

    fn move_cursor_down(&mut self) {
        self.move_cursor(
            (self.cursor_position.0 + 1, self.cursor_position.1));
    }

    fn move_cursor_up(&mut self) {
        self.move_cursor(
            (self.cursor_position.0 - 1, self.cursor_position.1));
    }

    fn move_cursor(&mut self, position: Position) {
        // TODO: optimize by not doing anything when
        // trying to go beyong edges (unless scrolling).
        let y = num::clamp(position.0, 0, self.lines() - 1);
        let x = num::clamp(position.1, 0, self.cols() - 1);
        
        ncurses::wmove(self.curses_window, y, x);
        self.cursor_position = (y, x);
    }

    // endregion

    // region Data

    pub fn get_data(&self, from: Position, length: usize) -> String {
        let mut output: Vec<u32> = Vec::with_capacity(length);
        ncurses::winchnstr(
            self.curses_window,
            &mut output,
            length.try_into().unwrap());

        let chars: Vec<&str> = output
            .iter()
            .map(|c| ascii_to_char(*c as i32))
            .collect();

        chars.iter().fold(String::new(), |acc, c| acc + c)
    }

    // endregion
}

impl Drop for Window2 {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

pub struct MainWindow {
    data: Vec<String>,
    window: Window2
}

impl MainWindow {
    pub fn new(size: ScreenSize) -> MainWindow {
        MainWindow {
            data: vec![],
            window: Window2::new(size)
        }
    }
}

use crate::git::commands as git;

impl BaseWindow for MainWindow {
    fn on_keypress(&mut self, c: i32) {
        // TODO: remove, just for testing getting data.
        match c {
            KEY_LF => {
                let line = self.window.get_data(
                    self.window.cursor_position(),
                    (self.window.cols() -1).try_into().unwrap());
                self.window.queue_write(line, (0, 0));
            }
            _ => {}
        }
    }

    fn on_activate(&mut self) {
        let git_status: Vec<String> = git::status();

        // TODO: lists folders instead of all files in the newly
        // added folder
        let mut added: Vec<String> = git_status
            .iter()
            .filter(|c| c.starts_with("??"))
            .cloned()
            .collect();

        let mut deleted: Vec<String> = git_status
            .iter()
            .filter(|c| c.starts_with(" D"))
            .cloned()
            .collect();

        let mut unstaged: Vec<String> = git_status
            .iter()
            .filter(|c| c.starts_with(" M") || c.starts_with("MM"))
            .cloned()
            .collect();

        let mut staged: Vec<String> = git_status
            .iter()
            .filter(|c| c.starts_with('M') || c.starts_with('A') || c.starts_with('D'))
            .cloned()
            .collect();

        let mut recent_commits: Vec<String> = git::log(Some(10));

        let mut status: Vec<String> = vec![];

        if staged.is_empty() && unstaged.is_empty() && added.is_empty() && deleted.is_empty() {
            status.push("No changes found".to_string());
        }

        if !added.is_empty() {
            status.push("Untracked files:".to_string());
            status.append(&mut added);

            status.push("".to_string());
        }

        if !deleted.is_empty() {
            status.push("Deleted files:".to_string());
            status.append(&mut deleted);

            status.push("".to_string());
        }

        if !unstaged.is_empty() {
            status.push("Modified files:".to_string());
            status.append(&mut unstaged);

            status.push("".to_string());
        }

        if !staged.is_empty() {
            status.push("Staged files:".to_string());
            status.append(&mut staged);
        }

        if !recent_commits.is_empty() {
            status.append(&mut vec!["".to_string(); 5]);
            status.push("Recent commits:".to_string());
            status.append(&mut recent_commits);
        }

        if status.is_empty() {
            status.push("No changes found.".to_string());
        }

        self.data = status.clone();

        for (i, line) in self.data.iter().enumerate() {
            self.window.queue_write(line.to_string(), (i as i32, 0));
        }
    }

    // TODO: Passthrough methods are evil!
    // Think of a better way.

    fn window(&self) -> ncurses::WINDOW { 
        self.window.curses_window
    }

    fn cursor_position(&self) -> Position {
        self.window.cursor_position()
    }

    fn move_cursor_down(&mut self) {
        self.window.move_cursor_down();
    }

    fn move_cursor_up(&mut self) {
        self.window.move_cursor_up();
    }

    fn move_cursor(&mut self, position: Position) {
        self.window.move_cursor(position);
    }
}
