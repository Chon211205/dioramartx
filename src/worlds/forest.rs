use crate::core::vec3::Vec3;
use crate::materials::material::Material;

use crate::objects::cone::Cone;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

pub fn create_forest_diorama() -> Vec<Object> {
    let grass = Material::new(
        Vec3::new(0.12, 0.65, 0.18),
        0.8,
        0.25,
        0.0,
        0.05,
    );

    let wood = Material::new(
        Vec3::new(0.35, 0.16, 0.05),
        0.75,
        0.15,
        0.0,
        0.0,
    );

    let leaves = Material::new(
        Vec3::new(0.05, 0.45, 0.10),
        0.8,
        0.2,
        0.0,
        0.02,
    );

    let rock = Material::new(
        Vec3::new(0.38, 0.42, 0.40),
        0.7,
        0.25,
        0.0,
        0.05,
    );

    let mut objects: Vec<Object> = Vec::new();

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(0.0, 0.0, 0.0),
                1.8,
                grass,
            ),
        ),
    );

    add_tree(
        &mut objects,
        Vec3::new(0.0, 2.15, 0.0),
        0.8,
        wood,
        leaves,
    );

    add_tree(
        &mut objects,
        Vec3::new(-0.85, 1.85, 0.15),
        0.6,
        wood,
        leaves,
    );

    add_tree(
        &mut objects,
        Vec3::new(0.9, 1.75, -0.2),
        0.55,
        wood,
        leaves,
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(-1.1, 1.3, 0.45),
                0.28,
                rock,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(1.1, 1.2, 0.4),
                0.22,
                rock,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(0.55, 1.55, 0.75),
                0.18,
                rock,
            ),
        ),
    );

    objects
}

fn add_tree(
    objects: &mut Vec<Object>,
    position: Vec3,
    scale: f32,
    wood: Material,
    leaves: Material,
) {
    let trunk_height =
        1.0 * scale;

    let trunk_radius =
        0.12 * scale;

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    position.x,
                    position.y,
                    position.z,
                ),
                trunk_radius,
                trunk_height,
                wood,
            ),
        ),
    );

    objects.push(
        Object::Cone(
            Cone::new(
                Vec3::new(
                    position.x,
                    position.y + 0.65 * scale,
                    position.z,
                ),
                0.55 * scale,
                1.1 * scale,
                leaves,
            ),
        ),
    );

    objects.push(
        Object::Cone(
            Cone::new(
                Vec3::new(
                    position.x,
                    position.y + 1.05 * scale,
                    position.z,
                ),
                0.42 * scale,
                0.9 * scale,
                leaves,
            ),
        ),
    );
}