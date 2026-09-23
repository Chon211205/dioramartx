use std::f32::consts::PI;

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

    let width =
        framebuffer.width as f32;

    let height =
        framebuffer.height as f32;

    let aspect_ratio =
        width / height;

    let forward =
        (camera.target - camera.position)
            .normalize();

    let world_up =
        Vec3::new(
            0.0,
            1.0,
            0.0,
        );

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
            camera.fov.to_radians()
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

    let mut closest_distance =
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
            if distance
                < closest_distance
            {
                closest_distance =
                    distance;

                closest_object =
                    Some(object);
            }
        }
    }

    let Some(object) =
        closest_object
    else {
        return (
            skybox_color(direction),
            f32::INFINITY,
        );
    };

    let hit_point =
        *origin
            + *direction
                * closest_distance;

    let geometric_normal =
        object
            .normal_at(
                &hit_point,
            )
            .normalize();

    let material =
        object.material();

    let (
        u,
        v,
    ) = spherical_uv(
        &geometric_normal,
    );

    let surface_color =
        get_material_color(
            &material,
            &hit_point,
            &geometric_normal,
            u,
            v,
        );

    let shading_normal =
        get_shading_normal(
            &material,
            &geometric_normal,
            u,
            v,
        );

    let ao =
        get_ao_factor(
            &material,
            u,
            v,
        );

    let specular_strength =
        get_specular_strength(
            &material,
            u,
            v,
        );

    let light_direction =
        (
            light.position
                - hit_point
        )
            .normalize();

    let diffuse =
        shading_normal
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

    let reflected_light =
        reflect(
            -light_direction,
            shading_normal,
        )
            .normalize();

    let specular_intensity =
        view_direction
            .dot(
                &reflected_light,
            )
            .max(0.0)
            .powf(32.0);

    let ambient =
        0.12_f32
            * (
                0.45_f32
                    + ao
                        * 0.55_f32
            );

    let diffuse_component =
        diffuse
            * material.albedo
            * light.intensity
            * (
                0.65_f32
                    + ao
                        * 0.35_f32
            );

    let specular_component =
        specular_intensity
            * specular_strength
            * light.intensity;

    let red =
        (
            surface_color.x
                * light.color.x
                * (
                    ambient
                        + diffuse_component
                )
                + specular_component
        )
            .clamp(
                0.0_f32,
                1.0_f32,
            );

    let green =
        (
            surface_color.y
                * light.color.y
                * (
                    ambient
                        + diffuse_component
                )
                + specular_component
        )
            .clamp(
                0.0_f32,
                1.0_f32,
            );

    let blue =
        (
            surface_color.z
                * light.color.z
                * (
                    ambient
                        + diffuse_component
                )
                + specular_component
        )
            .clamp(
                0.0_f32,
                1.0_f32,
            );

    let rendered_color =
        Color::new(
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
            255,
        );

    if material.transparency > 0.0 {
        let epsilon =
            0.002_f32;

        let transparent_origin =
            hit_point
                + *direction
                    * epsilon;

        let (
            behind_color,
            _,
        ) = cast_ray(
            &transparent_origin,
            direction,
            objects,
            light,
            depth + 1,
        );

        let transparency =
            material
                .transparency
                .clamp(
                    0.0_f32,
                    1.0_f32,
                );

        let opacity =
            1.0_f32
                - transparency;

        let final_red =
            rendered_color.r as f32
                * opacity
                + behind_color.r as f32
                    * transparency;

        let final_green =
            rendered_color.g as f32
                * opacity
                + behind_color.g as f32
                    * transparency;

        let final_blue =
            rendered_color.b as f32
                * opacity
                + behind_color.b as f32
                    * transparency;

        return (
            Color::new(
                final_red
                    .clamp(
                        0.0,
                        255.0,
                    ) as u8,

                final_green
                    .clamp(
                        0.0,
                        255.0,
                    ) as u8,

                final_blue
                    .clamp(
                        0.0,
                        255.0,
                    ) as u8,

                255,
            ),
            closest_distance,
        );
    }

    (
        rendered_color,
        closest_distance,
    )
}

fn get_material_color(
    material: &Material,
    point: &Vec3,
    normal: &Vec3,
    u: f32,
    v: f32,
) -> Vec3 {
    if let Some(texture) =
        material.albedo_texture
    {
        return texture.sample(
            u,
            v,
        );
    }

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

fn get_shading_normal(
    material: &Material,
    geometric_normal: &Vec3,
    u: f32,
    v: f32,
) -> Vec3 {
    let Some(normal_texture) =
        material.normal_texture
    else {
        return *geometric_normal;
    };

    let sampled =
        normal_texture.sample(
            u,
            v,
        );

    let tangent_space_normal =
        Vec3::new(
            sampled.x * 2.0 - 1.0,
            sampled.y * 2.0 - 1.0,
            sampled.z * 2.0 - 1.0,
        )
            .normalize();

    let helper =
        if geometric_normal.y.abs()
            < 0.999
        {
            Vec3::new(
                0.0,
                1.0,
                0.0,
            )
        } else {
            Vec3::new(
                1.0,
                0.0,
                0.0,
            )
        };

    let tangent =
        helper
            .cross(
                geometric_normal,
            )
            .normalize();

    let bitangent =
        geometric_normal
            .cross(
                &tangent,
            )
            .normalize();

    (
        tangent
            * tangent_space_normal.x

            + bitangent
                * tangent_space_normal.y

            + *geometric_normal
                * tangent_space_normal.z
    )
        .normalize()
}

fn get_ao_factor(
    material: &Material,
    u: f32,
    v: f32,
) -> f32 {
    let Some(texture) =
        material.ao_texture
    else {
        return 1.0;
    };

    let ao =
        texture
            .sample_scalar(
                u,
                v,
            )
            .clamp(
                0.0_f32,
                1.0_f32,
            );

    (
        0.40_f32
            + ao * 0.60_f32
    )
        .clamp(
            0.0_f32,
            1.0_f32,
        )
}

fn get_specular_strength(
    material: &Material,
    u: f32,
    v: f32,
) -> f32 {
    let Some(texture) =
        material.roughness_texture
    else {
        return material.specular;
    };

    let roughness =
        texture
            .sample_scalar(
                u,
                v,
            )
            .clamp(
                0.0_f32,
                1.0_f32,
            );

    let gloss =
        (
            1.0_f32
                - roughness
        )
            .clamp(
                0.05_f32,
                1.0_f32,
            );

    material.specular
        * gloss
}

fn spherical_uv(
    normal: &Vec3,
) -> (f32, f32) {
    let n =
        normal.normalize();

    let u =
        0.5_f32
            + n.z.atan2(
                n.x,
            )
                / (
                    2.0_f32
                        * PI
                );

    let v =
        0.5_f32
            - n.y
                .clamp(
                    -1.0_f32,
                    1.0_f32,
                )
                .asin()
                / PI;

    let texture_scale =
        1.25_f32;

    (
        (
            u
                * texture_scale
        )
            .rem_euclid(
                1.0_f32,
            ),

        (
            v
                * texture_scale
        )
            .rem_euclid(
                1.0_f32,
            ),
    )
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
        )
            * 0.035;

    let small_pattern =
        (
            (point.x * 31.0).sin()
                * (point.y * 27.0).cos()
                * (point.z * 29.0).sin()
        )
            * 0.025;

    let normal_pattern =
        (
            (normal.x * 15.0).sin()
                + (normal.y * 17.0).cos()
                + (normal.z * 19.0).sin()
        )
            * 0.015;

    let variation =
        large_pattern
            + small_pattern
            + normal_pattern;

    Vec3::new(
        (
            base.x
                + variation * 0.45
        )
            .clamp(
                0.0,
                1.0,
            ),

        (
            base.y
                + variation
        )
            .clamp(
                0.0,
                1.0,
            ),

        (
            base.z
                + variation * 0.35
        )
            .clamp(
                0.0,
                1.0,
            ),
    )
}

fn skybox_color(
    direction: &Vec3,
) -> Color {
    let d =
        direction.normalize();

    let vertical =
        (
            d.y
                + 1.0
        )
            * 0.5;

    let nebula =
        (
            (d.x * 4.0).sin()
                * (d.y * 5.0).cos()
                * (d.z * 3.0).sin()
        )
            .abs();

    let secondary_nebula =
        (
            (
                d.x * 9.0
                    + d.z * 5.0
            )
                .sin()
                * (d.y * 7.0).cos()
        )
            .abs();

    let mut red =
        0.008
            + vertical * 0.008;

    let mut green =
        0.008
            + vertical * 0.010;

    let mut blue =
        0.035
            + vertical * 0.030;

    if nebula > 0.72 {
        let intensity =
            (
                nebula - 0.72
            )
                / 0.28;

        red +=
            0.05 * intensity;

        green +=
            0.015 * intensity;

        blue +=
            0.10 * intensity;
    }

    if secondary_nebula > 0.82 {
        let intensity =
            (
                secondary_nebula
                    - 0.82
            )
                / 0.18;

        red +=
            0.025 * intensity;

        green +=
            0.035 * intensity;

        blue +=
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
                (
                    star_value
                        - 0.994
                )
                    / 0.006
            )
                .clamp(
                    0.0,
                    1.0,
                );

        let brightness =
            0.55
                + star * 0.45;

        red += brightness;
        green += brightness;
        blue += brightness;
    }

    let bright_star =
        procedural_hash(
            d.x * 3.7 + 10.0,
            d.y * 3.1 + 4.0,
            d.z * 4.3 + 8.0,
        );

    if bright_star > 0.9992 {
        red = 1.0;
        green = 0.95;
        blue = 0.78;
    }

    Color::new(
        (
            red
                .clamp(
                    0.0,
                    1.0,
                )
                * 255.0
        ) as u8,

        (
            green
                .clamp(
                    0.0,
                    1.0,
                )
                * 255.0
        ) as u8,

        (
            blue
                .clamp(
                    0.0,
                    1.0,
                )
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

    value - value.floor()
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