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

use crate::objects::ellipsoid::Ellipsoid;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::object::Object;
use crate::objects::torus::Torus;

use crate::objects::plane::Plane;

use crate::objects::sphere::Sphere;

use crate::scene::light::Light;

use crate::scene::scene::Scene;

const MAX_DEPTH: u32 = 4;

const EPSILON: f32 = 0.002;

pub fn render(framebuffer: &mut Framebuffer, scene: &Scene, light: &Light, camera: &Camera) {
    render_with_skybox(framebuffer, scene, light, camera, 0);
}

pub fn render_with_skybox(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    light: &Light,
    camera: &Camera,
    skybox_id: usize,
) {
    render_rotated_with_skybox(framebuffer, scene, light, camera, 0.0, skybox_id);
}

pub fn render_rotated(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    light: &Light,
    camera: &Camera,
    rotation_y: f32,
) {
    render_rotated_with_skybox(framebuffer, scene, light, camera, rotation_y, 0);
}

pub fn render_rotated_with_skybox(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    light: &Light,
    camera: &Camera,
    rotation_y: f32,
    skybox_id: usize,
) {
    let width = framebuffer.width as usize;

    let height = framebuffer.height as usize;

    let thread_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .max(1);

    let rows_per_thread = (height + thread_count - 1) / thread_count;

    let chunk_size = rows_per_thread * width;

    let pixels = framebuffer.pixels_mut();

    thread::scope(|scope| {
        for (chunk_index, chunk) in pixels.chunks_mut(chunk_size).enumerate() {
            scope.spawn(move || {
                let start_row = chunk_index * rows_per_thread;

                for local_index in 0..chunk.len() {
                    let local_y = local_index / width;

                    let x = local_index % width;

                    let y = start_row + local_y;

                    if y >= height {
                        continue;
                    }

                    let ray =
                        camera.get_ray(x as f32 + 0.5, y as f32 + 0.5, width as f32, height as f32);

                    let color = cast_ray(
                        &ray.origin,
                        &ray.direction,
                        scene,
                        light,
                        0,
                        rotation_y,
                        skybox_id,
                    );

                    chunk[local_index] = to_color(color);
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
    skybox_id: usize,
) -> Vec3 {
    if depth >= MAX_DEPTH {
        return skybox_color(direction, skybox_id);
    }

    let hit = scene.bvh.intersect(origin, direction, &scene.objects);

    let (object_index, distance) = match hit {
        Some(value) => value,

        None => {
            return skybox_color(direction, skybox_id);
        }
    };

    let object = &scene.objects[object_index];

    let hit_point = *origin + *direction * distance;

    let geometric_normal = object.normal_at(&hit_point).normalize();

    let front_face = direction.dot(&geometric_normal) < 0.0;

    let mut normal = if front_face {
        geometric_normal
    } else {
        -geometric_normal
    };

    let material = object.material_at(&hit_point);

    let (u, v) = object_uv(object, &hit_point, rotation_y);

    let mut surface_color = material.color;

    if let Some(texture) = material.albedo_texture {
        let texture_color = texture.sample(u, v);

        surface_color = multiply_vec3(surface_color, texture_color);
    } else {
        match material.pattern {
            MaterialPattern::Grass => {
                surface_color = multiply_vec3(surface_color, procedural_grass(u, v, &hit_point));
            }

            MaterialPattern::Solid => {}
        }
    }

    if let Some(normal_texture) = material.normal_texture {
        let sample = normal_texture.sample(u, v);

        let tangent_normal = Vec3::new(
            sample.x * 2.0 - 1.0,
            sample.y * 2.0 - 1.0,
            sample.z * 2.0 - 1.0,
        )
        .normalize();

        let (tangent, bitangent) = object_tangent_basis(object, &hit_point, normal);

        normal =
            (tangent * tangent_normal.x + bitangent * tangent_normal.y + normal * tangent_normal.z)
                .normalize();

        if direction.dot(&normal) > 0.0 {
            normal = -normal;
        }
    }

    let roughness = material
        .roughness_texture
        .map(|texture| texture.sample_scalar(u, v).clamp(0.0, 1.0))
        .unwrap_or(0.5);

    let ao = material
        .ao_texture
        .map(|texture| texture.sample_scalar(u, v).clamp(0.0, 1.0))
        .unwrap_or(1.0);

    let to_light = light.position - hit_point;

    let light_distance = to_light.length();

    let light_direction = to_light / light_distance;

    let shadow_origin = hit_point + normal * EPSILON;

    let in_shadow = if normal.dot(&light_direction) <= 0.0 {
        true
    } else {
        scene.bvh.any_hit(
            &shadow_origin,
            &light_direction,
            light_distance - EPSILON,
            &scene.objects,
        )
    };

    let ndotl = normal.dot(&light_direction).max(0.0);

    let ambient = 0.22 * ao;

    let diffuse_factor = if in_shadow { ndotl * 0.18 } else { ndotl };

    let view_direction = -*direction;

    let reflected_light = reflect(-light_direction, normal).normalize();

    let specular_dot = reflected_light.dot(&view_direction).max(0.0);

    let shininess = 8.0 + (1.0 - roughness) * 120.0;

    let mut specular = specular_dot.powf(shininess) * material.specular * (1.0 - roughness * 0.65);

    if in_shadow {
        specular *= 0.08;
    }

    let diffuse_light = ambient + diffuse_factor * material.albedo * light.intensity;

    let mut final_color = surface_color * diffuse_light;

    let lava_light_position =
        Vec3::new(
            0.0,
            -0.85,
            0.0,
        );

    let lava_light_color =
        Vec3::new(
            1.0,
            0.20,
            0.02,
        );

    let lava_light_intensity =
        2.8;

    let lava_light_radius =
        6.0;

    let to_lava_light =
        lava_light_position
            - hit_point;

    let lava_distance =
        to_lava_light.length();

    if lava_distance < lava_light_radius {
        let lava_direction =
            to_lava_light
                / lava_distance.max(0.001);

        let lava_ndotl =
            normal
                .dot(
                    &lava_direction,
                )
                .max(
                    0.0,
                );

        if lava_ndotl > 0.0 {
            let lava_shadow_origin =
                hit_point
                    + normal
                        * EPSILON;

            let lava_in_shadow =
                scene
                    .bvh
                    .any_hit(
                        &lava_shadow_origin,
                        &lava_direction,
                        lava_distance
                            - EPSILON,
                        &scene.objects,
                    );

            let attenuation =
                (
                    1.0
                        - lava_distance
                            / lava_light_radius
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            let attenuation =
                attenuation
                    * attenuation;

            let shadow_factor =
                if lava_in_shadow {
                    0.18
                } else {
                    1.0
                };

            let lava_diffuse =
                lava_ndotl
                    * lava_light_intensity
                    * attenuation
                    * shadow_factor;

            final_color =
                final_color
                    + multiply_vec3(
                        surface_color,
                        lava_light_color,
                    )
                        * lava_diffuse;
        }
    }

    final_color = final_color + light.color * (specular * light.intensity);

    let reflectivity = material.reflectivity.clamp(0.0, 1.0);

    if reflectivity > 0.001 {
        let reflected_direction = reflect(*direction, normal).normalize();

        let reflected_origin = hit_point + normal * EPSILON;

        let reflected_color = cast_ray(
            &reflected_origin,
            &reflected_direction,
            scene,
            light,
            depth + 1,
            rotation_y,
            skybox_id,
        );

        final_color = final_color * (1.0 - reflectivity) + reflected_color * reflectivity;
    }

    let transparency = material.transparency.clamp(0.0, 1.0);

    if transparency > 0.001 {
        let eta = if front_face { 1.0 / 1.33 } else { 1.33 };

        let refracted_direction = refract(*direction, normal, eta)
            .unwrap_or(*direction)
            .normalize();

        let refracted_origin = hit_point + refracted_direction * (EPSILON * 2.0);

        let refracted_color = cast_ray(
            &refracted_origin,
            &refracted_direction,
            scene,
            light,
            depth + 1,
            rotation_y,
            skybox_id,
        );

        final_color = final_color * (1.0 - transparency) + refracted_color * transparency;
    }

    final_color
}

fn object_uv(object: &Object, point: &Vec3, rotation_y: f32) -> (f32, f32) {
    match object {
        Object::Sphere(sphere) => sphere_uv(sphere, point, rotation_y),
        Object::Plane(plane) => planar_uv(plane, point),
        Object::Cylinder(cylinder) => cylinder_uv(cylinder, point),
        Object::Cone(cone) => cone_uv(cone, point),
        Object::Cube(cube) => cube_uv(cube, point),
        Object::Hemisphere(hemisphere) => hemisphere_uv(hemisphere, point, rotation_y),
        Object::Torus(torus) => torus_uv(torus, point, rotation_y),
        Object::Ellipsoid(ellipsoid) => ellipsoid_uv(ellipsoid, point, rotation_y),
    }
}

fn rotate_y_inverse(point: Vec3, angle: f32) -> Vec3 {
    let c = angle.cos();
    let s = angle.sin();

    Vec3::new(
        point.x * c - point.z * s,
        point.y,
        point.x * s + point.z * c,
    )
}

fn sphere_uv(sphere: &Sphere, point: &Vec3, rotation_y: f32) -> (f32, f32) {
    let local = (*point - sphere.center).normalize();
    let p = rotate_y_inverse(local, rotation_y);

    let u = 0.5 + p.z.atan2(p.x) / (2.0 * PI);

    let v = 0.5 - p.y.clamp(-1.0, 1.0).asin() / PI;

    (u, v)
}

fn planar_uv(plane: &Plane, point: &Vec3) -> (f32, f32) {
    let normal = plane.normal.normalize();

    let (tangent, bitangent) = tangent_basis(normal);

    let local = *point - plane.point;

    (local.dot(&tangent) * 0.5, local.dot(&bitangent) * 0.5)
}

fn cylinder_uv(cylinder: &Cylinder, point: &Vec3) -> (f32, f32) {
    let axis = cylinder.axis.normalize();

    let local = *point - cylinder.center;

    let axial = local.dot(&axis);

    let (tangent, bitangent) = axis_basis(axis);

    let radial = local - axis * axial;

    let x = radial.dot(&tangent);

    let z = radial.dot(&bitangent);

    let half_height = cylinder.height * 0.5;

    let cap_epsilon = 0.015;

    if axial.abs() >= half_height - cap_epsilon {
        let u = 0.5 + x / (cylinder.radius * 2.0);

        let v = 0.5 + z / (cylinder.radius * 2.0);

        return (u, v);
    }

    let angle = z.atan2(x);

    let u = 0.5 + angle / (2.0 * PI);

    let v = axial / cylinder.height + 0.5;

    (u * 2.0, v * 1.5)
}

fn cone_uv(cone: &Cone, point: &Vec3) -> (f32, f32) {
    let axis = cone.axis.normalize();

    let local = *point - cone.center;

    let axial = local.dot(&axis);

    let (tangent, bitangent) = axis_basis(axis);

    let radial = local - axis * axial;

    let x = radial.dot(&tangent);

    let z = radial.dot(&bitangent);

    let u = 0.5 + z.atan2(x) / (2.0 * PI);

    let v = axial / cone.height + 0.5;

    (u * 2.0, v)
}

fn cube_uv(cube: &Cube, point: &Vec3) -> (f32, f32) {
    let delta = *point - cube.center;

    let local = Vec3::new(
        delta.dot(&cube.right),
        delta.dot(&cube.up),
        delta.dot(&cube.forward),
    ) / cube.half_size;

    let ax = local.x.abs();
    let ay = local.y.abs();
    let az = local.z.abs();

    if ax >= ay && ax >= az {
        ((local.z + 1.0) * 0.5, (local.y + 1.0) * 0.5)
    } else if ay >= ax && ay >= az {
        ((local.x + 1.0) * 0.5, (local.z + 1.0) * 0.5)
    } else {
        ((local.x + 1.0) * 0.5, (local.y + 1.0) * 0.5)
    }
}

fn hemisphere_uv(hemisphere: &Hemisphere, point: &Vec3, rotation_y: f32) -> (f32, f32) {
    let local = *point - hemisphere.center;

    let normal = hemisphere.normal.normalize();

    let plane_distance = local.dot(&normal);

    if plane_distance.abs() < 0.006 {
        let (tangent, bitangent) = axis_basis(normal);

        return (
            0.5 + local.dot(&tangent) / (hemisphere.radius * 2.0),
            0.5 + local.dot(&bitangent) / (hemisphere.radius * 2.0),
        );
    }

    let p = rotate_y_inverse(local.normalize(), rotation_y);

    let u = 0.5 + p.z.atan2(p.x) / (2.0 * PI);

    let v = 0.5 - p.y.clamp(-1.0, 1.0).asin() / PI;

    (u, v)
}

fn torus_uv(torus: &Torus, point: &Vec3, rotation_y: f32) -> (f32, f32) {
    let local = rotate_y_inverse(*point - torus.center, rotation_y);

    let major_angle = local.z.atan2(local.x);

    let radial = (local.x * local.x + local.z * local.z).sqrt();

    let tube_x = radial - torus.major_radius;

    let tube_angle = local.y.atan2(tube_x);

    (
        0.5 + major_angle / (2.0 * PI),
        0.5 + tube_angle / (2.0 * PI),
    )
}

fn ellipsoid_uv(ellipsoid: &Ellipsoid, point: &Vec3, rotation_y: f32) -> (f32, f32) {
    let local = *point - ellipsoid.center;

    let normalized = Vec3::new(
        local.x / ellipsoid.radii.x,
        local.y / ellipsoid.radii.y,
        local.z / ellipsoid.radii.z,
    );

    let p = rotate_y_inverse(normalized, rotation_y).normalize();

    let u = 0.5 + p.z.atan2(p.x) / (2.0 * PI);

    let v = 0.5 - p.y.clamp(-1.0, 1.0).asin() / PI;

    (u, v)
}

fn object_tangent_basis(object: &Object, point: &Vec3, normal: Vec3) -> (Vec3, Vec3) {
    match object {
        Object::Sphere(sphere) => {
            let local = (*point - sphere.center).normalize();

            let mut tangent = Vec3::new(-local.z, 0.0, local.x);

            if tangent.length() < 0.001 {
                tangent = Vec3::new(1.0, 0.0, 0.0);
            }

            tangent = tangent.normalize();

            let bitangent = normal.cross(&tangent).normalize();

            (tangent, bitangent)
        }

        Object::Plane(_) => tangent_basis(normal),

        Object::Cylinder(cylinder) => {
            let axis = cylinder.axis.normalize();

            let surface_normal = cylinder.normal_at(point);

            if surface_normal.dot(&axis).abs() > 0.85 {
                axis_basis(axis)
            } else {
                let mut tangent = axis.cross(&surface_normal);

                if tangent.length() < 0.001 {
                    return tangent_basis(normal);
                }

                tangent = tangent.normalize();

                let bitangent = normal.cross(&tangent).normalize();

                (tangent, bitangent)
            }
        }

        Object::Cone(cone) => {
            let axis = cone.axis.normalize();

            let local = *point - cone.center;

            let axial = local.dot(&axis);

            let radial = local - axis * axial;

            if radial.length() < 0.001 {
                return tangent_basis(normal);
            }

            let tangent = axis.cross(&radial.normalize()).normalize();

            let bitangent = normal.cross(&tangent).normalize();

            (tangent, bitangent)
        }

        Object::Cube(cube) => {
            let delta = *point - cube.center;

            let lx = delta.dot(&cube.right).abs();

            let ly = delta.dot(&cube.up).abs();

            let lz = delta.dot(&cube.forward).abs();

            if lx >= ly && lx >= lz {
                let tangent = cube.forward.normalize();

                let bitangent = normal.cross(&tangent).normalize();

                (tangent, bitangent)
            } else if ly >= lx && ly >= lz {
                let tangent = cube.right.normalize();

                let bitangent = normal.cross(&tangent).normalize();

                (tangent, bitangent)
            } else {
                let tangent = cube.right.normalize();

                let bitangent = normal.cross(&tangent).normalize();

                (tangent, bitangent)
            }
        }

        Object::Hemisphere(hemisphere) => {
            let local = *point - hemisphere.center;

            let axis = hemisphere.normal.normalize();

            if local.dot(&axis).abs() < 0.006 {
                axis_basis(axis)
            } else {
                let p = local.normalize();

                let mut tangent = Vec3::new(-p.z, 0.0, p.x);

                if tangent.length() < 0.001 {
                    return tangent_basis(normal);
                }

                tangent = tangent.normalize();

                let bitangent = normal.cross(&tangent).normalize();

                (tangent, bitangent)
            }
        }

        Object::Torus(torus) => torus_tangent_basis(torus, point, normal),

        Object::Ellipsoid(ellipsoid) => ellipsoid_tangent_basis(ellipsoid, point, normal),
    }
}

fn torus_tangent_basis(torus: &Torus, point: &Vec3, normal: Vec3) -> (Vec3, Vec3) {
    let local = *point - torus.center;

    let mut tangent = Vec3::new(-local.z, 0.0, local.x);

    if tangent.length() < 0.0001 {
        return tangent_basis(normal);
    }

    tangent = tangent.normalize();

    let bitangent = normal.cross(&tangent).normalize();

    (tangent, bitangent)
}

fn ellipsoid_tangent_basis(ellipsoid: &Ellipsoid, point: &Vec3, normal: Vec3) -> (Vec3, Vec3) {
    let local = *point - ellipsoid.center;

    let normalized = Vec3::new(
        local.x / ellipsoid.radii.x,
        local.y / ellipsoid.radii.y,
        local.z / ellipsoid.radii.z,
    );

    let mut tangent = Vec3::new(-normalized.z, 0.0, normalized.x);

    if tangent.length() < 0.0001 {
        return tangent_basis(normal);
    }

    tangent = tangent.normalize();

    let bitangent = normal.cross(&tangent).normalize();

    (tangent, bitangent)
}

fn tangent_basis(normal: Vec3) -> (Vec3, Vec3) {
    let helper = if normal.y.abs() < 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };

    let tangent = helper.cross(&normal).normalize();

    let bitangent = normal.cross(&tangent).normalize();

    (tangent, bitangent)
}

fn axis_basis(axis: Vec3) -> (Vec3, Vec3) {
    let helper = if axis.y.abs() < 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };

    let tangent = helper.cross(&axis).normalize();

    let bitangent = axis.cross(&tangent).normalize();

    (tangent, bitangent)
}

fn procedural_grass(u: f32, v: f32, point: &Vec3) -> Vec3 {
    let noise = procedural_hash(u * 80.0 + point.x * 17.0, v * 80.0 + point.z * 19.0);

    let blade = ((u * 120.0 + v * 40.0).sin() * 0.5 + 0.5) * 0.12;

    let brightness = 0.72 + noise * 0.30 + blade;

    Vec3::new(brightness * 0.80, brightness, brightness * 0.75)
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * (2.0 * incident.dot(&normal))
}

fn refract(incident: Vec3, normal: Vec3, eta: f32) -> Option<Vec3> {
    let i = incident.normalize();

    let n = normal.normalize();

    let cos_i = (-i.dot(&n)).clamp(-1.0, 1.0);

    let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);

    if k < 0.0 {
        None
    } else {
        Some(i * eta + n * (eta * cos_i - k.sqrt()))
    }
}

fn skybox_color(direction: &Vec3, skybox_id: usize) -> Vec3 {
    match skybox_id {
        1 => skybox_galaxy_2(direction),

        _ => skybox_galaxy_1(direction),
    }
}

fn skybox_galaxy_1(direction: &Vec3) -> Vec3 {
    let dir = direction.normalize();

    let vertical = (dir.y * 0.5 + 0.5).clamp(0.0, 1.0);

    let top = Vec3::new(0.04, 0.18, 0.30);

    let bottom = Vec3::new(0.005, 0.02, 0.08);

    let mut color = bottom * (1.0 - vertical) + top * vertical;

    let nebula_1 = (dir.x * 5.0 + dir.y * 3.0 + (dir.z * 4.0).sin()).sin() * 0.5 + 0.5;

    let nebula_2 = (dir.z * 7.0 - dir.y * 4.0 + (dir.x * 6.0).cos()).sin() * 0.5 + 0.5;

    let nebula_3 = (dir.x * 11.0 + dir.z * 9.0).cos() * 0.5 + 0.5;

    let nebula_strength = (nebula_1 * nebula_2 * 0.55 + nebula_3 * 0.18).clamp(0.0, 1.0);

    color = color + Vec3::new(0.01, 0.10, 0.12) * nebula_strength;

    let green_nebula = (nebula_1 * nebula_3).powf(2.0);

    color = color + Vec3::new(0.01, 0.10, 0.05) * green_nebula;

    let glow_direction = Vec3::new(-1.0, 0.10, 0.15).normalize();

    let glow_dot = dir.dot(&glow_direction).max(0.0);

    let broad_glow = glow_dot.powf(6.0);

    let core_glow = glow_dot.powf(45.0);

    color = color + Vec3::new(0.10, 0.45, 0.22) * broad_glow;

    color = color + Vec3::new(0.45, 1.00, 0.65) * core_glow * 1.8;

    let cyan_direction = Vec3::new(0.15, 0.20, -1.0).normalize();

    let cyan_glow = dir.dot(&cyan_direction).max(0.0).powf(10.0);

    color = color + Vec3::new(0.03, 0.22, 0.35) * cyan_glow;

    let star_u = (dir.x * 437.0).floor();

    let star_v = (dir.y * 613.0).floor();

    let star_w = (dir.z * 521.0).floor();

    let star_noise = procedural_hash(star_u + star_w * 0.37, star_v + star_w * 0.71);

    if star_noise > 0.9965 {
        let star_strength = (star_noise - 0.9965) / 0.0035;

        let tint = procedural_hash(star_u * 0.31, star_v * 0.73);

        let star_color = if tint < 0.33 {
            Vec3::new(0.75, 0.90, 1.0)
        } else if tint < 0.66 {
            Vec3::new(0.85, 1.0, 0.90)
        } else {
            Vec3::new(1.0, 1.0, 1.0)
        };

        color = color + star_color * (star_strength * 1.8);
    }

    color
}

fn skybox_galaxy_2(direction: &Vec3) -> Vec3 {
    let dir = direction.normalize();

    let vertical = (dir.y * 0.5 + 0.5).clamp(0.0, 1.0);

    let bottom = Vec3::new(0.008, 0.003, 0.025);

    let top = Vec3::new(0.055, 0.015, 0.11);

    let mut color = bottom * (1.0 - vertical) + top * vertical;

    let wave_1 = (dir.x * 4.0 + dir.z * 7.0 + (dir.y * 3.0).sin() * 2.0).sin() * 0.5 + 0.5;

    let wave_2 = (dir.z * 5.0 - dir.x * 6.0 + (dir.y * 8.0).cos()).cos() * 0.5 + 0.5;

    let wave_3 = (dir.x * 13.0 + dir.y * 9.0 + dir.z * 4.0).sin() * 0.5 + 0.5;

    let purple_nebula = (wave_1 * wave_2).powf(1.6);

    color = color + Vec3::new(0.13, 0.025, 0.25) * purple_nebula * 1.2;

    let magenta_nebula = (wave_2 * wave_3).powf(2.4);

    color = color + Vec3::new(0.22, 0.015, 0.16) * magenta_nebula;

    let blue_nebula = (wave_1 * (1.0 - wave_3)).powf(2.0);

    color = color + Vec3::new(0.015, 0.055, 0.20) * blue_nebula;

    let galaxy_band = (1.0 - (dir.y + (dir.x * 2.7).sin() * 0.16).abs() * 3.7)
        .clamp(0.0, 1.0)
        .powf(2.0);

    color = color + Vec3::new(0.12, 0.04, 0.18) * galaxy_band;

    let cloud_detail =
        ((dir.x * 25.0 + dir.z * 17.0).sin() * (dir.y * 21.0 - dir.x * 9.0).cos() * 0.5 + 0.5)
            .powf(3.0);

    color = color + Vec3::new(0.08, 0.025, 0.14) * cloud_detail * galaxy_band;

    let glow_direction = Vec3::new(0.75, 0.08, -0.65).normalize();

    let glow = dir.dot(&glow_direction).max(0.0);

    let broad_glow = glow.powf(8.0);

    let center_glow = glow.powf(45.0);

    color = color + Vec3::new(0.25, 0.04, 0.32) * broad_glow;

    color = color + Vec3::new(0.85, 0.28, 1.0) * center_glow * 1.4;

    let blue_glow_direction = Vec3::new(-0.8, 0.25, 0.45).normalize();

    let blue_glow = dir.dot(&blue_glow_direction).max(0.0).powf(18.0);

    color = color + Vec3::new(0.08, 0.28, 0.70) * blue_glow;

    let star_x = (dir.x * 813.0).floor();

    let star_y = (dir.y * 991.0).floor();

    let star_z = (dir.z * 677.0).floor();

    let star_hash = procedural_hash(star_x + star_z * 0.17, star_y + star_x * 0.23);

    if star_hash > 0.992 {
        let intensity = (star_hash - 0.992) / 0.008;

        let cold = procedural_hash(star_x * 0.31, star_z * 0.73);

        let star_color = if cold < 0.35 {
            Vec3::new(0.55, 0.70, 1.0)
        } else if cold < 0.70 {
            Vec3::new(1.0, 0.72, 0.95)
        } else {
            Vec3::new(1.0, 1.0, 1.0)
        };

        color = color + star_color * intensity * 1.8;
    }

    let bright_star_hash = procedural_hash(star_x * 1.91 + 7.0, star_z * 2.17 + star_y);

    if bright_star_hash > 0.9985 {
        let intensity = (bright_star_hash - 0.9985) / 0.0015;

        color = color + Vec3::new(0.70, 0.82, 1.0) * intensity * 2.5;
    }

    color
}

fn procedural_hash(x: f32, y: f32) -> f32 {
    let value = (x * 12.9898 + y * 78.233).sin() * 43758.5453;

    value - value.floor()
}

fn multiply_vec3(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

fn to_color(color: Vec3) -> Color {
    let r = (color.x.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8;

    let g = (color.y.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8;

    let b = (color.z.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8;

    Color::new(r, g, b, 255)
}
