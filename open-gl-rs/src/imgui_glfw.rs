use std::time::Instant;

use tracing::{debug, info};

use glfw::{Action, Cursor, Key as GlfwKey, Modifiers, StandardCursor, Window, WindowEvent};
use imgui::{Key as ImGuiKey, MouseCursor};
use imgui_glow_renderer::AutoRenderer;

pub struct ImguiGlfw {
    imgui: imgui::Context,
    renderer: AutoRenderer,
    last_frame: Instant,
    last_cursor: Option<MouseCursor>,
}

impl ImguiGlfw {
    pub fn new(window: &mut Window) -> Self {
        debug!("initializing ImGui context");
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        {
            let io = imgui.io_mut();
            let (win_w, win_h) = window.get_size();
            let (fb_w, fb_h) = window.get_framebuffer_size();
            io.display_size = [win_w as f32, win_h as f32];
            if win_w > 0 && win_h > 0 {
                io.display_framebuffer_scale =
                    [fb_w as f32 / win_w as f32, fb_h as f32 / win_h as f32];
            }
        }

        let gl_context = unsafe {
            glow::Context::from_loader_function(|s| {
                window
                    .get_proc_address(s)
                    .map_or(std::ptr::null(), |f| f as *const _)
            })
        };

        let renderer =
            AutoRenderer::new(gl_context, &mut imgui).expect("failed to create imgui renderer");
        info!("ImGui renderer initialized");

        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            last_cursor: None,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        let io = self.imgui.io_mut();
        match *event {
            WindowEvent::Key(key, _scancode, action, modifiers) => {
                if let Some(imgui_key) = map_glfw_key(key) {
                    let pressed = action == Action::Press || action == Action::Repeat;
                    io.add_key_event(imgui_key, pressed);
                }
                io.add_key_event(
                    ImGuiKey::ModCtrl,
                    modifiers.contains(Modifiers::Control),
                );
                io.add_key_event(
                    ImGuiKey::ModShift,
                    modifiers.contains(Modifiers::Shift),
                );
                io.add_key_event(ImGuiKey::ModAlt, modifiers.contains(Modifiers::Alt));
                io.add_key_event(
                    ImGuiKey::ModSuper,
                    modifiers.contains(Modifiers::Super),
                );
            }
            WindowEvent::Char(c) => {
                io.add_input_character(c);
            }
            WindowEvent::CursorPos(x, y) => {
                io.add_mouse_pos_event([x as f32, y as f32]);
            }
            WindowEvent::MouseButton(button, action, _modifiers) => {
                let imgui_button = match button {
                    glfw::MouseButton::Button1 => Some(imgui::MouseButton::Left),
                    glfw::MouseButton::Button2 => Some(imgui::MouseButton::Right),
                    glfw::MouseButton::Button3 => Some(imgui::MouseButton::Middle),
                    glfw::MouseButton::Button4 => Some(imgui::MouseButton::Extra1),
                    glfw::MouseButton::Button5 => Some(imgui::MouseButton::Extra2),
                    _ => None,
                };
                if let Some(btn) = imgui_button {
                    io.add_mouse_button_event(btn, action == Action::Press);
                }
            }
            WindowEvent::Scroll(x, y) => {
                io.add_mouse_wheel_event([x as f32, y as f32]);
            }
            WindowEvent::FramebufferSize(w, h) => {
                io.display_size = [w as f32, h as f32];
            }
            WindowEvent::Focus(focused) => {
                if !focused {
                    io.app_focus_lost = true;
                }
            }
            _ => {}
        }
    }

    pub fn new_frame(&mut self, window: &mut Window) -> &mut imgui::Ui {
        let io = self.imgui.io_mut();

        let (win_w, win_h) = window.get_size();
        let (fb_w, fb_h) = window.get_framebuffer_size();
        io.display_size = [win_w as f32, win_h as f32];
        if win_w > 0 && win_h > 0 {
            io.display_framebuffer_scale =
                [fb_w as f32 / win_w as f32, fb_h as f32 / win_h as f32];
        }

        let now = Instant::now();
        io.update_delta_time(now.duration_since(self.last_frame));
        self.last_frame = now;

        if window.get_cursor_mode() != glfw::CursorMode::Disabled {
            let cursor = self.imgui.mouse_cursor();
            if cursor != self.last_cursor {
                self.last_cursor = cursor;
                match cursor {
                    Some(cursor_type) => {
                        window.set_cursor_mode(glfw::CursorMode::Normal);
                        let std_cursor = match cursor_type {
                            MouseCursor::Arrow => StandardCursor::Arrow,
                            MouseCursor::TextInput => StandardCursor::IBeam,
                            MouseCursor::ResizeAll => StandardCursor::Arrow,
                            MouseCursor::ResizeNS => StandardCursor::VResize,
                            MouseCursor::ResizeEW => StandardCursor::HResize,
                            MouseCursor::ResizeNESW => StandardCursor::Arrow,
                            MouseCursor::ResizeNWSE => StandardCursor::Arrow,
                            MouseCursor::Hand => StandardCursor::Hand,
                            MouseCursor::NotAllowed => StandardCursor::Arrow,
                        };
                        window.set_cursor(Some(Cursor::standard(std_cursor)));
                    }
                    None => {
                        window.set_cursor_mode(glfw::CursorMode::Hidden);
                    }
                }
            }
        }

        self.imgui.new_frame()
    }

    pub fn render(&mut self) {
        let draw_data = self.imgui.render();
        self.renderer
            .render(draw_data)
            .expect("imgui rendering failed");
    }

    pub fn want_capture_keyboard(&self) -> bool {
        self.imgui.io().want_capture_keyboard
    }

    pub fn want_capture_mouse(&self) -> bool {
        self.imgui.io().want_capture_mouse
    }
}

fn map_glfw_key(key: GlfwKey) -> Option<ImGuiKey> {
    match key {
        GlfwKey::Tab => Some(ImGuiKey::Tab),
        GlfwKey::Left => Some(ImGuiKey::LeftArrow),
        GlfwKey::Right => Some(ImGuiKey::RightArrow),
        GlfwKey::Up => Some(ImGuiKey::UpArrow),
        GlfwKey::Down => Some(ImGuiKey::DownArrow),
        GlfwKey::PageUp => Some(ImGuiKey::PageUp),
        GlfwKey::PageDown => Some(ImGuiKey::PageDown),
        GlfwKey::Home => Some(ImGuiKey::Home),
        GlfwKey::End => Some(ImGuiKey::End),
        GlfwKey::Insert => Some(ImGuiKey::Insert),
        GlfwKey::Delete => Some(ImGuiKey::Delete),
        GlfwKey::Backspace => Some(ImGuiKey::Backspace),
        GlfwKey::Space => Some(ImGuiKey::Space),
        GlfwKey::Enter => Some(ImGuiKey::Enter),
        GlfwKey::Escape => Some(ImGuiKey::Escape),
        GlfwKey::LeftControl => Some(ImGuiKey::LeftCtrl),
        GlfwKey::LeftShift => Some(ImGuiKey::LeftShift),
        GlfwKey::LeftAlt => Some(ImGuiKey::LeftAlt),
        GlfwKey::LeftSuper => Some(ImGuiKey::LeftSuper),
        GlfwKey::RightControl => Some(ImGuiKey::RightCtrl),
        GlfwKey::RightShift => Some(ImGuiKey::RightShift),
        GlfwKey::RightAlt => Some(ImGuiKey::RightAlt),
        GlfwKey::RightSuper => Some(ImGuiKey::RightSuper),
        GlfwKey::Menu => Some(ImGuiKey::Menu),
        GlfwKey::Num0 => Some(ImGuiKey::Alpha0),
        GlfwKey::Num1 => Some(ImGuiKey::Alpha1),
        GlfwKey::Num2 => Some(ImGuiKey::Alpha2),
        GlfwKey::Num3 => Some(ImGuiKey::Alpha3),
        GlfwKey::Num4 => Some(ImGuiKey::Alpha4),
        GlfwKey::Num5 => Some(ImGuiKey::Alpha5),
        GlfwKey::Num6 => Some(ImGuiKey::Alpha6),
        GlfwKey::Num7 => Some(ImGuiKey::Alpha7),
        GlfwKey::Num8 => Some(ImGuiKey::Alpha8),
        GlfwKey::Num9 => Some(ImGuiKey::Alpha9),
        GlfwKey::A => Some(ImGuiKey::A),
        GlfwKey::B => Some(ImGuiKey::B),
        GlfwKey::C => Some(ImGuiKey::C),
        GlfwKey::D => Some(ImGuiKey::D),
        GlfwKey::E => Some(ImGuiKey::E),
        GlfwKey::F => Some(ImGuiKey::F),
        GlfwKey::G => Some(ImGuiKey::G),
        GlfwKey::H => Some(ImGuiKey::H),
        GlfwKey::I => Some(ImGuiKey::I),
        GlfwKey::J => Some(ImGuiKey::J),
        GlfwKey::K => Some(ImGuiKey::K),
        GlfwKey::L => Some(ImGuiKey::L),
        GlfwKey::M => Some(ImGuiKey::M),
        GlfwKey::N => Some(ImGuiKey::N),
        GlfwKey::O => Some(ImGuiKey::O),
        GlfwKey::P => Some(ImGuiKey::P),
        GlfwKey::Q => Some(ImGuiKey::Q),
        GlfwKey::R => Some(ImGuiKey::R),
        GlfwKey::S => Some(ImGuiKey::S),
        GlfwKey::T => Some(ImGuiKey::T),
        GlfwKey::U => Some(ImGuiKey::U),
        GlfwKey::V => Some(ImGuiKey::V),
        GlfwKey::W => Some(ImGuiKey::W),
        GlfwKey::X => Some(ImGuiKey::X),
        GlfwKey::Y => Some(ImGuiKey::Y),
        GlfwKey::Z => Some(ImGuiKey::Z),
        GlfwKey::F1 => Some(ImGuiKey::F1),
        GlfwKey::F2 => Some(ImGuiKey::F2),
        GlfwKey::F3 => Some(ImGuiKey::F3),
        GlfwKey::F4 => Some(ImGuiKey::F4),
        GlfwKey::F5 => Some(ImGuiKey::F5),
        GlfwKey::F6 => Some(ImGuiKey::F6),
        GlfwKey::F7 => Some(ImGuiKey::F7),
        GlfwKey::F8 => Some(ImGuiKey::F8),
        GlfwKey::F9 => Some(ImGuiKey::F9),
        GlfwKey::F10 => Some(ImGuiKey::F10),
        GlfwKey::F11 => Some(ImGuiKey::F11),
        GlfwKey::F12 => Some(ImGuiKey::F12),
        GlfwKey::Apostrophe => Some(ImGuiKey::Apostrophe),
        GlfwKey::Comma => Some(ImGuiKey::Comma),
        GlfwKey::Minus => Some(ImGuiKey::Minus),
        GlfwKey::Period => Some(ImGuiKey::Period),
        GlfwKey::Slash => Some(ImGuiKey::Slash),
        GlfwKey::Semicolon => Some(ImGuiKey::Semicolon),
        GlfwKey::Equal => Some(ImGuiKey::Equal),
        GlfwKey::LeftBracket => Some(ImGuiKey::LeftBracket),
        GlfwKey::Backslash => Some(ImGuiKey::Backslash),
        GlfwKey::RightBracket => Some(ImGuiKey::RightBracket),
        GlfwKey::GraveAccent => Some(ImGuiKey::GraveAccent),
        GlfwKey::CapsLock => Some(ImGuiKey::CapsLock),
        GlfwKey::ScrollLock => Some(ImGuiKey::ScrollLock),
        GlfwKey::NumLock => Some(ImGuiKey::NumLock),
        GlfwKey::PrintScreen => Some(ImGuiKey::PrintScreen),
        GlfwKey::Pause => Some(ImGuiKey::Pause),
        GlfwKey::Kp0 => Some(ImGuiKey::Keypad0),
        GlfwKey::Kp1 => Some(ImGuiKey::Keypad1),
        GlfwKey::Kp2 => Some(ImGuiKey::Keypad2),
        GlfwKey::Kp3 => Some(ImGuiKey::Keypad3),
        GlfwKey::Kp4 => Some(ImGuiKey::Keypad4),
        GlfwKey::Kp5 => Some(ImGuiKey::Keypad5),
        GlfwKey::Kp6 => Some(ImGuiKey::Keypad6),
        GlfwKey::Kp7 => Some(ImGuiKey::Keypad7),
        GlfwKey::Kp8 => Some(ImGuiKey::Keypad8),
        GlfwKey::Kp9 => Some(ImGuiKey::Keypad9),
        GlfwKey::KpDecimal => Some(ImGuiKey::KeypadDecimal),
        GlfwKey::KpDivide => Some(ImGuiKey::KeypadDivide),
        GlfwKey::KpMultiply => Some(ImGuiKey::KeypadMultiply),
        GlfwKey::KpSubtract => Some(ImGuiKey::KeypadSubtract),
        GlfwKey::KpAdd => Some(ImGuiKey::KeypadAdd),
        GlfwKey::KpEnter => Some(ImGuiKey::KeypadEnter),
        GlfwKey::KpEqual => Some(ImGuiKey::KeypadEqual),
        _ => None,
    }
}
