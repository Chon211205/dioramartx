use std::f32::consts::PI;
use std::thread;

use raylib::prelude::Color;

use crate::core::camera::Camera;
use crate::core::framebuffer::Framebuffer;
use crate::core::vec3::Vec3;

use crate::materials::material::{Material, MaterialPattern};

use crate::objects::cone::Cone;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::ellipsoid::Ellipsoid;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::object::Object;
use crate::objects::plane::Plane;
use crate::objects::sphere::Sphere;
use crate::objects::torus::Torus;

use crate::scene::light::Light;
use crate::scene::scene::Scene;

const MAX_DEPTH: u32 = 4;
const EPSILON: f32 = 0.002;

pub fn render(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    light: &Light,
    camera: &Camera,
) {
    render_rotated_with_skybox(
        framebuffer,
        scene,
        light,
        camera,
        0.0,
        0,
    );
}

pub fn render_rotated(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    light: &Light,
    camera: &Camera,
    rotation_y: f32,
) {
    render_rotated_with_skybox(
        framebuffer,
        scene,
        light,
        camera,
        rotation_y,
        0,
    );
}

pub fn render_with_skybox(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    light: &Light,
    camera: &Camera,
    galaxy_index: usize,
) {
    render_rotated_with_skybox(
        framebuffer,
        scene,
        light,
        camera,
        0.0,
        galaxy_index,
    );
}

pub fn render_rotated_with_skybox(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    light: &Light,
    camera: &Camera,
    rotation_y: f32,
    galaxy_index: usize,
) {
    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;

    let thread_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .max(1);

    let rows_per_thread =
        (height + thread_count - 1)
            / thread_count;

    let chunk_size =
        rows_per_thread
            * width;

    let pixels =
        framebuffer.pixels_mut();

    thread::scope(|scope| {
        for (
            chunk_index,
            chunk,
        ) in pixels
            .chunks_mut(chunk_size)
            .enumerate()
        {
            scope.spawn(move || {
                let start_row =
                    chunk_index
                        * rows_per_thread;

                for local_index in 0..chunk.len() {
                    let local_y =
                        local_index
                            / width;

                    let x =
                        local_index
                            % width;

                    let y =
                        start_row
                            + local_y;

                    if y >= height {
                        continue;
                    }

                    let ray =
                        camera.get_ray(
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            width as f32,
                            height as f32,
                        );

                    let color =
                        cast_ray(
                            &ray.origin,
                            &ray.direction,
                            scene,
                            light,
                            0,
                            rotation_y,
                            galaxy_index,
                        );

                    chunk[local_index] =
                        to_color(
                            color,
                        );
                }
            });
        }
    });
}

fn cast_ray(
    origin: &Vec3,
    direction: &Vec3,
    scene: &Scene,
    light: &Light,
    depth: u32,
    rotation_y: f32,
    galaxy_index: usize,
) -> Vec3 {
    if depth >= MAX_DEPTH {
        return skybox_color_for_galaxy(
            direction,
            galaxy_index,
        );
    }

    let hit =
        scene
            .bvh
            .intersect(
                origin,
                direction,
                &scene.objects,
            );

    let (
        object_index,
        distance,
    ) = match hit {
        Some(value) => value,

        None => {
            return skybox_color_for_galaxy(
                direction,
                galaxy_index,
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
                * distance;

    let geometric_normal =
        object
            .normal_at(
                &hit_point,
            )
            .normalize();

    let front_face =
        direction
            .dot(
                &geometric_normal,
            )
            < 0.0;

    let mut normal =
        if front_face {
            geometric_normal
        } else {
            -geometric_normal
        };

    let material =
        object.material_at(
            &hit_point,
        );

    let (
        u,
        v,
    ) =
        object_uv(
            object,
            &hit_point,
            rotation_y,
        );

    let mut surface_color =
        material.color;

    match material.pattern {
        MaterialPattern::Kirby => {
            surface_color =
                kirby_decal_color(
                    &material,
                    u,
                    v,
                );
        }

        _ => {
            if let Some(texture) =
                material.albedo_texture
            {
                let texture_color =
                    texture.sample(
                        u,
                        v,
                    );

                surface_color =
                    multiply_vec3(
                        surface_color,
                        texture_color,
                    );
            } else {
                match material.pattern {
                    MaterialPattern::Grass => {
                        surface_color =
                            multiply_vec3(
                                surface_color,
                                procedural_grass(
                                    u,
                                    v,
                                    &hit_point,
                                ),
                            );
                    }

                    MaterialPattern::Solid => {}

                    MaterialPattern::Kirby => {}
                }
            }
        }
    }

    if let Some(normal_texture) =
        material.normal_texture
    {
        let sample =
            normal_texture
                .sample(
                    u,
                    v,
                );

        let tangent_normal =
            Vec3::new(
                sample.x * 2.0 - 1.0,
                sample.y * 2.0 - 1.0,
                sample.z * 2.0 - 1.0,
            )
            .normalize();

        let (
            tangent,
            bitangent,
        ) =
            object_tangent_basis(
                object,
                &hit_point,
                normal,
            );

        normal =
            (
                tangent
                    * tangent_normal.x
                    + bitangent
                        * tangent_normal.y
                    + normal
                        * tangent_normal.z
            )
                .normalize();

        if direction
            .dot(
                &normal,
            )
            > 0.0
        {
            normal =
                -normal;
        }
    }

    let roughness =
        material
            .roughness_texture
            .map(
                |texture| {
                    texture
                        .sample_scalar(
                            u,
                            v,
                        )
                        .clamp(
                            0.0,
                            1.0,
                        )
                },
            )
            .unwrap_or(
                0.5,
            );

    let ao =
        material
            .ao_texture
            .map(
                |texture| {
                    texture
                        .sample_scalar(
                            u,
                            v,
                        )
                        .clamp(
                            0.0,
                            1.0,
                        )
                },
            )
            .unwrap_or(
                1.0,
            );

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
                * EPSILON;

    let in_shadow =
        if normal
            .dot(
                &light_direction,
            )
            <= 0.0
        {
            true
        } else {
            scene
                .bvh
                .any_hit(
                    &shadow_origin,
                    &light_direction,
                    light_distance
                        - EPSILON,
                    &scene.objects,
                )
        };

    let ndotl =
        normal
            .dot(
                &light_direction,
            )
            .max(
                0.0,
            );

    let ambient =
        0.22
            * ao;

    let diffuse_factor =
        if in_shadow {
            ndotl
                * 0.18
        } else {
            ndotl
        };

    let view_direction =
        -*direction;

    let reflected_light =
        reflect(
            -light_direction,
            normal,
        )
        .normalize();

    let specular_dot =
        reflected_light
            .dot(
                &view_direction,
            )
            .max(
                0.0,
            );

    let shininess =
        8.0
            + (
                1.0
                    - roughness
            )
                * 120.0;

    let mut specular =
        specular_dot
            .powf(
                shininess,
            )
            * material.specular
            * (
                1.0
                    - roughness
                        * 0.65
            );

    if in_shadow {
        specular *=
            0.08;
    }

    let diffuse_light =
        ambient
            + diffuse_factor
                * material.albedo
                * light.intensity;

    let mut final_color =
        surface_color
            * diffuse_light;

    final_color =
        final_color
            + light.color
                * (
                    specular
                        * light.intensity
                );

    let reflectivity =
        material
            .reflectivity
            .clamp(
                0.0,
                1.0,
            );

    if reflectivity > 0.001 {
        let reflected_direction =
            reflect(
                *direction,
                normal,
            )
            .normalize();

        let reflected_origin =
            hit_point
                + normal
                    * EPSILON;

        let reflected_color =
            cast_ray(
                &reflected_origin,
                &reflected_direction,
                scene,
                light,
                depth + 1,
                rotation_y,
                galaxy_index,
            );

        final_color =
            final_color
                * (
                    1.0
                        - reflectivity
                )
                + reflected_color
                    * reflectivity;
    }

    let transparency =
        material
            .transparency
            .clamp(
                0.0,
                1.0,
            );

    if transparency > 0.001 {
        let eta =
            if front_face {
                1.0 / 1.33
            } else {
                1.33
            };

        let refracted_direction =
            refract(
                *direction,
                normal,
                eta,
            )
            .unwrap_or(
                *direction,
            )
            .normalize();

        let refracted_origin =
            hit_point
                + refracted_direction
                    * (
                        EPSILON
                            * 2.0
                    );

        let refracted_color =
            cast_ray(
                &refracted_origin,
                &refracted_direction,
                scene,
                light,
                depth + 1,
                rotation_y,
                galaxy_index,
            );

        final_color =
            final_color
                * (
                    1.0
                        - transparency
                )
                + refracted_color
                    * transparency;
    }

    final_color
}

fn object_uv(
    object: &Object,
    point: &Vec3,
    rotation_y: f32,
) -> (f32, f32) {
    match object {
        Object::Sphere(sphere) =>
            sphere_uv(
                sphere,
                point,
                rotation_y,
            ),

        Object::Plane(plane) =>
            planar_uv(
                plane,
                point,
            ),

        Object::Cylinder(cylinder) =>
            cylinder_uv(
                cylinder,
                point,
                rotation_y,
            ),

        Object::Cone(cone) =>
            cone_uv(
                cone,
                point,
            ),

        Object::Cube(cube) =>
            cube_uv(
                cube,
                point,
            ),

        Object::Hemisphere(hemisphere) =>
            hemisphere_uv(
                hemisphere,
                point,
                rotation_y,
            ),

        Object::Torus(torus) =>
            torus_uv(
                torus,
                point,
                rotation_y,
            ),

        Object::Ellipsoid(ellipsoid) =>
            ellipsoid_uv(
                ellipsoid,
                point,
                rotation_y,
            ),
    }
}

fn rotate_y_inverse(
    point: Vec3,
    angle: f32,
) -> Vec3 {
    let c =
        angle.cos();

    let s =
        angle.sin();

    Vec3::new(
        point.x * c
            - point.z * s,
        point.y,
        point.x * s
            + point.z * c,
    )
}

fn sphere_uv(
    sphere: &Sphere,
    point: &Vec3,
    rotation_y: f32,
) -> (f32, f32) {
    let local =
        (
            *point
                - sphere.center
        )
            .normalize();

    let p =
        rotate_y_inverse(
            local,
            rotation_y,
        );

    let u =
        0.5
            + p.z.atan2(
                p.x,
            )
                / (
                    2.0
                        * PI
                );

    let v =
        0.5
            - p.y
                .clamp(
                    -1.0,
                    1.0,
                )
                .asin()
                / PI;

    (
        u,
        v,
    )
}

fn planar_uv(
    plane: &Plane,
    point: &Vec3,
) -> (f32, f32) {
    let normal =
        plane.normal
            .normalize();

    let (
        tangent,
        bitangent,
    ) =
        tangent_basis(
            normal,
        );

    let local =
        *point
            - plane.point;

    (
        local.dot(
            &tangent,
        ) * 0.5,

        local.dot(
            &bitangent,
        ) * 0.5,
    )
}

fn cylinder_uv(
    cylinder: &Cylinder,
    point: &Vec3,
    rotation_y: f32,
) -> (f32, f32) {
    let local_world =
        *point
            - cylinder.center;

    let local =
        rotate_y_inverse(
            local_world,
            rotation_y,
        );

    let axis =
        rotate_y_inverse(
            cylinder.axis,
            rotation_y,
        )
        .normalize();

    let axial =
        local.dot(
            &axis,
        );

    let (
        tangent,
        bitangent,
    ) =
        axis_basis(
            axis,
        );

    let radial =
        local
            - axis
                * axial;

    let x =
        radial.dot(
            &tangent,
        );

    let z =
        radial.dot(
            &bitangent,
        );

    let half_height =
        cylinder.height
            * 0.5;

    let cap_epsilon =
        0.015;

    if axial.abs()
        >= half_height
            - cap_epsilon
    {
        let u =
            0.5
                + x
                    / (
                        cylinder.radius
                            * 2.0
                    );

        let v =
            0.5
                + z
                    / (
                        cylinder.radius
                            * 2.0
                    );

        return (
            u,
            v,
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
        axial
            / cylinder.height
            + 0.5;

    (
        u * 2.0,
        v * 1.5,
    )
}

fn cone_uv(
    cone: &Cone,
    point: &Vec3,
) -> (f32, f32) {
    let axis =
        cone.axis
            .normalize();

    let local =
        *point
            - cone.center;

    let axial =
        local.dot(
            &axis,
        );

    let (
        tangent,
        bitangent,
    ) =
        axis_basis(
            axis,
        );

    let radial =
        local
            - axis
                * axial;

    let x =
        radial.dot(
            &tangent,
        );

    let z =
        radial.dot(
            &bitangent,
        );

    let u =
        0.5
            + z.atan2(
                x,
            )
                / (
                    2.0
                        * PI
                );

    let v =
        axial
            / cone.height
            + 0.5;

    (
        u * 2.0,
        v,
    )
}

fn cube_uv(
    cube: &Cube,
    point: &Vec3,
) -> (f32, f32) {
    let delta =
        *point
            - cube.center;

    let local =
        Vec3::new(
            delta.dot(
                &cube.right,
            ),
            delta.dot(
                &cube.up,
            ),
            delta.dot(
                &cube.forward,
            ),
        )
            / cube.half_size;

    let ax =
        local.x.abs();

    let ay =
        local.y.abs();

    let az =
        local.z.abs();

    if ax >= ay
        && ax >= az
    {
        (
            (local.z + 1.0)
                * 0.5,

            (local.y + 1.0)
                * 0.5,
        )
    } else if ay >= ax
        && ay >= az
    {
        (
            (local.x + 1.0)
                * 0.5,

            (local.z + 1.0)
                * 0.5,
        )
    } else {
        (
            (local.x + 1.0)
                * 0.5,

            (local.y + 1.0)
                * 0.5,
        )
    }
}

fn hemisphere_uv(
    hemisphere: &Hemisphere,
    point: &Vec3,
    rotation_y: f32,
) -> (f32, f32) {
    let local =
        *point
            - hemisphere.center;

    let normal =
        hemisphere.normal
            .normalize();

    let plane_distance =
        local.dot(
            &normal,
        );

    if plane_distance.abs()
        < 0.006
    {
        let (
            tangent,
            bitangent,
        ) =
            axis_basis(
                normal,
            );

        return (
            0.5
                + local.dot(
                    &tangent,
                )
                    / (
                        hemisphere.radius
                            * 2.0
                    ),

            0.5
                + local.dot(
                    &bitangent,
                )
                    / (
                        hemisphere.radius
                            * 2.0
                    ),
        );
    }

    let p =
        rotate_y_inverse(
            local.normalize(),
            rotation_y,
        );

    let u =
        0.5
            + p.z.atan2(
                p.x,
            )
                / (
                    2.0
                        * PI
                );

    let v =
        0.5
            - p.y
                .clamp(
                    -1.0,
                    1.0,
                )
                .asin()
                / PI;

    (
        u,
        v,
    )
}

fn torus_uv(
    torus: &Torus,
    point: &Vec3,
    rotation_y: f32,
) -> (f32, f32) {
    let local =
        rotate_y_inverse(
            *point
                - torus.center,
            rotation_y,
        );

    let major_angle =
        local.z.atan2(
            local.x,
        );

    let radial =
        (
            local.x
                * local.x
                + local.z
                    * local.z
        )
            .sqrt();

    let tube_x =
        radial
            - torus.major_radius;

    let tube_angle =
        local.y.atan2(
            tube_x,
        );

    (
        0.5
            + major_angle
                / (
                    2.0
                        * PI
                ),

        0.5
            + tube_angle
                / (
                    2.0
                        * PI
                ),
    )
}

fn ellipsoid_uv(
    ellipsoid: &Ellipsoid,
    point: &Vec3,
    rotation_y: f32,
) -> (f32, f32) {
    let local =
        *point
            - ellipsoid.center;

    let normalized =
        Vec3::new(
            local.x
                / ellipsoid.radii.x,
            local.y
                / ellipsoid.radii.y,
            local.z
                / ellipsoid.radii.z,
        );

    let p =
        rotate_y_inverse(
            normalized,
            rotation_y,
        )
        .normalize();

    let u =
        0.5
            + p.z.atan2(
                p.x,
            )
                / (
                    2.0
                        * PI
                );

    let v =
        0.5
            - p.y
                .clamp(
                    -1.0,
                    1.0,
                )
                .asin()
                / PI;

    (
        u,
        v,
    )
}

fn object_tangent_basis(
    object: &Object,
    point: &Vec3,
    normal: Vec3,
) -> (Vec3, Vec3) {
    match object {
        Object::Sphere(sphere) => {
            let local =
                (
                    *point
                        - sphere.center
                )
                    .normalize();

            let mut tangent =
                Vec3::new(
                    -local.z,
                    0.0,
                    local.x,
                );

            if tangent.length()
                < 0.001
            {
                tangent =
                    Vec3::new(
                        1.0,
                        0.0,
                        0.0,
                    );
            }

            tangent =
                tangent.normalize();

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
        }

        Object::Plane(_) => {
            tangent_basis(
                normal,
            )
        }

        Object::Cylinder(cylinder) => {
            let axis =
                cylinder.axis
                    .normalize();

            let surface_normal =
                cylinder.normal_at(
                    point,
                );

            if surface_normal
                .dot(
                    &axis,
                )
                .abs()
                > 0.85
            {
                axis_basis(
                    axis,
                )
            } else {
                let mut tangent =
                    axis.cross(
                        &surface_normal,
                    );

                if tangent.length()
                    < 0.001
                {
                    return tangent_basis(
                        normal,
                    );
                }

                tangent =
                    tangent.normalize();

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
            }
        }

        Object::Cone(cone) => {
            let axis =
                cone.axis
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
                < 0.001
            {
                return tangent_basis(
                    normal,
                );
            }

            let tangent =
                axis
                    .cross(
                        &radial.normalize(),
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
        }

        Object::Cube(cube) => {
            let delta =
                *point
                    - cube.center;

            let lx =
                delta
                    .dot(
                        &cube.right,
                    )
                    .abs();

            let ly =
                delta
                    .dot(
                        &cube.up,
                    )
                    .abs();

            let lz =
                delta
                    .dot(
                        &cube.forward,
                    )
                    .abs();

            if lx >= ly
                && lx >= lz
            {
                let tangent =
                    cube.forward
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
            } else if ly >= lx
                && ly >= lz
            {
                let tangent =
                    cube.right
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
                let tangent =
                    cube.right
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
            }
        }

        Object::Hemisphere(
            hemisphere,
        ) => {
            let local =
                *point
                    - hemisphere.center;

            let axis =
                hemisphere.normal
                    .normalize();

            if local
                .dot(
                    &axis,
                )
                .abs()
                < 0.006
            {
                axis_basis(
                    axis,
                )
            } else {
                let p =
                    local.normalize();

                let mut tangent =
                    Vec3::new(
                        -p.z,
                        0.0,
                        p.x,
                    );

                if tangent.length()
                    < 0.001
                {
                    return tangent_basis(
                        normal,
                    );
                }

                tangent =
                    tangent.normalize();

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
            }
        }

        Object::Torus(torus) => {
            torus_tangent_basis(
                torus,
                point,
                normal,
            )
        }

        Object::Ellipsoid(
            ellipsoid,
        ) => {
            ellipsoid_tangent_basis(
                ellipsoid,
                point,
                normal,
            )
        }
    }
}

fn torus_tangent_basis(
    torus: &Torus,
    point: &Vec3,
    normal: Vec3,
) -> (Vec3, Vec3) {
    let local =
        *point
            - torus.center;

    let mut tangent =
        Vec3::new(
            -local.z,
            0.0,
            local.x,
        );

    if tangent.length()
        < 0.0001
    {
        return tangent_basis(
            normal,
        );
    }

    tangent =
        tangent.normalize();

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
}

fn ellipsoid_tangent_basis(
    ellipsoid: &Ellipsoid,
    point: &Vec3,
    normal: Vec3,
) -> (Vec3, Vec3) {
    let local =
        *point
            - ellipsoid.center;

    let normalized =
        Vec3::new(
            local.x
                / ellipsoid.radii.x,
            local.y
                / ellipsoid.radii.y,
            local.z
                / ellipsoid.radii.z,
        );

    let mut tangent =
        Vec3::new(
            -normalized.z,
            0.0,
            normalized.x,
        );

    if tangent.length()
        < 0.0001
    {
        return tangent_basis(
            normal,
        );
    }

    tangent =
        tangent.normalize();

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
}

fn tangent_basis(
    normal: Vec3,
) -> (Vec3, Vec3) {
    let helper =
        if normal.y.abs()
            < 0.9
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
                &normal,
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
}

fn axis_basis(
    axis: Vec3,
) -> (Vec3, Vec3) {
    let helper =
        if axis.y.abs()
            < 0.9
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
    u: f32,
    v: f32,
    point: &Vec3,
) -> Vec3 {
    let noise =
        procedural_hash(
            u * 80.0
                + point.x
                    * 17.0,
            v * 80.0
                + point.z
                    * 19.0,
        );

    let blade =
        (
            (
                u * 120.0
                    + v * 40.0
            )
                .sin()
                * 0.5
                + 0.5
        )
            * 0.12;

    let brightness =
        0.72
            + noise
                * 0.30
            + blade;

    Vec3::new(
        brightness
            * 0.80,
        brightness,
        brightness
            * 0.75,
    )
}

fn reflect(
    incident: Vec3,
    normal: Vec3,
) -> Vec3 {
    incident
        - normal
            * (
                2.0
                    * incident
                        .dot(
                            &normal,
                        )
            )
}

fn refract(
    incident: Vec3,
    normal: Vec3,
    eta: f32,
) -> Option<Vec3> {
    let i =
        incident.normalize();

    let n =
        normal.normalize();

    let cos_i =
        (
            -i.dot(
                &n,
            )
        )
            .clamp(
                -1.0,
                1.0,
            );

    let k =
        1.0
            - eta
                * eta
                * (
                    1.0
                        - cos_i
                            * cos_i
                );

    if k < 0.0 {
        None
    } else {
        Some(
            i * eta
                + n
                    * (
                        eta
                            * cos_i
                            - k.sqrt()
                    ),
        )
    }
}

fn skybox_color_galaxy_1(
    direction: &Vec3,
) -> Vec3 {
    let dir =
        direction
            .normalize();

    let vertical =
        (
            dir.y
                * 0.5
                + 0.5
        )
            .clamp(
                0.0,
                1.0,
            );

    let top =
        Vec3::new(
            0.04,
            0.18,
            0.30,
        );

    let bottom =
        Vec3::new(
            0.005,
            0.02,
            0.08,
        );

    let mut color =
        bottom
            * (
                1.0
                    - vertical
            )
            + top
                * vertical;

    let nebula_1 =
        (
            dir.x
                * 5.0
                + dir.y
                    * 3.0
                + (
                    dir.z
                        * 4.0
                )
                    .sin()
        )
            .sin()
            * 0.5
            + 0.5;

    let nebula_2 =
        (
            dir.z
                * 7.0
                - dir.y
                    * 4.0
                + (
                    dir.x
                        * 6.0
                )
                    .cos()
        )
            .sin()
            * 0.5
            + 0.5;

    let nebula_3 =
        (
            dir.x
                * 11.0
                + dir.z
                    * 9.0
        )
            .cos()
            * 0.5
            + 0.5;

    let nebula_strength =
        (
            nebula_1
                * nebula_2
                * 0.55
            + nebula_3
                * 0.18
        )
            .clamp(
                0.0,
                1.0,
            );

    color =
        color
            + Vec3::new(
                0.01,
                0.10,
                0.12,
            )
                * nebula_strength;

    let green_nebula =
        (
            nebula_1
                * nebula_3
        )
            .powf(
                2.0,
            );

    color =
        color
            + Vec3::new(
                0.01,
                0.10,
                0.05,
            )
                * green_nebula;

    let glow_direction =
        Vec3::new(
            -1.0,
            0.10,
            0.15,
        )
        .normalize();

    let glow_dot =
        dir
            .dot(
                &glow_direction,
            )
            .max(
                0.0,
            );

    let broad_glow =
        glow_dot
            .powf(
                6.0,
            );

    let core_glow =
        glow_dot
            .powf(
                45.0,
            );

    color =
        color
            + Vec3::new(
                0.10,
                0.45,
                0.22,
            )
                * broad_glow;

    color =
        color
            + Vec3::new(
                0.45,
                1.00,
                0.65,
            )
                * core_glow
                * 1.8;

    let cyan_direction =
        Vec3::new(
            0.15,
            0.20,
            -1.0,
        )
        .normalize();

    let cyan_glow =
        dir
            .dot(
                &cyan_direction,
            )
            .max(
                0.0,
            )
            .powf(
                10.0,
            );

    color =
        color
            + Vec3::new(
                0.03,
                0.22,
                0.35,
            )
                * cyan_glow;

    let star_u =
        (
            dir.x
                * 437.0
        )
            .floor();

    let star_v =
        (
            dir.y
                * 613.0
        )
            .floor();

    let star_w =
        (
            dir.z
                * 521.0
        )
            .floor();

    let star_noise =
        procedural_hash(
            star_u
                + star_w
                    * 0.37,
            star_v
                + star_w
                    * 0.71,
        );

    if star_noise
        > 0.9965
    {
        let star_strength =
            (
                star_noise
                    - 0.9965
            )
                / 0.0035;

        let tint =
            procedural_hash(
                star_u
                    * 0.31,
                star_v
                    * 0.73,
            );

        let star_color =
            if tint < 0.33 {
                Vec3::new(
                    0.75,
                    0.90,
                    1.0,
                )
            } else if tint < 0.66 {
                Vec3::new(
                    0.85,
                    1.0,
                    0.90,
                )
            } else {
                Vec3::new(
                    1.0,
                    1.0,
                    1.0,
                )
            };

        color =
            color
                + star_color
                    * (
                        star_strength
                            * 1.8
                    );
    }

    color
}

fn skybox_color_galaxy_2(
    direction: &Vec3,
) -> Vec3 {
    let dir =
        direction.normalize();

    let vertical =
        (
            dir.y
                * 0.5
                + 0.5
        )
            .clamp(
                0.0,
                1.0,
            );

    let bottom =
        Vec3::new(
            0.05,
            0.01,
            0.10,
        );

    let top =
        Vec3::new(
            0.28,
            0.08,
            0.42,
        );

    let mut color =
        bottom
            * (
                1.0
                    - vertical
            )
            + top
                * vertical;

    let nebula_a =
        (
            dir.x * 5.0
                + dir.z * 3.0
                + (
                    dir.y
                        * 7.0
                )
                    .sin()
        )
            .sin()
            * 0.5
            + 0.5;

    let nebula_b =
        (
            dir.z * 8.0
                - dir.x * 4.0
                + (
                    dir.y
                        * 5.0
                )
                    .cos()
        )
            .sin()
            * 0.5
            + 0.5;

    let nebula_c =
        (
            dir.x * 13.0
                + dir.z * 9.0
        )
            .cos()
            * 0.5
            + 0.5;

    let nebula =
        (
            nebula_a
                * nebula_b
                * 0.75
            + nebula_c
                * 0.20
        )
            .clamp(
                0.0,
                1.0,
            );

    color =
        color
            + Vec3::new(
                0.28,
                0.04,
                0.42,
            )
                * nebula;

    color =
        color
            + Vec3::new(
                0.10,
                0.03,
                0.28,
            )
                * (
                    nebula_a
                        * nebula_c
                )
                    .powf(
                        2.0,
                    );

    let glow_direction =
        Vec3::new(
            0.75,
            0.20,
            -1.0,
        )
        .normalize();

    let glow =
        dir
            .dot(
                &glow_direction,
            )
            .max(
                0.0,
            );

    color =
        color
            + Vec3::new(
                0.65,
                0.08,
                0.85,
            )
                * glow.powf(
                    6.0,
                );

    color =
        color
            + Vec3::new(
                1.0,
                0.28,
                0.95,
            )
                * glow.powf(
                    34.0,
                )
                * 1.35;

    let star_u =
        (
            dir.x
                * 463.0
        )
            .floor();

    let star_v =
        (
            dir.y
                * 607.0
        )
            .floor();

    let star_w =
        (
            dir.z
                * 547.0
        )
            .floor();

    let star_noise =
        procedural_hash(
            star_u
                + star_w
                    * 0.43,
            star_v
                + star_w
                    * 0.79,
        );

    if star_noise > 0.9960 {
        let strength =
            (
                (
                    star_noise
                        - 0.9960
                )
                    / 0.0040
            )
                .clamp(
                    0.0,
                    1.0,
                );

        color =
            color
                + Vec3::new(
                    1.0,
                    0.92,
                    1.0,
                )
                    * strength
                    * 1.8;
    }

    color
}

fn skybox_color_galaxy_3(
    direction: &Vec3,
) -> Vec3 {
    let dir =
        direction.normalize();

    let vertical =
        (
            dir.y
                * 0.5
                + 0.5
        )
            .clamp(
                0.0,
                1.0,
            );

    let base_dark =
        Vec3::new(
            0.035,
            0.045,
            0.060,
        );

    let base_light =
        Vec3::new(
            0.38,
            0.43,
            0.50,
        );

    let mut color =
        base_dark
            * (
                1.0
                    - vertical
            )
            + base_light
                * vertical;

    let angle =
        dir.z.atan2(
            dir.x,
        );

    let radius =
        (
            dir.x
                * dir.x
                + dir.z
                    * dir.z
        )
            .sqrt()
            .max(
                0.001,
            );

    let spiral_a =
        (
            angle * 4.0
                + radius * 8.0
                + dir.y * 4.0
        )
            .sin()
            * 0.5
            + 0.5;

    let spiral_b =
        (
            angle * 7.0
                - radius * 5.5
                + dir.y * 9.0
        )
            .cos()
            * 0.5
            + 0.5;

    let cloud_a =
        (
            dir.x * 5.5
                + dir.z * 4.2
                + (
                    dir.y
                        * 7.0
                )
                    .sin()
                    * 2.0
        )
            .sin()
            * 0.5
            + 0.5;

    let cloud_b =
        (
            dir.z * 8.0
                - dir.x * 3.5
                + (
                    dir.y
                        * 10.0
                )
                    .cos()
                    * 1.5
        )
            .sin()
            * 0.5
            + 0.5;

    let cloud_c =
        (
            dir.x * 13.0
                + dir.z * 11.0
                + dir.y * 4.0
        )
            .cos()
            * 0.5
            + 0.5;

    let clouds =
        (
            cloud_a
                * 0.35
                + cloud_b
                    * 0.25
                + cloud_c
                    * 0.12
                + spiral_a
                    * spiral_b
                    * 0.45
        )
            .clamp(
                0.0,
                1.0,
            );

    let bright_clouds =
        clouds.powf(
            2.0,
        );

    color =
        color
            * (
                1.0
                    - bright_clouds
                        * 0.72
            )
            + Vec3::new(
                0.94,
                0.97,
                1.0,
            )
                * (
                    bright_clouds
                        * 0.72
                );

    let dark_void =
        (
            dir.x * 9.0
                - dir.z * 6.0
                + (
                    dir.y
                        * 8.0
                )
                    .sin()
        )
            .sin()
            * 0.5
            + 0.5;

    color =
        color
            * (
                1.0
                    - dark_void
                        .powf(
                            4.0,
                        )
                        * 0.30
            );

    let core_direction =
        Vec3::new(
            0.22,
            0.16,
            -1.0,
        )
        .normalize();

    let core_dot =
        dir
            .dot(
                &core_direction,
            )
            .max(
                0.0,
            );

    let broad_core =
        core_dot
            .powf(
                5.0,
            );

    let hot_core =
        core_dot
            .powf(
                24.0,
            );

    color =
        color
            + Vec3::new(
                0.48,
                0.54,
                0.62,
            )
                * broad_core
                * 0.70;

    color =
        color
            + Vec3::new(
                1.0,
                0.98,
                0.86,
            )
                * hot_core
                * 1.50;

    let star_u =
        (
            dir.x
                * 523.0
        )
            .floor();

    let star_v =
        (
            dir.y
                * 719.0
        )
            .floor();

    let star_w =
        (
            dir.z
                * 631.0
        )
            .floor();

    let star_noise =
        procedural_hash(
            star_u
                + star_w
                    * 0.37,
            star_v
                + star_w
                    * 0.83,
        );

    if star_noise > 0.9955 {
        let strength =
            (
                (
                    star_noise
                        - 0.9955
                )
                    / 0.0045
            )
                .clamp(
                    0.0,
                    1.0,
                );

        color =
            color
                + Vec3::new(
                    1.0,
                    1.0,
                    1.0,
                )
                    * strength
                    * 1.9;
    }

    Vec3::new(
        color.x
            .clamp(
                0.0,
                1.0,
            ),
        color.y
            .clamp(
                0.0,
                1.0,
            ),
        color.z
            .clamp(
                0.0,
                1.0,
            ),
    )
}

fn skybox_color_for_galaxy(
    direction: &Vec3,
    galaxy_index: usize,
) -> Vec3 {
    match galaxy_index {
        0 => {
            skybox_color_galaxy_1(
                direction,
            )
        }

        1 => {
            skybox_color_galaxy_2(
                direction,
            )
        }

        2 => {
            skybox_color_galaxy_3(
                direction,
            )
        }

        _ => {
            skybox_color_galaxy_1(
                direction,
            )
        }
    }
}

fn procedural_hash(
    x: f32,
    y: f32,
) -> f32 {
    let value =
        (
            x * 12.9898
                + y * 78.233
        )
            .sin()
            * 43758.5453;

    value
        - value.floor()
}

fn multiply_vec3(
    a: Vec3,
    b: Vec3,
) -> Vec3 {
    Vec3::new(
        a.x * b.x,
        a.y * b.y,
        a.z * b.z,
    )
}

fn kirby_decal_color(
    material: &Material,
    u: f32,
    v: f32,
) -> Vec3 {
    let pink = material.color;

    let u_min = 0.33;
    let u_max = 0.67;

    let v_min = 0.28;
    let v_max = 0.72;

    if u < u_min || u > u_max || v < v_min || v > v_max {
        return pink;
    }

    let decal_u = 1.0 - ((u - u_min) / (u_max - u_min));
    let decal_v = 1.0 - ((v - v_min) / (v_max - v_min));

    if let Some(texture) = material.albedo_texture {
        let (texture_color, alpha) = texture.sample_rgba(decal_u, decal_v);
        pink * (1.0 - alpha) + texture_color * alpha
    } else {
        pink
    }
}

fn to_color(
    color: Vec3,
) -> Color {
    let r =
        (
            color.x
                .clamp(
                    0.0,
                    1.0,
                )
                .powf(
                    1.0 / 2.2,
                )
                * 255.0
        ) as u8;

    let g =
        (
            color.y
                .clamp(
                    0.0,
                    1.0,
                )
                .powf(
                    1.0 / 2.2,
                )
                * 255.0
        ) as u8;

    let b =
        (
            color.z
                .clamp(
                    0.0,
                    1.0,
                )
                .powf(
                    1.0 / 2.2,
                )
                * 255.0
        ) as u8;

    Color::new(
        r,
        g,
        b,
        255,
    )
}