use raylib::prelude::*;

use crate::core::camera::Camera;
use crate::core::framebuffer::Framebuffer;
use crate::core::vec3::Vec3;
use crate::objects::object::Object;
use crate::scene::light::Light;

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Object],
    light: &Light,
    camera: &Camera,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    let aspect_ratio = width / height;

    let forward =
        (camera.target - camera.position).normalize();

    let world_up =
        Vec3::new(0.0, 1.0, 0.0);

    let right =
        forward
            .cross(&world_up)
            .normalize();

    let up =
        right
            .cross(&forward)
            .normalize();

    let scale =
        (camera.fov.to_radians() * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let px =
                (
                    2.0
                        * ((x as f32 + 0.5) / width)
                        - 1.0
                )
                    * aspect_ratio
                    * scale;

            let py =
                (
                    1.0
                        - 2.0
                            * ((y as f32 + 0.5) / height)
                )
                    * scale;

            let ray_direction =
                (
                    forward
                        + right * px
                        + up * py
                )
                    .normalize();

            let pixel_color =
                cast_ray(
                    &camera.position,
                    &ray_direction,
                    objects,
                    light,
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
    light: &Light,
) -> Color {
    let mut closest_distance =
        f32::INFINITY;

    let mut final_color =
        Color::new(
            5,
            5,
            20,
            255,
        );

    for object in objects {
        if let Some(distance) =
            object.intersect(
                origin,
                direction,
            )
        {
            if distance < closest_distance {
                closest_distance =
                    distance;

                let hit_point =
                    *origin
                        + *direction * distance;

                let normal =
                    object.normal_at(
                        &hit_point,
                    );

                let material =
                    object.material();

                let light_direction =
                    (
                        light.position
                            - hit_point
                    )
                        .normalize();

                let diffuse =
                    normal
                        .dot(
                            &light_direction,
                        )
                        .max(0.0);

                let view_direction =
                    (
                        *origin
                            - hit_point
                    )
                        .normalize();

                let reflect_direction =
                    reflect(
                        -light_direction,
                        normal,
                    );

                let specular_intensity =
                    view_direction
                        .dot(
                            &reflect_direction,
                        )
                        .max(0.0)
                        .powf(32.0);

                let ambient = 0.12;

                let diffuse_component =
                    diffuse
                        * material.albedo
                        * light.intensity;

                let specular_component =
                    specular_intensity
                        * material.specular
                        * light.intensity;

                let r =
                    (
                        material.color.x
                            * light.color.x
                            * (
                                ambient
                                    + diffuse_component
                            )
                            + specular_component
                    )
                        .min(1.0);

                let g =
                    (
                        material.color.y
                            * light.color.y
                            * (
                                ambient
                                    + diffuse_component
                            )
                            + specular_component
                    )
                        .min(1.0);

                let b =
                    (
                        material.color.z
                            * light.color.z
                            * (
                                ambient
                                    + diffuse_component
                            )
                            + specular_component
                    )
                        .min(1.0);

                final_color =
                    Color::new(
                        (r * 255.0) as u8,
                        (g * 255.0) as u8,
                        (b * 255.0) as u8,
                        255,
                    );
            }
        }
    }

    final_color
}

fn reflect(
    direction: Vec3,
    normal: Vec3,
) -> Vec3 {
    direction
        - normal
            * 2.0
            * direction.dot(&normal)
}