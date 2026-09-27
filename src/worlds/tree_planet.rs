use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::textures::texture::TextureMap;

static GROUND_COLOR: OnceLock<TextureMap> = OnceLock::new();
static GROUND_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static GROUND_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();
static GROUND_AO: OnceLock<TextureMap> = OnceLock::new();

static BARK_COLOR: OnceLock<TextureMap> = OnceLock::new();
static BARK_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static BARK_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();
static BARK_AO: OnceLock<TextureMap> = OnceLock::new();

static GRASS_COLOR: OnceLock<TextureMap> = OnceLock::new();
static GRASS_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static GRASS_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();
static GRASS_AO: OnceLock<TextureMap> = OnceLock::new();

fn ground_color() -> &'static TextureMap {
    GROUND_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ground/Ground048_1K-PNG_Color.png",
        )
    })
}

fn ground_normal() -> &'static TextureMap {
    GROUND_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ground/Ground048_1K-PNG_NormalGL.png",
        )
    })
}

fn ground_roughness() -> &'static TextureMap {
    GROUND_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ground/Ground048_1K-PNG_Roughness.png",
        )
    })
}

fn ground_ao() -> &'static TextureMap {
    GROUND_AO.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ground/Ground048_1K-PNG_AmbientOcclusion.png",
        )
    })
}

fn bark_color() -> &'static TextureMap {
    BARK_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bark/Bark014_1K-PNG_Color.png",
        )
    })
}

fn bark_normal() -> &'static TextureMap {
    BARK_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bark/Bark014_1K-PNG_NormalGL.png",
        )
    })
}

fn bark_roughness() -> &'static TextureMap {
    BARK_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bark/Bark014_1K-PNG_Roughness.png",
        )
    })
}

fn bark_ao() -> &'static TextureMap {
    BARK_AO.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/bark/Bark014_1K-PNG_AmbientOcclusion.png",
        )
    })
}

fn grass_color() -> &'static TextureMap {
    GRASS_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_1K-PNG_Color.png",
        )
    })
}

fn grass_normal() -> &'static TextureMap {
    GRASS_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_1K-PNG_NormalGL.png",
        )
    })
}

fn grass_roughness() -> &'static TextureMap {
    GRASS_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_1K-PNG_Roughness.png",
        )
    })
}

fn grass_ao() -> &'static TextureMap {
    GRASS_AO.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_1K-PNG_AmbientOcclusion.png",
        )
    })
}

pub fn create_tree_planet_world() -> Vec<Object> {
    let mut objects = Vec::new();

    let ground_material = Material::textured(
        Vec3::new(
            1.0,
            1.0,
            1.0,
        ),
        0.92,
        0.24,
        0.0,
        0.03,
        Some(ground_color()),
        Some(ground_normal()),
        Some(ground_roughness()),
        Some(ground_ao()),
    );

    let bark_material = Material::textured(
        Vec3::new(
            1.0,
            1.0,
            1.0,
        ),
        0.90,
        0.30,
        0.0,
        0.03,
        Some(bark_color()),
        Some(bark_normal()),
        Some(bark_roughness()),
        Some(bark_ao()),
    );

    let grass_material = Material::textured(
        Vec3::new(
            0.85,
            1.0,
            0.85,
        ),
        0.95,
        0.25,
        0.0,
        0.02,
        Some(grass_color()),
        Some(grass_normal()),
        Some(grass_roughness()),
        Some(grass_ao()),
    );

    let planet_radius = 1.90;

    add_planet(
        &mut objects,
        planet_radius,
        ground_material,
    );

    add_rock_details(
        &mut objects,
        planet_radius,
        ground_material,
    );

    add_grass_base(
        &mut objects,
        planet_radius,
        grass_material,
    );

    add_tree(
        &mut objects,
        planet_radius,
        bark_material,
        grass_material,
    );

    objects
}

fn add_planet(
    objects: &mut Vec<Object>,
    planet_radius: f32,
    material: Material,
) {
    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                planet_radius,
                material,
            ),
        ),
    );
}

fn add_rock_details(
    objects: &mut Vec<Object>,
    planet_radius: f32,
    material: Material,
) {
    let details = [
        (
            Vec3::new(0.95, -0.20, 0.95),
            0.22,
        ),
        (
            Vec3::new(-1.05, 0.10, 0.82),
            0.18,
        ),
        (
            Vec3::new(0.75, -0.90, 0.65),
            0.16,
        ),
        (
            Vec3::new(-0.70, -0.82, 0.90),
            0.20,
        ),
        (
            Vec3::new(1.05, 0.45, -0.55),
            0.17,
        ),
        (
            Vec3::new(-1.10, 0.35, -0.42),
            0.19,
        ),
        (
            Vec3::new(0.25, -1.32, 0.60),
            0.14,
        ),
        (
            Vec3::new(-0.25, -1.25, -0.72),
            0.15,
        ),
        (
            Vec3::new(0.65, 0.65, 1.25),
            0.13,
        ),
        (
            Vec3::new(-0.55, 0.72, 1.18),
            0.12,
        ),
        (
            Vec3::new(1.18, -0.35, -0.45),
            0.11,
        ),
        (
            Vec3::new(-1.15, -0.45, -0.40),
            0.13,
        ),
    ];

    for (direction, radius) in details {
        let dir = direction.normalize();

        let center =
            dir
                * (
                    planet_radius
                        + radius
                            * 0.30
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
}

fn add_grass_base(
    objects: &mut Vec<Object>,
    planet_radius: f32,
    material: Material,
) {
    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    0.0,
                    planet_radius + 0.01,
                    0.0,
                ),
                0.72,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    0.38,
                    planet_radius,
                    0.15,
                ),
                0.43,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    -0.40,
                    planet_radius,
                    0.10,
                ),
                0.44,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    0.08,
                    planet_radius,
                    -0.40,
                ),
                0.39,
                material,
            ),
        ),
    );
}

fn add_tree(
    objects: &mut Vec<Object>,
    planet_radius: f32,
    bark_material: Material,
    leaves_material: Material,
) {
    let trunk_height = 1.60;
    let trunk_radius = 0.32;

    let trunk_center =
        Vec3::new(
            0.0,
            planet_radius
                + trunk_height
                    * 0.5,
            0.0,
        );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                trunk_center,
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                trunk_radius,
                trunk_height,
                bark_material,
            ),
        ),
    );

    add_roots(
        objects,
        planet_radius,
        bark_material,
    );

    let canopy_y =
        planet_radius
            + trunk_height
            + 0.34;

    let canopy = [
        (
            Vec3::new(
                0.0,
                canopy_y,
                0.0,
            ),
            0.78,
        ),
        (
            Vec3::new(
                -0.58,
                canopy_y - 0.06,
                0.03,
            ),
            0.58,
        ),
        (
            Vec3::new(
                0.58,
                canopy_y - 0.05,
                0.05,
            ),
            0.59,
        ),
        (
            Vec3::new(
                0.0,
                canopy_y + 0.45,
                0.0,
            ),
            0.53,
        ),
        (
            Vec3::new(
                0.05,
                canopy_y - 0.04,
                0.53,
            ),
            0.52,
        ),
        (
            Vec3::new(
                -0.08,
                canopy_y,
                -0.52,
            ),
            0.51,
        ),
        (
            Vec3::new(
                -0.42,
                canopy_y + 0.27,
                -0.20,
            ),
            0.47,
        ),
        (
            Vec3::new(
                0.42,
                canopy_y + 0.25,
                -0.18,
            ),
            0.47,
        ),
        (
            Vec3::new(
                -0.35,
                canopy_y + 0.20,
                0.35,
            ),
            0.44,
        ),
        (
            Vec3::new(
                0.35,
                canopy_y + 0.18,
                0.37,
            ),
            0.44,
        ),
    ];

    for (center, radius) in canopy {
        objects.push(
            Object::Sphere(
                Sphere::new(
                    center,
                    radius,
                    leaves_material,
                ),
            ),
        );
    }
}

fn add_roots(
    objects: &mut Vec<Object>,
    planet_radius: f32,
    material: Material,
) {
    let roots = [
        Vec3::new(
            0.34,
            planet_radius + 0.10,
            0.0,
        ),
        Vec3::new(
            -0.34,
            planet_radius + 0.10,
            0.0,
        ),
        Vec3::new(
            0.0,
            planet_radius + 0.10,
            0.34,
        ),
        Vec3::new(
            0.0,
            planet_radius + 0.10,
            -0.34,
        ),
        Vec3::new(
            0.25,
            planet_radius + 0.08,
            0.25,
        ),
        Vec3::new(
            -0.25,
            planet_radius + 0.08,
            -0.25,
        ),
    ];

    for root in roots {
        objects.push(
            Object::Sphere(
                Sphere::new(
                    root,
                    0.26,
                    material,
                ),
            ),
        );
    }
}