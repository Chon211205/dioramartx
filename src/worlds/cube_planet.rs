use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cone::Cone;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

pub fn create_cube_planet() -> Vec<Object> {
    let mut objects = Vec::new();

    let cube_body = Material::new(
        Vec3::new(0.24, 0.72, 0.28),
        0.92,
        0.25,
        0.0,
        0.03,
    );

    let dirt = Material::new(
        Vec3::new(0.44, 0.28, 0.14),
        0.95,
        0.12,
        0.0,
        0.0,
    );

    let grass = Material::new(
        Vec3::new(0.12, 0.78, 0.18),
        0.95,
        0.22,
        0.0,
        0.02,
    );

    let trunk = Material::new(
        Vec3::new(0.43, 0.26, 0.14),
        0.95,
        0.10,
        0.0,
        0.0,
    );

    let leaves = Material::new(
        Vec3::new(0.18, 0.62, 0.20),
        0.92,
        0.18,
        0.0,
        0.01,
    );

    let water = Material::new(
        Vec3::new(0.15, 0.55, 0.95),
        0.85,
        0.85,
        0.0,
        0.08,
    );

    let rock = Material::new(
        Vec3::new(0.55, 0.55, 0.58),
        0.90,
        0.20,
        0.0,
        0.02,
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                2.8,
                cube_body,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(0.0, 1.10, 0.0),
                2.1,
                grass,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(0.0, 0.55, 0.0),
                2.35,
                dirt,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(0.65, 1.32, -0.35),
                0.75,
                water,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(-0.95, 1.30, 0.55),
                0.38,
                rock,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(-0.45, 1.28, 0.95),
                0.28,
                rock,
            ),
        ),
    );

    add_tree(
        &mut objects,
        Vec3::new(-0.75, 1.38, -0.45),
        0.85,
        trunk,
        leaves,
    );

    add_tree(
        &mut objects,
        Vec3::new(0.95, 1.38, 0.70),
        0.72,
        trunk,
        leaves,
    );

    add_tree(
        &mut objects,
        Vec3::new(0.15, 1.38, 0.95),
        0.68,
        trunk,
        leaves,
    );

    add_bush(
        &mut objects,
        Vec3::new(-1.05, 1.28, -0.95),
        0.22,
        leaves,
    );

    add_bush(
        &mut objects,
        Vec3::new(0.35, 1.28, -1.00),
        0.20,
        leaves,
    );

    add_bush(
        &mut objects,
        Vec3::new(1.05, 1.28, -0.15),
        0.18,
        leaves,
    );

    add_flower(
        &mut objects,
        Vec3::new(-0.10, 1.28, -0.35),
    );

    add_flower(
        &mut objects,
        Vec3::new(0.55, 1.28, 0.35),
    );

    add_flower(
        &mut objects,
        Vec3::new(-0.65, 1.28, 0.15),
    );

    add_side_deco(
        &mut objects,
        Vec3::new(1.45, 0.20, 0.10),
        rock,
        leaves,
    );

    add_side_deco(
        &mut objects,
        Vec3::new(-1.45, -0.15, -0.35),
        rock,
        leaves,
    );

    add_side_deco(
        &mut objects,
        Vec3::new(0.20, -1.10, 1.45),
        rock,
        leaves,
    );

    objects
}

fn add_tree(
    objects: &mut Vec<Object>,
    position: Vec3,
    scale: f32,
    trunk: Material,
    leaves: Material,
) {
    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                position + Vec3::new(0.0, 0.22 * scale, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                0.10 * scale,
                0.42 * scale,
                trunk,
            ),
        ),
    );

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                position + Vec3::new(0.0, 0.58 * scale, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                0.34 * scale,
                0.42 * scale,
                leaves,
            ),
        ),
    );

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                position + Vec3::new(0.0, 0.82 * scale, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                0.25 * scale,
                0.34 * scale,
                leaves,
            ),
        ),
    );
}

fn add_bush(
    objects: &mut Vec<Object>,
    position: Vec3,
    radius: f32,
    material: Material,
) {
    objects.push(
        Object::Sphere(
            Sphere::new(
                position,
                radius,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                position + Vec3::new(0.16, 0.02, 0.04),
                radius * 0.72,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                position + Vec3::new(-0.14, 0.01, -0.05),
                radius * 0.68,
                material,
            ),
        ),
    );
}

fn add_flower(
    objects: &mut Vec<Object>,
    position: Vec3,
) {
    let stem = Material::new(
        Vec3::new(0.18, 0.72, 0.16),
        0.95,
        0.08,
        0.0,
        0.0,
    );

    let petal = Material::new(
        Vec3::new(1.0, 0.92, 0.18),
        0.95,
        0.25,
        0.0,
        0.0,
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                position + Vec3::new(0.0, 0.08, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                0.015,
                0.16,
                stem,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                position + Vec3::new(0.0, 0.18, 0.0),
                0.05,
                petal,
            ),
        ),
    );
}

fn add_side_deco(
    objects: &mut Vec<Object>,
    position: Vec3,
    rock: Material,
    leaves: Material,
) {
    objects.push(
        Object::Sphere(
            Sphere::new(
                position,
                0.16,
                rock,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                position + Vec3::new(0.18, 0.12, 0.04),
                0.10,
                leaves,
            ),
        ),
    );
}