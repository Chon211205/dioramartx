use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

pub fn create_industrial_planet() -> Vec<Object> {
    let mut objects =
        Vec::with_capacity(
            40,
        );

    let rock =
        Material::new(
            Vec3::new(
                0.34,
                0.32,
                0.27,
            ),
            0.88,
            0.20,
            0.0,
            0.02,
        );

    let rock_dark =
        Material::new(
            Vec3::new(
                0.22,
                0.21,
                0.18,
            ),
            0.85,
            0.15,
            0.0,
            0.01,
        );

    let rusty_metal =
        Material::new(
            Vec3::new(
                0.55,
                0.20,
                0.07,
            ),
            0.82,
            0.75,
            0.0,
            0.10,
        );

    let rusty_dark =
        Material::new(
            Vec3::new(
                0.32,
                0.10,
                0.035,
            ),
            0.82,
            0.60,
            0.0,
            0.06,
        );

    let metal_band =
        Material::new(
            Vec3::new(
                0.68,
                0.27,
                0.08,
            ),
            0.88,
            0.90,
            0.0,
            0.13,
        );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                2.65,
                rock,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    -1.75,
                    0.30,
                    1.70,
                ),
                0.50,
                rock_dark,
            ),
        ),
    );

    add_tower(
        &mut objects,
        Vec3::new(
            0.15,
            2.42,
            0.10,
        ),
        Vec3::new(
            0.10,
            1.0,
            0.12,
        )
            .normalize(),
        2.55,
        1.18,
        0.82,
        rusty_metal,
        rusty_dark,
        metal_band,
    );

    add_tower(
        &mut objects,
        Vec3::new(
            -2.35,
            -0.15,
            0.15,
        ),
        Vec3::new(
            -1.0,
            -0.05,
            0.0,
        ),
        1.85,
        1.05,
        0.75,
        rusty_metal,
        rusty_dark,
        metal_band,
    );

    add_tower(
        &mut objects,
        Vec3::new(
            1.85,
            -1.45,
            1.05,
        ),
        Vec3::new(
            0.70,
            -0.48,
            0.52,
        )
            .normalize(),
        2.00,
        1.12,
        0.78,
        rusty_metal,
        rusty_dark,
        metal_band,
    );

    add_tower(
        &mut objects,
        Vec3::new(
            0.40,
            0.20,
            -2.45,
        ),
        Vec3::new(
            0.20,
            0.10,
            -1.0,
        )
            .normalize(),
        1.95,
        1.05,
        0.72,
        rusty_metal,
        rusty_dark,
        metal_band,
    );

    objects
}

#[allow(clippy::too_many_arguments)]
fn add_tower(
    objects: &mut Vec<Object>,
    base: Vec3,
    axis: Vec3,
    height: f32,
    base_radius: f32,
    top_radius: f32,
    rusty_metal: Material,
    rusty_dark: Material,
    metal_band: Material,
) {
    let axis =
        axis.normalize();

    let sections =
        4;

    let section_height =
        height
            / sections as f32;

    for i in
        0..sections
    {
        let t =
            i as f32
                / (
                    sections
                        - 1
                )
                    as f32;

        let radius =
            base_radius
                + (
                    top_radius
                        - base_radius
                )
                    * t;

        let center =
            base
                + axis
                    * (
                        section_height
                            * (
                                i as f32
                                    + 0.5
                            )
                    );

        let material =
            if i % 2
                == 0
            {
                rusty_metal
            } else {
                rusty_dark
            };

        objects.push(
            Object::Cylinder(
                Cylinder::new_oriented(
                    center,
                    axis,
                    radius,
                    section_height
                        + 0.05,
                    material,
                ),
            ),
        );

        if i
            < sections
                - 1
        {
            let band_center =
                base
                    + axis
                        * (
                            section_height
                                * (
                                    i as f32
                                        + 1.0
                                )
                        );

            objects.push(
                Object::Cylinder(
                    Cylinder::new_oriented(
                        band_center,
                        axis,
                        radius
                            + 0.08,
                        0.10,
                        metal_band,
                    ),
                ),
            );
        }
    }

    let base_band =
        base
            + axis
                * 0.10;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                base_band,
                axis,
                base_radius
                    + 0.12,
                0.18,
                metal_band,
            ),
        ),
    );

    let top =
        base
            + axis
                * height;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                top,
                axis,
                top_radius
                    + 0.14,
                0.20,
                metal_band,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                top
                    + axis
                        * 0.02,
                axis,
                top_radius
                    * 0.78,
                0.08,
                rusty_dark,
            ),
        ),
    );
}