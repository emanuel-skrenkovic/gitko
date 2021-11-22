use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub mod commands;

pub enum FileState {
    Unknown,
    Modified,
    Deleted,
    Added
}

pub fn parse_file_state(path: &str) -> FileState {
    let state_letters = &path[..3];

    if      state_letters.contains("M") { FileState::Modified }
    else if state_letters.contains("D") { FileState::Deleted }
    else if state_letters.contains("A") { FileState::Added }
    else                                { FileState::Unknown }
}
