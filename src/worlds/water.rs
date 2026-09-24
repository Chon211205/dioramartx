use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cone::Cone;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::textures::texture::TextureMap;

const PLANET_RADIUS: f32 = 1.8;

static GRASS_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static GRASS_AO_MAP: OnceLock<TextureMap> = OnceLock::new();

static GROUND_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();
static GROUND_AO_MAP: OnceLock<TextureMap> = OnceLock::new();
static SAND_COLOR_MAP: OnceLock<TextureMap> = OnceLock::new();
static SAND_NORMAL_MAP: OnceLock<TextureMap> = OnceLock::new();
static SAND_ROUGHNESS_MAP: OnceLock<TextureMap> = OnceLock::new();

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

    let ground = Material::textured(
        Vec3::new(
            0.70,
            0.58,
            0.42,
        ),
        0.78,
        0.12,
        0.0,
        0.02,
        Some(ground_color_map()),
        Some(ground_normal_map()),
        Some(ground_roughness_map()),
        Some(ground_ao_map()),
    );

    let grass = Material::textured(
        Vec3::new(
            0.55,
            0.90,
            0.55,
        ),
        0.85,
        0.12,
        0.0,
        0.02,
        Some(grass_color_map()),
        Some(grass_normal_map()),
        Some(grass_roughness_map()),
        Some(grass_ao_map()),
    );

    let sand = Material::textured(
        Vec3::new(
            0.95,
            0.90,
            0.78,
        ),
        0.82,
        0.10,
        0.0,
        0.0,
        Some(sand_color_map()),
        Some(sand_normal_map()),
        Some(sand_roughness_map()),
        None,
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
        Vec3::new(
            -0.80,
            0.55,
            0.45,
        ),
        0.40,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(
            -0.35,
            0.80,
            0.55,
        ),
        0.30,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(
            0.10,
            0.92,
            0.45,
        ),
        0.34,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(
            0.65,
            0.62,
            0.30,
        ),
        0.38,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(
            -0.55,
            0.40,
            -0.72,
        ),
        0.28,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(
            0.10,
            0.55,
            -0.88,
        ),
        0.25,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(
            -0.10,
            -0.75,
            0.70,
        ),
        0.24,
        ground,
        sand,
        grass,
    );

    add_island(
        &mut objects,
        Vec3::new(
            0.65,
            -0.55,
            0.45,
        ),
        0.27,
        ground,
        sand,
        grass,
    );

    add_plateau(
        &mut objects,
        Vec3::new(
            0.65,
            0.55,
            -0.25,
        ),
        0.50,
        0.72,
        ground,
        grass,
    );

    add_plateau(
        &mut objects,
        Vec3::new(
            0.72,
            0.45,
            -0.38,
        ),
        0.40,
        0.55,
        ground,
        grass,
    );

    add_plateau(
        &mut objects,
        Vec3::new(
            0.52,
            0.72,
            -0.18,
        ),
        0.30,
        0.42,
        ground,
        grass,
    );

    add_plateau(
        &mut objects,
        Vec3::new(
            0.40,
            0.85,
            -0.05,
        ),
        0.24,
        0.28,
        ground,
        grass,
    );

    add_palm(
        &mut objects,
        Vec3::new(
            -0.80,
            0.55,
            0.45,
        ),
        0.42,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(
            -0.35,
            0.80,
            0.55,
        ),
        0.34,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(
            0.10,
            0.92,
            0.45,
        ),
        0.36,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(
            0.65,
            0.62,
            0.30,
        ),
        0.38,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(
            0.52,
            0.72,
            -0.18,
        ),
        0.44,
        palm_wood,
        palm_leaf,
    );

    add_palm(
        &mut objects,
        Vec3::new(
            0.40,
            0.85,
            -0.05,
        ),
        0.38,
        palm_wood,
        palm_leaf,
    );

    add_rock_column(
        &mut objects,
        Vec3::new(
            -0.20,
            0.35,
            0.95,
        ),
        0.12,
        0.30,
        dark_rock,
        grass,
    );

    add_rock_column(
        &mut objects,
        Vec3::new(
            0.35,
            0.20,
            0.95,
        ),
        0.10,
        0.24,
        dark_rock,
        grass,
    );

    add_rock_column(
        &mut objects,
        Vec3::new(
            -0.85,
            0.10,
            -0.45,
        ),
        0.13,
        0.32,
        dark_rock,
        grass,
    );

    add_rock_column(
        &mut objects,
        Vec3::new(
            0.85,
            -0.10,
            0.35,
        ),
        0.11,
        0.27,
        dark_rock,
        grass,
    );

    add_waterfall(
        &mut objects,
        Vec3::new(
            0.62,
            0.60,
            -0.28,
        ),
        0.55,
        waterfall,
    );

    add_waterfall(
        &mut objects,
        Vec3::new(
            0.48,
            0.72,
            -0.16,
        ),
        0.38,
        waterfall,
    );

    add_small_water_stream(
        &mut objects,
        Vec3::new(
            0.58,
            0.66,
            -0.20,
        ),
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
    ground: Material,
    sand: Material,
    grass: Material,
) {
    let normal =
        direction.normalize();

    let ground_center =
        normal
            * (
                PLANET_RADIUS
                    - radius * 0.42
            );

    objects.push(
        Object::Sphere(
            Sphere::new(
                ground_center,
                radius,
                ground,
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
    ground: Material,
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
                ground,
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
        if normal.y.abs()
            < 0.9
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

    let bitangent =
        normal
            .cross(
                &tangent,
            )
            .normalize();

    let leaf_directions = [
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

    for leaf_direction in leaf_directions {
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
        if normal.y.abs()
            < 0.9
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
        if normal.y.abs()
            < 0.9
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

    let start =
        surface
            - tangent
                * 0.22;

    let end =
        surface
            + tangent
                * 0.22;

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

    if length
        <= 0.0001
    {
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