use std::f32::consts::PI;
use std::thread;

use raylib::prelude::Color;

use crate::core::camera::Camera;
use crate::core::framebuffer::Framebuffer;
use crate::core::vec3::Vec3;

use crate::materials::material::MaterialPattern;

use crate::objects::cone::Cone;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::object::Object;
use crate::objects::plane::Plane;
use crate::objects::sphere::Sphere;
use crate::objects::torus::Torus;
use crate::objects::ellipsoid::Ellipsoid;

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
    let width =
        framebuffer.width as usize;

    let height =
        framebuffer.height as usize;

    let thread_count =
        thread::available_parallelism()
            .map(
                |n| n.get(),
            )
            .unwrap_or(
                4,
            )
            .max(
                1,
            );

    let rows_per_thread =
        (
            height
                + thread_count
                - 1
        )
            / thread_count;

    let chunk_size =
        rows_per_thread
            * width;

    let pixels =
        framebuffer.pixels_mut();

    thread::scope(
        |scope| {
            for (
                chunk_index,
                chunk,
            ) in pixels
                .chunks_mut(
                    chunk_size,
                )
                .enumerate()
            {
                scope.spawn(
                    move || {
                        let start_row =
                            chunk_index
                                * rows_per_thread;

                        for local_index in
                            0..chunk.len()
                        {
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
                                );

                            chunk[
                                local_index
                            ] =
                                to_color(
                                    color,
                                );
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
    if depth >= MAX_DEPTH {
        return skybox_color(
            direction,
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
    ) =
        match hit {
            Some(
                value,
            ) => {
                value
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
        );

    let mut surface_color =
        material.color;

    if let Some(
        texture,
    ) =
        material
            .albedo_texture
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
        }
    }

    if let Some(
        normal_texture,
    ) =
        material
            .normal_texture
    {
        let sample =
            normal_texture
                .sample(
                    u,
                    v,
                );

        let tangent_normal =
            Vec3::new(
                sample.x
                    * 2.0
                    - 1.0,

                sample.y
                    * 2.0
                    - 1.0,

                sample.z
                    * 2.0
                    - 1.0,
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

    if reflectivity
        > 0.001
    {
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

    if transparency
        > 0.001
    {
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
) -> (f32, f32) {
    match object {
        Object::Sphere(
            sphere,
        ) => {
            sphere_uv(
                sphere,
                point,
            )
        }

        Object::Plane(
            plane,
        ) => {
            planar_uv(
                plane,
                point,
            )
        }

        Object::Cylinder(
            cylinder,
        ) => {
            cylinder_uv(
                cylinder,
                point,
            )
        }

        Object::Cone(
            cone,
        ) => {
            cone_uv(
                cone,
                point,
            )
        }

        Object::Cube(
            cube,
        ) => {
            cube_uv(
                cube,
                point,
            )
        }

        Object::Hemisphere(
            hemisphere,
        ) => {
            hemisphere_uv(
                hemisphere,
                point,
            )
        }

        Object::Torus(torus) => {
            torus_uv(
                torus,
                point,
            )
        }

        Object::Ellipsoid(ellipsoid) => {
            ellipsoid_uv(
                ellipsoid,
                point,
            )
        }
    }
}

fn sphere_uv(
    sphere: &Sphere,
    point: &Vec3,
) -> (f32, f32) {
    let p =
        (
            *point
                - sphere.center
        )
            .normalize();

    let u =
        0.5
            + p.z
                .atan2(
                    p.x,
                )
                / (
                    2.0
                        * PI
                );

    let v =
        0.5
            - p.y
                .asin()
                / PI;

    (
        u,
        v,
    )
}

fn hemisphere_uv(
    hemisphere: &Hemisphere,
    point: &Vec3,
) -> (f32, f32) {
    let local =
        *point
            - hemisphere.center;

    let plane_distance =
        local.dot(
            &hemisphere.normal,
        );

    if plane_distance.abs()
        < 0.004
    {
        let (
            tangent,
            bitangent,
        ) =
            axis_basis(
                hemisphere.normal,
            );

        let u =
            0.5
                + local
                    .dot(
                        &tangent,
                    )
                    / (
                        hemisphere.radius
                            * 2.0
                    );

        let v =
            0.5
                + local
                    .dot(
                        &bitangent,
                    )
                    / (
                        hemisphere.radius
                            * 2.0
                    );

        return (
            u,
            v,
        );
    }

    let p =
        local.normalize();

    let u =
        0.5
            + p.z
                .atan2(
                    p.x,
                )
                / (
                    2.0
                        * PI
                );

    let v =
        0.5
            - p.y
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
        local
            .dot(
                &tangent,
            )
            * 0.5,

        local
            .dot(
                &bitangent,
            )
            * 0.5,
    )
}

fn cylinder_uv(
    cylinder: &Cylinder,
    point: &Vec3,
) -> (f32, f32) {
    let axis =
        cylinder.axis.normalize();

    let local =
        *point
            - cylinder.center;

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
        local
            .dot(
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
        radial
            .dot(
                &tangent,
            );

    let z =
        radial
            .dot(
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
    let local =
        (
            *point
                - cube.center
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
            (
                local.z
                    + 1.0
            )
                * 0.5,

            (
                local.y
                    + 1.0
            )
                * 0.5,
        )
    } else if ay >= ax
        && ay >= az
    {
        (
            (
                local.x
                    + 1.0
            )
                * 0.5,

            (
                local.z
                    + 1.0
            )
                * 0.5,
        )
    } else {
        (
            (
                local.x
                    + 1.0
            )
                * 0.5,

            (
                local.y
                    + 1.0
            )
                * 0.5,
        )
    }
}

fn torus_uv(
    torus: &Torus,
    point: &Vec3,
) -> (f32, f32) {
    let local =
        *point
            - torus.center;

    let major_angle =
        local.z
            .atan2(
                local.x,
            );

    let radial =
        (
            local.x * local.x
                + local.z * local.z
        )
            .sqrt();

    let tube_x =
        radial
            - torus.major_radius;

    let tube_angle =
        local.y
            .atan2(
                tube_x,
            );

    let u =
        0.5
            + major_angle
                / (
                    2.0
                        * PI
                );

    let v =
        0.5
            + tube_angle
                / (
                    2.0
                        * PI
                );

    (
        u,
        v,
    )
}

fn ellipsoid_uv(
    ellipsoid: &Ellipsoid,
    point: &Vec3,
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
        )
        .normalize();

    let u =
        0.5
            + normalized.z
                .atan2(
                    normalized.x,
                )
                / (
                    2.0
                        * std::f32::consts::PI
                );

    let v =
        0.5
            - normalized.y
                .asin()
                / std::f32::consts::PI;

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
        Object::Sphere(
            sphere,
        ) => {
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

        Object::Plane(
            _,
        ) => {
            tangent_basis(
                normal,
            )
        }

        Object::Cylinder(
            cylinder,
        ) => {
            let axis =
                cylinder.axis.normalize();

            let cylinder_normal =
                cylinder.normal_at(
                    point,
                );

            if cylinder_normal
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
                tangent_basis(
                    cylinder_normal,
                )
            }
        }

        Object::Cone(
            cone,
        ) => {
            let axis =
                cone.axis
                    .normalize();

            let local =
                *point
                    - cone.center;

            let axial =
                local
                    .dot(
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
                        &radial
                            .normalize(),
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

        Object::Cube(
            _,
        ) => {
            tangent_basis(
                normal,
            )
        }

        Object::Hemisphere(
            hemisphere,
        ) => {
            let local =
                *point
                    - hemisphere.center;

            let plane_distance =
                local.dot(
                    &hemisphere.normal,
                );

            if plane_distance.abs()
                < 0.004
            {
                axis_basis(
                    hemisphere.normal,
                )
            } else {
                let local_normal =
                    local.normalize();

                let mut tangent =
                    Vec3::new(
                        -local_normal.z,
                        0.0,
                        local_normal.x,
                    );

                if tangent.length()
                    < 0.001
                {
                    tangent =
                        tangent_basis(
                            normal,
                        ).0;
                } else {
                    tangent =
                        tangent.normalize();
                }

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

        Object::Ellipsoid(ellipsoid) => {
            ellipsoid_tangent_basis(
                ellipsoid,
                point,
                normal,
            )
        }
    }
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

fn skybox_color(
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
            } else if tint
                < 0.66
            {
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
        )
            as u8;

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
        )
            as u8;

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
        )
            as u8;

    Color::new(
        r,
        g,
        b,
        255,
    )
}