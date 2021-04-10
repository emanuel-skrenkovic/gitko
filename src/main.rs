mod git;
mod render;

use crate::render::Render;

fn main() {
    // Git should be stateless, atleast for now.
    // Window should be a struct which contains the entire result of the
    // last command and the displayed part. This way we can control what is
    // being displayed without touching git every time we move the screen.

    let lines = git::run_status_command();

    let mut renderer = render::renderer::Renderer::new(800, 600);
    renderer.render()
}
