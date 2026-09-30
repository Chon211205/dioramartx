use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::object::Object;
use crate::textures::texture::TextureMap;

fn pyramid_color() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| TextureMap::from_file("assets/textures/pyramid/Bricks087_1K-PNG_Color.png"))
}

fn pyramid_normal() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/pyramid/Bricks087_1K-PNG_NormalGL.png")
    })
}

fn pyramid_roughness() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/pyramid/Bricks087_1K-PNG_Roughness.png")
    })
}

fn pyramid_ao() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/pyramid/Bricks087_1K-PNG_AmbientOcclusion.png")
    })
}

fn pyramid_material() -> Material {
    Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.25,
        0.0,
        0.02,
        Some(pyramid_color()),
        Some(pyramid_normal()),
        Some(pyramid_roughness()),
        Some(pyramid_ao()),
    )
}

pub fn create_pyramid_planet() -> Vec<Object> {
    let mut objects = Vec::new();

    let material = pyramid_material();

    let block_size = 0.52;

    let levels = 6;

    for level in 0..levels {
        let grid_size = levels - level;

        let y = level as f32 * block_size;

        let offset = (grid_size as f32 - 1.0) * block_size * 0.5;

        for x in 0..grid_size {
            for z in 0..grid_size {
                let px = x as f32 * block_size - offset;

                let pz = z as f32 * block_size - offset;

                objects.push(Object::Cube(Cube::new(
                    Vec3::new(px, y, pz),
                    block_size,
                    material,
                )));
            }
        }
    }

    objects
}
