use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;

use crate::objects::ellipsoid::Ellipsoid;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

use crate::textures::texture::TextureMap;

static GRASS_COLOR: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_NORMAL: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_ROUGHNESS: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_AO: OnceLock<TextureMap> =
    OnceLock::new();

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
        Material::new(
            Vec3::new(
                0.90,
                0.88,
                0.80,
            ),
            0.88,
            0.30,
            0.0,
            0.06,
        );

    let grass_material =
        Material::textured(
            Vec3::new(
                0.75,
                1.0,
                0.75,
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

    add_grass_patches(
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

fn add_grass_patches(
    objects: &mut Vec<Object>,
    material: Material,
) {
    add_patch(
        objects,
        Vec3::new(
            0.0,
            0.82,
            0.91,
        ),
        0.43,
        material,
    );

    add_patch(
        objects,
        Vec3::new(
            0.82,
            0.26,
            0.62,
        ),
        0.34,
        material,
    );

    add_patch(
        objects,
        Vec3::new(
            -0.86,
            0.10,
            0.58,
        ),
        0.36,
        material,
    );

    add_patch(
        objects,
        Vec3::new(
            0.48,
            -0.82,
            0.77,
        ),
        0.31,
        material,
    );

    add_patch(
        objects,
        Vec3::new(
            -0.55,
            -0.72,
            0.82,
        ),
        0.27,
        material,
    );

    add_patch(
        objects,
        Vec3::new(
            0.68,
            0.82,
            -0.50,
        ),
        0.30,
        material,
    );

    add_patch(
        objects,
        Vec3::new(
            -0.70,
            0.62,
            -0.56,
        ),
        0.28,
        material,
    );
}

fn add_patch(
    objects: &mut Vec<Object>,
    position: Vec3,
    radius: f32,
    material: Material,
) {
    objects.push(
        Object::Sphere(
            Sphere::new(
                position,
                radius,
                material,
            ),
        ),
    );
}