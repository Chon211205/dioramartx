use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cuboid::Cuboid;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

const TRACK_POINTS: [Vec3; 20] = [
    Vec3 { x: -6.5, y: 0.0, z: -5.0 },
    Vec3 { x: 5.8, y: 0.0, z: -5.0 },
    Vec3 { x: 6.8, y: 0.0, z: -4.0 },
    Vec3 { x: 6.8, y: 0.0, z: 1.0 },
    Vec3 { x: 5.6, y: 0.0, z: 2.2 },
    Vec3 { x: -1.2, y: 0.0, z: 2.2 },
    Vec3 { x: -2.3, y: 0.0, z: 3.2 },
    Vec3 { x: -2.3, y: 0.0, z: 6.0 },
    Vec3 { x: -1.2, y: 0.0, z: 7.0 },
    Vec3 { x: 6.2, y: 0.0, z: 7.0 },
    Vec3 { x: 7.2, y: 0.0, z: 6.0 },
    Vec3 { x: 7.2, y: 0.0, z: -7.0 },
    Vec3 { x: 6.0, y: 0.0, z: -8.2 },
    Vec3 { x: -3.8, y: 0.0, z: -8.2 },
    Vec3 { x: -5.0, y: 0.0, z: -7.0 },
    Vec3 { x: -5.0, y: 0.0, z: -4.0 },
    Vec3 { x: -6.2, y: 0.0, z: -2.8 },
    Vec3 { x: -7.2, y: 0.0, z: -2.8 },
    Vec3 { x: -7.2, y: 0.0, z: -4.0 },
    Vec3 { x: -6.5, y: 0.0, z: -5.0 },
];

fn palette() -> [Material; 7] {
    [
        Material::new(Vec3::new(1.0, 0.05, 0.08), 0.95, 0.72, 0.0, 0.16),
        Material::new(Vec3::new(1.0, 0.38, 0.02), 0.95, 0.72, 0.0, 0.16),
        Material::new(Vec3::new(1.0, 0.92, 0.02), 0.95, 0.72, 0.0, 0.18),
        Material::new(Vec3::new(0.10, 0.95, 0.20), 0.95, 0.72, 0.0, 0.16),
        Material::new(Vec3::new(0.04, 0.72, 1.0), 0.95, 0.72, 0.0, 0.18),
        Material::new(Vec3::new(0.18, 0.18, 1.0), 0.95, 0.72, 0.0, 0.16),
        Material::new(Vec3::new(0.66, 0.16, 1.0), 0.95, 0.72, 0.0, 0.18),
    ]
}

pub fn create_rainbow_kart_world() -> Vec<Object> {
    let mut objects = Vec::with_capacity(520);
    let colors = palette();
    let tile_length = 0.42;
    let lane_width = 0.22;

    for segment in 0..TRACK_POINTS.len() - 1 {
        let start = TRACK_POINTS[segment];
        let end = TRACK_POINTS[segment + 1];
        let delta = end - start;
        let length = delta.length();
        let forward = delta.normalize();
        let right = Vec3::new(forward.z, 0.0, -forward.x).normalize();
        let steps = (length / tile_length).ceil() as usize;

        for step in 0..steps {
            let t = (step as f32 + 0.5) / steps as f32;
            let center = start + delta * t;

            for lane in -3..=3 {
                let material = colors[(lane + 3) as usize];
                objects.push(Object::Cuboid(Cuboid::from_basis(
                    center + right * (lane as f32 * lane_width),
                    Vec3::new(lane_width + 0.025, 0.10, length / steps as f32 + 0.035),
                    right,
                    Vec3::new(0.0, 1.0, 0.0),
                    forward,
                    material,
                )));
            }
        }
    }

    add_item_boxes(&mut objects);
    add_start_gate(&mut objects);
    add_karts(&mut objects);
    objects
}

fn add_item_boxes(objects: &mut Vec<Object>) {
    let cyan = Material::new(Vec3::new(0.12, 0.95, 1.0), 1.0, 0.9, 0.0, 0.35);
    let white = Material::new(Vec3::new(1.0, 1.0, 1.0), 1.0, 0.9, 0.0, 0.25);
    for (segment, offset) in [(1usize, 0.0), (5, 0.18), (10, -0.18), (13, 0.0)] {
        let center = (TRACK_POINTS[segment] + TRACK_POINTS[segment + 1]) * 0.5;
        let direction = (TRACK_POINTS[segment + 1] - TRACK_POINTS[segment]).normalize();
        let side = Vec3::new(direction.z, 0.0, -direction.x).normalize();
        for lane in -1..=1 {
            let position = center + side * (lane as f32 * 0.43 + offset) + Vec3::new(0.0, 0.34, 0.0);
            objects.push(Object::Cuboid(Cuboid::from_basis(
                position,
                Vec3::new(0.34, 0.34, 0.34),
                side,
                Vec3::new(0.0, 1.0, 0.0),
                direction,
                cyan,
            )));
            objects.push(Object::Sphere(Sphere::new(position, 0.09, white)));
        }
    }
}

fn add_start_gate(objects: &mut Vec<Object>) {
    let white = Material::new(Vec3::new(1.0, 1.0, 1.0), 0.95, 0.7, 0.0, 0.1);
    let black = Material::new(Vec3::new(0.015, 0.015, 0.025), 0.5, 0.3, 0.0, 0.0);
    let center = Vec3::new(-0.4, 0.0, -5.0);
    for lane in -3..=3 {
        objects.push(Object::Cuboid(Cuboid::from_basis(
            center + Vec3::new(0.0, 0.08, lane as f32 * 0.22),
            Vec3::new(0.32, 0.12, 0.22),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            if lane % 2 == 0 { white } else { black },
        )));
    }
}

fn add_karts(objects: &mut Vec<Object>) {
    let red = Material::new(Vec3::new(0.95, 0.04, 0.08), 0.85, 0.6, 0.0, 0.04);
    let dark = Material::new(Vec3::new(0.02, 0.02, 0.03), 0.3, 0.2, 0.0, 0.0);
    for (x, z) in [(-1.1, -5.2), (-1.6, -4.8), (-2.1, -5.2)] {
        objects.push(Object::Cuboid(Cuboid::from_basis(
            Vec3::new(x, 0.25, z),
            Vec3::new(0.48, 0.18, 0.34),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            red,
        )));
        for dz in [-0.22, 0.22] {
            objects.push(Object::Cylinder(Cylinder::new_oriented(
                Vec3::new(x, 0.18, z + dz),
                Vec3::new(0.0, 0.0, 1.0),
                0.10,
                0.54,
                dark,
            )));
        }
    }
}
