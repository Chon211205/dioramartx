use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::textures::texture::TextureMap;

static KIRBY_TEXTURE: OnceLock<TextureMap> = OnceLock::new();

fn kirby_texture() -> &'static TextureMap {
    KIRBY_TEXTURE.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/kirby.png",
        )
    })
}

pub fn create_kirby_planet_world() -> Vec<Object> {
    let kirby_material =
        Material::kirby_textured(
            Vec3::new(
                1.0,
                0.55,
                0.78,
            ),
            1.0,
            0.22,
            0.0,
            0.02,
            Some(kirby_texture()),
        );

    vec![
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                1.85,
                kirby_material,
            ),
        ),
    ]
}
