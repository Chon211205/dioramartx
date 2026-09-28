use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::object::Object;
use crate::textures::texture::TextureMap;

struct CubeTextures {
    face1_color: &'static TextureMap,
    face1_normal: &'static TextureMap,
    face1_roughness: &'static TextureMap,

    face2_color: &'static TextureMap,
    face2_normal: &'static TextureMap,
    face2_roughness: &'static TextureMap,

    face3_color: &'static TextureMap,
    face3_normal: &'static TextureMap,
    face3_roughness: &'static TextureMap,

    face4_color: &'static TextureMap,
    face4_normal: &'static TextureMap,
    face4_roughness: &'static TextureMap,

    face5_color: &'static TextureMap,
    face5_normal: &'static TextureMap,
    face5_roughness: &'static TextureMap,

    face6_color: &'static TextureMap,
    face6_normal: &'static TextureMap,
    face6_roughness: &'static TextureMap,
}

static CUBE_TEXTURES: OnceLock<CubeTextures> = OnceLock::new();

fn load_texture(path: &str) -> &'static TextureMap {
    Box::leak(Box::new(TextureMap::from_file(path)))
}

fn cube_textures() -> &'static CubeTextures {
    CUBE_TEXTURES.get_or_init(|| CubeTextures {
        face1_color: load_texture("assets/textures/Fase1Cube/WoodFloor051_1K-PNG_Color.png"),

        face1_normal: load_texture("assets/textures/Fase1Cube/WoodFloor051_1K-PNG_NormalGL.png"),

        face1_roughness: load_texture(
            "assets/textures/Fase1Cube/WoodFloor051_1K-PNG_Roughness.png",
        ),

        face2_color: load_texture("assets/textures/Fase2Cube/WoodFloor057_1K-PNG_Color.png"),

        face2_normal: load_texture("assets/textures/Fase2Cube/WoodFloor057_1K-PNG_NormalGL.png"),

        face2_roughness: load_texture(
            "assets/textures/Fase2Cube/WoodFloor057_1K-PNG_Roughness.png",
        ),

        face3_color: load_texture("assets/textures/Fase3Cube/Wood081_1K-PNG_Color.png"),

        face3_normal: load_texture("assets/textures/Fase3Cube/Wood081_1K-PNG_NormalGL.png"),

        face3_roughness: load_texture("assets/textures/Fase3Cube/Wood081_1K-PNG_Roughness.png"),

        face4_color: load_texture("assets/textures/Face4cube/Wood025_1K-PNG_Color.png"),

        face4_normal: load_texture("assets/textures/Face4cube/Wood025_1K-PNG_NormalGL.png"),

        face4_roughness: load_texture("assets/textures/Face4cube/Wood025_1K-PNG_Roughness.png"),

        face5_color: load_texture("assets/textures/Face5Cube/WoodFloor035_1K-PNG_Color.png"),

        face5_normal: load_texture("assets/textures/Face5Cube/WoodFloor035_1K-PNG_NormalGL.png"),

        face5_roughness: load_texture(
            "assets/textures/Face5Cube/WoodFloor035_1K-PNG_Roughness.png",
        ),

        face6_color: load_texture("assets/textures/Face6Cube/Wood005_1K-PNG_Color.png"),

        face6_normal: load_texture("assets/textures/Face6Cube/Wood005_1K-PNG_NormalGL.png"),

        face6_roughness: load_texture("assets/textures/Face6Cube/Wood005_1K-PNG_Roughness.png"),
    })
}

pub fn create_cube_planet() -> Vec<Object> {
    let textures = cube_textures();

    let face1_material = Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.30,
        0.0,
        0.02,
        Some(textures.face1_color),
        Some(textures.face1_normal),
        Some(textures.face1_roughness),
        None,
    );

    let face2_material = Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.30,
        0.0,
        0.02,
        Some(textures.face2_color),
        Some(textures.face2_normal),
        Some(textures.face2_roughness),
        None,
    );

    let face3_material = Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.30,
        0.0,
        0.02,
        Some(textures.face3_color),
        Some(textures.face3_normal),
        Some(textures.face3_roughness),
        None,
    );

    let face4_material = Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.30,
        0.0,
        0.02,
        Some(textures.face4_color),
        Some(textures.face4_normal),
        Some(textures.face4_roughness),
        None,
    );

    let face5_material = Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.30,
        0.0,
        0.02,
        Some(textures.face5_color),
        Some(textures.face5_normal),
        Some(textures.face5_roughness),
        None,
    );

    let face6_material = Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.30,
        0.0,
        0.02,
        Some(textures.face6_color),
        Some(textures.face6_normal),
        Some(textures.face6_roughness),
        None,
    );

    vec![Object::Cube(Cube::new_with_faces(
        Vec3::new(0.0, 0.0, 0.0),
        3.0,
        face2_material,
        face3_material,
        face4_material,
        face5_material,
        face1_material,
        face6_material,
    ))]
}
