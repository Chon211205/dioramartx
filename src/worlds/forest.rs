use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cone::Cone;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::textures::texture::TextureMap;

static GRASS_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_AO_MAP: OnceLock<TextureMap> = OnceLock::new();

static BARK_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static BARK_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static BARK_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static BARK_AO_MAP: OnceLock<TextureMap> = OnceLock::new();

static ROCK_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static ROCK_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static ROCK_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();

static FLOWER_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static FLOWER_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static FLOWER_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static FLOWER_AO_MAP: OnceLock<TextureMap> = OnceLock::new();

static GROUND_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_AO_MAP: OnceLock<TextureMap> = OnceLock::new();

static BRICK_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static BRICK_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static BRICK_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static BRICK_AO_MAP: OnceLock<TextureMap> = OnceLock::new();

fn grass_color_map() -> &'static TextureMap {
    GRASS_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_1K-PNG_Color.png",
        )
    })
}

fn grass_normal_map() -> &'static TextureMap {
    GRASS_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_1K-PNG_NormalGL.png",
        )
    })
}

fn grass_roughness_map() -> &'static TextureMap {
    GRASS_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_1K-PNG_Roughness.png",
        )
    })
}

fn grass_ao_map() -> &'static TextureMap {
    GRASS_AO_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_1K-PNG_AmbientOcclusion.png",
        )
    })
}

fn bark_color_map() -> &'static TextureMap {
    BARK_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bark/Bark014_1K-PNG_Color.png",
        )
    })
}

fn bark_normal_map() -> &'static TextureMap {
    BARK_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bark/Bark014_1K-PNG_NormalGL.png",
        )
    })
}

fn bark_roughness_map() -> &'static TextureMap {
    BARK_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bark/Bark014_1K-PNG_Roughness.png",
        )
    })
}

fn bark_ao_map() -> &'static TextureMap {
    BARK_AO_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bark/Bark014_1K-PNG_AmbientOcclusion.png",
        )
    })
}

fn rock_color_map() -> &'static TextureMap {
    ROCK_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/rock/Rock041_1K-PNG_Color.png",
        )
    })
}

fn rock_normal_map() -> &'static TextureMap {
    ROCK_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/rock/Rock041_1K-PNG_NormalGL.png",
        )
    })
}

fn rock_roughness_map() -> &'static TextureMap {
    ROCK_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/rock/Rock041_1K-PNG_Roughness.png",
        )
    })
}

fn flower_color_map() -> &'static TextureMap {
    FLOWER_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/leaves/ScatteredLeaves008_1K-PNG_Color.png",
        )
    })
}

fn flower_normal_map() -> &'static TextureMap {
    FLOWER_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/leaves/ScatteredLeaves008_1K-PNG_NormalGL.png",
        )
    })
}

fn flower_roughness_map() -> &'static TextureMap {
    FLOWER_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/leaves/ScatteredLeaves008_1K-PNG_Roughness.png",
        )
    })
}

fn flower_ao_map() -> &'static TextureMap {
    FLOWER_AO_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/leaves/ScatteredLeaves008_1K-PNG_AmbientOcclusion.png",
        )
    })
}

fn ground_color_map() -> &'static TextureMap {
    GROUND_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ground/Ground048_1K-PNG_Color.png",
        )
    })
}

fn ground_normal_map() -> &'static TextureMap {
    GROUND_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ground/Ground048_1K-PNG_NormalGL.png",
        )
    })
}

fn ground_roughness_map() -> &'static TextureMap {
    GROUND_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ground/Ground048_1K-PNG_Roughness.png",
        )
    })
}

fn ground_ao_map() -> &'static TextureMap {
    GROUND_AO_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ground/Ground048_1K-PNG_AmbientOcclusion.png",
        )
    })
}

fn brick_color_map() -> &'static TextureMap {
    BRICK_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bricks/Bricks060_1K-PNG_Color.png",
        )
    })
}

fn brick_normal_map() -> &'static TextureMap {
    BRICK_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bricks/Bricks060_1K-PNG_NormalGL.png",
        )
    })
}

fn brick_roughness_map() -> &'static TextureMap {
    BRICK_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bricks/Bricks060_1K-PNG_Roughness.png",
        )
    })
}

fn brick_ao_map() -> &'static TextureMap {
    BRICK_AO_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bricks/Bricks060_1K-PNG_AmbientOcclusion.png",
        )
    })
}


pub fn create_forest_diorama() -> Vec<Object> {
    let mut objects = Vec::new();

    let grass = Material::grass_textured(
        Vec3::new(0.14, 0.55, 0.12),
        0.85,
        0.12,
        0.0,
        0.02,
        Some(grass_color_map()),
        Some(grass_normal_map()),
        Some(grass_roughness_map()),
        Some(grass_ao_map()),
    );

    let soil = Material::textured(
        Vec3::new(
            0.55,
            0.40,
            0.28,
        ),
        0.82,
        0.10,
        0.0,
        0.01,
        Some(ground_color_map()),
        Some(ground_normal_map()),
        Some(ground_roughness_map()),
        Some(ground_ao_map()),
    );

    let wood = Material::textured(
        Vec3::new(0.30, 0.12, 0.035),
        0.78,
        0.12,
        0.0,
        0.02,
        Some(bark_color_map()),
        Some(bark_normal_map()),
        Some(bark_roughness_map()),
        Some(bark_ao_map()),
    );

    let leaves_dark = Material::textured(
        Vec3::new(0.04, 0.30, 0.05),
        0.85,
        0.15,
        0.0,
        0.02,
        Some(grass_color_map()),
        Some(grass_normal_map()),
        Some(grass_roughness_map()),
        Some(grass_ao_map()),
    );

    let leaves_light = Material::textured(
        Vec3::new(0.08, 0.46, 0.08),
        0.88,
        0.15,
        0.0,
        0.02,
        Some(grass_color_map()),
        Some(grass_normal_map()),
        Some(grass_roughness_map()),
        Some(grass_ao_map()),
    );

    let bush = Material::textured(
        Vec3::new(0.05, 0.35, 0.07),
        0.82,
        0.12,
        0.0,
        0.02,
        Some(grass_color_map()),
        Some(grass_normal_map()),
        Some(grass_roughness_map()),
        Some(grass_ao_map()),
    );

    let rock = Material::textured(
        Vec3::new(
            0.40,
            0.40,
            0.38,
        ),
        0.78,
        0.18,
        0.0,
        0.06,
        Some(rock_color_map()),
        Some(rock_normal_map()),
        Some(rock_roughness_map()),
        None,
    );

    let rock_light = Material::textured(
        Vec3::new(
            0.55,
            0.55,
            0.52,
        ),
        0.82,
        0.16,
        0.0,
        0.05,
        Some(rock_color_map()),
        Some(rock_normal_map()),
        Some(rock_roughness_map()),
        None,
    );

    let yellow = Material::textured(
        Vec3::new(
            1.0,
            0.82,
            0.05,
        ),
        0.85,
        0.25,
        0.0,
        0.02,
        Some(flower_color_map()),
        Some(flower_normal_map()),
        Some(flower_roughness_map()),
        Some(flower_ao_map()),
    );

    let purple = Material::textured(
        Vec3::new(
            0.60,
            0.18,
            0.80,
        ),
        0.82,
        0.25,
        0.0,
        0.02,
        Some(flower_color_map()),
        Some(flower_normal_map()),
        Some(flower_roughness_map()),
        Some(flower_ao_map()),
    );

    let pink = Material::textured(
        Vec3::new(
            1.0,
            0.30,
            0.55,
        ),
        0.82,
        0.25,
        0.0,
        0.02,
        Some(flower_color_map()),
        Some(flower_normal_map()),
        Some(flower_roughness_map()),
        Some(flower_ao_map()),
    );

    let white = Material::textured(
        Vec3::new(
            0.95,
            0.95,
            0.95,
        ),
        0.85,
        0.25,
        0.0,
        0.02,
        Some(flower_color_map()),
        Some(flower_normal_map()),
        Some(flower_roughness_map()),
        Some(flower_ao_map()),
    );

    let wall = Material::textured(
        Vec3::new(
            0.90,
            0.88,
            0.84,
        ),
        0.82,
        0.16,
        0.0,
        0.02,
        Some(brick_color_map()),
        Some(brick_normal_map()),
        Some(brick_roughness_map()),
        Some(brick_ao_map()),
    );

    let roof = Material::new(
        Vec3::new(0.55, 0.10, 0.05),
        0.78,
        0.15,
        0.0,
        0.02,
    );

    let pipe = Material::new(
        Vec3::new(
            0.10,
            0.45,
            0.14,
        ),
        0.60,
        0.85,
        0.0,
        0.22,
    );

    let mushroom_red = Material::new(
        Vec3::new(0.85, 0.05, 0.04),
        0.80,
        0.30,
        0.0,
        0.02,
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(0.0, 0.0, 0.0),
                1.8,
                grass,
            ),
        ),
    );


    let patches = [
        (Vec3::new(0.20, 1.0, 0.20), 0.28),
        (Vec3::new(-0.65, 0.65, 0.40), 0.22),
        (Vec3::new(0.65, 0.55, -0.45), 0.25),
        (Vec3::new(-0.30, 0.75, -0.75), 0.20),

        (Vec3::new(0.30, -0.90, 0.20), 0.20),
        (Vec3::new(-0.50, -0.75, 0.35), 0.22),
        (Vec3::new(0.50, -0.65, -0.55), 0.20),
        (Vec3::new(-0.55, -0.65, -0.45), 0.18),
    ];

    for (direction, radius) in patches {
        add_patch(
            &mut objects,
            direction,
            radius,
            soil,
        );
    }

    let trees = [
        (Vec3::new(-0.55, 0.85, 0.20), 0.55),
        (Vec3::new(0.30, 1.00, 0.10), 0.60),
        (Vec3::new(0.75, 0.60, 0.30), 0.48),
        (Vec3::new(-0.80, 0.45, -0.35), 0.50),
        (Vec3::new(0.55, 0.45, -0.65), 0.52),
        (Vec3::new(-0.15, 0.70, 0.80), 0.46),
        (Vec3::new(0.10, 0.50, -0.90), 0.44),

        (Vec3::new(0.35, -0.90, 0.25), 0.48),
        (Vec3::new(-0.45, -0.85, 0.20), 0.52),
        (Vec3::new(0.60, -0.65, -0.45), 0.44),
        (Vec3::new(-0.65, -0.60, -0.40), 0.46),
        (Vec3::new(0.10, -0.75, 0.75), 0.43),
        (Vec3::new(-0.15, -0.75, -0.80), 0.42),

        (Vec3::new(0.95, 0.05, 0.20), 0.42),
        (Vec3::new(-0.95, -0.05, 0.15), 0.44),
        (Vec3::new(0.85, 0.05, -0.50), 0.40),
        (Vec3::new(-0.85, 0.10, -0.50), 0.43),
    ];

    for (direction, size) in trees {
        add_tree_on_planet(
            &mut objects,
            direction,
            size,
            wood,
            leaves_dark,
            leaves_light,
        );
    }

    let bushes = [
        (Vec3::new(-0.15, 0.90, 0.55), 0.18),
        (Vec3::new(0.60, 0.75, 0.10), 0.20),
        (Vec3::new(-0.60, 0.60, -0.55), 0.17),

        (Vec3::new(0.20, -0.95, 0.40), 0.18),
        (Vec3::new(-0.50, -0.75, 0.45), 0.17),
        (Vec3::new(0.55, -0.70, -0.45), 0.19),
        (Vec3::new(-0.60, -0.65, -0.45), 0.16),

        (Vec3::new(0.95, 0.15, -0.10), 0.16),
        (Vec3::new(-0.95, 0.10, 0.10), 0.17),
        (Vec3::new(0.15, 0.15, 0.98), 0.18),
        (Vec3::new(-0.15, -0.10, -0.98), 0.17),
    ];

    for (direction, size) in bushes {
        add_bush_on_planet(
            &mut objects,
            direction,
            size,
            bush,
        );
    }

    let rocks = [
        (Vec3::new(0.15, 0.85, 0.65), 0.16, rock),
        (Vec3::new(-0.35, 0.95, -0.40), 0.13, rock_light),
        (Vec3::new(0.75, 0.45, 0.55), 0.18, rock),
        (Vec3::new(-0.75, 0.40, 0.45), 0.14, rock_light),
        (Vec3::new(0.25, 0.60, -0.85), 0.15, rock),

        (Vec3::new(0.45, -0.85, 0.15), 0.15, rock),
        (Vec3::new(-0.40, -0.90, -0.20), 0.13, rock_light),
        (Vec3::new(0.75, -0.45, 0.40), 0.16, rock),
        (Vec3::new(-0.75, -0.45, 0.35), 0.14, rock_light),

        (Vec3::new(0.95, -0.10, -0.35), 0.13, rock),
        (Vec3::new(-0.95, 0.05, -0.25), 0.14, rock_light),
    ];

    for (direction, radius, material) in rocks {
        add_rock_on_planet(
            &mut objects,
            direction,
            radius,
            material,
        );
    }

    add_flower_patch(
        &mut objects,
        Vec3::new(0.15, 1.0, 0.45),
        yellow,
        white,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(-0.45, 0.85, 0.40),
        purple,
        white,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(0.55, 0.75, -0.30),
        pink,
        white,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(-0.25, 0.70, -0.75),
        yellow,
        purple,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(0.25, -0.95, 0.25),
        yellow,
        white,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(-0.30, -0.90, 0.35),
        purple,
        white,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(0.50, -0.70, -0.50),
        pink,
        white,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(-0.55, -0.65, -0.45),
        yellow,
        purple,
    );

    add_pipe_on_planet(
        &mut objects,
        Vec3::new(0.85, 0.45, -0.15),
        pipe,
    );

    add_pipe_on_planet(
        &mut objects,
        Vec3::new(-0.85, 0.40, 0.05),
        pipe,
    );

    add_tower_on_planet(
        &mut objects,
        Vec3::new(0.0, 1.0, -0.30),
        wall,
        roof,
    );

    add_tower_on_planet(
        &mut objects,
        Vec3::new(-0.45, 0.80, -0.50),
        wall,
        roof,
    );

    add_stump_on_planet(
        &mut objects,
        Vec3::new(0.35, 0.85, 0.55),
        wood,
    );

    add_stump_on_planet(
        &mut objects,
        Vec3::new(-0.55, 0.70, 0.55),
        wood,
    );

    add_stump_on_planet(
        &mut objects,
        Vec3::new(0.45, -0.75, 0.55),
        wood,
    );

    add_stump_on_planet(
        &mut objects,
        Vec3::new(-0.45, -0.75, -0.55),
        wood,
    );

    add_mushroom_on_planet(
        &mut objects,
        Vec3::new(0.45, 0.90, 0.35),
        white,
        mushroom_red,
    );

    add_mushroom_on_planet(
        &mut objects,
        Vec3::new(-0.35, 0.90, 0.55),
        white,
        mushroom_red,
    );

    add_mushroom_on_planet(
        &mut objects,
        Vec3::new(0.35, -0.90, 0.45),
        white,
        mushroom_red,
    );

    add_mushroom_on_planet(
        &mut objects,
        Vec3::new(-0.40, -0.85, -0.45),
        white,
        mushroom_red,
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
        normal * (1.8 + offset);

    (
        point,
        normal,
    )
}


fn add_tree_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    size: f32,
    wood: Material,
    leaves_dark: Material,
    leaves_light: Material,
) {
    let (surface, normal) =
        surface_point(
            direction,
            0.0,
        );

    let trunk_height =
        size * 0.62;

    let trunk_radius =
        size * 0.09;

    let trunk_center =
        surface
            + normal
                * (trunk_height * 0.5);

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                trunk_center,
                normal,
                trunk_radius,
                trunk_height,
                wood,
            ),
        ),
    );

    let lower_height =
        size * 0.68;

    let lower_center =
        surface
            + normal
                * (
                    trunk_height
                        + lower_height * 0.22
                );

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                lower_center,
                normal,
                size * 0.34,
                lower_height,
                leaves_dark,
            ),
        ),
    );

    let upper_height =
        size * 0.55;

    let upper_center =
        surface
            + normal
                * (
                    trunk_height
                        + size * 0.30
                );

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                upper_center,
                normal,
                size * 0.26,
                upper_height,
                leaves_light,
            ),
        ),
    );
}

fn add_bush_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    size: f32,
    material: Material,
) {
    let (point, normal) =
        surface_point(
            direction,
            size * 0.35,
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
            .cross(
                &helper,
            )
            .normalize();

    let bitangent =
        normal
            .cross(
                &tangent,
            )
            .normalize();

    objects.push(
        Object::Sphere(
            Sphere::new(
                point,
                size,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                point
                    + tangent
                        * (size * 0.65),
                size * 0.72,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                point
                    - tangent
                        * (size * 0.60)
                    + bitangent
                        * (size * 0.15),
                size * 0.68,
                material,
            ),
        ),
    );
}

fn add_rock_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    material: Material,
) {
    let (point, _) =
        surface_point(
            direction,
            radius * 0.35,
        );

    objects.push(
        Object::Sphere(
            Sphere::new(
                point,
                radius,
                material,
            ),
        ),
    );
}

fn add_patch(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    material: Material,
) {
    let normal =
        direction.normalize();

    let center =
        normal
            * (
                1.8
                    - radius * 0.72
            );

    objects.push(
        Object::Sphere(
            Sphere::new(
                center,
                radius,
                material,
            ),
        ),
    );
}

fn add_flower_patch(
    objects: &mut Vec<Object>,
    direction: Vec3,
    material_a: Material,
    material_b: Material,
) {
    let normal =
        direction.normalize();

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
            .cross(
                &helper,
            )
            .normalize();

    let bitangent =
        normal
            .cross(
                &tangent,
            )
            .normalize();

    let (center, _) =
        surface_point(
            direction,
            0.045,
        );

    let positions = [
        center,
        center + tangent * 0.08,
        center - tangent * 0.08,
        center + bitangent * 0.07,
        center - bitangent * 0.07,
    ];

    for (i, position) in
        positions
            .iter()
            .enumerate()
    {
        let material =
            if i % 2 == 0 {
                material_a
            } else {
                material_b
            };

        objects.push(
            Object::Sphere(
                Sphere::new(
                    *position,
                    0.035,
                    material,
                ),
            ),
        );
    }
}

fn add_pipe_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    material: Material,
) {
    let (surface, normal) =
        surface_point(
            direction,
            0.0,
        );

    let height =
        0.30;

    let center =
        surface
            + normal
                * (height * 0.5);

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                center,
                normal,
                0.11,
                height,
                material,
            ),
        ),
    );

    let top =
        surface
            + normal
                * (height + 0.025);

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                top,
                normal,
                0.15,
                0.07,
                material,
            ),
        ),
    );
}

fn add_tower_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    wall: Material,
    roof: Material,
) {
    let (surface, normal) =
        surface_point(
            direction,
            0.0,
        );

    let body_height =
        0.44;

    let body_center =
        surface
            + normal
                * (body_height * 0.5);

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                body_center,
                normal,
                0.18,
                body_height,
                wall,
            ),
        ),
    );

    let roof_height =
        0.26;

    let roof_center =
        surface
            + normal
                * (
                    body_height
                        + roof_height * 0.28
                );

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                roof_center,
                normal,
                0.27,
                roof_height,
                roof,
            ),
        ),
    );
}

fn add_stump_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    material: Material,
) {
    let (surface, normal) =
        surface_point(
            direction,
            0.0,
        );

    let height =
        0.20;

    let center =
        surface
            + normal
                * (height * 0.5);

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                center,
                normal,
                0.13,
                height,
                material,
            ),
        ),
    );
}

fn add_mushroom_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    stem_material: Material,
    cap_material: Material,
) {
    let (surface, normal) =
        surface_point(
            direction,
            0.0,
        );

    let stem_height =
        0.17;

    let stem_center =
        surface
            + normal
                * (stem_height * 0.5);

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                stem_center,
                normal,
                0.045,
                stem_height,
                stem_material,
            ),
        ),
    );

    let cap_center =
        surface
            + normal
                * (stem_height + 0.035);

    objects.push(
        Object::Sphere(
            Sphere::new(
                cap_center,
                0.12,
                cap_material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                cap_center
                    + normal * 0.105,
                0.025,
                stem_material,
            ),
        ),
    );
}