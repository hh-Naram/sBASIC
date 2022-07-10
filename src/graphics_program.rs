use glfw::Context;

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 360;

pub struct GraphicsProgram {
    pub window: glfw::Window,
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

        GraphicsProgram {
            window: window,
            glfw: glfw,
        }
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }
}
