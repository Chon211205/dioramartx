use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::textures::texture::TextureMap;

fn ground_color() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| TextureMap::from_file("assets/textures/ground/Ground048_1K-PNG_Color.png"))
}

fn ground_normal() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/ground/Ground048_1K-PNG_NormalGL.png")
    })
}

fn ground_roughness() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/ground/Ground048_1K-PNG_Roughness.png")
    })
}

fn ground_ao() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/ground/Ground048_1K-PNG_AmbientOcclusion.png")
    })
}

fn metal_color() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| TextureMap::from_file("assets/textures/metal/Metal056C_1K-PNG_Color.png"))
}

fn metal_normal() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| TextureMap::from_file("assets/textures/metal/Metal056C_1K-PNG_NormalGL.png"))
}

fn metal_roughness() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();

    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/metal/Metal056C_1K-PNG_Roughness.png")
    })
}

fn create_ground_material() -> Material {
    Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.30,
        0.0,
        0.02,
        Some(ground_color()),
        Some(ground_normal()),
        Some(ground_roughness()),
        Some(ground_ao()),
    )
}

fn create_metal_material() -> Material {
    Material::textured(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.75,
        0.0,
        0.08,
        Some(metal_color()),
        Some(metal_normal()),
        Some(metal_roughness()),
        None,
    )
}

pub fn create_industrial_planet() -> Vec<Object> {
    let mut objects = Vec::with_capacity(40);

    let ground_material = create_ground_material();

    let metal_material = create_metal_material();

    objects.push(Object::Sphere(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.65,
        ground_material,
    )));

    add_tower(
        &mut objects,
        Vec3::new(0.15, 2.42, 0.10),
        Vec3::new(0.10, 1.0, 0.12).normalize(),
        2.55,
        1.18,
        0.82,
        metal_material,
        ground_material,
    );

    add_tower(
        &mut objects,
        Vec3::new(-2.35, -0.15, 0.15),
        Vec3::new(-1.0, -0.05, 0.0).normalize(),
        1.85,
        1.05,
        0.75,
        metal_material,
        ground_material,
    );

    add_tower(
        &mut objects,
        Vec3::new(1.85, -1.45, 1.05),
        Vec3::new(0.70, -0.48, 0.52).normalize(),
        2.00,
        1.12,
        0.78,
        metal_material,
        ground_material,
    );

    add_tower(
        &mut objects,
        Vec3::new(0.40, 0.20, -2.45),
        Vec3::new(0.20, 0.10, -1.0).normalize(),
        1.95,
        1.05,
        0.72,
        metal_material,
        ground_material,
    );

    objects
}

fn add_tower(
    objects: &mut Vec<Object>,
    base: Vec3,
    axis: Vec3,
    height: f32,
    base_radius: f32,
    top_radius: f32,
    metal_material: Material,
    ground_material: Material,
) {
    let axis = axis.normalize();

    let sections = 4;

    let section_height = height / sections as f32;

    for i in 0..sections {
        let t = i as f32 / (sections - 1) as f32;

        let radius = base_radius + (top_radius - base_radius) * t;

        let center = base + axis * (section_height * (i as f32 + 0.5));

        objects.push(Object::Cylinder(Cylinder::new_oriented(
            center,
            axis,
            radius,
            section_height + 0.05,
            metal_material,
        )));

        if i < sections - 1 {
            let band_center = base + axis * (section_height * (i as f32 + 1.0));

            objects.push(Object::Cylinder(Cylinder::new_oriented(
                band_center,
                axis,
                radius + 0.08,
                0.10,
                metal_material,
            )));
        }
    }

    let base_band = base + axis * 0.10;

    objects.push(Object::Cylinder(Cylinder::new_oriented(
        base_band,
        axis,
        base_radius + 0.12,
        0.18,
        metal_material,
    )));

    let tower_end = base + axis * height;

    objects.push(Object::Cylinder(Cylinder::new_oriented(
        tower_end,
        axis,
        top_radius + 0.14,
        0.20,
        metal_material,
    )));

    let ground_cap_center = tower_end + axis * 0.08;

    objects.push(Object::Cylinder(Cylinder::new_oriented(
        ground_cap_center,
        axis,
        top_radius * 0.92,
        0.16,
        ground_material,
    )));
}
