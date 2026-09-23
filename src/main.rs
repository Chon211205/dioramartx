mod framebuffer;
mod vec3;
mod ray;
mod object;
mod raytracer;

use raylib::prelude::*;

use framebuffer::Framebuffer;
use object::Object;
use vec3::Vec3;

fn main() {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Raytracer - Galaxy Diorama")
        .build();

    rl.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let objects = vec![
        Object::new(
            Vec3::new(0.0, 0.0, -5.0),
            1.5,
            Vec3::new(0.2, 0.8, 0.3),
        ),
    ];

    raytracer::render(
        &mut framebuffer,
        &objects,
    );

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        framebuffer.draw(&mut d);
    }
}