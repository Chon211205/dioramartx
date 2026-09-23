use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::object::Object;
use crate::vec3::Vec3;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Object]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio;

            let ray_direction = Vec3::new(
                screen_x,
                screen_y,
                -1.0,
            )
            .normalize();

            let ray_origin = Vec3::new(0.0, 0.0, 0.0);

            let pixel_color = cast_ray(
                &ray_origin,
                &ray_direction,
                objects,
            );

            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}

fn cast_ray(
    origin: &Vec3,
    direction: &Vec3,
    objects: &[Object],
) -> Color {
    let mut closest_distance = f32::INFINITY;
    let mut final_color = Color::new(5, 5, 20, 255);

    let light_position = Vec3::new(-4.0, 4.0, 0.0);

    for object in objects {
        if let Some(distance) = object.intersect(origin, direction) {
            if distance < closest_distance {
                closest_distance = distance;

                let hit_point = *origin + *direction * distance;

                let normal = (hit_point - object.center).normalize();

                let light_direction =
                    (light_position - hit_point).normalize();

                let diffuse = normal
                    .dot(&light_direction)
                    .max(0.0);

                let ambient = 0.15;

                let intensity =
                    (ambient + diffuse * 0.85).min(1.0);

                let r =
                    (object.color.x * intensity * 255.0) as u8;

                let g =
                    (object.color.y * intensity * 255.0) as u8;

                let b =
                    (object.color.z * intensity * 255.0) as u8;

                final_color = Color::new(
                    r,
                    g,
                    b,
                    255,
                );
            }
        }
    }

    final_color
}