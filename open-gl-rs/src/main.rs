extern crate nalgebra_glm as glm;
extern crate glfw;
extern crate gl;

use anyhow::Result;
use glfw::Context;

mod camera;
mod shader;
mod mesh;
mod model;

fn main() -> Result<()> {
    use glfw::fail_on_errors;

    let mut glfw = glfw::init(fail_on_errors!())?;
    let (mut window, events) = glfw
        .create_window(800, 600, "Hello LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("failed to create window");
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

    println!("Starting render engine");
    while !window.should_close() {

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => { window.set_should_close(true) }
                _ => {},
            }
        }
    }

    Ok(())
}
