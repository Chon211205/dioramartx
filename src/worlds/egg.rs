use std::f32::consts::PI;
use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;

use crate::objects::ellipsoid::Ellipsoid;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

use crate::textures::texture::TextureMap;

static SHELL_COLOR: OnceLock<TextureMap> =
    OnceLock::new();

static SHELL_NORMAL: OnceLock<TextureMap> =
    OnceLock::new();

static SHELL_ROUGHNESS: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_COLOR: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_NORMAL: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_ROUGHNESS: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_AO: OnceLock<TextureMap> =
    OnceLock::new();

fn shell_color() -> &'static TextureMap {
    SHELL_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/egg/PaintedPlaster017_1K-PNG_Color.png",
        )
    })
}

fn shell_normal() -> &'static TextureMap {
    SHELL_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/egg/PaintedPlaster017_1K-PNG_NormalGL.png",
        )
    })
}

fn shell_roughness() -> &'static TextureMap {
    SHELL_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/egg/PaintedPlaster017_1K-PNG_Roughness.png",
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

pub fn create_egg_diorama() -> Vec<Object> {
    let mut objects =
        Vec::new();

    let shell_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            0.92,
            0.28,
            0.0,
            0.04,
            Some(
                shell_color(),
            ),
            Some(
                shell_normal(),
            ),
            Some(
                shell_roughness(),
            ),
            None,
        );

    let grass_material =
        Material::textured(
            Vec3::new(
                0.55,
                1.0,
                0.55,
            ),
            0.95,
            0.30,
            0.0,
            0.04,
            Some(
                grass_color(),
            ),
            Some(
                grass_normal(),
            ),
            Some(
                grass_roughness(),
            ),
            Some(
                grass_ao(),
            ),
        );

    add_egg_body(
        &mut objects,
        shell_material,
    );

    add_yoshi_pattern(
        &mut objects,
        grass_material,
    );

    objects
}

fn add_egg_body(
    objects: &mut Vec<Object>,
    material: Material,
) {
    objects.push(
        Object::Ellipsoid(
            Ellipsoid::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                Vec3::new(
                    1.12,
                    1.55,
                    1.12,
                ),
                material,
            ),
        ),
    );
}

fn add_yoshi_pattern(
    objects: &mut Vec<Object>,
    material: Material,
) {
    let egg_radii =
        Vec3::new(
            1.12,
            1.55,
            1.12,
        );

    add_surface_spot(
        objects,
        egg_radii,
        58.0,
        78.0,
        0.46,
        material,
    );

    add_surface_spot(
        objects,
        egg_radii,
        22.0,
        18.0,
        0.43,
        material,
    );

    add_surface_spot(
        objects,
        egg_radii,
        10.0,
        118.0,
        0.38,
        material,
    );

    add_surface_spot(
        objects,
        egg_radii,
        -18.0,
        55.0,
        0.45,
        material,
    );

    add_surface_spot(
        objects,
        egg_radii,
        -35.0,
        155.0,
        0.37,
        material,
    );

    add_surface_spot(
        objects,
        egg_radii,
        36.0,
        205.0,
        0.40,
        material,
    );

    add_surface_spot(
        objects,
        egg_radii,
        -8.0,
        245.0,
        0.44,
        material,
    );

    add_surface_spot(
        objects,
        egg_radii,
        -46.0,
        300.0,
        0.34,
        material,
    );

    add_surface_spot(
        objects,
        egg_radii,
        16.0,
        330.0,
        0.39,
        material,
    );
}

fn add_surface_spot(
    objects: &mut Vec<Object>,
    egg_radii: Vec3,
    latitude_degrees: f32,
    longitude_degrees: f32,
    radius: f32,
    material: Material,
) {
    let latitude =
        latitude_degrees
            * PI
            / 180.0;

    let longitude =
        longitude_degrees
            * PI
            / 180.0;

    let cos_lat =
        latitude.cos();

    let sin_lat =
        latitude.sin();

    let cos_lon =
        longitude.cos();

    let sin_lon =
        longitude.sin();

    let surface =
        Vec3::new(
            egg_radii.x
                * cos_lat
                * cos_lon,

            egg_radii.y
                * sin_lat,

            egg_radii.z
                * cos_lat
                * sin_lon,
        );

    let outward =
        Vec3::new(
            surface.x
                / (
                    egg_radii.x
                        * egg_radii.x
                ),

            surface.y
                / (
                    egg_radii.y
                        * egg_radii.y
                ),

            surface.z
                / (
                    egg_radii.z
                        * egg_radii.z
                ),
        )
        .normalize();

    let inset =
        radius
            * 0.64;

    let center =
        surface
            - outward
                * inset;

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