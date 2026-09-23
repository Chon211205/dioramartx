mod core;
mod materials;
mod objects;
mod renderer;
mod scene;

use raylib::prelude::*;

use core::camera::Camera;
use core::framebuffer::Framebuffer;
use core::vec3::Vec3;

use materials::material::Material;

use objects::object::Object;

use scene::light::Light;

fn main() {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Raytracer - Galaxy Diorama")
        .build();

    rl.set_target_fps(60);

    let mut framebuffer =
        Framebuffer::new(WIDTH, HEIGHT);

    let planet_material =
        Material::new(
            Vec3::new(
                0.2,
                0.8,
                0.3,
            ),
            0.8,
            0.5,
            0.0,
            0.0,
        );

    let objects =
        vec![
            Object::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                1.5,
                planet_material,
            ),
        ];

    let light =
        Light::new(
            Vec3::new(
                -4.0,
                4.0,
                4.0,
            ),
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
        );

    let mut camera =
        Camera::new(
            Vec3::new(
                0.0,
                0.0,
                0.0,
            ),
            5.0,
            60.0,
        );

    while !rl.window_should_close() {
        if rl.is_mouse_button_down(
            MouseButton::MOUSE_BUTTON_LEFT,
        ) {
            let mouse_delta =
                rl.get_mouse_delta();

            camera.rotate(
                mouse_delta.x,
                mouse_delta.y,
            );
        }

        let wheel =
            rl.get_mouse_wheel_move();

        if wheel != 0.0 {
            camera.zoom(wheel);
        }

        renderer::raytracer::render(
            &mut framebuffer,
            &objects,
            &light,
            &camera,
        );

        let mut d =
            rl.begin_drawing(&thread);

        d.clear_background(
            Color::BLACK,
        );

        framebuffer.draw(
            &mut d,
        );
    }
}