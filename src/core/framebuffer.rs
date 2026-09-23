use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pixels: Vec<Color>,
    zbuffer: Vec<f32>,
}

impl Framebuffer {
    pub fn new(
        width: i32,
        height: i32,
    ) -> Self {
        let size =
            (width * height) as usize;

        Self {
            width,
            height,
            pixels: vec![
                Color::BLACK;
                size
            ],
            zbuffer: vec![
                f32::INFINITY;
                size
            ],
        }
    }

    pub fn clear(
        &mut self,
        color: Color,
    ) {
        self.pixels.fill(
            color,
        );

        self.zbuffer.fill(
            f32::INFINITY,
        );
    }

    pub fn set_pixel(
        &mut self,
        x: i32,
        y: i32,
        color: Color,
    ) {
        if x < 0
            || x >= self.width
            || y < 0
            || y >= self.height
        {
            return;
        }

        let index =
            (
                y * self.width
                    + x
            ) as usize;

        self.pixels[index] =
            color;
    }

    pub fn pixels(
        &self,
    ) -> &[Color] {
        &self.pixels
    }

    pub fn pixels_mut(
        &mut self,
    ) -> &mut [Color] {
        &mut self.pixels
    }

    pub fn set_depth(
        &mut self,
        x: i32,
        y: i32,
        depth: f32,
    ) {
        if x < 0
            || x >= self.width
            || y < 0
            || y >= self.height
        {
            return;
        }

        self.zbuffer[
            (
                y * self.width
                    + x
            ) as usize
        ] = depth;
    }

    pub fn get_depth(
        &self,
        x: i32,
        y: i32,
    ) -> f32 {
        if x < 0
            || x >= self.width
            || y < 0
            || y >= self.height
        {
            return f32::INFINITY;
        }

        self.zbuffer[
            (
                y * self.width
                    + x
            ) as usize
        ]
    }
}