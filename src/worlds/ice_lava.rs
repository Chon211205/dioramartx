use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;

use crate::objects::cone::Cone;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::object::Object;
use crate::objects::torus::Torus;

use crate::textures::texture::TextureMap;

static ICE_COLOR: OnceLock<TextureMap> =
    OnceLock::new();

static ICE_NORMAL: OnceLock<TextureMap> =
    OnceLock::new();

static ICE_ROUGHNESS: OnceLock<TextureMap> =
    OnceLock::new();

static LAVA_FLAT_COLOR: OnceLock<TextureMap> =
    OnceLock::new();

static LAVA_FLAT_NORMAL: OnceLock<TextureMap> =
    OnceLock::new();

static LAVA_FLAT_ROUGHNESS: OnceLock<TextureMap> =
    OnceLock::new();

static LAVA_CURVED_COLOR: OnceLock<TextureMap> =
    OnceLock::new();

static LAVA_CURVED_NORMAL: OnceLock<TextureMap> =
    OnceLock::new();

static LAVA_CURVED_ROUGHNESS: OnceLock<TextureMap> =
    OnceLock::new();

fn ice_color() -> &'static TextureMap {
    ICE_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ice/Ice002_1K-PNG_Color.png",
        )
    })
}

fn ice_normal() -> &'static TextureMap {
    ICE_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ice/Ice002_1K-PNG_NormalGL.png",
        )
    })
}

fn ice_roughness() -> &'static TextureMap {
    ICE_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/ice/Ice002_1K-PNG_Roughness.png",
        )
    })
}

fn lava_flat_color() -> &'static TextureMap {
    LAVA_FLAT_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava004_1K-PNG_Color.png",
        )
    })
}

fn lava_flat_normal() -> &'static TextureMap {
    LAVA_FLAT_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava004_1K-PNG_NormalGL.png",
        )
    })
}

fn lava_flat_roughness() -> &'static TextureMap {
    LAVA_FLAT_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava004_1K-PNG_Roughness.png",
        )
    })
}

fn lava_curved_color() -> &'static TextureMap {
    LAVA_CURVED_COLOR.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_Color.png",
        )
    })
}

fn lava_curved_normal() -> &'static TextureMap {
    LAVA_CURVED_NORMAL.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_NormalGL.png",
        )
    })
}

fn lava_curved_roughness() -> &'static TextureMap {
    LAVA_CURVED_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/lava/Lava002_1K-PNG_Roughness.png",
        )
    })
}

pub fn create_ice_lava_diorama() -> Vec<Object> {
    let mut objects =
        Vec::new();

    let ice_material =
        Material::textured(
            Vec3::new(
                0.82,
                0.94,
                1.0,
            ),
            0.90,
            0.95,
            0.12,
            0.42,
            Some(
                ice_color(),
            ),
            Some(
                ice_normal(),
            ),
            Some(
                ice_roughness(),
            ),
            None,
        );

    let crystal_material =
        Material::textured(
            Vec3::new(
                0.72,
                0.95,
                1.0,
            ),
            0.92,
            1.0,
            0.10,
            0.35,
            Some(
                ice_color(),
            ),
            Some(
                ice_normal(),
            ),
            Some(
                ice_roughness(),
            ),
            None,
        );

    let lava_outer_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            0.95,
            0.40,
            0.0,
            0.04,
            Some(
                lava_curved_color(),
            ),
            Some(
                lava_curved_normal(),
            ),
            Some(
                lava_curved_roughness(),
            ),
            None,
        );

    let lava_inner_material =
        Material::textured(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            0.98,
            0.78,
            0.0,
            0.05,
            Some(
                lava_flat_color(),
            ),
            Some(
                lava_flat_normal(),
            ),
            Some(
                lava_flat_roughness(),
            ),
            None,
        );

    add_ice_ring(
        &mut objects,
        ice_material,
    );

    add_crystals(
        &mut objects,
        crystal_material,
    );

    add_lava_planet(
        &mut objects,
        lava_outer_material,
        lava_inner_material,
    );

    objects
}

fn add_ice_ring(
    objects: &mut Vec<Object>,
    material: Material,
) {
    objects.push(
        Object::Torus(
            Torus::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                1.35,
                0.28,
                material,
            ),
        ),
    );
}

fn add_crystals(
    objects: &mut Vec<Object>,
    material: Material,
) {
    let major_radius =
        1.35;

    let minor_radius =
        0.28;

    let ring_angles = [
        0.25_f32,
        0.85_f32,
        1.45_f32,
        2.05_f32,
        2.65_f32,
        3.25_f32,
        3.85_f32,
        4.45_f32,
        5.05_f32,
        5.65_f32,
    ];

    let tube_angles = [
        0.30_f32,
        0.90_f32,
        1.57_f32,
        2.20_f32,
        2.85_f32,
        3.45_f32,
        4.10_f32,
        4.71_f32,
        5.35_f32,
        5.95_f32,
    ];

    for (
        ring_index,
        ring_angle,
    ) in ring_angles
        .iter()
        .enumerate()
    {
        for (
            tube_index,
            tube_angle,
        ) in tube_angles
            .iter()
            .enumerate()
        {
            if (
                ring_index
                    + tube_index
            ) % 3
                != 0
            {
                continue;
            }

            let cos_ring =
                ring_angle.cos();

            let sin_ring =
                ring_angle.sin();

            let cos_tube =
                tube_angle.cos();

            let sin_tube =
                tube_angle.sin();

            let radial_distance =
                major_radius
                    + minor_radius
                        * cos_tube;

            let x =
                radial_distance
                    * cos_ring;

            let y =
                minor_radius
                    * sin_tube;

            let z =
                radial_distance
                    * sin_ring;

            let surface_normal =
                Vec3::new(
                    cos_tube
                        * cos_ring,
                    sin_tube,
                    cos_tube
                        * sin_ring,
                )
                .normalize();

            let base =
                Vec3::new(
                    x,
                    y,
                    z,
                )
                    - surface_normal
                        * 0.025;

            let size =
                if tube_index % 3
                    == 0
                {
                    0.30
                } else {
                    0.23
                };

            add_crystal_cluster(
                objects,
                base,
                surface_normal,
                size,
                material,
            );
        }
    }
}

fn add_crystal_cluster(
    objects: &mut Vec<Object>,
    base: Vec3,
    surface_normal: Vec3,
    size: f32,
    material: Material,
) {
    let normal =
        surface_normal
            .normalize();

    let reference =
        if normal.y.abs()
            < 0.90
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
        reference
            .cross(
                &normal,
            )
            .normalize();

    let bitangent =
        normal
            .cross(
                &tangent,
            )
            .normalize();

    let directions = [
        normal,

        (
            normal
                + tangent
                    * 0.30
        )
            .normalize(),

        (
            normal
                - tangent
                    * 0.24
                + bitangent
                    * 0.15
        )
            .normalize(),
    ];

    let scales = [
        1.0_f32,
        0.72_f32,
        0.60_f32,
    ];

    for i in 0..directions.len() {
        let axis =
            directions[i];

        let height =
            size
                * scales[i];

        let radius =
            if i == 0 {
                size
                    * 0.17
            } else {
                size
                    * 0.12
            };

        let center =
            base
                + axis
                    * (
                        height
                            * 0.46
                    );

        objects.push(
            Object::Cone(
                Cone::new_oriented(
                    center,
                    axis,
                    radius,
                    height,
                    material,
                ),
            ),
        );
    }
}

fn add_lava_planet(
    objects: &mut Vec<Object>,
    outer_material: Material,
    inner_material: Material,
) {
    objects.push(
        Object::Hemisphere(
            Hemisphere::new_with_materials(
                Vec3::new(
                    0.32,
                    0.46,
                    -1.76,
                ),
                0.82,
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                outer_material,
                inner_material,
            ),
        ),
    );

    objects.push(
        Object::Hemisphere(
            Hemisphere::new_with_materials(
                Vec3::new(
                    0.32,
                    -0.46,
                    -1.76,
                ),
                0.82,
                Vec3::new(
                    0.0,
                    -1.0,
                    0.0,
                ),
                outer_material,
                inner_material,
            ),
        ),
    );
}