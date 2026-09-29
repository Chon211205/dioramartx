use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;

pub fn create_brick_planet() -> Vec<Object> {
    let mut objects = Vec::new();

    let cube_material = Material::new(
        Vec3::new(0.72, 0.36, 0.12),
        1.0,
        0.25,
        0.0,
        0.0,
    );

    let pipe_material = Material::new(
        Vec3::new(0.05, 0.72, 0.10),
        1.0,
        0.75,
        0.0,
        0.04,
    );

    let pipe_dark = Material::new(
        Vec3::new(0.02, 0.15, 0.03),
        1.0,
        0.20,
        0.0,
        0.0,
    );

    objects.push(Object::Cube(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.4,
        cube_material,
    )));

    add_top_pipe(&mut objects, pipe_material, pipe_dark);
    add_bottom_pipe(&mut objects, pipe_material, pipe_dark);

    objects
}

fn add_top_pipe(
    objects: &mut Vec<Object>,
    pipe_material: Material,
    pipe_dark: Material,
) {
    objects.push(Object::Cylinder(Cylinder::new(
        Vec3::new(0.0, 1.55, 0.0),
        0.42,
        0.70,
        pipe_material,
    )));

    objects.push(Object::Cylinder(Cylinder::new(
        Vec3::new(0.0, 1.90, 0.0),
        0.56,
        0.18,
        pipe_material,
    )));

    objects.push(Object::Cylinder(Cylinder::new(
        Vec3::new(0.0, 1.98, 0.0),
        0.30,
        0.05,
        pipe_dark,
    )));
}

fn add_bottom_pipe(
    objects: &mut Vec<Object>,
    pipe_material: Material,
    pipe_dark: Material,
) {
    objects.push(Object::Cylinder(Cylinder::new(
        Vec3::new(0.0, -1.55, 0.0),
        0.42,
        0.70,
        pipe_material,
    )));

    objects.push(Object::Cylinder(Cylinder::new(
        Vec3::new(0.0, -1.90, 0.0),
        0.56,
        0.18,
        pipe_material,
    )));

    objects.push(Object::Cylinder(Cylinder::new(
        Vec3::new(0.0, -1.98, 0.0),
        0.30,
        0.05,
        pipe_dark,
    )));
}