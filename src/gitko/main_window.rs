use crate::git::commands as git;
use crate::git::FileState;
use crate::git::parse_file_state;
use crate::render::ascii_table::*;
use crate::render::display::Display;
use crate::render::window::ScreenSize;
use crate::render::window::Window;
use crate::gitko::log_window::LogWindow;
use crate::gitko::diff_window::DiffWindow;
use crate::gitko::branch_window::BranchWindow;
use crate::gitko::command_window::CommandWindow;

pub struct MainWindow {
    data: Vec<String>,
    display: Display
}

impl MainWindow {
    pub fn new(size: ScreenSize) -> MainWindow {
        MainWindow {
            data: vec![],
            display: Display::new(size)
        }
    }
}

impl Window for MainWindow {
    fn on_keypress(&mut self, c: i32) -> bool {
        // TODO: remove, just for testing getting data.
        match c {
            KEY_B_LOWER => {
                self.render_child(BranchWindow::new(ScreenSize::max()));
            }
            KEY_C_LOWER => {
                let line = self.display.get_cursor_line_data();
                let file_state = parse_file_state(&line);

                if matches!(file_state, FileState::Modified) {
                    git::checkout_file(line[3..].trim());
                }

                self.refresh();
            }
            KEY_L_LOWER => {
                self.render_child(LogWindow::new(ScreenSize::max()));
            }
            KEY_T_LOWER => {
                // TODO: add parse git status that returns file state
                // and file path?
                let line = self.display.get_cursor_line_data();
                let file_state = parse_file_state(&line);

                if !matches!(file_state, FileState::Staged) {
                    git::add_file(line[3..].trim());
                }

                // TODO: is refresh always necessary?
                self.refresh();
            }
            KEY_U_LOWER => {
                let line = self.display.get_cursor_line_data();
                let file_state = parse_file_state(&line);

                if matches!(file_state, FileState::Staged) {
                    git::unstage_file(line[3..].trim());
                }

                self.refresh();
            }
            KEY_COLON => {
                self.render_child(
                    CommandWindow::new(ScreenSize {
                        lines: self.display.lines(), cols: self.display.cols()
                    }));
            }
            KEY_LF => {
                let line = self.display.get_cursor_line_data();
                let file_state = parse_file_state(&line);

                if !matches!(file_state, FileState::Unknown) {
                    let path = line[3..].trim();
                    let diff_window = DiffWindow::new(ScreenSize::max(), path);
                    self.render_child(diff_window);
                }
            }
            _ => {}
        }

        true
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

        let sections_count = 4;
        let lines_between = 5;

        let used_lines = added.len()
            + deleted.len()
            + unstaged.len()
            + staged.len()
            + sections_count
            + lines_between;

        let remaining_lines = self.display().lines() - used_lines as i32;
        let recent_commits_count = (remaining_lines - 1) as u32;

        let mut recent_commits: Vec<String> = git::log(Some(recent_commits_count));

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
            status.append(&mut vec!["".to_string(); lines_between]);
            status.push("Recent commits:".to_string());
            status.append(&mut recent_commits);
        }

        if status.is_empty() {
            status.push("No changes found.".to_string());
        }

        self.data = status.clone();

        for (i, line) in self.data.iter().enumerate() {
            self.display.queue_write(&line.to_string(), (i as i32, 0));
        }
    }

    // TODO: Passthrough methods are evil!
    // Think of a better way.

    fn data(&self) -> &Vec<String> {
        &self.data
    }

    fn start_position(&self) -> usize { 0 }

    fn set_start_position(&mut self, _new_position: usize) { }

    fn display(&self) -> &Display { &self.display }
    fn display_mut(&mut self) -> &mut Display { &mut self.display }
}
