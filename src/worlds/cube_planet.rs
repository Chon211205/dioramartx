use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::object::Object;
use crate::textures::texture::TextureMap;

pub fn create_cube_planet() -> Vec<Object> {
    let face1_color =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase1Cube/WoodFloor051_1K-PNG_Color.png",
                ),
            ),
        );

    let face1_normal =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase1Cube/WoodFloor051_1K-PNG_NormalGL.png",
                ),
            ),
        );

    let face1_roughness =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase1Cube/WoodFloor051_1K-PNG_Roughness.png",
                ),
            ),
        );

    let face2_color =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase2Cube/WoodFloor057_1K-PNG_Color.png",
                ),
            ),
        );

    let face2_normal =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase2Cube/WoodFloor057_1K-PNG_NormalGL.png",
                ),
            ),
        );

    let face2_roughness =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase2Cube/WoodFloor057_1K-PNG_Roughness.png",
                ),
            ),
        );

    let face3_color =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase3Cube/Wood081_1K-PNG_Color.png",
                ),
            ),
        );

    let face3_normal =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase3Cube/Wood081_1K-PNG_NormalGL.png",
                ),
            ),
        );

    let face3_roughness =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Fase3Cube/Wood081_1K-PNG_Roughness.png",
                ),
            ),
        );

    let face4_color =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face4cube/Wood025_1K-PNG_Color.png",
                ),
            ),
        );

    let face4_normal =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face4cube/Wood025_1K-PNG_NormalGL.png",
                ),
            ),
        );

    let face4_roughness =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face4cube/Wood025_1K-PNG_Roughness.png",
                ),
            ),
        );

    let face5_color =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face5Cube/WoodFloor035_1K-PNG_Color.png",
                ),
            ),
        );

    let face5_normal =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face5Cube/WoodFloor035_1K-PNG_NormalGL.png",
                ),
            ),
        );

    let face5_roughness =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face5Cube/WoodFloor035_1K-PNG_Roughness.png",
                ),
            ),
        );

    let face6_color =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face6Cube/Wood005_1K-PNG_Color.png",
                ),
            ),
        );

    let face6_normal =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face6Cube/Wood005_1K-PNG_NormalGL.png",
                ),
            ),
        );

    let face6_roughness =
        Box::leak(
            Box::new(
                TextureMap::from_file(
                    "assets/textures/Face6Cube/Wood005_1K-PNG_Roughness.png",
                ),
            ),
        );

    let face1_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
            0.30,
            0.0,
            0.02,
            Some(face1_color),
            Some(face1_normal),
            Some(face1_roughness),
            None,
        );

    let face2_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
            0.30,
            0.0,
            0.02,
            Some(face2_color),
            Some(face2_normal),
            Some(face2_roughness),
            None,
        );

    let face3_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
            0.30,
            0.0,
            0.02,
            Some(face3_color),
            Some(face3_normal),
            Some(face3_roughness),
            None,
        );

    let face4_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
            0.30,
            0.0,
            0.02,
            Some(face4_color),
            Some(face4_normal),
            Some(face4_roughness),
            None,
        );

    let face5_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
            0.30,
            0.0,
            0.02,
            Some(face5_color),
            Some(face5_normal),
            Some(face5_roughness),
            None,
        );

    let face6_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
            0.30,
            0.0,
            0.02,
            Some(face6_color),
            Some(face6_normal),
            Some(face6_roughness),
            None,
        );

    vec![
        Object::Cube(
            Cube::new_with_faces(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                3.0,

                face2_material,
                face3_material,
                face4_material,
                face5_material,
                face1_material,
                face6_material,
            ),
        ),
    ]
}