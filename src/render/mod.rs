pub mod ascii_table;
pub mod renderer;
pub mod window;

pub trait Render {
    fn render(&mut self);
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}
