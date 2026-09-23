use std::f32::consts::PI;

use raylib::prelude::*;

use crate::core::camera::Camera;
use crate::core::framebuffer::Framebuffer;
use crate::core::vec3::Vec3;

use crate::materials::material::MaterialPattern;

use crate::objects::object::Object;

use crate::scene::light::Light;
use crate::scene::scene::Scene;

pub fn render(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    light: &Light,
    camera: &Camera,
) {
    let width =
        framebuffer.width as usize;

    let height =
        framebuffer.height as usize;

    framebuffer.clear(
        Color::BLACK,
    );

    let aspect_ratio =
        width as f32
            / height as f32;

    let scale =
        (
            camera
                .fov
                .to_radians()
                * 0.5
        )
            .tan();

    let camera_position =
        camera.position;

    let forward =
        (
            camera.target
                - camera.position
        )
            .normalize();

    let world_up =
        Vec3::new(
            0.0,
            1.0,
            0.0,
        );

    let right =
        forward
            .cross(
                &world_up,
            )
            .normalize();

    let up =
        right
            .cross(
                &forward,
            )
            .normalize();

    let thread_count =
        std::thread::available_parallelism()
            .map(
                |value| {
                    value.get()
                },
            )
            .unwrap_or(
                4,
            )
            .min(
                height,
            );

    let rows_per_thread =
        (
            height
                + thread_count
                - 1
        )
            / thread_count;

    let pixels =
        framebuffer.pixels_mut();

    let pixels_per_chunk =
        rows_per_thread
            * width;

    std::thread::scope(
        |scope| {
            for (
                chunk_index,
                pixel_chunk,
            ) in pixels
                .chunks_mut(
                    pixels_per_chunk,
                )
                .enumerate()
            {
                let start_y =
                    chunk_index
                        * rows_per_thread;

                scope.spawn(
                    move || {
                        let rows_in_chunk =
                            pixel_chunk.len()
                                / width;

                        for local_y in
                            0..rows_in_chunk
                        {
                            let y =
                                start_y
                                    + local_y;

                            let py =
                                (
                                    1.0
                                        - 2.0
                                            * (
                                                (
                                                    y as f32
                                                        + 0.5
                                                )
                                                    / height
                                                        as f32
                                            )
                                )
                                    * scale;

                            for x in
                                0..width
                            {
                                let px =
                                    (
                                        2.0
                                            * (
                                                (
                                                    x as f32
                                                        + 0.5
                                                )
                                                    / width
                                                        as f32
                                            )
                                            - 1.0
                                    )
                                        * aspect_ratio
                                        * scale;

                                let direction =
                                    (
                                        forward
                                            + right
                                                * px
                                            + up
                                                * py
                                    )
                                        .normalize();

                                let color =
                                    cast_ray(
                                        &camera_position,
                                        &direction,
                                        scene,
                                        light,
                                        0,
                                    );

                                let index =
                                    local_y
                                        * width
                                        + x;

                                pixel_chunk[
                                    index
                                ] =
                                    to_color(
                                        color,
                                    );
                            }
                        }
                    },
                );
            }
        },
    );
}

fn cast_ray(
    origin: &Vec3,
    direction: &Vec3,
    scene: &Scene,
    light: &Light,
    depth: u32,
) -> Vec3 {
    if depth > 4 {
        return skybox_color(
            direction,
        );
    }

    let (
        object_index,
        closest_distance,
    ) =
        match scene
            .bvh
            .intersect(
                origin,
                direction,
                &scene.objects,
            )
        {
            Some(hit) => {
                hit
            }

            None => {
                return skybox_color(
                    direction,
                );
            }
        };

    let object =
        &scene.objects[
            object_index
        ];

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
    ) =
        object_uv(
            object,
            &hit_point,
            &geometric_normal,
        );

    let mut surface_color =
        if let Some(
            texture,
        ) =
            material.albedo_texture
        {
            texture.sample(
                u,
                v,
            )
        } else {
            match material.pattern {
                MaterialPattern::Solid => {
                    material.color
                }

                MaterialPattern::Grass => {
                    procedural_grass(
                        &hit_point,
                        &material.color,
                    )
                }
            }
        };

    if material
        .albedo_texture
        .is_some()
    {
        surface_color =
            multiply_vec3(
                surface_color,
                material.color,
            );
    }

    let mut normal =
        geometric_normal;

    if let Some(
        normal_map,
    ) =
        material.normal_texture
    {
        let sampled =
            normal_map.sample(
                u,
                v,
            );

        let tangent_normal =
            Vec3::new(
                sampled.x
                    * 2.0
                    - 1.0,

                sampled.y
                    * 2.0
                    - 1.0,

                sampled.z
                    * 2.0
                    - 1.0,
            )
                .normalize();

        let (
            tangent,
            bitangent,
        ) =
            tangent_basis(
                object,
                &hit_point,
                &geometric_normal,
            );

        normal =
            (
                tangent
                    * tangent_normal.x

                    + bitangent
                        * tangent_normal.y

                    + geometric_normal
                        * tangent_normal.z
            )
                .normalize();
    }

    let ao =
        match material.ao_texture {
            Some(
                ao_map,
            ) => {
                ao_map
                    .sample_scalar(
                        u,
                        v,
                    )
                    .clamp(
                        0.0,
                        1.0,
                    )
            }

            None => {
                1.0
            }
        };

    let roughness =
        match material
            .roughness_texture
        {
            Some(
                roughness_map,
            ) => {
                roughness_map
                    .sample_scalar(
                        u,
                        v,
                    )
                    .clamp(
                        0.0,
                        1.0,
                    )
            }

            None => {
                0.5
            }
        };

    let to_light =
        light.position
            - hit_point;

    let light_distance =
        to_light.length();

    let light_direction =
        to_light
            / light_distance;

    let shadow_origin =
        hit_point
            + normal
                * 0.003;

    let in_shadow =
        scene
            .bvh
            .any_hit(
                &shadow_origin,
                &light_direction,
                light_distance
                    - 0.005,
                &scene.objects,
            );

    let ambient =
        0.12
            * ao;

    let diffuse_factor =
        if in_shadow {
            0.0
        } else {
            normal
                .dot(
                    &light_direction,
                )
                .max(
                    0.0,
                )
        };

    let diffuse =
        diffuse_factor
            * material.albedo
            * light.intensity
            * ao;

    let view_direction =
        (-*direction)
            .normalize();

    let reflected_light =
        reflect(
            -light_direction,
            normal,
        )
            .normalize();

    let gloss =
        (
            1.0
                - roughness
        )
            .clamp(
                0.05,
                1.0,
            );

    let shininess =
        8.0
            + gloss
                * 120.0;

    let specular_factor =
        if in_shadow {
            0.0
        } else {
            view_direction
                .dot(
                    &reflected_light,
                )
                .max(
                    0.0,
                )
                .powf(
                    shininess,
                )
        };

    let specular =
        specular_factor
            * material.specular
            * gloss
            * light.intensity;

    let lighting =
        ambient
            + diffuse;

    let mut final_color =
        surface_color
            * lighting;

    final_color =
        final_color
            + light.color
                * specular;

    let transparency =
        material
            .transparency
            .clamp(
                0.0,
                1.0,
            );

    if transparency
        > 0.001
    {
        let transparent_origin =
            hit_point
                + *direction
                    * 0.01;

        let behind =
            cast_ray(
                &transparent_origin,
                direction,
                scene,
                light,
                depth + 1,
            );

        final_color =
            final_color
                * (
                    1.0
                        - transparency
                )
                + behind
                    * transparency;
    }

    final_color
}

fn object_uv(
    object: &Object,
    point: &Vec3,
    normal: &Vec3,
) -> (f32, f32) {
    match object {
        Object::Sphere(
            sphere,
        ) => {
            sphere_uv(
                point,
                &sphere.center,
            )
        }

        Object::Cylinder(
            cylinder,
        ) => {
            cylinder_uv(
                point,
                &cylinder.center,
                &cylinder.axis,
                cylinder.radius,
                cylinder.height,
            )
        }

        Object::Cone(
            cone,
        ) => {
            cone_uv(
                point,
                &cone.center,
                &cone.axis,
                cone.radius,
                cone.height,
            )
        }

        Object::Plane(
            _plane,
        ) => {
            planar_uv(
                point,
                normal,
            )
        }
    }
}

fn sphere_uv(
    point: &Vec3,
    center: &Vec3,
) -> (f32, f32) {
    let n =
        (
            *point
                - *center
        )
            .normalize();

    let u =
        0.5
            + n.z
                .atan2(
                    n.x,
                )
                / (
                    2.0
                        * PI
                );

    let v =
        0.5
            - n.y
                .clamp(
                    -1.0,
                    1.0,
                )
                .asin()
                / PI;

    (
        u.rem_euclid(
            1.0,
        ),

        v.rem_euclid(
            1.0,
        ),
    )
}

fn cylinder_uv(
    point: &Vec3,
    center: &Vec3,
    axis: &Vec3,
    radius: f32,
    height: f32,
) -> (f32, f32) {
    let axis =
        axis.normalize();

    let local =
        *point
            - *center;

    let axial =
        local.dot(
            &axis,
        );

    let radial =
        local
            - axis
                * axial;

    let (
        tangent,
        bitangent,
    ) =
        axis_basis(
            &axis,
        );

    let x =
        radial.dot(
            &tangent,
        );

    let z =
        radial.dot(
            &bitangent,
        );

    let half_height =
        height
            * 0.5;

    let cap_epsilon =
        0.003;

    if (
        axial.abs()
            - half_height
    )
        .abs()
        < cap_epsilon
    {
        let u =
            0.5
                + x
                    / (
                        radius
                            * 2.0
                    );

        let v =
            0.5
                + z
                    / (
                        radius
                            * 2.0
                    );

        return (
            u.clamp(
                0.0,
                1.0,
            ),

            v.clamp(
                0.0,
                1.0,
            ),
        );
    }

    let angle =
        z.atan2(
            x,
        );

    let u =
        0.5
            + angle
                / (
                    2.0
                        * PI
                );

    let v =
        (
            axial
                + half_height
        )
            / height;

    let u_scale =
        2.0;

    let v_scale =
        1.5;

    (
        (
            u
                * u_scale
        )
            .rem_euclid(
                1.0,
            ),

        (
            v
                * v_scale
        )
            .rem_euclid(
                1.0,
            ),
    )
}

fn cone_uv(
    point: &Vec3,
    center: &Vec3,
    axis: &Vec3,
    radius: f32,
    height: f32,
) -> (f32, f32) {
    let axis =
        axis.normalize();

    let local =
        *point
            - *center;

    let axial =
        local.dot(
            &axis,
        );

    let radial =
        local
            - axis
                * axial;

    let (
        tangent,
        bitangent,
    ) =
        axis_basis(
            &axis,
        );

    let x =
        radial.dot(
            &tangent,
        );

    let z =
        radial.dot(
            &bitangent,
        );

    let half_height =
        height
            * 0.5;

    let base_distance =
        (
            axial
                + half_height
        )
            .abs();

    if base_distance
        < 0.003
    {
        let u =
            0.5
                + x
                    / (
                        radius
                            * 2.0
                    );

        let v =
            0.5
                + z
                    / (
                        radius
                            * 2.0
                    );

        return (
            u.clamp(
                0.0,
                1.0,
            ),

            v.clamp(
                0.0,
                1.0,
            ),
        );
    }

    let angle =
        z.atan2(
            x,
        );

    let u =
        0.5
            + angle
                / (
                    2.0
                        * PI
                );

    let v =
        (
            axial
                + half_height
        )
            / height;

    let u_scale =
        2.0;

    let v_scale =
        1.0;

    (
        (
            u
                * u_scale
        )
            .rem_euclid(
                1.0,
            ),

        (
            v
                * v_scale
        )
            .rem_euclid(
                1.0,
            ),
    )
}

fn planar_uv(
    point: &Vec3,
    normal: &Vec3,
) -> (f32, f32) {
    let (
        tangent,
        bitangent,
    ) =
        axis_basis(
            normal,
        );

    let scale =
        0.5;

    let u =
        point.dot(
            &tangent,
        )
            * scale;

    let v =
        point.dot(
            &bitangent,
        )
            * scale;

    (
        u.rem_euclid(
            1.0,
        ),

        v.rem_euclid(
            1.0,
        ),
    )
}

fn tangent_basis(
    object: &Object,
    point: &Vec3,
    normal: &Vec3,
) -> (Vec3, Vec3) {
    match object {
        Object::Cylinder(
            cylinder,
        ) => {
            let axis =
                cylinder
                    .axis
                    .normalize();

            let local =
                *point
                    - cylinder.center;

            let axial =
                local.dot(
                    &axis,
                );

            let radial =
                local
                    - axis
                        * axial;

            if radial.length()
                > 0.001
            {
                let tangent =
                    axis
                        .cross(
                            &radial,
                        )
                        .normalize();

                let bitangent =
                    normal
                        .cross(
                            &tangent,
                        )
                        .normalize();

                (
                    tangent,
                    bitangent,
                )
            } else {
                axis_basis(
                    normal,
                )
            }
        }

        Object::Cone(
            cone,
        ) => {
            let axis =
                cone
                    .axis
                    .normalize();

            let local =
                *point
                    - cone.center;

            let axial =
                local.dot(
                    &axis,
                );

            let radial =
                local
                    - axis
                        * axial;

            if radial.length()
                > 0.001
            {
                let tangent =
                    axis
                        .cross(
                            &radial,
                        )
                        .normalize();

                let bitangent =
                    normal
                        .cross(
                            &tangent,
                        )
                        .normalize();

                (
                    tangent,
                    bitangent,
                )
            } else {
                axis_basis(
                    normal,
                )
            }
        }

        Object::Sphere(
            _sphere,
        ) => {
            axis_basis(
                normal,
            )
        }

        Object::Plane(
            _plane,
        ) => {
            axis_basis(
                normal,
            )
        }
    }
}

fn axis_basis(
    axis: &Vec3,
) -> (Vec3, Vec3) {
    let axis =
        axis.normalize();

    let helper =
        if axis.y.abs()
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
                &axis,
            )
            .normalize();

    let bitangent =
        axis
            .cross(
                &tangent,
            )
            .normalize();

    (
        tangent,
        bitangent,
    )
}

fn procedural_grass(
    point: &Vec3,
    base_color: &Vec3,
) -> Vec3 {
    let noise =
        (
            (
                point.x
                    * 18.0

                    + point.z
                        * 13.0

                    + point.y
                        * 9.0
            )
                .sin()
                * 0.5
                + 0.5
        )
            * 0.15;

    Vec3::new(
        (
            base_color.x
                + noise
                    * 0.30
        )
            .clamp(
                0.0,
                1.0,
            ),

        (
            base_color.y
                + noise
        )
            .clamp(
                0.0,
                1.0,
            ),

        (
            base_color.z
                + noise
                    * 0.20
        )
            .clamp(
                0.0,
                1.0,
            ),
    )
}

fn multiply_vec3(
    a: Vec3,
    b: Vec3,
) -> Vec3 {
    Vec3::new(
        a.x
            * b.x,

        a.y
            * b.y,

        a.z
            * b.z,
    )
}

fn reflect(
    direction: Vec3,
    normal: Vec3,
) -> Vec3 {
    direction
        - normal
            * (
                2.0
                    * direction.dot(
                        &normal,
                    )
            )
}

fn skybox_color(
    direction: &Vec3,
) -> Vec3 {
    let d =
        direction.normalize();

    let mut color =
        Vec3::new(
            0.003,
            0.006,
            0.018,
        );

    let nebula =
        (
            d.x
                * 4.0

                + d.y
                    * 2.5

                + d.z
                    * 3.0
        )
            .sin()
            * 0.5
            + 0.5;

    color =
        color
            + Vec3::new(
                0.012,
                0.008,
                0.025,
            )
                * nebula
                * 0.35;

    let sx =
        (
            d.x
                * 900.0
        )
            .floor()
            as i32;

    let sy =
        (
            d.y
                * 900.0
        )
            .floor()
            as i32;

    let sz =
        (
            d.z
                * 900.0
        )
            .floor()
            as i32;

    let star =
        procedural_hash(
            sx,
            sy,
            sz,
        );

    if star
        > 0.9975
    {
        let brightness =
            (
                star
                    - 0.9975
            )
                / 0.0025;

        color =
            color
                + Vec3::new(
                    1.0,
                    1.0,
                    1.0,
                )
                    * brightness;
    }

    color
}

fn procedural_hash(
    x: i32,
    y: i32,
    z: i32,
) -> f32 {
    let mut n =
        x
            .wrapping_mul(
                374761393,
            )
            .wrapping_add(
                y
                    .wrapping_mul(
                        668265263,
                    ),
            )
            .wrapping_add(
                z
                    .wrapping_mul(
                        2147483647,
                    ),
            );

    n =
        (
            n
                ^ (
                    n >> 13
                )
        )
            .wrapping_mul(
                1274126177,
            );

    let value =
        n
            ^ (
                n >> 16
            );

    (
        value as u32
            & 0x00FF_FFFF
    ) as f32
        / 0x00FF_FFFF
            as f32
}

fn to_color(
    color: Vec3,
) -> Color {
    Color::new(
        (
            color.x
                .clamp(
                    0.0,
                    1.0,
                )
                * 255.0
        ) as u8,

        (
            color.y
                .clamp(
                    0.0,
                    1.0,
                )
                * 255.0
        ) as u8,

        (
            color.z
                .clamp(
                    0.0,
                    1.0,
                )
                * 255.0
        ) as u8,

        255,
    )
}