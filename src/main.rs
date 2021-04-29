mod git;
mod num;
mod render;

use crate::render::Render;

fn main() {
    render::renderer::Renderer::new(800, 600).render();
}
