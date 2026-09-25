use std::f32::consts::PI;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cone::Cone;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::objects::hemisphere::Hemisphere;

pub fn create_ice_lava_diorama() -> Vec<Object> {
    let mut objects = Vec::new();

    let ice = Material::new(
        Vec3::new(
            0.32,
            0.72,
            0.95,
        ),
        0.72,
        0.95,
        0.12,
        0.18,
    );

    let ice_light = Material::new(
        Vec3::new(
            0.55,
            0.90,
            1.0,
        ),
        0.82,
        1.0,
        0.08,
        0.22,
    );

    let lava_rock = Material::new(
        Vec3::new(
            0.20,
            0.07,
            0.025,
        ),
        0.70,
        0.28,
        0.0,
        0.08,
    );

    let lava = Material::new(
        Vec3::new(
            1.0,
            0.20,
            0.015,
        ),
        1.0,
        0.80,
        0.0,
        0.10,
    );

    let orange_lava = Material::new(
        Vec3::new(
            1.0,
            0.48,
            0.02,
        ),
        1.0,
        0.90,
        0.0,
        0.08,
    );

    let crystal = Material::new(
        Vec3::new(
            0.08,
            0.92,
            1.0,
        ),
        0.75,
        1.0,
        0.16,
        0.20,
    );

    let crystal_core = Material::new(
        Vec3::new(
            0.95,
            0.90,
            0.12,
        ),
        1.0,
        1.0,
        0.0,
        0.10,
    );

    let stone = Material::new(
        Vec3::new(
            0.32,
            0.36,
            0.40,
        ),
        0.76,
        0.30,
        0.0,
        0.05,
    );

    let black_rock = Material::new(
        Vec3::new(
            0.055,
            0.045,
            0.04,
        ),
        0.65,
        0.20,
        0.0,
        0.04,
    );

    add_ice_ring(
        &mut objects,
        ice,
        ice_light,
    );

    add_lava_planet(
        &mut objects,
        lava_rock,
        lava,
        orange_lava,
    );

    add_crystal_cluster(
        &mut objects,
        Vec3::new(
            -1.38,
            0.20,
            0.35,
        ),
        0.42,
        crystal,
        crystal_core,
    );

    add_crystal_cluster(
        &mut objects,
        Vec3::new(
            1.20,
            0.18,
            0.40,
        ),
        0.46,
        crystal,
        crystal_core,
    );

    add_crystal_cluster(
        &mut objects,
        Vec3::new(
            -0.90,
            0.15,
            -1.02,
        ),
        0.36,
        crystal,
        crystal_core,
    );

    add_crystal_cluster(
        &mut objects,
        Vec3::new(
            0.95,
            0.14,
            -0.95,
        ),
        0.34,
        crystal,
        crystal_core,
    );

    add_stone_platform(
        &mut objects,
        Vec3::new(
            -1.05,
            0.20,
            0.95,
        ),
        0.28,
        0.30,
        stone,
    );

    add_stone_platform(
        &mut objects,
        Vec3::new(
            0.78,
            0.20,
            0.92,
        ),
        0.27,
        0.34,
        stone,
    );

    add_stone_platform(
        &mut objects,
        Vec3::new(
            1.20,
            0.17,
            -0.20,
        ),
        0.20,
        0.22,
        stone,
    );

    add_stone_platform(
        &mut objects,
        Vec3::new(
            -0.05,
            0.14,
            -1.30,
        ),
        0.18,
        0.18,
        stone,
    );

    add_black_rock(
        &mut objects,
        Vec3::new(
            0.05,
            0.23,
            0.10,
        ),
        black_rock,
        lava,
    );

    add_small_ice_rocks(
        &mut objects,
        stone,
    );

    objects
}

fn add_ice_ring(
    objects: &mut Vec<Object>,
    ice: Material,
    ice_light: Material,
) {
    const SEGMENTS: usize = 28;

    let inner_radius = 0.92;
    let middle_radius = 1.20;
    let outer_radius = 1.48;

    for i in 0..SEGMENTS {
        let angle =
            i as f32
                / SEGMENTS as f32
                * 2.0
                * PI;

        let cos_a =
            angle.cos();

        let sin_a =
            angle.sin();

        add_ice_piece(
            objects,
            Vec3::new(
                cos_a * inner_radius,
                0.0,
                sin_a * inner_radius,
            ),
            0.32,
            ice,
        );

        add_ice_piece(
            objects,
            Vec3::new(
                cos_a * middle_radius,
                -0.01,
                sin_a * middle_radius,
            ),
            0.34,
            ice_light,
        );

        add_ice_piece(
            objects,
            Vec3::new(
                cos_a * outer_radius,
                -0.03,
                sin_a * outer_radius,
            ),
            0.33,
            ice,
        );
    }
}

fn add_ice_piece(
    objects: &mut Vec<Object>,
    position: Vec3,
    radius: f32,
    material: Material,
) {
    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                position,
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                radius,
                0.16,
                material,
            ),
        ),
    );
}

fn add_lava_planet(
    objects: &mut Vec<Object>,
    rock: Material,
    _lava: Material,
    _orange_lava: Material,
) {
    // Hemisphere de arriba
    objects.push(
        Object::Hemisphere(
            Hemisphere::new(
                Vec3::new(
                    0.32,
                    0.30,
                    -1.76,
                ),
                0.82,
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                rock,
            ),
        ),
    );

    // Hemisphere de abajo
    objects.push(
        Object::Hemisphere(
            Hemisphere::new(
                Vec3::new(
                    0.32,
                    -0.30,
                    -1.76,
                ),
                0.82,
                Vec3::new(
                    0.0,
                    -1.0,
                    0.0,
                ),
                rock,
            ),
        ),
    );
}

fn add_lava_crack(
    objects: &mut Vec<Object>,
    center: Vec3,
    radius: f32,
    direction: Vec3,
    material: Material,
) {
    let normal =
        direction.normalize();

    let crack_center =
        center
            + normal
                * (
                    radius
                        + 0.025
                );

    let helper =
        if normal.y.abs()
            < 0.90
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
        normal
            .cross(
                &helper,
            )
            .normalize();

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                crack_center,
                tangent,
                0.025,
                0.32,
                material,
            ),
        ),
    );
}

fn add_crystal_cluster(
    objects: &mut Vec<Object>,
    position: Vec3,
    size: f32,
    crystal: Material,
    core: Material,
) {
    let directions = [
        (
            Vec3::new(
                0.0,
                1.0,
                0.0,
            ),
            1.0,
        ),
        (
            Vec3::new(
                0.28,
                0.96,
                0.05,
            ),
            0.82,
        ),
        (
            Vec3::new(
                -0.34,
                0.93,
                0.08,
            ),
            0.76,
        ),
        (
            Vec3::new(
                0.10,
                0.93,
                0.35,
            ),
            0.68,
        ),
        (
            Vec3::new(
                -0.10,
                0.94,
                -0.32,
            ),
            0.62,
        ),
    ];

    for (
        index,
        (
            direction,
            scale,
        ),
    ) in directions
        .iter()
        .enumerate()
    {
        let direction =
            direction.normalize();

        let height =
            size
                * scale;

        let center =
            position
                + direction
                    * (
                        height
                            * 0.40
                    );

        objects.push(
            Object::Cone(
                Cone::new_oriented(
                    center,
                    direction,
                    size
                        * (
                            if index == 0 {
                                0.24
                            } else {
                                0.17
                            }
                        ),
                    height,
                    crystal,
                ),
            ),
        );
    }

    objects.push(
        Object::Sphere(
            Sphere::new(
                position
                    + Vec3::new(
                        0.0,
                        size
                            * 0.32,
                        0.0,
                    ),
                size
                    * 0.105,
                core,
            ),
        ),
    );
}

fn add_stone_platform(
    objects: &mut Vec<Object>,
    position: Vec3,
    radius: f32,
    height: f32,
    material: Material,
) {
    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    position.x,
                    position.y
                        + height
                            * 0.5,
                    position.z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                radius,
                height,
                material,
            ),
        ),
    );
}

fn add_black_rock(
    objects: &mut Vec<Object>,
    position: Vec3,
    rock: Material,
    lava: Material,
) {
    let rock_radius =
        0.30;

    objects.push(
        Object::Sphere(
            Sphere::new(
                position,
                rock_radius,
                rock,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                position
                    + Vec3::new(
                        0.10,
                        0.015,
                        0.25,
                    ),
                0.065,
                lava,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                position
                    + Vec3::new(
                        -0.08,
                        0.08,
                        0.24,
                    ),
                0.040,
                lava,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                position
                    + Vec3::new(
                        0.02,
                        -0.07,
                        0.27,
                    ),
                0.035,
                lava,
            ),
        ),
    );
}

fn add_small_ice_rocks(
    objects: &mut Vec<Object>,
    material: Material,
) {
    let rocks = [
        (
            Vec3::new(
                -1.50,
                0.08,
                -0.30,
            ),
            0.10,
        ),
        (
            Vec3::new(
                1.48,
                0.07,
                0.22,
            ),
            0.08,
        ),
        (
            Vec3::new(
                0.35,
                0.06,
                1.42,
            ),
            0.075,
        ),
        (
            Vec3::new(
                -0.45,
                0.07,
                -1.43,
            ),
            0.09,
        ),
    ];

    for (
        position,
        radius,
    ) in rocks
    {
        objects.push(
            Object::Sphere(
                Sphere::new(
                    position,
                    radius,
                    material,
                ),
            ),
        );
    }
}