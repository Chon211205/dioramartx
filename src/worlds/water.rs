use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::textures::texture::TextureMap;

const PLANET_RADIUS: f32 = 1.45;

static GRASS_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_AO_MAP: OnceLock<TextureMap> = OnceLock::new();

static GROUND_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_AO_MAP: OnceLock<TextureMap> = OnceLock::new();

static ROCK_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static ROCK_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static ROCK_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();

static SAND_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static SAND_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static SAND_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();

static PALM_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static PALM_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static PALM_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();

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

fn sand_color_map() -> &'static TextureMap {
    SAND_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/sand/Ground093C_1K-PNG_Color.png",
        )
    })
}

fn sand_normal_map() -> &'static TextureMap {
    SAND_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/sand/Ground093C_1K-PNG_NormalGL.png",
        )
    })
}

fn sand_roughness_map() -> &'static TextureMap {
    SAND_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/sand/Ground093C_1K-PNG_Roughness.png",
        )
    })
}

fn palm_color_map() -> &'static TextureMap {
    PALM_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/palm/Plastic017A_1K-PNG_Color.png",
        )
    })
}

fn palm_normal_map() -> &'static TextureMap {
    PALM_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/palm/Plastic017A_1K-PNG_NormalGL.png",
        )
    })
}

fn palm_roughness_map() -> &'static TextureMap {
    PALM_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/palm/Plastic017A_1K-PNG_Roughness.png",
        )
    })
}

pub fn create_water_diorama() -> Vec<Object> {
    let mut objects = Vec::new();

    let planet_center = Vec3::new(
        0.0,
        0.0,
        0.0,
    );

    let water = Material::new(
        Vec3::new(
            0.03,
            0.38,
            0.72,
        ),
        0.72,
        0.85,
        0.08,
        0.12,
    );

    let waterfall = Material::new(
        Vec3::new(
            0.15,
            0.70,
            0.95,
        ),
        0.75,
        0.80,
        0.03,
        0.02,
    );

    let ground = Material::textured(
        Vec3::new(
            0.80,
            0.74,
            0.68,
        ),
        0.80,
        0.12,
        0.0,
        0.01,
        Some(ground_color_map()),
        Some(ground_normal_map()),
        Some(ground_roughness_map()),
        Some(ground_ao_map()),
    );

    let grass = Material::textured(
        Vec3::new(
            0.80,
            1.0,
            0.80,
        ),
        0.85,
        0.12,
        0.0,
        0.01,
        Some(grass_color_map()),
        Some(grass_normal_map()),
        Some(grass_roughness_map()),
        Some(grass_ao_map()),
    );

    let sand = Material::textured(
        Vec3::new(
            1.0,
            0.97,
            0.90,
        ),
        0.82,
        0.08,
        0.0,
        0.0,
        Some(sand_color_map()),
        Some(sand_normal_map()),
        Some(sand_roughness_map()),
        None,
    );

    let rock = Material::textured(
        Vec3::new(
            0.80,
            0.78,
            0.75,
        ),
        0.78,
        0.15,
        0.0,
        0.03,
        Some(rock_color_map()),
        Some(rock_normal_map()),
        Some(rock_roughness_map()),
        None,
    );

    let palm_wood = Material::new(
        Vec3::new(
            0.38,
            0.20,
            0.07,
        ),
        0.75,
        0.10,
        0.0,
        0.01,
    );

    let palm_leaf = Material::textured(
        Vec3::new(
            0.85,
            1.0,
            0.85,
        ),
        0.85,
        0.12,
        0.0,
        0.01,
        Some(palm_color_map()),
        Some(palm_normal_map()),
        Some(palm_roughness_map()),
        None,
    );

    let block_yellow = Material::new(
        Vec3::new(
            1.0,
            0.78,
            0.05,
        ),
        0.82,
        0.35,
        0.0,
        0.03,
    );

    let block_white = Material::new(
        Vec3::new(
            0.98,
            0.98,
            0.98,
        ),
        0.88,
        0.20,
        0.0,
        0.01,
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                planet_center,
                PLANET_RADIUS,
                water,
            ),
        ),
    );

    add_island(
        &mut objects,
        planet_center,
        -0.82,
        0.40,
        0.26,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        planet_center,
        -0.50,
        0.82,
        0.19,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        planet_center,
        -0.10,
        0.62,
        0.17,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        planet_center,
        0.37,
        0.72,
        0.21,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        planet_center,
        -0.80,
        -0.35,
        0.20,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        planet_center,
        -0.20,
        -0.62,
        0.17,
        ground,
        sand,
        grass,
    );

    let big_x = 0.52;
    let big_z = -0.18;

    let big_surface = sphere_surface_point(
        planet_center,
        big_x,
        big_z,
    );

    let big_normal = sphere_normal(
        planet_center,
        big_surface,
    );

    let big_height = 0.34;

    add_plateau(
        &mut objects,
        big_surface,
        big_normal,
        0.40,
        big_height,
        ground,
        grass,
    );

    let second_surface = sphere_surface_point(
        planet_center,
        0.88,
        -0.43,
    );

    let second_normal = sphere_normal(
        planet_center,
        second_surface,
    );

    add_plateau(
        &mut objects,
        second_surface,
        second_normal,
        0.27,
        0.22,
        ground,
        grass,
    );

    let third_surface = sphere_surface_point(
        planet_center,
        0.55,
        -0.68,
    );

    let third_normal = sphere_normal(
        planet_center,
        third_surface,
    );

    add_plateau(
        &mut objects,
        third_surface,
        third_normal,
        0.22,
        0.18,
        ground,
        grass,
    );

    let block_base =
        big_surface
            + big_normal
                * (
                    big_height
                        + 0.03
                );

    add_question_block(
        &mut objects,
        block_base,
        big_normal,
        0.22,
        0.18,
        block_yellow,
        block_white,
    );

    add_palm(
        &mut objects,
        planet_center,
        -0.82,
        0.40,
        0.25,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        planet_center,
        -0.50,
        0.82,
        0.22,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        planet_center,
        0.37,
        0.72,
        0.22,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        planet_center,
        -0.80,
        -0.35,
        0.20,
        palm_wood,
        palm_leaf,
    );

    add_palm_at_point(
        &mut objects,
        second_surface
            + second_normal
                * 0.23,
        second_normal,
        0.22,
        palm_wood,
        palm_leaf,
    );

    add_rock(
        &mut objects,
        planet_center,
        -0.12,
        1.00,
        0.085,
        rock,
    );

    add_rock(
        &mut objects,
        planet_center,
        0.52,
        0.95,
        0.07,
        rock,
    );

    add_rock(
        &mut objects,
        planet_center,
        -1.03,
        -0.10,
        0.09,
        rock,
    );

    add_rock(
        &mut objects,
        planet_center,
        0.92,
        0.18,
        0.065,
        rock,
    );

    let waterfall_start =
        big_surface
            + big_normal
                * (
                    big_height
                        + 0.025
                );

    add_waterfall(
        &mut objects,
        waterfall_start,
        big_normal,
        waterfall,
    );

    objects
}

fn sphere_surface_point(
    center: Vec3,
    x: f32,
    z: f32,
) -> Vec3 {
    let dx =
        x
            - center.x;

    let dz =
        z
            - center.z;

    let y_squared =
        PLANET_RADIUS
            * PLANET_RADIUS
            - dx
                * dx
            - dz
                * dz;

    let y =
        center.y
            + y_squared
                .max(0.0)
                .sqrt();

    Vec3::new(
        x,
        y,
        z,
    )
}

fn sphere_normal(
    center: Vec3,
    point: Vec3,
) -> Vec3 {
    (
        point
            - center
    )
        .normalize()
}

fn add_island(
    objects: &mut Vec<Object>,
    planet_center: Vec3,
    x: f32,
    z: f32,
    radius: f32,
    ground: Material,
    sand: Material,
    grass: Material,
) {
    let surface =
        sphere_surface_point(
            planet_center,
            x,
            z,
        );

    let normal =
        sphere_normal(
            planet_center,
            surface,
        );

    let body_height =
        radius
            * 0.28;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (
                            body_height
                                * 0.42
                        ),
                normal,
                radius,
                body_height,
                ground,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (
                            body_height
                                + 0.006
                        ),
                normal,
                radius
                    * 0.92,
                0.022,
                sand,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (
                            body_height
                                + 0.025
                        ),
                normal,
                radius
                    * 0.58,
                0.018,
                grass,
            ),
        ),
    );
}

fn add_plateau(
    objects: &mut Vec<Object>,
    surface: Vec3,
    normal: Vec3,
    radius: f32,
    height: f32,
    ground: Material,
    grass: Material,
) {
    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (
                            height
                                * 0.46
                        ),
                normal,
                radius,
                height,
                ground,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (
                            height
                                + 0.016
                        ),
                normal,
                radius
                    * 1.01,
                0.032,
                grass,
            ),
        ),
    );
}

fn add_palm(
    objects: &mut Vec<Object>,
    planet_center: Vec3,
    x: f32,
    z: f32,
    size: f32,
    wood: Material,
    leaves: Material,
) {
    let surface =
        sphere_surface_point(
            planet_center,
            x,
            z,
        );

    let normal =
        sphere_normal(
            planet_center,
            surface,
        );

    add_palm_at_point(
        objects,
        surface
            + normal
                * 0.06,
        normal,
        size,
        wood,
        leaves,
    );
}

fn add_palm_at_point(
    objects: &mut Vec<Object>,
    base: Vec3,
    normal: Vec3,
    size: f32,
    wood: Material,
    leaves: Material,
) {
    let trunk_height =
        size
            * 0.80;

    let trunk_center =
        base
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
                size
                    * 0.045,
                trunk_height,
                wood,
            ),
        ),
    );

    let crown =
        base
            + normal
                * trunk_height;

    objects.push(
        Object::Sphere(
            Sphere::new(
                crown,
                size
                    * 0.065,
                leaves,
            ),
        ),
    );

    let (
        tangent,
        bitangent,
    ) =
        tangent_basis(
            normal,
        );

    let leaf_distance =
        size
            * 0.40;

    let leaf_radius =
        size
            * 0.075;

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
        (
            -tangent
                + bitangent
        )
            .normalize(),
        (
            -tangent
                - bitangent
        )
            .normalize(),
    ];

    for direction in
        directions
    {
        objects.push(
            Object::Sphere(
                Sphere::new(
                    crown
                        + direction
                            * leaf_distance
                        + normal
                            * (
                                size
                                    * 0.02
                            ),
                    leaf_radius,
                    leaves,
                ),
            ),
        );
    }
}

fn add_rock(
    objects: &mut Vec<Object>,
    planet_center: Vec3,
    x: f32,
    z: f32,
    radius: f32,
    material: Material,
) {
    let surface =
        sphere_surface_point(
            planet_center,
            x,
            z,
        );

    let normal =
        sphere_normal(
            planet_center,
            surface,
        );

    objects.push(
        Object::Sphere(
            Sphere::new(
                surface
                    + normal
                        * (
                            radius
                                * 0.55
                        ),
                radius,
                material,
            ),
        ),
    );
}

fn add_waterfall(
    objects: &mut Vec<Object>,
    start: Vec3,
    normal: Vec3,
    material: Material,
) {
    let (
        tangent,
        bitangent,
    ) =
        tangent_basis(
            normal,
        );

    for offset in [
        -0.055,
        -0.0275,
        0.0,
        0.0275,
        0.055,
    ] {
        let stream_start =
            start
                + bitangent
                    * offset;

        let stream_end =
            stream_start
                - normal
                    * 0.30
                + tangent
                    * 0.03;

        add_cylinder_between(
            objects,
            stream_start,
            stream_end,
            0.012,
            material,
        );
    }
}

fn add_question_block(
    objects: &mut Vec<Object>,
    platform_top: Vec3,
    normal: Vec3,
    size: f32,
    float_height: f32,
    yellow: Material,
    white: Material,
) {
    let center =
        platform_top
            + normal
                * (
                    float_height
                        + size
                            * 0.5
                );

    objects.push(
        Object::Cube(
            Cube::new(
                center,
                size,
                yellow,
            ),
        ),
    );

    let pixel =
        size
            * 0.105;

    let half =
        size
            * 0.5;

    let question_pixels = [
        (-1, 2),
        (0, 2),
        (1, 2),
        (2, 1),
        (1, 0),
        (0, -1),
        (0, -3),
    ];

    let (
        tangent,
        bitangent,
    ) =
        tangent_basis(
            normal,
        );

    let front_direction =
        Vec3::new(
            0.0,
            0.0,
            1.0,
        );

    for (
        px,
        py,
    ) in
        question_pixels
    {
        let position =
            center
                + tangent
                    * (
                        px as f32
                            * pixel
                    )
                + normal
                    * (
                        py as f32
                            * pixel
                    )
                + front_direction
                    * (
                        half
                            + pixel
                                * 0.55
                    );

        objects.push(
            Object::Cube(
                Cube::new(
                    position,
                    pixel,
                    white,
                ),
            ),
        );
    }

    let corner_radius =
        size
            * 0.045;

    let corner_offset =
        size
            * 0.36;

    for sx in [
        -1.0,
        1.0,
    ] {
        for sy in [
            -1.0,
            1.0,
        ] {
            let corner =
                center
                    + tangent
                        * (
                            sx
                                * corner_offset
                        )
                    + normal
                        * (
                            sy
                                * corner_offset
                        )
                    + front_direction
                        * (
                            half
                                + corner_radius
                        );

            objects.push(
                Object::Sphere(
                    Sphere::new(
                        corner,
                        corner_radius,
                        yellow,
                    ),
                ),
            );
        }
    }

    let _ =
        bitangent;
}

fn tangent_basis(
    normal: Vec3,
) -> (
    Vec3,
    Vec3,
) {
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
        helper
            .cross(
                &normal,
            )
            .normalize();

    let bitangent =
        normal
            .cross(
                &tangent,
            )
            .normalize();

    (
        tangent,
        bitangent,
    )
}

fn add_cylinder_between(
    objects: &mut Vec<Object>,
    start: Vec3,
    end: Vec3,
    radius: f32,
    material: Material,
) {
    let delta =
        end
            - start;

    let length =
        delta.length();

    if length
        <= 0.0001
    {
        return;
    }

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                (
                    start
                        + end
                )
                    * 0.5,
                delta
                    / length,
                radius,
                length,
                material,
            ),
        ),
    );
}