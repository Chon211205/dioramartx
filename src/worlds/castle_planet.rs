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

static LAVA_COLOR: OnceLock<TextureMap> = OnceLock::new();
static LAVA_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static LAVA_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();

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

fn lava_color() -> &'static TextureMap {
    LAVA_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_Color.png",
        )
    })
}

fn lava_normal() -> &'static TextureMap {
    LAVA_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_NormalGL.png",
        )
    })
}

fn lava_roughness() -> &'static TextureMap {
    LAVA_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_Roughness.png",
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
        Vec3::new(0.75, 0.75, 0.75),
        1.0,
        0.35,
        0.0,
        0.02,
        Some(castle_color()),
        Some(castle_normal()),
        Some(castle_roughness()),
        Some(castle_ao()),
    )
}

fn lava_material() -> Material {
    Material::textured(
        Vec3::new(1.0, 0.45, 0.15),
        1.0,
        0.90,
        0.0,
        0.08,
        Some(lava_color()),
        Some(lava_normal()),
        Some(lava_roughness()),
        None,
    )
}

fn rock_material() -> Material {
    Material::textured(
        Vec3::new(0.45, 0.40, 0.38),
        1.0,
        0.20,
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
        0.30,
        0.0,
        0.0,
    )
}

fn portal_inner_material() -> Material {
    Material::new(
        Vec3::new(0.82, 0.05, 0.95),
        1.0,
        0.80,
        0.0,
        0.10,
    )
}

fn pole_material() -> Material {
    Material::new(
        Vec3::new(0.18, 0.18, 0.18),
        1.0,
        0.50,
        0.0,
        0.0,
    )
}

fn flag_material() -> Material {
    Material::new(
        Vec3::new(0.90, 0.04, 0.04),
        1.0,
        0.50,
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
    let lava = lava_material();
    let rock = rock_material();

    objects.push(
        Object::Hemisphere(
            Hemisphere::new_with_materials(
                Vec3::new(
                    0.0,
                    -1.55,
                    0.0,
                ),
                3.00,
                Vec3::new(
                    0.0,
                    -1.0,
                    0.0,
                ),
                rock,
                lava,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    0.0,
                    -1.32,
                    0.0,
                ),
                3.08,
                0.34,
                lava,
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
                    -0.10,
                    0.0,
                ),
                1.85,
                2.20,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    0.0,
                    1.15,
                    -0.05,
                ),
                1.30,
                1.20,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    -1.30,
                    0.05,
                    0.05,
                ),
                0.55,
                1.85,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    1.30,
                    0.05,
                    0.05,
                ),
                0.55,
                1.85,
                castle,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new(
                Vec3::new(
                    -1.30,
                    1.04,
                    0.05,
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
                    1.30,
                    1.04,
                    0.05,
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
                    1.84,
                    -0.05,
                ),
                1.40,
                0.17,
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
    let count = 10;
    let radius = 1.68;

    for i in 0..count {
        let angle =
            i as f32
                / count as f32
                * std::f32::consts::PI
                * 2.0;

        let x =
            angle.cos()
                * radius;

        let z =
            angle.sin()
                * radius;

        objects.push(
            Object::Cube(
                Cube::new(
                    Vec3::new(
                        x,
                        1.20,
                        z,
                    ),
                    0.32,
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
    let count = 8;
    let radius = 1.14;

    for i in 0..count {
        let angle =
            i as f32
                / count as f32
                * std::f32::consts::PI
                * 2.0;

        let x =
            angle.cos()
                * radius;

        let z =
            angle.sin()
                * radius
                - 0.05;

        objects.push(
            Object::Cube(
                Cube::new(
                    Vec3::new(
                        x,
                        2.08,
                        z,
                    ),
                    0.28,
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
                    -0.55,
                    1.90,
                ),
                0.82,
                outer,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    0.0,
                    -0.55,
                    2.02,
                ),
                0.58,
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
                    2.95,
                    -0.05,
                ),
                0.025,
                1.55,
                pole,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    0.27,
                    3.43,
                    -0.05,
                ),
                0.26,
                flag,
            ),
        ),
    );
}