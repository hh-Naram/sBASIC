use crate::shaders;

use gl::types::*;
use glfw::Context;

use std::ffi::CString;
use std::mem;
use std::ptr;

use cgmath::Matrix;
use cgmath::SquareMatrix;

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 360;

pub struct Renderer {
    pub shader: u32,

    pub mvp_matrix: cgmath::Matrix4<f32>,

    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
}

impl Renderer {
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

        unsafe {
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }

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

        Renderer {
            shader: shader,
            mvp_matrix: cgmath::Matrix4::identity(),
            glfw: glfw,
            window: window,
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.window.set_size(width, height);
        self.mvp_matrix = self.mvp_matrix
            * cgmath::ortho::<f32>(
                -width as f32,
                width as f32,
                -height as f32,
                height as f32,
                -1.0f32,
                1.0f32,
            );
    }

    pub fn update(&mut self, is_running: &mut bool) {
        self.window.swap_buffers();
        self.glfw.poll_events();

        unsafe {
            let uniform_name = CString::new("u_MVP").unwrap();
            let uniform_location = gl::GetUniformLocation(self.shader, uniform_name.as_ptr());

            gl::UseProgram(self.shader);
            gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, self.mvp_matrix.as_ptr());
        }

        *is_running = self.window.should_close();
    }

    pub fn render_clear(&mut self, red: i32, green: i32, blue: i32) {
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

    pub fn render_dot(&mut self, _x: i32, _y: i32) {}
    pub fn render_line(&mut self, _x1: i32, _y1: i32, _x2: i32, _y2: i32) {}
    pub fn render_circle(&mut self, _x: i32, _y: i32, _r: i32) {}
}
