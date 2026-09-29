use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::object::Object;
use crate::textures::texture::TextureMap;

static CASTLE_COLOR: OnceLock<TextureMap> = OnceLock::new();
static CASTLE_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static CASTLE_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();
static CASTLE_AO: OnceLock<TextureMap> = OnceLock::new();

static LAVA002_COLOR: OnceLock<TextureMap> = OnceLock::new();
static LAVA002_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static LAVA002_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();

static LAVA004_COLOR: OnceLock<TextureMap> = OnceLock::new();
static LAVA004_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static LAVA004_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();

static ROCK_COLOR: OnceLock<TextureMap> = OnceLock::new();
static ROCK_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static ROCK_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();

fn castle_color() -> &'static TextureMap {
    CASTLE_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/castle/Bricks066_1K-PNG_Color.png",
        )
    })
}

fn castle_normal() -> &'static TextureMap {
    CASTLE_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/castle/Bricks066_1K-PNG_NormalGL.png",
        )
    })
}

fn castle_roughness() -> &'static TextureMap {
    CASTLE_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/castle/Bricks066_1K-PNG_Roughness.png",
        )
    })
}

fn castle_ao() -> &'static TextureMap {
    CASTLE_AO.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/castle/Bricks066_1K-PNG_AmbientOcclusion.png",
        )
    })
}

fn lava002_color() -> &'static TextureMap {
    LAVA002_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_Color.png",
        )
    })
}

fn lava002_normal() -> &'static TextureMap {
    LAVA002_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_NormalGL.png",
        )
    })
}

fn lava002_roughness() -> &'static TextureMap {
    LAVA002_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_Roughness.png",
        )
    })
}

fn lava004_color() -> &'static TextureMap {
    LAVA004_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava004_1K-PNG_Color.png",
        )
    })
}

fn lava004_normal() -> &'static TextureMap {
    LAVA004_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava004_1K-PNG_NormalGL.png",
        )
    })
}

fn lava004_roughness() -> &'static TextureMap {
    LAVA004_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava004_1K-PNG_Roughness.png",
        )
    })
}

fn rock_color() -> &'static TextureMap {
    ROCK_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/rock/Rock041_1K-PNG_Color.png",
        )
    })
}

fn rock_normal() -> &'static TextureMap {
    ROCK_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/rock/Rock041_1K-PNG_NormalGL.png",
        )
    })
}

fn rock_roughness() -> &'static TextureMap {
    ROCK_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/rock/Rock041_1K-PNG_Roughness.png",
        )
    })
}

fn castle_material() -> Material {
    Material::textured(
        Vec3::new(0.82, 0.82, 0.82),
        1.0,
        0.30,
        0.0,
        0.02,
        Some(castle_color()),
        Some(castle_normal()),
        Some(castle_roughness()),
        Some(castle_ao()),
    )
}

fn lava_curved_material() -> Material {
    Material::textured(
        Vec3::new(1.0, 0.72, 0.52),
        1.0,
        0.75,
        0.0,
        0.06,
        Some(lava002_color()),
        Some(lava002_normal()),
        Some(lava002_roughness()),
        None,
    )
}

fn lava_flat_material() -> Material {
    Material::textured(
        Vec3::new(1.0, 0.82, 0.62),
        1.0,
        0.80,
        0.0,
        0.06,
        Some(lava004_color()),
        Some(lava004_normal()),
        Some(lava004_roughness()),
        None,
    )
}

fn rock_material() -> Material {
    Material::textured(
        Vec3::new(0.65, 0.62, 0.60),
        1.0,
        0.18,
        0.0,
        0.0,
        Some(rock_color()),
        Some(rock_normal()),
        Some(rock_roughness()),
        None,
    )
}

fn portal_outer_material() -> Material {
    Material::new(
        Vec3::new(0.24, 0.00, 0.30),
        1.0,
        0.25,
        0.0,
        0.0,
    )
}

fn portal_inner_material() -> Material {
    Material::new(
        Vec3::new(0.82, 0.04, 0.95),
        1.0,
        0.90,
        0.0,
        0.12,
    )
}

fn pole_material() -> Material {
    Material::new(
        Vec3::new(0.16, 0.16, 0.16),
        1.0,
        0.45,
        0.0,
        0.0,
    )
}

fn flag_material() -> Material {
    Material::new(
        Vec3::new(0.92, 0.03, 0.03),
        1.0,
        0.45,
        0.0,
        0.0,
    )
}

pub fn create_castle_planet() -> Vec<Object> {
    let mut objects = Vec::new();

    add_base(&mut objects);
    add_castle(&mut objects);
    add_portal(&mut objects);
    add_flag(&mut objects);

    objects
}

fn add_base(objects: &mut Vec<Object>) {
    let rock = rock_material();
    let lava_curved = lava_curved_material();
    let lava_flat = lava_flat_material();

    objects.push(
        Object::Hemisphere(
            Hemisphere::new_with_materials(
                Vec3::new(
                    0.0,
                    -1.58,
                    0.0,
                ),
                2.75,
                Vec3::new(
                    0.0,
                    -1.0,
                    0.0,
                ),
                rock,
                rock,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    0.0,
                    -1.34,
                    0.0,
                ),
                3.05,
                0.46,
                lava_curved,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    0.0,
                    -1.08,
                    0.0,
                ),
                2.92,
                0.08,
                lava_flat,
            ),
        ),
    );
}

fn add_castle(objects: &mut Vec<Object>) {
    let castle = castle_material();

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    0.0,
                    -0.05,
                    0.0,
                ),
                1.76,
                1.95,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    0.0,
                    1.08,
                    -0.10,
                ),
                1.23,
                1.18,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    -1.28,
                    0.03,
                    0.08,
                ),
                0.54,
                1.72,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    1.28,
                    0.03,
                    0.08,
                ),
                0.54,
                1.72,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    -1.28,
                    0.94,
                    0.08,
                ),
                0.66,
                0.16,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    1.28,
                    0.94,
                    0.08,
                ),
                0.66,
                0.16,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    0.0,
                    1.75,
                    -0.10,
                ),
                1.34,
                0.18,
                castle,
            ),
        ),
    );

    add_lower_battlements(
        objects,
        castle,
    );

    add_upper_battlements(
        objects,
        castle,
    );
}

fn add_lower_battlements(
    objects: &mut Vec<Object>,
    material: Material,
) {
    let radius = 1.60;
    let count = 10;

    for i in 0..count {
        let angle =
            i as f32
                / count as f32
                * std::f32::consts::PI
                * 2.0;

        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        objects.push(
            Object::Cube(
                Cube::new(
                    Vec3::new(
                        x,
                        1.09,
                        z,
                    ),
                    0.30,
                    material,
                ),
            ),
        );
    }
}

fn add_upper_battlements(
    objects: &mut Vec<Object>,
    material: Material,
) {
    let radius = 1.08;
    let count = 8;

    for i in 0..count {
        let angle =
            i as f32
                / count as f32
                * std::f32::consts::PI
                * 2.0;

        let x = angle.cos() * radius;

        let z =
            angle.sin() * radius
                - 0.10;

        objects.push(
            Object::Cube(
                Cube::new(
                    Vec3::new(
                        x,
                        1.97,
                        z,
                    ),
                    0.26,
                    material,
                ),
            ),
        );
    }
}

fn add_portal(objects: &mut Vec<Object>) {
    let outer = portal_outer_material();
    let inner = portal_inner_material();

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    0.0,
                    -0.48,
                    1.77,
                ),
                0.76,
                outer,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    0.0,
                    -0.48,
                    1.89,
                ),
                0.52,
                inner,
            ),
        ),
    );
}

fn add_flag(objects: &mut Vec<Object>) {
    let pole = pole_material();
    let flag = flag_material();

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    0.0,
                    2.87,
                    -0.10,
                ),
                0.020,
                1.55,
                pole,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    0.23,
                    3.36,
                    -0.10,
                ),
                0.22,
                flag,
            ),
        ),
    );
}