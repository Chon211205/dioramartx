mod framebuffer;
mod ray;
mod object;
mod raytracer;

use raylib::prelude::*;
use framebuffer::Framebuffer;

fn main() {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Raytracer - Galaxy Diorama")
        .build();

    rl.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    while !rl.window_should_close() {
        framebuffer.clear(Color::BLACK);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        framebuffer.draw(&mut d);
    }
}