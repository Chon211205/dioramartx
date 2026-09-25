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

    let planet_center =
        Vec3::new(
            0.0,
            -0.45,
            0.0,
        );

    let water =
        Material::new(
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

    let waterfall =
        Material::new(
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

    let ground =
        Material::textured(
            Vec3::new(
                0.80,
                0.74,
                0.68,
            ),
            0.80,
            0.12,
            0.0,
            0.01,
            Some(
                ground_color_map(),
            ),
            Some(
                ground_normal_map(),
            ),
            Some(
                ground_roughness_map(),
            ),
            Some(
                ground_ao_map(),
            ),
        );

    let grass =
        Material::textured(
            Vec3::new(
                0.80,
                1.0,
                0.80,
            ),
            0.85,
            0.12,
            0.0,
            0.01,
            Some(
                grass_color_map(),
            ),
            Some(
                grass_normal_map(),
            ),
            Some(
                grass_roughness_map(),
            ),
            Some(
                grass_ao_map(),
            ),
        );

    let sand =
        Material::textured(
            Vec3::new(
                1.0,
                0.97,
                0.90,
            ),
            0.82,
            0.08,
            0.0,
            0.0,
            Some(
                sand_color_map(),
            ),
            Some(
                sand_normal_map(),
            ),
            Some(
                sand_roughness_map(),
            ),
            None,
        );

    let rock =
        Material::textured(
            Vec3::new(
                0.80,
                0.78,
                0.75,
            ),
            0.78,
            0.15,
            0.0,
            0.03,
            Some(
                rock_color_map(),
            ),
            Some(
                rock_normal_map(),
            ),
            Some(
                rock_roughness_map(),
            ),
            None,
        );

    let palm_wood =
        Material::new(
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

    let palm_leaf =
        Material::textured(
            Vec3::new(
                0.85,
                1.0,
                0.85,
            ),
            0.85,
            0.12,
            0.0,
            0.01,
            Some(
                palm_color_map(),
            ),
            Some(
                palm_normal_map(),
            ),
            Some(
                palm_roughness_map(),
            ),
            None,
        );

    let block_yellow =
        Material::new(
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

    let block_white =
        Material::new(
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

    add_spherical_island(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.85,
        0.45,
        0.27,
        ground,
        sand,
        grass,
    );

    add_spherical_island(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.45,
        0.85,
        0.20,
        ground,
        sand,
        grass,
    );

    add_spherical_island(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.10,
        0.35,
        0.16,
        ground,
        sand,
        grass,
    );

    add_spherical_island(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        0.35,
        0.80,
        0.22,
        ground,
        sand,
        grass,
    );

    add_spherical_island(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.75,
        -0.40,
        0.21,
        ground,
        sand,
        grass,
    );

    add_spherical_island(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.15,
        -0.70,
        0.18,
        ground,
        sand,
        grass,
    );

    let big_platform_x =
        0.60;

    let big_platform_z =
        -0.20;

    let big_surface_y =
        sphere_surface_y(
            planet_center,
            PLANET_RADIUS,
            big_platform_x,
            big_platform_z,
        );

    let big_platform_radius =
        0.42;

    let big_platform_height =
        0.42;

    add_spherical_plateau(
        &mut objects,
        big_platform_x,
        big_surface_y,
        big_platform_z,
        big_platform_radius,
        big_platform_height,
        ground,
        grass,
    );

    let second_platform_x =
        0.95;

    let second_platform_z =
        -0.45;

    let second_surface_y =
        sphere_surface_y(
            planet_center,
            PLANET_RADIUS,
            second_platform_x,
            second_platform_z,
        );

    add_spherical_plateau(
        &mut objects,
        second_platform_x,
        second_surface_y,
        second_platform_z,
        0.28,
        0.28,
        ground,
        grass,
    );

    let third_platform_x =
        0.55;

    let third_platform_z =
        -0.72;

    let third_surface_y =
        sphere_surface_y(
            planet_center,
            PLANET_RADIUS,
            third_platform_x,
            third_platform_z,
        );

    add_spherical_plateau(
        &mut objects,
        third_platform_x,
        third_surface_y,
        third_platform_z,
        0.23,
        0.20,
        ground,
        grass,
    );

    let big_platform_top =
        big_surface_y
            + big_platform_height
            + 0.032;

    add_floating_question_block(
        &mut objects,
        big_platform_x,
        big_platform_z,
        big_platform_top,
        0.22,
        0.19,
        block_yellow,
        block_white,
    );

    add_spherical_palm(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.85,
        0.45,
        0.26,
        palm_wood,
        palm_leaf,
    );

    add_spherical_palm(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.45,
        0.85,
        0.23,
        palm_wood,
        palm_leaf,
    );

    add_spherical_palm(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        0.35,
        0.80,
        0.23,
        palm_wood,
        palm_leaf,
    );

    add_spherical_palm(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.75,
        -0.40,
        0.21,
        palm_wood,
        palm_leaf,
    );

    add_plateau_palm(
        &mut objects,
        second_platform_x,
        second_surface_y
            + 0.28
            + 0.032,
        second_platform_z,
        0.25,
        palm_wood,
        palm_leaf,
    );

    add_spherical_rock(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -0.10,
        1.05,
        0.085,
        rock,
    );

    add_spherical_rock(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        0.55,
        0.95,
        0.07,
        rock,
    );

    add_spherical_rock(
        &mut objects,
        planet_center,
        PLANET_RADIUS,
        -1.05,
        -0.10,
        0.09,
        rock,
    );

    add_waterfall(
        &mut objects,
        Vec3::new(
            big_platform_x
                - big_platform_radius
                * 0.90,
            big_platform_top,
            big_platform_z,
        ),
        planet_center,
        PLANET_RADIUS,
        waterfall,
    );

    objects
}

fn sphere_surface_y(
    center: Vec3,
    radius: f32,
    x: f32,
    z: f32,
) -> f32 {
    let dx =
        x
            - center.x;

    let dz =
        z
            - center.z;

    let inside =
        (
            radius
                * radius
                - dx
                    * dx
                - dz
                    * dz
        )
            .max(
                0.0,
            );

    center.y
        + inside.sqrt()
}

fn add_spherical_island(
    objects: &mut Vec<Object>,
    center: Vec3,
    planet_radius: f32,
    x: f32,
    z: f32,
    radius: f32,
    ground: Material,
    sand: Material,
    grass: Material,
) {
    let surface_y =
        sphere_surface_y(
            center,
            planet_radius,
            x,
            z,
        );

    let body_height =
        radius
            * 0.34;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    x,
                    surface_y
                        + body_height
                            * 0.5,
                    z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                radius,
                body_height,
                ground,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    x,
                    surface_y
                        + body_height
                        + 0.012,
                    z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
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
                Vec3::new(
                    x,
                    surface_y
                        + body_height
                        + 0.031,
                    z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                radius
                    * 0.58,
                0.018,
                grass,
            ),
        ),
    );
}

fn add_spherical_plateau(
    objects: &mut Vec<Object>,
    x: f32,
    surface_y: f32,
    z: f32,
    radius: f32,
    height: f32,
    ground: Material,
    grass: Material,
) {
    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    x,
                    surface_y
                        + height
                            * 0.5,
                    z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                radius,
                height,
                ground,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    x,
                    surface_y
                        + height
                        + 0.016,
                    z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                radius
                    * 1.01,
                0.032,
                grass,
            ),
        ),
    );
}

fn add_spherical_palm(
    objects: &mut Vec<Object>,
    center: Vec3,
    planet_radius: f32,
    x: f32,
    z: f32,
    size: f32,
    wood: Material,
    leaves: Material,
) {
    let base_y =
        sphere_surface_y(
            center,
            planet_radius,
            x,
            z,
        )
            + 0.08;

    add_plateau_palm(
        objects,
        x,
        base_y,
        z,
        size,
        wood,
        leaves,
    );
}

fn add_plateau_palm(
    objects: &mut Vec<Object>,
    x: f32,
    base_y: f32,
    z: f32,
    size: f32,
    wood: Material,
    leaves: Material,
) {
    let trunk_height =
        size
            * 0.75;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    x,
                    base_y
                        + trunk_height
                            * 0.5,
                    z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                size
                    * 0.045,
                trunk_height,
                wood,
            ),
        ),
    );

    let crown =
        Vec3::new(
            x,
            base_y
                + trunk_height,
            z,
        );

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

    let leaf_positions = [
        Vec3::new(
            0.10,
            0.0,
            0.0,
        ),

        Vec3::new(
            -0.10,
            0.0,
            0.0,
        ),

        Vec3::new(
            0.0,
            0.0,
            0.10,
        ),

        Vec3::new(
            0.0,
            0.0,
            -0.10,
        ),

        Vec3::new(
            0.07,
            0.0,
            0.07,
        ),

        Vec3::new(
            -0.07,
            0.0,
            -0.07,
        ),
    ];

    for offset in
        leaf_positions
    {
        objects.push(
            Object::Sphere(
                Sphere::new(
                    crown
                        + offset
                            * (
                                size
                                    * 6.0
                            ),
                    size
                        * 0.075,
                    leaves,
                ),
            ),
        );
    }
}

fn add_spherical_rock(
    objects: &mut Vec<Object>,
    center: Vec3,
    planet_radius: f32,
    x: f32,
    z: f32,
    radius: f32,
    material: Material,
) {
    let surface_y =
        sphere_surface_y(
            center,
            planet_radius,
            x,
            z,
        );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    x,
                    surface_y
                        + radius
                            * 0.55,
                    z,
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
    planet_center: Vec3,
    planet_radius: f32,
    material: Material,
) {
    let bottom_y =
        sphere_surface_y(
            planet_center,
            planet_radius,
            start.x,
            start.z,
        )
            + 0.03;

    for offset in [
        -0.055,
        -0.0275,
        0.0,
        0.0275,
        0.055,
    ] {
        let waterfall_start =
            Vec3::new(
                start.x,
                start.y,
                start.z
                    + offset,
            );

        let waterfall_end =
            Vec3::new(
                start.x,
                bottom_y,
                start.z
                    + offset,
            );

        add_cylinder_between(
            objects,
            waterfall_start,
            waterfall_end,
            0.012,
            material,
        );
    }
}

fn add_floating_question_block(
    objects: &mut Vec<Object>,
    x: f32,
    z: f32,
    platform_top_y: f32,
    size: f32,
    float_height: f32,
    yellow: Material,
    white: Material,
) {
    let center =
        Vec3::new(
            x,
            platform_top_y
                + float_height,
            z,
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

    let half =
        size
            * 0.5;

    let pixel =
        size
            * 0.115;

    let question_pixels = [
        (-1, 2),
        (0, 2),
        (1, 2),
        (2, 1),
        (1, 0),
        (0, -1),
        (0, -3),
    ];

    for (
        px,
        py,
    ) in question_pixels
    {
        objects.push(
            Object::Cube(
                Cube::new(
                    Vec3::new(
                        center.x
                            + px as f32
                                * pixel,

                        center.y
                            + py as f32
                                * pixel,

                        center.z
                            + half
                            + pixel
                                * 0.54,
                    ),
                    pixel,
                    white,
                ),
            ),
        );
    }

    for (
        px,
        py,
    ) in question_pixels
    {
        objects.push(
            Object::Cube(
                Cube::new(
                    Vec3::new(
                        center.x
                            + half
                            + pixel
                                * 0.54,

                        center.y
                            + py as f32
                                * pixel,

                        center.z
                            - px as f32
                                * pixel,
                    ),
                    pixel,
                    white,
                ),
            ),
        );
    }

    let corner_offset =
        size
            * 0.36;

    let corner_radius =
        size
            * 0.045;

    let front_z =
        center.z
            + half
            + corner_radius
                * 0.9;

    let front_corners = [
        Vec3::new(
            center.x
                - corner_offset,
            center.y
                + corner_offset,
            front_z,
        ),

        Vec3::new(
            center.x
                + corner_offset,
            center.y
                + corner_offset,
            front_z,
        ),

        Vec3::new(
            center.x
                - corner_offset,
            center.y
                - corner_offset,
            front_z,
        ),

        Vec3::new(
            center.x
                + corner_offset,
            center.y
                - corner_offset,
            front_z,
        ),
    ];

    for corner in
        front_corners
    {
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

    let right_x =
        center.x
            + half
            + corner_radius
                * 0.9;

    let right_corners = [
        Vec3::new(
            right_x,
            center.y
                + corner_offset,
            center.z
                - corner_offset,
        ),

        Vec3::new(
            right_x,
            center.y
                + corner_offset,
            center.z
                + corner_offset,
        ),

        Vec3::new(
            right_x,
            center.y
                - corner_offset,
            center.z
                - corner_offset,
        ),

        Vec3::new(
            right_x,
            center.y
                - corner_offset,
            center.z
                + corner_offset,
        ),
    ];

    for corner in
        right_corners
    {
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