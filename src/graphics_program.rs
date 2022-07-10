use glfw::Context;

use std::ffi::CString;
use std::ptr;

use crate::shaders;

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 360;

pub struct GraphicsProgram {
    pub window: glfw::Window,
    pub shader: u32,

    pub glfw: glfw::Glfw,
}

impl GraphicsProgram {
    pub fn default() -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, _) = glfw
            .create_window(
                DEFAULT_WIDTH,
                DEFAULT_HEIGHT,
                "sBASIC",
                glfw::WindowMode::Windowed,
            )
            .expect("ERR: Could not create GLFW window.");

        window.set_resizable(false);
        window.make_current();
        window.hide();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let shader = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            let vertex_shader_source =
                CString::new(shaders::VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
            let fragment_shader_source =
                CString::new(shaders::FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();

            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_shader_source.as_ptr(),
                ptr::null(),
            );

            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_shader_source.as_ptr(),
                ptr::null(),
            );

            gl::CompileShader(fragment_shader);
            gl::CompileShader(vertex_shader);

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader_program
        };

        GraphicsProgram {
            window: window,
            shader: shader,

            glfw: glfw,
        }
    }

    pub fn update(&mut self, is_running: &mut bool) {
        self.window.swap_buffers();
        self.glfw.poll_events();

        unsafe {
            gl::UseProgram(self.shader);
        }

        *is_running = self.window.should_close();
    }

    pub fn clear(&mut self, red: i32, green: i32, blue: i32) {
        unsafe {
            gl::ClearColor(
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
                1.0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
