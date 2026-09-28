use std::f32::consts::TAU;
use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::textures::texture::TextureMap;

fn water_texture() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| TextureMap::from_file("assets/textures/water_circuit/water_flow.png"))
}

fn create_water_material() -> Material {
    Material::textured(
        Vec3::new(0.70, 0.92, 1.0),
        1.0,
        0.85,
        0.0,
        0.12,
        Some(water_texture()),
        None,
        None,
        None,
    )
}

pub fn create_water_circuit_world() -> Vec<Object> {
    let mut objects = Vec::with_capacity(120);

    let water = create_water_material();

    let foam = Material::new(Vec3::new(0.92, 0.99, 1.0), 1.0, 0.75, 0.0, 0.0);

    let yellow = Material::new(Vec3::new(1.0, 0.82, 0.05), 0.95, 0.80, 0.0, 0.02);

    let green = Material::new(Vec3::new(0.20, 0.95, 0.15), 0.95, 0.65, 0.0, 0.01);

    let green_dark = Material::new(Vec3::new(0.04, 0.45, 0.12), 0.90, 0.45, 0.0, 0.0);

    let white = Material::new(Vec3::new(0.94, 0.97, 1.0), 0.90, 0.45, 0.0, 0.0);

    let gold = Material::new(Vec3::new(1.0, 0.70, 0.08), 0.95, 0.80, 0.0, 0.03);

    let blue = Material::new(Vec3::new(0.04, 0.30, 0.95), 0.90, 0.60, 0.0, 0.0);

    add_water_track(&mut objects, water, foam);

    add_yellow_gates(&mut objects, yellow);

    add_green_tunnel(&mut objects, green, green_dark);

    add_finish_gate(&mut objects, gold);

    add_start_panels(&mut objects, blue, yellow);

    add_platforms(&mut objects, white, gold);

    add_supports(&mut objects, white);

    add_splashes(&mut objects, foam);

    objects
}

fn track_point(t: f32) -> Vec3 {
    let t = t.clamp(0.0, 1.0);

    if t < 0.18 {
        let u = t / 0.18;

        Vec3::new(-8.0 + u * 6.0, -0.7 + u * 1.8, 3.8 - u * 2.0)
    } else if t < 0.38 {
        let u = (t - 0.18) / 0.20;

        let angle = std::f32::consts::PI + u * std::f32::consts::PI;

        Vec3::new(
            -1.8 + angle.cos() * 4.0,
            1.0 + (u * std::f32::consts::PI).sin() * 1.1,
            0.8 + angle.sin() * 3.3,
        )
    } else if t < 0.55 {
        let u = (t - 0.38) / 0.17;

        Vec3::new(2.0 + u * 4.2, 1.0 + u * 2.6, 0.8 - u * 1.6)
    } else if t < 0.72 {
        let u = (t - 0.55) / 0.17;

        let angle = u * std::f32::consts::PI * 1.45;

        Vec3::new(
            6.2 + angle.sin() * 2.0,
            3.6 + angle.sin() * 0.7,
            -0.8 + angle.cos() * 2.5,
        )
    } else if t < 0.87 {
        let u = (t - 0.72) / 0.15;

        Vec3::new(7.9 - u * 4.5, 3.4 - u * 1.4, -3.1 + u * 1.8)
    } else {
        let u = (t - 0.87) / 0.13;

        let angle = u * TAU;

        Vec3::new(
            3.5 + angle.cos() * 2.2,
            2.0 + u * 2.0,
            -1.3 + angle.sin() * 2.2,
        )
    }
}

fn track_side(t: f32) -> Vec3 {
    let p0 = track_point((t - 0.004).max(0.0));

    let p1 = track_point((t + 0.004).min(1.0));

    let direction = (p1 - p0).normalize();

    let up = Vec3::new(0.0, 1.0, 0.0);

    let mut side = direction.cross(&up);

    if side.length() < 0.001 {
        side = Vec3::new(1.0, 0.0, 0.0);
    }

    side.normalize()
}

fn add_water_track(objects: &mut Vec<Object>, water: Material, foam: Material) {
    let segments = 36;

    for i in 0..segments {
        let t0 = i as f32 / segments as f32;

        let t1 = (i + 1) as f32 / segments as f32;

        let p0 = track_point(t0);

        let p1 = track_point(t1);

        let direction = p1 - p0;

        let length = direction.length();

        if length <= 0.001 {
            continue;
        }

        objects.push(Object::Cylinder(Cylinder::new_oriented(
            (p0 + p1) * 0.5,
            direction.normalize(),
            0.68,
            length + 0.20,
            water,
        )));

        if i % 8 == 0 {
            let middle = (p0 + p1) * 0.5;

            let side = track_side((t0 + t1) * 0.5);

            objects.push(Object::Sphere(Sphere::new(
                middle + side * 0.72,
                0.13,
                foam,
            )));

            objects.push(Object::Sphere(Sphere::new(
                middle - side * 0.72,
                0.13,
                foam,
            )));
        }
    }
}

fn add_yellow_gates(objects: &mut Vec<Object>, material: Material) {
    for t in [0.14, 0.19] {
        let center = track_point(t) + Vec3::new(0.0, 0.60, 0.0);

        let forward =
            (track_point((t + 0.01).min(1.0)) - track_point((t - 0.01).max(0.0))).normalize();

        add_ring(objects, center, forward, 1.20, 0.22, 8, material);
    }
}

fn add_green_tunnel(objects: &mut Vec<Object>, green: Material, green_dark: Material) {
    for i in 0..3 {
        let t = 0.65 + i as f32 * 0.04;

        let center = track_point(t) + Vec3::new(0.0, 0.15, 0.0);

        let forward =
            (track_point((t + 0.01).min(1.0)) - track_point((t - 0.01).max(0.0))).normalize();

        add_ring(
            objects,
            center,
            forward,
            1.15,
            0.20,
            8,
            if i % 2 == 0 { green } else { green_dark },
        );
    }
}

fn add_ring(
    objects: &mut Vec<Object>,
    center: Vec3,
    forward: Vec3,
    radius: f32,
    thickness: f32,
    segments: usize,
    material: Material,
) {
    let up = Vec3::new(0.0, 1.0, 0.0);

    let mut right = forward.cross(&up);

    if right.length() < 0.001 {
        right = Vec3::new(1.0, 0.0, 0.0);
    }

    right = right.normalize();

    let real_up = right.cross(&forward).normalize();

    for i in 0..segments {
        let angle = i as f32 / segments as f32 * TAU;

        let position = center + right * (angle.cos() * radius) + real_up * (angle.sin() * radius);

        objects.push(Object::Sphere(Sphere::new(position, thickness, material)));
    }
}

fn add_finish_gate(objects: &mut Vec<Object>, material: Material) {
    let t = 0.80;

    let center = track_point(t) + Vec3::new(0.0, 1.20, 0.0);

    let forward = (track_point(t + 0.01) - track_point(t - 0.01)).normalize();

    add_ring(objects, center, forward, 1.45, 0.24, 8, material);
}

fn add_start_panels(objects: &mut Vec<Object>, blue: Material, yellow: Material) {
    let base = track_point(0.04);

    let side = track_side(0.04);

    for i in 0..2 {
        let center = base - side * (1.3 + i as f32 * 0.75) + Vec3::new(0.0, 0.30, 0.0);

        objects.push(Object::Sphere(Sphere::new(center, 0.30, blue)));

        objects.push(Object::Sphere(Sphere::new(
            center + Vec3::new(0.0, 0.18, 0.0),
            0.10,
            yellow,
        )));
    }
}

fn add_platforms(objects: &mut Vec<Object>, white: Material, gold: Material) {
    let center = Vec3::new(7.2, 1.6, -4.1);

    objects.push(Object::Cylinder(Cylinder::new(center, 0.58, 0.20, white)));

    objects.push(Object::Sphere(Sphere::new(
        center + Vec3::new(0.0, 0.28, 0.0),
        0.27,
        white,
    )));

    objects.push(Object::Cylinder(Cylinder::new(
        center + Vec3::new(0.0, 0.47, 0.0),
        0.04,
        0.45,
        gold,
    )));
}

fn add_supports(objects: &mut Vec<Object>, material: Material) {
    for t in [0.25, 0.70] {
        let point = track_point(t);

        let bottom = -4.5;

        let height = (point.y - bottom).max(0.5);

        objects.push(Object::Cylinder(Cylinder::new(
            Vec3::new(point.x, bottom + height * 0.5, point.z),
            0.09,
            height,
            material,
        )));
    }
}

fn add_splashes(objects: &mut Vec<Object>, foam: Material) {
    for t in [0.38, 0.73] {
        let base = track_point(t);

        for i in 0..2 {
            let angle = i as f32 * std::f32::consts::PI;

            objects.push(Object::Sphere(Sphere::new(
                base + Vec3::new(
                    angle.cos() * 0.30,
                    0.25 + i as f32 * 0.13,
                    angle.sin() * 0.30,
                ),
                0.075,
                foam,
            )));
        }
    }
}
