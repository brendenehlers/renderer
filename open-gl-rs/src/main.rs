extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use ::tracing::{info, info_span, trace};
use anyhow::Result;
use glfw::{Context, ffi::glfwGetTime};

mod camera;
mod imgui_glfw;
mod mesh;
mod model;
mod shader;
mod tracing;

fn main() -> Result<()> {
    let _tracing_guard = tracing::init();

    let _app_span = info_span!("app").entered();
    let _startup_span = info_span!("startup").entered();

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
    window.set_char_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map_or(std::ptr::null(), |f| f as *const _)
    });
    unsafe { gl::Enable(gl::DEPTH_TEST) };

    let mut imgui_glfw = imgui_glfw::ImguiGlfw::new(&mut window);

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
    let mut ui_mode = false;

    let mut light_color = LightColor {
        r: 255,
        g: 255,
        b: 255,
    };

    info!(
        model_path = "src/models/backpack/backpack.obj",
        vertex_shader = "src/shaders/model_vs.glsl",
        fragment_shader = "src/shaders/model_fs.glsl",
        "renderer initialization complete"
    );
    drop(_startup_span);

    info!("starting render engine (Space to toggle UI mode)");
    let _render_loop = ::tracing::info_span!("render_loop").entered();
    while !window.should_close() {
        let _frame_span = ::tracing::debug_span!("frame").entered();

        let current_frame = unsafe { glfwGetTime() };
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        trace!(delta_ms = delta_time * 1000.0, "frame begin");

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            // Toggle between camera mode and UI mode with Space
            if let glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) = event {
                ui_mode = !ui_mode;
                info!(ui_mode, "toggled UI mode");
                if ui_mode {
                    window.set_cursor_mode(glfw::CursorMode::Normal);
                } else {
                    window.set_cursor_mode(glfw::CursorMode::Disabled);
                    first_mouse = false;
                }
            }

            if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
                window.set_should_close(true);
            }

            if ui_mode {
                imgui_glfw.handle_event(&event);
            }
        }

        if !ui_mode {
            process_movement(&mut window, &mut camera, &(delta_time as f32));
            process_cursor_pos(
                &window,
                &mut camera,
                &mut first_mouse,
                &mut last_x,
                &mut last_y,
            );
        }

        // rendering
        unsafe {
            gl::ClearColor(0.05, 0.05, 0.05, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader.use_shader();
        shader.set_vec3("viewPos", &camera.camera_pos)?;

        // dir light
        shader.set_vec3("dirLight.direction", &glm::vec3(-0.2, -1.0, -0.3))?;
        shader.set_vec3("dirLight.ambient", &glm::vec3(0.05, 0.05, 0.05))?;
        shader.set_vec3("dirLight.diffuse", &glm::vec3(0.4, 0.4, 0.4))?;
        shader.set_vec3("dirLight.specular", &glm::vec3(0.5, 0.5, 0.5))?;

        shader.set_int("numPointLights", 1)?;
        shader.set_vec3("pointLights[0].position", &glm::vec3(0.7, 0.2, 2.0))?;
        shader.set_vec3(
            "pointLights[0].ambient",
            &glm::vec3(
                0.05 * light_color.red_norm(),
                0.05 * light_color.green_norm(),
                0.05 * light_color.blue_norm(),
            ),
        )?;
        shader.set_vec3(
            "pointLights[0].diffuse",
            &glm::vec3(
                0.8 * light_color.red_norm(),
                0.8 * light_color.green_norm(),
                0.8 * light_color.blue_norm(),
            ),
        )?;
        shader.set_vec3(
            "pointLights[0].specular",
            &glm::vec3(
                1.0 * light_color.red_norm(),
                1.0 * light_color.green_norm(),
                1.0 * light_color.blue_norm(),
            ),
        )?;
        shader.set_float("pointLights[0].constant", 1.0)?;
        shader.set_float("pointLights[0].linear", 0.09)?;
        shader.set_float("pointLights[0].quadratic", 0.032)?;

        let projection = glm::perspective(800.0 / 600.0, camera.zoom.to_radians(), 0.1, 100.0);
        let view = camera.view_matrix();
        shader.set_mat4("projection", projection)?;
        shader.set_mat4("view", view)?;

        let mut model_mat = glm::identity();
        model_mat = glm::translate(&model_mat, &glm::vec3(0.0, 0.0, 0.0));
        model_mat = glm::scale(&model_mat, &glm::vec3(1.0, 1.0, 1.0));
        shader.set_mat4("model", model_mat)?;
        {
            let _draw_span = ::tracing::trace_span!("scene_draw").entered();
            model.draw(&shader)?;
        }

        {
            let _imgui_span = ::tracing::trace_span!("imgui").entered();
            let ui = imgui_glfw.new_frame(&mut window);
            render_ui(ui, &mut light_color);
            imgui_glfw.render();
        }

        trace!("frame end");
        window.swap_buffers();
    }
    drop(_render_loop);
    drop(_app_span);

    Ok(())
}

fn process_movement(window: &mut glfw::Window, camera: &mut camera::Camera, delta_time: &f32) {
    match window.get_key(glfw::Key::W) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::Forward, delta_time);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::A) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::Left, delta_time);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::S) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::Backward, delta_time);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::D) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::Right, delta_time);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::Space) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::Up, delta_time);
        }
        _ => {}
    }

    match window.get_key(glfw::Key::LeftControl) {
        glfw::Action::Press | glfw::Action::Repeat => {
            camera.process_keyboard_input(camera::CameraMovement::Down, delta_time);
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

struct LightColor {
    r: i32,
    g: i32,
    b: i32,
}

impl LightColor {
    fn red_norm(&self) -> f32 {
        self.norm(self.r)
    }

    fn green_norm(&self) -> f32 {
        self.norm(self.g)
    }

    fn blue_norm(&self) -> f32 {
        self.norm(self.b)
    }

    fn norm(&self, v: i32) -> f32 {
        v as f32 / 255.0
    }
}

fn render_ui(ui: &mut imgui::Ui, light: &mut LightColor) {
    ui.window("config")
        .size([300.0, 100.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.slider("red", 0, 255, &mut light.r);
            ui.slider("green", 0, 255, &mut light.g);
            ui.slider("blue", 0, 255, &mut light.b);
        });
}
