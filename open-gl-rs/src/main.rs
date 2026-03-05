extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use anyhow::Result;
use glfw::{Context, ffi::glfwGetTime};

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

    if cfg!(target_os = "macos") {
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("failed to create window");
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map_or(std::ptr::null(), |f| f as *const _)
    });
    unsafe { gl::Enable(gl::DEPTH_TEST) };

    let shader = shader::Shader::new("src/shaders/model_vs.glsl", "src/shaders/model_fs.glsl")?;

    let mut importer = assimp::Importer::new();
    importer.triangulate(true);
    let model = model::Model::load(&importer, "src/models/backpack/backpack.obj")?;

    let mut camera = camera::Camera::new(
        glm::vec3(0.0, 0.0, 3.0),
        glm::vec3(0.0, 1.0, 0.0),
        -90.0,
        0.0,
    );

    let mut delta_time;
    let mut last_frame = 0.0;
    let mut first_mouse = false;
    let mut last_x = 0.0;
    let mut last_y = 0.0;

    println!("Starting render engine");
    while !window.should_close() {
        // frame data processing
        let current_frame = unsafe { glfwGetTime() };
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_keyboard_input(&mut window, &mut camera, &(delta_time as f32));
        process_cursor_pos(
            &window,
            &mut camera,
            &mut first_mouse,
            &mut last_x,
            &mut last_y,
        );
        // todo process scroll, process window resize

        // rendering
        unsafe {
            gl::ClearColor(0.05, 0.05, 0.05, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader.use_shader();

        let projection = glm::perspective(800.0 / 600.0, camera.zoom.to_radians(), 0.1, 100.0);
        let view = camera.view_matrix();
        shader.set_mat4("projection", projection)?;
        shader.set_mat4("view", view)?;

        let mut model_mat = glm::identity();
        model_mat = glm::translate(&model_mat, &glm::vec3(0.0, 0.0, 0.0));
        model_mat = glm::scale(&model_mat, &glm::vec3(1.0, 1.0, 1.0));
        shader.set_mat4("model", model_mat)?;
        model.draw(&shader)?;

        window.swap_buffers();
        glfw.poll_events();
    }

    Ok(())
}

fn process_keyboard_input(
    window: &mut glfw::Window,
    camera: &mut camera::Camera,
    delta_time: &f32,
) {
    // escape
    match window.get_key(glfw::Key::Escape) {
        glfw::Action::Press | glfw::Action::Repeat => {
            window.set_should_close(true);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::W) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::FORWARD, delta_time);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::A) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::LEFT, delta_time);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::S) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::BACKWARD, delta_time);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::D) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::RIGHT, delta_time);
        }
        _ => {}
    }
}

fn process_cursor_pos(
    window: &glfw::Window,
    camera: &mut camera::Camera,
    first_mouse: &mut bool,
    last_x: &mut f64,
    last_y: &mut f64,
) {
    let (x_pos, y_pos) = window.get_cursor_pos();
    if *first_mouse {
        *last_x = x_pos;
        *last_y = y_pos;
        *first_mouse = false;
    }

    let x_off = (x_pos - *last_x) as f32;
    let y_off = (*last_y - y_pos) as f32; // reversed bc coords go bottom to top

    *last_x = x_pos;
    *last_y = y_pos;

    camera.process_mouse_movement(&x_off, &y_off);
}
