use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pixels: Vec<Color>,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::BLACK; (width * height) as usize],
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in self.pixels.iter_mut() {
            *pixel = color;
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }

        let index = (y * self.width + x) as usize;
        self.pixels[index] = self.current_color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }

        let index = (y * self.width + x) as usize;
        self.pixels[index] = color;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                d.draw_pixel(x, y, self.pixels[index]);
            }
        }
    }
}