use sdl2::event::Event;
use sdl2::pixels::Color;

use std::f32::consts::PI;

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 360;

pub struct Renderer {
    canvas: sdl2::render::WindowCanvas,
    context: sdl2::Sdl,
}

impl Renderer {
    pub fn default() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut window = video_subsystem
            .window("sBASIC", DEFAULT_WIDTH, DEFAULT_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        window.hide();

        let canvas = window.into_canvas().build().unwrap();

        Renderer {
            canvas: canvas,
            context: sdl_context,
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        let window = self.canvas.window_mut();
        window.set_size(width as u32, height as u32).unwrap();
        window.show();
    }

    pub fn update(&mut self, is_running: &mut bool) {
        self.canvas.present();

        for event in self.context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => *is_running = false,
                _ => {}
            }
        }
    }
    pub fn render_clear(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }
    pub fn render_setcolor(&mut self, red: i32, green: i32, blue: i32) {
        self.canvas
            .set_draw_color(Color::RGB(red as u8, green as u8, blue as u8));
    }

    pub fn render_dot(&mut self, x: i32, y: i32) {
        let point = sdl2::rect::Point::new(x, y);
        self.canvas.draw_point(point).unwrap();
    }

    pub fn render_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let points = [
            sdl2::rect::Point::new(x1, y1),
            sdl2::rect::Point::new(x2, y2),
        ];
        self.canvas.draw_line(points[0], points[1]).unwrap();
    }

    pub fn render_circle(&mut self, pos_x: i32, pos_y: i32, r: i32) {
        let mut alpha: f32 = 0.0;
        while alpha < 2.0 * PI {
            let x = alpha.cos() * (r as f32) + (pos_x as f32);
            let y = alpha.sin() * (r as f32) + (pos_y as f32);
            self.render_dot(x as i32, y as i32);

            alpha += 0.001;
        }
    }
}
