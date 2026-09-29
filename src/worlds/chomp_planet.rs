use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cone::Cone;
use crate::objects::cube::Cube;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;
use crate::textures::texture::TextureMap;

fn chomp_color() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();
    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/chomp/Metal046B_1K-PNG_Color.png")
    })
}

fn chomp_normal() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();
    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/chomp/Metal046B_1K-PNG_NormalGL.png")
    })
}

fn chomp_roughness() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();
    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/chomp/Metal046B_1K-PNG_Roughness.png")
    })
}

fn chomp_metalness() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();
    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/chomp/Metal046B_1K-PNG_Metalness.png")
    })
}

fn lava_color() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();
    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/lava/Lava002_1K-PNG_Color.png")
    })
}

fn lava_normal() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();
    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/lava/Lava002_1K-PNG_NormalGL.png")
    })
}

fn lava_roughness() -> &'static TextureMap {
    static TEX: OnceLock<TextureMap> = OnceLock::new();
    TEX.get_or_init(|| {
        TextureMap::from_file("assets/textures/lava/Lava002_1K-PNG_Roughness.png")
    })
}

fn create_chomp_material() -> Material {
    Material::textured(
        Vec3::new(0.08, 0.08, 0.09),
        1.0,
        1.0,
        0.0,
        0.08,
        Some(chomp_color()),
        Some(chomp_normal()),
        Some(chomp_roughness()),
        Some(chomp_metalness()),
    )
}

fn create_chomp_soft_material() -> Material {
    Material::textured(
        Vec3::new(0.12, 0.12, 0.13),
        1.0,
        0.85,
        0.0,
        0.04,
        Some(chomp_color()),
        Some(chomp_normal()),
        Some(chomp_roughness()),
        Some(chomp_metalness()),
    )
}

fn create_lava_base_material() -> Material {
    Material::textured(
        Vec3::new(1.0, 0.42, 0.20),
        1.0,
        0.90,
        0.0,
        0.14,
        Some(lava_color()),
        Some(lava_normal()),
        Some(lava_roughness()),
        None,
    )
}

pub fn create_chomp_planet_world() -> Vec<Object> {
    let mut objects = Vec::new();

    let black = create_chomp_material();
    let black_soft = create_chomp_soft_material();
    let lava_base = create_lava_base_material();

    let white = Material::new(
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        0.95,
        0.0,
        0.08,
    );

    let gray = Material::new(
        Vec3::new(0.42, 0.45, 0.48),
        1.0,
        0.75,
        0.0,
        0.12,
    );

    objects.push(Object::Cube(Cube::new(
        Vec3::new(0.0, 0.18, 0.0),
        2.10,
        lava_base,
    )));

    objects.push(Object::Cube(Cube::new(
        Vec3::new(-1.45, 0.18, 0.0),
        1.25,
        lava_base,
    )));

    objects.push(Object::Cube(Cube::new(
        Vec3::new(1.45, 0.18, 0.0),
        1.25,
        lava_base,
    )));

    objects.push(Object::Cube(Cube::new(
        Vec3::new(0.0, 0.48, 0.0),
        1.72,
        lava_base,
    )));

    objects.push(Object::Cube(Cube::new(
        Vec3::new(-1.10, 0.48, 0.0),
        0.95,
        lava_base,
    )));

    objects.push(Object::Cube(Cube::new(
        Vec3::new(1.10, 0.48, 0.0),
        0.95,
        lava_base,
    )));

    objects.push(Object::Cube(Cube::new(
        Vec3::new(0.0, 0.70, 0.0),
        1.28,
        lava_base,
    )));

    let bolt_positions = [
        Vec3::new(-1.82, 0.18, 0.92),
        Vec3::new(1.82, 0.18, 0.92),
        Vec3::new(-1.82, 0.18, -0.92),
        Vec3::new(1.82, 0.18, -0.92),
        Vec3::new(-0.78, 0.18, 1.02),
        Vec3::new(0.78, 0.18, 1.02),
        Vec3::new(-0.78, 0.18, -1.02),
        Vec3::new(0.78, 0.18, -1.02),
    ];

    for position in bolt_positions {
        objects.push(Object::Sphere(Sphere::new(
            position,
            0.075,
            gray,
        )));
    }

    let body_center = Vec3::new(0.0, 2.12, 0.0);

    objects.push(Object::Sphere(Sphere::new(
        body_center,
        1.38,
        black,
    )));

    let left_eye = body_center + Vec3::new(-0.46, 0.86, 0.96);
    let right_eye = body_center + Vec3::new(0.36, 0.88, 0.96);

    objects.push(Object::Sphere(Sphere::new(
        left_eye,
        0.33,
        white,
    )));

    objects.push(Object::Sphere(Sphere::new(
        right_eye,
        0.33,
        white,
    )));

    objects.push(Object::Sphere(Sphere::new(
        left_eye + Vec3::new(0.00, 0.20, 0.12),
        0.16,
        black_soft,
    )));

    objects.push(Object::Sphere(Sphere::new(
        right_eye + Vec3::new(0.00, 0.20, 0.12),
        0.16,
        black_soft,
    )));

    objects.push(Object::Sphere(Sphere::new(
        left_eye + Vec3::new(-0.05, 0.24, 0.21),
        0.048,
        white,
    )));

    objects.push(Object::Sphere(Sphere::new(
        right_eye + Vec3::new(-0.05, 0.24, 0.21),
        0.048,
        white,
    )));

    let upper_teeth = [
        (-0.78, 0.18),
        (-0.52, 0.10),
        (-0.26, 0.04),
        (0.00, 0.00),
        (0.26, -0.04),
        (0.52, -0.10),
        (0.78, -0.18),
    ];

    for (x, tilt) in upper_teeth {
        let axis = Vec3::new(tilt, -1.0, 0.10).normalize();

        objects.push(Object::Cone(
            Cone::new_oriented(
                body_center + Vec3::new(x, 0.04, 1.33),
                axis,
                0.17,
                0.34,
                white,
            ),
        ));
    }

    let lower_teeth_main = [
        (-0.65, -0.16),
        (-0.39, -0.08),
        (-0.13, -0.02),
        (0.13, 0.02),
        (0.39, 0.08),
        (0.65, 0.16),
    ];

    for (x, tilt) in lower_teeth_main {
        let axis = Vec3::new(tilt, 1.0, 0.10).normalize();

        objects.push(Object::Cone(
            Cone::new_oriented(
                body_center + Vec3::new(x, -0.04, 1.31),
                axis,
                0.17,
                0.34,
                white,
            ),
        ));
    }

    for (x, tilt) in lower_teeth_main {
        let axis = Vec3::new(tilt, 0.97, 0.24).normalize();

        objects.push(Object::Cone(
            Cone::new_oriented(
                body_center + Vec3::new(x, -0.12, 1.29),
                axis,
                0.18,
                0.42,
                white,
            ),
        ));
    }

    for (x, tilt) in upper_teeth {
        let axis = Vec3::new(tilt, -0.995, 0.10).normalize();

        objects.push(Object::Cone(
            Cone::new_oriented(
                body_center + Vec3::new(x, 0.12, 1.18),
                axis,
                0.14,
                0.31,
                white,
            ),
        ));
    }

    let lower_teeth_secondary = [
        (-0.56, -0.14),
        (-0.36, -0.08),
        (-0.16, -0.04),
        (0.04, 0.02),
        (0.24, 0.06),
        (0.44, 0.10),
        (0.62, 0.14),
    ];

    for (x, tilt) in lower_teeth_secondary {
        let axis = Vec3::new(tilt, 0.995, 0.10).normalize();

        objects.push(Object::Cone(
            Cone::new_oriented(
                body_center + Vec3::new(x, -0.02, 1.17),
                axis,
                0.14,
                0.31,
                white,
            ),
        ));
    }

    objects
}