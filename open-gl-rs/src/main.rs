extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use anyhow::Result;
use glfw::{Context, ffi::glfwGetTime};
use std::{cell::RefCell, rc};

mod camera;
mod mesh;
mod model;
mod shader;

fn main() -> Result<()> {
    use glfw::fail_on_errors;

    let mut glfw = glfw::init(fail_on_errors!())?;
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("failed to create window");
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);
    unsafe { gl::Enable(gl::DEPTH_TEST) };

    let shader = shader::Shader::new("src/shaders/model_vs.glsl", "src/shaders/model_fs.glsl");

    let mut importer = assimp::Importer::new();
    importer.triangulate(true);
    importer.flip_uvs(true);
    let model = model::Model::load(&importer, "src/models/backpack/backpack.obj");

    let camera = camera::Camera::new(
        glm::vec3(0.0, 0.0, 3.0),
        glm::vec3(0.0, 1.0, 0.0),
        -90.0,
        0.0,
    );

    let mut delta_time = 0.0;
    let mut last_frame = 0.0;

    println!("Starting render engine");
    while !window.should_close() {
        let current_frame = unsafe { glfwGetTime() };
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    Ok(())
}
