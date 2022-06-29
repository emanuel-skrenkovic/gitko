use std::fs::{metadata,read_dir,remove_file};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::git;
use crate::git::{parse_file_state, FileState};
use crate::ascii_table::*;
use crate::gitko::log_window::LogWindow;
use crate::gitko::diff_window::DiffWindow;
use crate::gitko::branch_window::BranchWindow;
use crate::gitko::command_window::CommandWindow;
use crate::gitko::prompt_window::PromptWindow;
use crate::gitko::push_options_window::PushOptionsWindow;
use crate::gitko::commit_options_window::CommitOptionsWindow;
use crate::searchable::{SearchableComponent, register_search_handlers};
use crate::render::{Bold, Colored, Line, Renderer, KeyHandlers, Component, ScreenSize, Underlined, Window,
                    Position};

pub struct MainWindow {
    term: String
}

impl MainWindow {
    pub fn new() -> MainWindow {
        MainWindow { term: "".to_owned() }
    }

    fn diff_file(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line();
        let file_state = parse_file_state(&line);

        if !matches!(file_state, FileState::Unknown) {
            let path = line[3..].trim();

            Renderer::new(
                &mut DiffWindow::new(path, file_state),
                ScreenSize { lines: window.height(), cols: window.width() },
                Position::default()
            ).render();
        }

        true
    }

    fn open_branch_window(&mut self, window: &mut Window) -> bool {
        Renderer::new(
            &mut BranchWindow{},
            ScreenSize::max(),
            Position::default()
        ).render();

        self.on_start(window);

        true
    }

    fn open_log_window(&mut self, window: &mut Window) -> bool {
        Renderer::new(
            &mut LogWindow::new(),
            ScreenSize::max(),
            Position::default()
        ).render();

        self.on_start(window);

        true
    }

    fn open_command_window(&mut self, window: &mut Window) -> bool {
        Renderer::new(
            &mut CommandWindow{},
            ScreenSize { lines: 2, cols: window.width() },
            Position { x: 0, y: window.height() - 2 }
        ).render();

        self.on_start(window);

        true
    }

    fn open_in_file_manager(&mut self, window: &mut Window) -> bool {
        let command = if cfg!(unix) {
            "xdg-open"
        } else if cfg!(windows) {
            "explorer"
        } else {
            panic!("Platform not supported.");
        };

        let path_str = window.get_cursor_line()[3..].trim().to_owned();
        if !Path::new(&path_str).exists() {
            return true
        }

        let mut path = PathBuf::from(window.get_cursor_line()[3..].trim());
        path.pop();

        if path.as_os_str().is_empty() {
            path.push(".");
        }

        let _ = Command::new(command)
            .arg(path)
            .spawn();

        true
    }

    fn delete_untracked_file(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line();
        let file_state = parse_file_state(&line);

        if matches!(file_state, FileState::Untracked) {
            let file = line[3..].trim();
            Renderer::new(
                &mut PromptWindow::new(&format!("Are you sure you want to delete file '{}'? y/n", file),
                                  || { let _ = remove_file(file); },
                                  || {}),
                ScreenSize { lines: 1, cols: 0 },
                Position { x: 0, y: window.height() - 1 }
            ).render();
        }

        self.on_start(window);

        true
    }

    fn git_checkout_file(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line();
        let file_state = parse_file_state(&line);

        if matches!(file_state, FileState::Modified) {
            let file = line[3..].trim();
            Renderer::new(
                &mut PromptWindow::new(&format!("Are you sure you want to checkout file '{}'? y/n", file),
                                  || { git::checkout_file(file); },
                                  || {}),
                ScreenSize { lines: 1, cols: 0 },
                Position { x: 0, y: window.height() - 1 }
            ).render();
        }

        self.on_start(window);

        true
    }

    fn git_add_file(&mut self, window: &mut Window) -> bool {
        // TODO: add parse git status that returns file state
        // and file path?
        let line = window.get_cursor_line();

        if git::is_file_modified(&line) {
            git::add_file(line[3..].trim());
        }

        self.on_start(window);

        true
    }

    fn git_unstage_file(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line();

        if git::is_in_worktree(&line) {
            git::unstage_file(line[3..].trim());
        }

        self.on_start(window);

        true
    }

    fn git_commit_options(&mut self, window: &mut Window) -> bool {
        Renderer::new(
            &mut CommitOptionsWindow{},
            ScreenSize { lines: 2, cols: window.width() },
            Position { x: 0, y: window.height() - 2 }
        ).render();

        self.on_start(window);
        true
    }

    fn git_push_options(&mut self, window: &mut Window) -> bool {
        Renderer::new(
            &mut PushOptionsWindow{},
            ScreenSize { lines: 2, cols: window.width() },
            Position { x: 0, y: window.height() - 2 }
        ).render();

        self.on_start(window);
        true
    }

    fn refresh(&mut self, window: &mut Window) -> bool {
        self.on_start(window);
        true
    }
}

impl Component<MainWindow> for MainWindow {
    fn on_start(&mut self, window: &mut Window) {
        let git_status: Vec<String> = git::status();

        let untracked: Vec<String> = git_status
            .iter()
            .filter(|c| c.starts_with("??") || c.starts_with("AM"))
            .cloned()
            .collect();

        let mut added: Vec<Line> = vec![];
        let mut added_modified: Vec<Line> = vec![];

        for u in &untracked {
            let untracked_path = &u[3..];
            let path_metadata = metadata(untracked_path);

            let modified = u.chars().nth(1).unwrap() == 'M';

            if let Ok(metadata) = path_metadata {
                if metadata.is_dir() {
                    let paths_result = read_dir(&untracked_path);

                    if let Ok(paths) = paths_result {
                        for path in paths.flatten() {
                            let line = Line::from_string(
                                format!("?? {}", path.path().display())
                            );

                            if modified {
                                added_modified.push(line);
                            } else {
                                added.push(line);
                            }
                        }
                    }
                } else if metadata.is_file() {
                    let line = Line::from_string(u.to_owned());

                    if modified {
                        added_modified.push(line);
                    } else {
                        added.push(line);
                    }
                }
            }
        }

        let mut deleted: Vec<Line> = git_status
            .iter()
            .filter(|c| c.starts_with(" D"))
            .map(|c| Line::from_string(c.to_owned()))
            .collect();

        let mut unstaged: Vec<Line> = git_status
            .iter()
            .filter(|c| c.starts_with(" M") || c.starts_with("MM"))
            .map(|c| Line::from_string(c.to_owned()))
            .collect();

        let mut staged: Vec<Line> = git_status
            .iter()
            .filter(|c| c.starts_with('M') || c.starts_with('A') || c.starts_with('D'))
            .map(|c| Line::from_string(c.to_owned()))
            .collect();

        let mut status: Vec<Line> = vec![
            Line::new(vec![
                Box::new(
                    Bold::new(Underlined::new("Head:"))
                ),
                Box::new(
                    Colored::new(
                        git::head_branch(),
                        ncurses::COLOR_CYAN,
                        ncurses::COLOR_BLACK
                    )
                ),
                Box::new(" ".to_owned()),
                Box::new(git::last_commit())
            ])
        ];

        let origin_hash = git::last_origin_commit_hash();
        let local_hash = git::last_commit_hash();

        if origin_hash != local_hash { // if HEAD different from origin HEAD
            status.push(
                Line::new(vec![
                    Box::new(
                        Bold::new(Underlined::new("Origin:"))
                    ),
                    Box::new(
                        Colored::new(
                            git::origin_head_branch(),
                            ncurses::COLOR_RED,
                            ncurses::COLOR_BLACK
                        )
                    ),
                    Box::new(" ".to_owned()),
                    Box::new(git::last_origin_commit())
                ])
            );
        }

        status.push(Line::empty());


        if !added.is_empty() {
            status.push(
                Line::new(vec![
                        Box::new(
                            Bold::new(
                                Underlined::new(
                                    format!("Untracked files: ({})", added.len())
                                )
                            )
                        )
                    ]
                )
            );
            status.append(&mut added);

            status.push(Line::empty());
        }

        if !added_modified.is_empty() {
            status.push(
                Line::new(vec![
                        Box::new(
                            Bold::new(
                                Underlined::new(
                                    format!("Untracked (modified) files: ({})", added_modified.len())
                                )
                            )
                        )
                    ]
                )
            );
            status.append(&mut added_modified);

            status.push(Line::empty());
        }

        if !deleted.is_empty() {
            status.push(Line::from_string("Deleted files:".to_owned()));
            status.append(&mut deleted);

            status.push(Line::from_string("".to_owned()));
        }

        status.push(
            Line::new(vec![
                Box::new(
                    Bold::new(
                        Underlined::new(format!("Modified files: ({})", unstaged.len()))
                    )
                )
            ])
        );
        if !unstaged.is_empty() {
            status.append(&mut unstaged);

            status.push(Line::from_string("".to_owned()));
        }

        status.push(Line::empty());

        if !staged.is_empty() {
            status.push(
                Line::new(vec![
                    Box::new(
                        Bold::new(
                            Underlined::new(format!("Staged files: ({})", staged.len()))
                        )
                    )
                ])
            );
            status.append(&mut staged);
        }

        if status.is_empty() {
            status.push(Line::from_string("No changes found.".to_owned()));
        }

        window.lines = status;
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<MainWindow>) {
        handlers.insert(KEY_LF, MainWindow::diff_file);
        handlers.insert(KEY_B_LOWER, MainWindow::open_branch_window);
        handlers.insert(KEY_C_LOWER, MainWindow::git_checkout_file);
        handlers.insert(KEY_D_LOWER, MainWindow::delete_untracked_file);
        handlers.insert(KEY_L_LOWER, MainWindow::open_log_window);
        handlers.insert(KEY_O_UPPER, MainWindow::open_in_file_manager);
        handlers.insert(KEY_T_LOWER, MainWindow::git_add_file);
        handlers.insert(KEY_U_LOWER, MainWindow::git_unstage_file);
        handlers.insert(KEY_COLON, MainWindow::open_command_window);
        handlers.insert(KEY_C_UPPER, MainWindow::git_commit_options);
        handlers.insert(KEY_P_UPPER, MainWindow::git_push_options);
        handlers.insert(KEY_R_UPPER, MainWindow::refresh);

        register_search_handlers(handlers);
    }
}

impl SearchableComponent<MainWindow> for MainWindow {
    fn term(&self) -> String {
        self.term.clone()
    }

    fn set_term(&mut self, term: String) {
        self.term = term;
    }
}
