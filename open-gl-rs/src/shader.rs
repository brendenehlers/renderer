use anyhow::{Ok, Result};
use gl;
use std::ffi::CString;
use std::ptr;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Shader> {
        let mut success: i32 = 1;
        let mut info_log: [i8; 512] = [32; 512]; // init with spaces

        // read and compile vertex shader
        let v_shader_code = CString::new(std::fs::read_to_string(vertex_path)?)?;
        let v_shader: u32;
        v_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        unsafe {
            gl::ShaderSource(v_shader, 1, [v_shader_code.as_ptr()].as_ptr(), ptr::null());
            gl::CompileShader(v_shader);
        }

        unsafe { gl::GetShaderiv(v_shader, gl::COMPILE_STATUS, &mut success) }
        if success == gl::FALSE.try_into().unwrap() {
            unsafe { gl::GetShaderInfoLog(v_shader, 512, ptr::null_mut(), info_log.as_mut_ptr()) };
            let msg: String = info_log
                .into_iter()
                .map(|i| char::from_u32(i as u32).unwrap())
                .collect();
            anyhow::bail!("ERROR::SHADER::VERTEX::COMPILATION::FAILED\n{}", msg.trim());
        }

        // read and compile fragment shader
        let f_shader_code = CString::new(std::fs::read_to_string(fragment_path)?)?;
        let f_shader: u32;
        f_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        unsafe {
            gl::ShaderSource(f_shader, 1, [f_shader_code.as_ptr()].as_ptr(), ptr::null());
            gl::CompileShader(f_shader);
        }

        unsafe { gl::GetShaderiv(f_shader, gl::COMPILE_STATUS, &mut success) }
        if success == gl::FALSE.try_into().unwrap() {
            unsafe { gl::GetShaderInfoLog(f_shader, 512, ptr::null_mut(), info_log.as_mut_ptr()) };
            let msg: String = info_log
                .into_iter()
                .map(|i| char::from_u32(i as u32).unwrap())
                .collect();
            anyhow::bail!(
                "ERROR::SHADER::FRAGMENT::COMPILATION::FAILED\n{}",
                msg.trim()
            );
        }

        // create program and link shaders
        let id: u32;
        id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, v_shader);
            gl::AttachShader(id, f_shader);
            gl::LinkProgram(id);
        }

        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) };
        if success == gl::FALSE.try_into().unwrap() {
            unsafe { gl::GetProgramInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr()) };
            let msg: String = info_log
                .into_iter()
                .map(|i| char::from_u32(i as u32).unwrap())
                .collect();
            anyhow::bail!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", msg.trim());
        }

        // shaders linked, no longer needed
        unsafe {
            gl::DeleteShader(v_shader);
            gl::DeleteShader(f_shader);
        }

        Ok(Shader { id })
    }

    pub fn use_shader(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn set_int(&self, name: &str, value: i32) -> Result<()> {
        unsafe {
            gl::Uniform1i(self.get_uniform_loc(name)?, value);
        }
        Ok(())
    }

    pub fn set_mat4(&self, name: &str, mat: glm::Mat4) -> Result<()> {
        unsafe {
            let loc = self.get_uniform_loc(name)?;
            gl::UniformMatrix4fv(
                loc,
                1,
                gl::FALSE.try_into().unwrap(),
                glm::value_ptr(&mat).as_ptr(),
            );
        }
        Ok(())
    }

    unsafe fn get_uniform_loc(&self, name: &str) -> Result<i32> {
        unsafe {
            Ok(gl::GetUniformLocation(
                self.id,
                CString::new(name)?.as_ptr(),
            ))
        }
    }
}
