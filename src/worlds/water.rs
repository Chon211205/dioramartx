use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cone::Cone;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

const PLANET_RADIUS: f32 = 1.8;

pub fn create_water_diorama() -> Vec<Object> {
    let mut objects = Vec::new();

    let water = Material::new(
        Vec3::new(
            0.04,
            0.42,
            0.78,
        ),
        0.48,
        0.90,
        0.16,
        0.12,
    );

    let waterfall = Material::new(
        Vec3::new(
            0.18,
            0.68,
            0.95,
        ),
        0.55,
        0.85,
        0.22,
        0.10,
    );

    let rock = Material::new(
        Vec3::new(
            0.48,
            0.34,
            0.20,
        ),
        0.75,
        0.15,
        0.0,
        0.03,
    );

    let grass = Material::new(
        Vec3::new(
            0.16,
            0.58,
            0.10,
        ),
        0.85,
        0.12,
        0.0,
        0.01,
    );

    let sand = Material::new(
        Vec3::new(
            0.88,
            0.76,
            0.50,
        ),
        0.82,
        0.10,
        0.0,
        0.0,
    );

    let palm_wood = Material::new(
        Vec3::new(
            0.42,
            0.23,
            0.08,
        ),
        0.75,
        0.12,
        0.0,
        0.02,
    );

    let palm_leaf = Material::new(
        Vec3::new(
            0.08,
            0.48,
            0.10,
        ),
        0.85,
        0.18,
        0.0,
        0.01,
    );

    let dark_rock = Material::new(
        Vec3::new(
            0.25,
            0.22,
            0.18,
        ),
        0.72,
        0.12,
        0.0,
        0.03,
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                PLANET_RADIUS,
                water,
            ),
        ),
    );

    add_island(
        &mut objects,
        Vec3::new(-0.80, 0.55, 0.45),
        0.40,
        rock,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(-0.35, 0.80, 0.55),
        0.30,
        rock,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(0.10, 0.92, 0.45),
        0.34,
        rock,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(0.65, 0.62, 0.30),
        0.38,
        rock,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(-0.55, 0.40, -0.72),
        0.28,
        rock,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(0.10, 0.55, -0.88),
        0.25,
        rock,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(-0.10, -0.75, 0.70),
        0.24,
        rock,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(0.65, -0.55, 0.45),
        0.27,
        rock,
        sand,
        grass,
    );

    add_plateau(
        &mut objects,
        Vec3::new(0.65, 0.55, -0.25),
        0.50,
        0.72,
        rock,
        grass,
    );

    add_plateau(
        &mut objects,
        Vec3::new(0.72, 0.45, -0.38),
        0.40,
        0.55,
        rock,
        grass,
    );

    add_plateau(
        &mut objects,
        Vec3::new(0.52, 0.72, -0.18),
        0.30,
        0.42,
        rock,
        grass,
    );

    add_plateau(
        &mut objects,
        Vec3::new(0.40, 0.85, -0.05),
        0.24,
        0.28,
        rock,
        grass,
    );

    add_palm(
        &mut objects,
        Vec3::new(-0.80, 0.55, 0.45),
        0.42,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(-0.35, 0.80, 0.55),
        0.34,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(0.10, 0.92, 0.45),
        0.36,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(0.65, 0.62, 0.30),
        0.38,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(0.52, 0.72, -0.18),
        0.44,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(0.40, 0.85, -0.05),
        0.38,
        palm_wood,
        palm_leaf,
    );

    add_rock_column(
        &mut objects,
        Vec3::new(-0.20, 0.35, 0.95),
        0.12,
        0.30,
        dark_rock,
        grass,
    );

    add_rock_column(
        &mut objects,
        Vec3::new(0.35, 0.20, 0.95),
        0.10,
        0.24,
        dark_rock,
        grass,
    );

    add_rock_column(
        &mut objects,
        Vec3::new(-0.85, 0.10, -0.45),
        0.13,
        0.32,
        dark_rock,
        grass,
    );

    add_rock_column(
        &mut objects,
        Vec3::new(0.85, -0.10, 0.35),
        0.11,
        0.27,
        dark_rock,
        grass,
    );

    add_waterfall(
        &mut objects,
        Vec3::new(0.62, 0.60, -0.28),
        0.55,
        waterfall,
    );

    add_waterfall(
        &mut objects,
        Vec3::new(0.48, 0.72, -0.16),
        0.38,
        waterfall,
    );

    add_small_water_stream(
        &mut objects,
        Vec3::new(0.58, 0.66, -0.20),
        waterfall,
    );

    objects
}

fn surface_point(
    direction: Vec3,
    offset: f32,
) -> (Vec3, Vec3) {
    let normal =
        direction.normalize();

    let point =
        normal
            * (
                PLANET_RADIUS
                    + offset
            );

    (
        point,
        normal,
    )
}

fn add_island(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    rock: Material,
    sand: Material,
    grass: Material,
) {
    let normal =
        direction.normalize();

    let rock_center =
        normal
            * (
                PLANET_RADIUS
                    - radius * 0.42
            );

    objects.push(
        Object::Sphere(
            Sphere::new(
                rock_center,
                radius,
                rock,
            ),
        ),
    );

    let sand_center =
        normal
            * (
                PLANET_RADIUS
                    + radius * 0.16
            );

    objects.push(
        Object::Sphere(
            Sphere::new(
                sand_center,
                radius * 0.80,
                sand,
            ),
        ),
    );

    let grass_center =
        normal
            * (
                PLANET_RADIUS
                    + radius * 0.26
            );

    objects.push(
        Object::Sphere(
            Sphere::new(
                grass_center,
                radius * 0.55,
                grass,
            ),
        ),
    );
}

fn add_plateau(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    height: f32,
    rock: Material,
    grass: Material,
) {
    let (
        surface,
        normal,
    ) =
        surface_point(
            direction,
            0.0,
        );

    let body_center =
        surface
            + normal
                * (
                    height * 0.5
                );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                body_center,
                normal,
                radius,
                height,
                rock,
            ),
        ),
    );

    let grass_top =
        surface
            + normal
                * (
                    height
                        + 0.025
                );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                grass_top,
                normal,
                radius * 1.03,
                0.05,
                grass,
            ),
        ),
    );
}

fn add_palm(
    objects: &mut Vec<Object>,
    direction: Vec3,
    size: f32,
    wood: Material,
    leaves: Material,
) {
    let (
        surface,
        normal,
    ) =
        surface_point(
            direction,
            0.08,
        );

    let trunk_height =
        size * 0.85;

    let trunk_center =
        surface
            + normal
                * (
                    trunk_height
                        * 0.5
                );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                trunk_center,
                normal,
                size * 0.055,
                trunk_height,
                wood,
            ),
        ),
    );

    let crown =
        surface
            + normal
                * trunk_height;

    objects.push(
        Object::Sphere(
            Sphere::new(
                crown,
                size * 0.10,
                leaves,
            ),
        ),
    );

    let helper =
        if normal.y.abs() < 0.9 {
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
            .cross(&helper)
            .normalize();

    let bitangent =
        normal
            .cross(&tangent)
            .normalize();

    let directions = [
        tangent,
        -tangent,
        bitangent,
        -bitangent,
        (
            tangent
                + bitangent
        )
            .normalize(),
        (
            tangent
                - bitangent
        )
            .normalize(),
    ];

    for leaf_direction in directions {
        let leaf_length =
            size * 0.38;

        let leaf_center =
            crown
                + leaf_direction
                    * (
                        leaf_length
                            * 0.50
                    )
                - normal
                    * (
                        size
                            * 0.035
                    );

        objects.push(
            Object::Cone(
                Cone::new_oriented(
                    leaf_center,
                    leaf_direction,
                    size * 0.075,
                    leaf_length,
                    leaves,
                ),
            ),
        );
    }
}

fn add_rock_column(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    height: f32,
    rock: Material,
    grass: Material,
) {
    let (
        surface,
        normal,
    ) =
        surface_point(
            direction,
            0.0,
        );

    let center =
        surface
            + normal
                * (
                    height
                        * 0.5
                );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                center,
                normal,
                radius,
                height,
                rock,
            ),
        ),
    );

    let top =
        surface
            + normal
                * (
                    height
                        + 0.015
                );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                top,
                normal,
                radius * 1.05,
                0.03,
                grass,
            ),
        ),
    );
}

fn add_waterfall(
    objects: &mut Vec<Object>,
    direction: Vec3,
    height: f32,
    material: Material,
) {
    let (
        surface,
        normal,
    ) =
        surface_point(
            direction,
            0.02,
        );

    let helper =
        if normal.y.abs() < 0.9 {
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
            .cross(&helper)
            .normalize();

    for offset in [
        -0.10,
        -0.05,
        0.0,
        0.05,
        0.10,
    ] {
        let start =
            surface
                + tangent * offset
                + normal
                    * (
                        height
                            * 0.65
                    );

        let end =
            surface
                + tangent * offset
                + normal
                    * 0.04;

        add_cylinder_between(
            objects,
            start,
            end,
            0.025,
            material,
        );
    }
}

fn add_small_water_stream(
    objects: &mut Vec<Object>,
    direction: Vec3,
    material: Material,
) {
    let (
        surface,
        normal,
    ) =
        surface_point(
            direction,
            0.30,
        );

    let helper =
        if normal.y.abs() < 0.9 {
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
            .cross(&helper)
            .normalize();

    let start =
        surface
            - tangent * 0.22;

    let end =
        surface
            + tangent * 0.22;

    add_cylinder_between(
        objects,
        start,
        end,
        0.045,
        material,
    );
}

fn add_cylinder_between(
    objects: &mut Vec<Object>,
    start: Vec3,
    end: Vec3,
    radius: f32,
    material: Material,
) {
    let delta =
        end - start;

    let length =
        delta.length();

    if length <= 0.0001 {
        return;
    }

    let axis =
        delta / length;

    let center =
        (
            start
                + end
        ) * 0.5;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                center,
                axis,
                radius,
                length,
                material,
            ),
        ),
    );
}