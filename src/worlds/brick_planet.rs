use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::textures::texture::TextureMap;

fn brick_color() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/mariobrick/Bricks071_1K-PNG_Color.png",
        )
    })
}

fn brick_normal() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/mariobrick/Bricks071_1K-PNG_NormalGL.png",
        )
    })
}

fn brick_roughness() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/mariobrick/Bricks071_1K-PNG_Roughness.png",
        )
    })
}

fn brick_ao() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/mariobrick/Bricks071_1K-PNG_AmbientOcclusion.png",
        )
    })
}

pub fn create_brick_planet() -> Vec<Object> {
    let mut objects = Vec::new();

    let cube_material = Material::textured(
        Vec3::new(
            1.0,
            1.0,
            1.0,
        ),
        1.0,
        0.28,
        0.0,
        0.02,
        Some(brick_color()),
        Some(brick_normal()),
        Some(brick_roughness()),
        Some(brick_ao()),
    );

    let pipe_material = Material::new(
        Vec3::new(
            0.04,
            0.72,
            0.08,
        ),
        1.0,
        0.75,
        0.0,
        0.04,
    );

    let pipe_dark = Material::new(
        Vec3::new(
            0.01,
            0.12,
            0.02,
        ),
        1.0,
        0.20,
        0.0,
        0.0,
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                2.4,
                cube_material,
            ),
        ),
    );

    add_top_pipe(
        &mut objects,
        pipe_material,
        pipe_dark,
    );

    add_bottom_pipe(
        &mut objects,
        pipe_material,
        pipe_dark,
    );

    objects
}

fn add_top_pipe(
    objects: &mut Vec<Object>,
    pipe_material: Material,
    pipe_dark: Material,
) {
    let pipe_x = 0.55;
    let pipe_z = 0.35;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    pipe_x,
                    1.55,
                    pipe_z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                0.42,
                0.70,
                pipe_material,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    pipe_x,
                    1.90,
                    pipe_z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                0.56,
                0.18,
                pipe_material,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    pipe_x,
                    1.99,
                    pipe_z,
                ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                0.30,
                0.05,
                pipe_dark,
            ),
        ),
    );
}

fn add_bottom_pipe(
    objects: &mut Vec<Object>,
    pipe_material: Material,
    pipe_dark: Material,
) {
    let pipe_x = 0.55;
    let pipe_z = 0.35;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    pipe_x,
                    -1.55,
                    pipe_z,
                ),
                Vec3::new(
                    0.0,
                    -1.0,
                    0.0,
                ),
                0.42,
                0.70,
                pipe_material,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    pipe_x,
                    -1.90,
                    pipe_z,
                ),
                Vec3::new(
                    0.0,
                    -1.0,
                    0.0,
                ),
                0.56,
                0.18,
                pipe_material,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                Vec3::new(
                    pipe_x,
                    -1.99,
                    pipe_z,
                ),
                Vec3::new(
                    0.0,
                    -1.0,
                    0.0,
                ),
                0.30,
                0.05,
                pipe_dark,
            ),
        ),
    );
}
