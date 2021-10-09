mod git;
mod num;
mod render;

use crate::render::renderer::Renderer;
use crate::render::Render;

fn main() {
    Renderer::new(800, 600).render();
}
