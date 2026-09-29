use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cylinder::Cylinder;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::object::Object;

pub fn create_pokeball_planet() -> Vec<Object> {
    let mut objects = Vec::new();

    let red = Material::new(
        Vec3::new(0.88, 0.05, 0.08),
        1.0,
        0.75,
        0.0,
        0.06,
    );

    let white = Material::new(
        Vec3::new(0.92, 0.94, 0.98),
        1.0,
        0.65,
        0.0,
        0.04,
    );

    let black = Material::new(
        Vec3::new(0.025, 0.025, 0.03),
        1.0,
        0.55,
        0.0,
        0.03,
    );

    let button_white = Material::new(
        Vec3::new(0.82, 0.84, 0.88),
        1.0,
        0.80,
        0.0,
        0.05,
    );

    let radius = 2.35;

    objects.push(
        Object::Hemisphere(
            Hemisphere::new_with_materials(
                Vec3::new(0.0, 0.0, 0.0),
                radius,
                Vec3::new(0.0, 1.0, 0.0),
                red,
                black,
            ),
        ),
    );

    objects.push(
        Object::Hemisphere(
            Hemisphere::new_with_materials(
                Vec3::new(0.0, 0.0, 0.0),
                radius,
                Vec3::new(0.0, -1.0, 0.0),
                white,
                black,
            ),
        ),
    );

    add_equator_band(
        &mut objects,
        black,
    );

    add_front_button(
        &mut objects,
        black,
        button_white,
    );

    objects
}

fn add_equator_band(
    objects: &mut Vec<Object>,
    black: Material,
) {
    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                2.41,
                0.24,
                black,
            ),
        ),
    );
}

fn add_front_button(
    objects: &mut Vec<Object>,
    black: Material,
    button_white: Material,
) {
    let axis =
        Vec3::new(
            0.0,
            0.0,
            1.0,
        );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    0.0,
                    0.0,
                    2.28,
                ),
                axis,
                0.64,
                0.18,
                black,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    0.0,
                    0.0,
                    2.40,
                ),
                axis,
                0.46,
                0.16,
                button_white,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    0.0,
                    0.0,
                    2.50,
                ),
                axis,
                0.30,
                0.10,
                button_white,
            ),
        ),
    );
}