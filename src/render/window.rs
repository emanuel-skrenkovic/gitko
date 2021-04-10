use crate::git;
use crate::git::GitRunner;
use crate::render::Render;

pub struct Window {
    height: i32,
    width: i32,

    pub buffer: Vec<String>,
    pub children: Vec<Window>,
}

impl Window {
    pub fn new(height: i32, width: i32) -> Window {
        Window {
            height: height,
            width: width,
            buffer: vec![],
            children: vec![],
        }
    }
}

impl Render for Window {
    fn render(&mut self) {
        self.run_git_command();

        println!("{:#?}", self.buffer);

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
