pub mod commands;

pub enum FileState {
    Unknown,
    Modified,
    Deleted,
    Added,
    Staged
}

pub fn parse_file_state(path: &str) -> FileState {
    let state_letters = &path[..3];

    if      state_letters.chars().nth(1).unwrap() == 'M' { FileState::Modified }
    else if state_letters.chars().nth(0).unwrap() == 'M' { FileState::Staged }
    else if state_letters.contains("D")                  { FileState::Deleted }
    else if state_letters.contains("A")                  { FileState::Added }
    else                                                 { FileState::Unknown }
}
