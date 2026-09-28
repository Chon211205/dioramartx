use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::objects::torus::Torus;

pub struct GalaxySelectorVisual {
    pub objects: Vec<Object>,
    pub label_position: Vec3,
    pub center: Vec3,
}

fn yellow_material() -> Material {
    Material::new(
        Vec3::new(
            1.0,
            0.85,
            0.1,
        ),
        0.25,
        24.0,
        0.0,
        0.0,
    )
}

fn blue_material() -> Material {
    Material::new(
        Vec3::new(
            0.20,
            0.70,
            1.0,
        ),
        0.25,
        24.0,
        0.0,
        0.0,
    )
}

fn orange_material() -> Material {
    Material::new(
        Vec3::new(
            1.0,
            0.45,
            0.12,
        ),
        0.25,
        24.0,
        0.0,
        0.0,
    )
}

fn green_material() -> Material {
    Material::new(
        Vec3::new(
            0.35,
            0.9,
            0.35,
        ),
        0.25,
        24.0,
        0.0,
        0.0,
    )
}

fn halo_material() -> Material {
    Material::new(
        Vec3::new(
            1.0,
            0.95,
            0.35,
        ),
        0.05,
        48.0,
        0.0,
        0.0,
    )
}

pub fn create_selector_galaxy_1(
    selected: bool,
) -> GalaxySelectorVisual {
    let mut objects =
        Vec::new();

    let center =
        Vec3::new(
            -8.0,
            0.0,
            0.0,
        );

    objects.push(
        Object::Sphere(
            Sphere::new(
                center,
                1.4,
                blue_material(),
            ),
        ),
    );

    objects.push(
        Object::Torus(
            Torus::new(
                center,
                2.5,
                0.05,
                yellow_material(),
            ),
        ),
    );

    objects.push(
        Object::Torus(
            Torus::new(
                center,
                3.3,
                0.05,
                yellow_material(),
            ),
        ),
    );

    let satellites = [
        Vec3::new(
            -5.4,
            0.4,
            0.0,
        ),
        Vec3::new(
            -10.3,
            -0.2,
            0.0,
        ),
        Vec3::new(
            -8.0,
            2.0,
            0.0,
        ),
        Vec3::new(
            -8.0,
            -2.2,
            0.0,
        ),
        Vec3::new(
            -6.1,
            -1.6,
            0.0,
        ),
        Vec3::new(
            -9.9,
            1.7,
            0.0,
        ),
    ];

    for pos in satellites {
        objects.push(
            Object::Sphere(
                Sphere::new(
                    pos,
                    0.28,
                    green_material(),
                ),
            ),
        );
    }

    if selected {
        objects.push(
            Object::Torus(
                Torus::new(
                    center,
                    4.2,
                    0.12,
                    halo_material(),
                ),
            ),
        );
    }

    GalaxySelectorVisual {
        objects,
        label_position:
            Vec3::new(
                center.x,
                -4.8,
                center.z,
            ),
        center,
    }
}

pub fn create_selector_galaxy_2(
    selected: bool,
) -> GalaxySelectorVisual {
    let mut objects =
        Vec::new();

    let center =
        Vec3::new(
            8.0,
            0.0,
            0.0,
        );

    objects.push(
        Object::Sphere(
            Sphere::new(
                center,
                1.4,
                orange_material(),
            ),
        ),
    );

    objects.push(
        Object::Torus(
            Torus::new(
                center,
                2.5,
                0.05,
                yellow_material(),
            ),
        ),
    );

    objects.push(
        Object::Torus(
            Torus::new(
                center,
                3.3,
                0.05,
                yellow_material(),
            ),
        ),
    );

    let satellites = [
        Vec3::new(
            10.4,
            0.3,
            0.0,
        ),
        Vec3::new(
            5.8,
            -0.4,
            0.0,
        ),
        Vec3::new(
            8.0,
            2.2,
            0.0,
        ),
        Vec3::new(
            8.0,
            -2.0,
            0.0,
        ),
        Vec3::new(
            9.8,
            -1.4,
            0.0,
        ),
        Vec3::new(
            6.2,
            1.6,
            0.0,
        ),
    ];

    for pos in satellites {
        objects.push(
            Object::Sphere(
                Sphere::new(
                    pos,
                    0.28,
                    yellow_material(),
                ),
            ),
        );
    }

    if selected {
        objects.push(
            Object::Torus(
                Torus::new(
                    center,
                    4.2,
                    0.12,
                    halo_material(),
                ),
            ),
        );
    }

    GalaxySelectorVisual {
        objects,
        label_position:
            Vec3::new(
                center.x,
                -4.8,
                center.z,
            ),
        center,
    }
}