use std::path::Path;
use std::collections::HashMap;

use crate::git;
use crate::gitko::diff_display::color_diff_line;

use gitko_common::ascii_table::{KEY_LF, KEY_ETB};
use gitko_render::{Component, KeyHandlers, Line, Window, Part, Style};

pub struct DetailedCommitWindow {
    commit_hash: String,
    commit_details: Vec<String>,
    file_changes: HashMap<String, Vec<String>>,
    expanded_changes: HashMap<String, bool>,
}

impl DetailedCommitWindow {
    pub fn new(commit_hash: &str) -> DetailedCommitWindow {
        DetailedCommitWindow{
            commit_hash: commit_hash.to_owned(),
            commit_details: vec![],
            file_changes: HashMap::new(),
            expanded_changes: HashMap::new(),
        }
    }

    fn on_press_enter(&mut self, window: &mut Window) -> bool {
        let line = window.get_cursor_line();
        let path = Path::new(&line);

        if !path.exists() {
            return true
        }

        if let Some(path_str) = path.to_str().to_owned() {
            match self.expanded_changes.get(path_str) {
                Some(&true) => {
                    self.expanded_changes
                        .entry(path_str.to_string())
                        .and_modify(|v| *v = false);
                }
                Some(&false) => {
                    self.expanded_changes
                        .entry(path_str.to_string())
                        .and_modify(|v| *v = true);
                }
                None => ()
            };

            self.on_start(window);
        }

        true
    }

    fn on_press_esc(&mut self, window: &mut Window) -> bool {
        for (_, val) in self.expanded_changes.iter_mut() { *val = false; }
        self.on_start(window);
        true
    }
}

impl Component<DetailedCommitWindow> for DetailedCommitWindow {
    fn on_start(&mut self, window: &mut Window)  {
        let details = git::show(&self.commit_hash);

        let mut current_path: Option<String> = None;

        for line in details.iter() {
            if line.starts_with("---") {
                let git_file_path_output       = line.trim_start_matches("--- ");
                let file_path_parts: Vec<&str> = git_file_path_output.split('/').collect();

                let file_path: String = file_path_parts[1..]
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>()
                    .join("/");

                self.file_changes.insert(file_path.clone(), vec![]);

                if !self.expanded_changes.contains_key(&file_path) {
                    self.expanded_changes.insert(file_path.clone(), true);
                }

                current_path = Some(file_path.clone());
            }

            if let Some(path) = &current_path {
                if let Some(changes) = self.file_changes.get_mut(path) {
                    changes.push(line.to_string())
                }
            }
        }

        let mut output: Vec<Line> = vec![];

        if let Some(description_end_position) = details.iter().position(|l| l.starts_with("diff")) {
            self.commit_details = details[0..description_end_position].to_vec();
            let mut description_lines = self.commit_details
                    .iter()
                    .map(|l| Line::from_str(l, None))
                    .collect();
            output.append(&mut description_lines)
        }

        for key in self.file_changes.keys() {
            output.push(Line::new(
                vec![Part::new(key, Some(vec![Style::Bold, Style::Underlined]))]
            ));

            if let Some(expanded) = self.expanded_changes.get(key) {
                if *expanded {
                    if let Some(changes) = self.file_changes.get(key) {
                        let mut change_lines = changes
                            .iter()
                            .map(|l| color_diff_line(l))
                            .collect();
                        output.append(&mut change_lines);
                        output.push(Line::empty());
                    }
                }
                            }
        }

        window.set_lines(output);
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<DetailedCommitWindow>) {
        handlers.insert(KEY_ETB, DetailedCommitWindow::on_press_esc);
        handlers.insert(KEY_LF, DetailedCommitWindow::on_press_enter);
    }
}
