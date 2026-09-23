use raylib::prelude::*;

use crate::core::camera::Camera;
use crate::core::framebuffer::Framebuffer;
use crate::core::vec3::Vec3;

use crate::materials::material::{
    Material,
    MaterialPattern,
};

use crate::objects::object::Object;
use crate::scene::light::Light;

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Object],
    light: &Light,
    camera: &Camera,
) {
    framebuffer.clear_zbuffer();

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    let aspect_ratio = width / height;

    let forward =
        (camera.target - camera.position)
            .normalize();

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
        (
            camera
                .fov
                .to_radians()
                * 0.5
        )
            .tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let px =
                (
                    2.0
                        * (
                            (x as f32 + 0.5)
                                / width
                        )
                        - 1.0
                )
                    * aspect_ratio
                    * scale;

            let py =
                (
                    1.0
                        - 2.0
                            * (
                                (y as f32 + 0.5)
                                    / height
                            )
                )
                    * scale;

            let ray_direction =
                (
                    forward
                        + right * px
                        + up * py
                )
                    .normalize();

            let (
                pixel_color,
                depth,
            ) = cast_ray(
                &camera.position,
                &ray_direction,
                objects,
                light,
                0,
            );

            framebuffer.set_depth(
                x,
                y,
                depth,
            );

            framebuffer.set_current_color(
                pixel_color,
            );

            framebuffer.point(
                x,
                y,
            );
        }
    }
}

fn cast_ray(
    origin: &Vec3,
    direction: &Vec3,
    objects: &[Object],
    light: &Light,
    depth: u32,
) -> (Color, f32) {
    if depth > 4 {
        return (
            skybox_color(direction),
            f32::INFINITY,
        );
    }

    let mut zbuffer =
        f32::INFINITY;

    let mut closest_object:
        Option<&Object> =
        None;

    for object in objects {
        if let Some(distance) =
            object.intersect(
                origin,
                direction,
            )
        {
            if distance < zbuffer {
                zbuffer = distance;
                closest_object = Some(object);
            }
        }
    }

    let Some(object) = closest_object else {
        return (
            skybox_color(direction),
            f32::INFINITY,
        );
    };

    let hit_point =
        *origin
            + *direction * zbuffer;

    let normal =
        object.normal_at(
            &hit_point,
        );

    let material =
        object.material();

    let surface_color =
        get_material_color(
            &material,
            &hit_point,
            &normal,
        );

    let light_direction =
        (
            light.position
                - hit_point
        )
            .normalize();

    let diffuse =
        normal
            .dot(&light_direction)
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
            .dot(&reflect_direction)
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
            surface_color.x
                * light.color.x
                * (
                    ambient
                        + diffuse_component
                )
                + specular_component
        )
            .clamp(0.0, 1.0);

    let g =
        (
            surface_color.y
                * light.color.y
                * (
                    ambient
                        + diffuse_component
                )
                + specular_component
        )
            .clamp(0.0, 1.0);

    let b =
        (
            surface_color.z
                * light.color.z
                * (
                    ambient
                        + diffuse_component
                )
                + specular_component
        )
            .clamp(0.0, 1.0);

    let rendered_color =
        Color::new(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            255,
        );

    if material.transparency > 0.0 {
        let epsilon = 0.002;

        let new_origin =
            hit_point
                + *direction * epsilon;

        let (
            behind_color,
            _,
        ) = cast_ray(
            &new_origin,
            direction,
            objects,
            light,
            depth + 1,
        );

        let transparency =
            material
                .transparency
                .clamp(0.0, 1.0);

        let opacity =
            1.0 - transparency;

        let final_r =
            rendered_color.r as f32
                * opacity
                + behind_color.r as f32
                    * transparency;

        let final_g =
            rendered_color.g as f32
                * opacity
                + behind_color.g as f32
                    * transparency;

        let final_b =
            rendered_color.b as f32
                * opacity
                + behind_color.b as f32
                    * transparency;

        return (
            Color::new(
                final_r as u8,
                final_g as u8,
                final_b as u8,
                255,
            ),
            zbuffer,
        );
    }

    (
        rendered_color,
        zbuffer,
    )
}

fn get_material_color(
    material: &Material,
    point: &Vec3,
    normal: &Vec3,
) -> Vec3 {
    match material.pattern {
        MaterialPattern::Solid => {
            material.color
        }

        MaterialPattern::Grass => {
            grass_color(
                material.color,
                point,
                normal,
            )
        }
    }
}

fn grass_color(
    base: Vec3,
    point: &Vec3,
    normal: &Vec3,
) -> Vec3 {
    let large_pattern =
        (
            (point.x * 8.0).sin()
                * (point.z * 9.0).cos()
                + (point.y * 7.0).sin()
        ) * 0.035;

    let small_pattern =
        (
            (point.x * 31.0).sin()
                * (point.y * 27.0).cos()
                * (point.z * 29.0).sin()
        ) * 0.025;

    let normal_pattern =
        (
            (normal.x * 15.0).sin()
                + (normal.y * 17.0).cos()
                + (normal.z * 19.0).sin()
        ) * 0.015;

    let variation =
        large_pattern
            + small_pattern
            + normal_pattern;

    Vec3::new(
        (
            base.x
                + variation * 0.45
        )
            .clamp(0.0, 1.0),

        (
            base.y
                + variation
        )
            .clamp(0.0, 1.0),

        (
            base.z
                + variation * 0.35
        )
            .clamp(0.0, 1.0),
    )
}

fn skybox_color(
    direction: &Vec3,
) -> Color {
    let d =
        direction.normalize();

    let vertical =
        (d.y + 1.0) * 0.5;

    let nebula =
        (
            (d.x * 4.0).sin()
                * (d.y * 5.0).cos()
                * (d.z * 3.0).sin()
        )
            .abs();

    let secondary_nebula =
        (
            (d.x * 9.0 + d.z * 5.0).sin()
                * (d.y * 7.0).cos()
        )
            .abs();

    let mut r =
        0.008
            + vertical * 0.008;

    let mut g =
        0.008
            + vertical * 0.010;

    let mut b =
        0.035
            + vertical * 0.030;

    if nebula > 0.72 {
        let intensity =
            (nebula - 0.72)
                / 0.28;

        r +=
            0.05 * intensity;

        g +=
            0.015 * intensity;

        b +=
            0.10 * intensity;
    }

    if secondary_nebula > 0.82 {
        let intensity =
            (secondary_nebula - 0.82)
                / 0.18;

        r +=
            0.025 * intensity;

        g +=
            0.035 * intensity;

        b +=
            0.09 * intensity;
    }

    let star_value =
        procedural_hash(
            d.x,
            d.y,
            d.z,
        );

    if star_value > 0.994 {
        let star =
            (
                (star_value - 0.994)
                    / 0.006
            )
                .clamp(0.0, 1.0);

        let brightness =
            0.55
                + star * 0.45;

        r += brightness;
        g += brightness;
        b += brightness;
    }

    let bright_star_value =
        procedural_hash(
            d.x * 3.7 + 10.0,
            d.y * 3.1 + 4.0,
            d.z * 4.3 + 8.0,
        );

    if bright_star_value > 0.9992 {
        r = 1.0;
        g = 0.95;
        b = 0.78;
    }

    Color::new(
        (
            r.clamp(0.0, 1.0)
                * 255.0
        ) as u8,

        (
            g.clamp(0.0, 1.0)
                * 255.0
        ) as u8,

        (
            b.clamp(0.0, 1.0)
                * 255.0
        ) as u8,

        255,
    )
}

fn procedural_hash(
    x: f32,
    y: f32,
    z: f32,
) -> f32 {
    let value =
        (
            x * 127.1
                + y * 311.7
                + z * 74.7
        )
            .sin()
            * 43758.5453;

    value
        - value.floor()
}

fn reflect(
    direction: Vec3,
    normal: Vec3,
) -> Vec3 {
    direction
        - normal
            * 2.0
            * direction.dot(
                &normal,
            )
}