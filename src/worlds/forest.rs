use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;

use crate::objects::cone::Cone;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

use crate::textures::texture::TextureMap;

const PLANET_RADIUS: f32 = 1.8;

static GRASS_COLOR_MAP: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_NORMAL_MAP: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_ROUGHNESS_MAP: OnceLock<TextureMap> =
    OnceLock::new();

static GRASS_AO_MAP: OnceLock<TextureMap> =
    OnceLock::new();

fn grass_color_map() -> &'static TextureMap {
    GRASS_COLOR_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_4K-PNG_Color.png",
        )
    })
}

fn grass_normal_map() -> &'static TextureMap {
    GRASS_NORMAL_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_4K-PNG_NormalGL.png",
        )
    })
}

fn grass_roughness_map() -> &'static TextureMap {
    GRASS_ROUGHNESS_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_4K-PNG_Roughness.png",
        )
    })
}

fn grass_ao_map() -> &'static TextureMap {
    GRASS_AO_MAP.get_or_init(|| {
        TextureMap::from_file(
            "assets/textures/grass/Grass005_4K-PNG_AmbientOcclusion.png",
        )
    })
}

pub fn create_forest_diorama() -> Vec<Object> {
    let grass = Material::grass_textured(
        Vec3::new(0.08, 0.55, 0.12),
        0.80,
        0.20,
        0.0,
        0.03,
        Some(grass_color_map()),
        Some(grass_normal_map()),
        Some(grass_roughness_map()),
        Some(grass_ao_map()),
    );

    let grass_detail = Material::new(
        Vec3::new(0.12, 0.72, 0.15),
        0.85,
        0.08,
        0.0,
        0.0,
    );

    let soil = Material::new(
        Vec3::new(0.30, 0.18, 0.07),
        0.78,
        0.12,
        0.0,
        0.02,
    );

    let wood = Material::new(
        Vec3::new(0.38, 0.18, 0.06),
        0.75,
        0.12,
        0.0,
        0.0,
    );

    let leaves_dark = Material::new(
        Vec3::new(0.05, 0.38, 0.08),
        0.80,
        0.18,
        0.0,
        0.02,
    );

    let leaves_light = Material::new(
        Vec3::new(0.10, 0.48, 0.12),
        0.82,
        0.18,
        0.0,
        0.02,
    );

    let bush = Material::new(
        Vec3::new(0.08, 0.45, 0.10),
        0.80,
        0.12,
        0.0,
        0.01,
    );

    let rock = Material::new(
        Vec3::new(0.18, 0.20, 0.20),
        0.68,
        0.18,
        0.0,
        0.02,
    );

    let rock_light = Material::new(
        Vec3::new(0.32, 0.34, 0.34),
        0.70,
        0.18,
        0.0,
        0.02,
    );

    let yellow = Material::new(
        Vec3::new(0.95, 0.82, 0.12),
        0.85,
        0.25,
        0.0,
        0.02,
    );

    let purple = Material::new(
        Vec3::new(0.58, 0.20, 0.82),
        0.85,
        0.25,
        0.0,
        0.02,
    );

    let pink = Material::new(
        Vec3::new(0.95, 0.30, 0.50),
        0.85,
        0.25,
        0.0,
        0.02,
    );

    let white = Material::new(
        Vec3::new(0.95, 0.95, 0.95),
        0.85,
        0.25,
        0.0,
        0.02,
    );

    let wall = Material::new(
        Vec3::new(0.72, 0.66, 0.50),
        0.75,
        0.15,
        0.0,
        0.02,
    );

    let roof = Material::new(
        Vec3::new(0.15, 0.22, 0.35),
        0.72,
        0.35,
        0.0,
        0.08,
    );

    let pipe = Material::new(
        Vec3::new(0.12, 0.70, 0.20),
        0.75,
        0.35,
        0.0,
        0.04,
    );

    let mushroom_red = Material::new(
        Vec3::new(0.82, 0.12, 0.10),
        0.82,
        0.18,
        0.0,
        0.01,
    );

    let mut objects = Vec::new();

    objects.push(
        Object::Sphere(
            Sphere::new(
                Vec3::new(0.0, 0.0, 0.0),
                PLANET_RADIUS,
                grass,
            ),
        ),
    );

    add_grass_field(
        &mut objects,
        grass_detail,
    );

    add_patch(
        &mut objects,
        Vec3::new(0.4, 0.8, 1.0),
        0.52,
        soil,
    );

    add_patch(
        &mut objects,
        Vec3::new(-0.8, 0.7, 0.6),
        0.44,
        soil,
    );

    add_patch(
        &mut objects,
        Vec3::new(0.7, 0.25, -0.8),
        0.34,
        soil,
    );

    add_patch(
        &mut objects,
        Vec3::new(-0.2, -0.15, 0.95),
        0.28,
        soil,
    );

    add_tree_on_planet(
        &mut objects,
        Vec3::new(0.0, 1.0, 0.0),
        0.72,
        wood,
        leaves_dark,
    );

    add_tree_on_planet(
        &mut objects,
        Vec3::new(-0.55, 0.78, 0.30),
        0.54,
        wood,
        leaves_light,
    );

    add_tree_on_planet(
        &mut objects,
        Vec3::new(0.62, 0.70, 0.35),
        0.52,
        wood,
        leaves_dark,
    );

    add_tree_on_planet(
        &mut objects,
        Vec3::new(-0.82, 0.28, 0.50),
        0.44,
        wood,
        leaves_light,
    );

    add_tree_on_planet(
        &mut objects,
        Vec3::new(0.82, 0.38, -0.42),
        0.48,
        wood,
        leaves_dark,
    );

    add_tree_on_planet(
        &mut objects,
        Vec3::new(-0.62, -0.15, 0.78),
        0.40,
        wood,
        leaves_light,
    );

    add_tree_on_planet(
        &mut objects,
        Vec3::new(0.15, 0.08, -0.98),
        0.42,
        wood,
        leaves_dark,
    );

    add_bush_on_planet(
        &mut objects,
        Vec3::new(0.35, 0.68, 0.62),
        0.12,
        bush,
    );

    add_bush_on_planet(
        &mut objects,
        Vec3::new(-0.42, 0.55, 0.72),
        0.11,
        bush,
    );

    add_bush_on_planet(
        &mut objects,
        Vec3::new(-0.78, 0.10, -0.32),
        0.10,
        bush,
    );

    add_rock_on_planet(
        &mut objects,
        Vec3::new(0.60, 0.25, 0.78),
        0.20,
        rock,
    );

    add_rock_on_planet(
        &mut objects,
        Vec3::new(-0.72, 0.20, 0.65),
        0.16,
        rock_light,
    );

    add_rock_on_planet(
        &mut objects,
        Vec3::new(0.25, -0.30, 0.92),
        0.18,
        rock,
    );

    add_rock_on_planet(
        &mut objects,
        Vec3::new(0.88, -0.05, -0.45),
        0.15,
        rock_light,
    );

    add_rock_on_planet(
        &mut objects,
        Vec3::new(0.10, 0.12, -0.96),
        0.13,
        rock,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(0.30, 0.85, 0.42),
        0.16,
        yellow,
        purple,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(-0.42, 0.82, 0.40),
        0.15,
        pink,
        purple,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(0.65, 0.55, 0.52),
        0.13,
        yellow,
        pink,
    );

    add_flower_patch(
        &mut objects,
        Vec3::new(-0.25, 0.55, -0.80),
        0.12,
        white,
        yellow,
    );

    add_pipe_on_planet(
        &mut objects,
        Vec3::new(-0.90, 0.35, 0.28),
        0.12,
        0.30,
        pipe,
    );

    add_pipe_on_planet(
        &mut objects,
        Vec3::new(0.78, -0.15, 0.55),
        0.09,
        0.22,
        pipe,
    );

    add_tower_on_planet(
        &mut objects,
        Vec3::new(-0.20, 0.88, -0.42),
        0.55,
        wall,
        roof,
    );

    add_tower_on_planet(
        &mut objects,
        Vec3::new(0.58, 0.18, -0.78),
        0.32,
        wall,
        roof,
    );

    add_stump_on_planet(
        &mut objects,
        Vec3::new(0.82, 0.52, 0.02),
        0.11,
        0.13,
        wood,
    );

    add_stump_on_planet(
        &mut objects,
        Vec3::new(-0.52, -0.25, 0.86),
        0.09,
        0.11,
        wood,
    );

    add_mushroom_on_planet(
        &mut objects,
        Vec3::new(0.48, 0.62, 0.58),
        0.10,
        mushroom_red,
        white,
    );

    add_mushroom_on_planet(
        &mut objects,
        Vec3::new(-0.35, 0.62, 0.68),
        0.09,
        mushroom_red,
        white,
    );

    objects
}

fn surface_point(
    direction: Vec3,
) -> (Vec3, Vec3) {
    let normal =
        direction.normalize();

    (
        normal * PLANET_RADIUS,
        normal,
    )
}

fn add_grass_field(
    objects: &mut Vec<Object>,
    material: Material,
) {
    let directions = [
        Vec3::new(0.25, 0.94, 0.20),
        Vec3::new(-0.30, 0.90, 0.25),
        Vec3::new(0.55, 0.78, 0.20),
        Vec3::new(-0.58, 0.75, 0.30),

        Vec3::new(0.15, 0.72, 0.68),
        Vec3::new(-0.20, 0.68, 0.70),

        Vec3::new(0.88, 0.28, 0.38),
        Vec3::new(-0.88, 0.30, 0.35),

        Vec3::new(0.30, 0.42, -0.86),
        Vec3::new(-0.30, 0.45, -0.84),

        Vec3::new(0.55, -0.20, 0.82),
        Vec3::new(-0.55, -0.18, 0.82),
    ];

    for direction in directions {
        add_grass_tuft(
            objects,
            direction,
            material,
        );
    }
}

fn add_grass_tuft(
    objects: &mut Vec<Object>,
    direction: Vec3,
    material: Material,
) {
    let normal =
        direction.normalize();

    let surface =
        normal * PLANET_RADIUS;

    let height = 0.075;
    let radius = 0.018;

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                surface
                    + normal
                        * (height * 0.45),
                normal,
                radius,
                height,
                material,
            ),
        ),
    );
}

fn add_tree_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    scale: f32,
    wood: Material,
    leaves: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    let trunk_height =
        0.75 * scale;

    let trunk_radius =
        0.11 * scale;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (trunk_height * 0.5),
                normal,
                trunk_radius,
                trunk_height,
                wood,
            ),
        ),
    );

    let lower_height =
        0.85 * scale;

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                surface
                    + normal
                        * (
                            trunk_height
                                + lower_height * 0.35
                        ),
                normal,
                0.48 * scale,
                lower_height,
                leaves,
            ),
        ),
    );

    let upper_height =
        0.65 * scale;

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                surface
                    + normal
                        * (
                            trunk_height
                                + 0.50 * scale
                        ),
                normal,
                0.34 * scale,
                upper_height,
                leaves,
            ),
        ),
    );
}

fn add_bush_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    material: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    let helper =
        if normal.y.abs() < 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

    let tangent =
        normal
            .cross(&helper)
            .normalize();

    objects.push(
        Object::Sphere(
            Sphere::new(
                surface
                    + normal * radius * 0.50,
                radius,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                surface
                    + normal * radius * 0.40
                    + tangent * radius * 0.85,
                radius * 0.82,
                material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                surface
                    + normal * radius * 0.40
                    - tangent * radius * 0.85,
                radius * 0.82,
                material,
            ),
        ),
    );
}

fn add_rock_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    material: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    objects.push(
        Object::Sphere(
            Sphere::new(
                surface
                    + normal * radius * 0.35,
                radius,
                material,
            ),
        ),
    );
}

fn add_patch(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    material: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    objects.push(
        Object::Sphere(
            Sphere::new(
                surface
                    - normal * radius * 0.65,
                radius,
                material,
            ),
        ),
    );
}

fn add_flower_patch(
    objects: &mut Vec<Object>,
    direction: Vec3,
    size: f32,
    material_a: Material,
    material_b: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    let helper =
        if normal.y.abs() < 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

    let tangent_a =
        normal
            .cross(&helper)
            .normalize();

    let tangent_b =
        normal
            .cross(&tangent_a)
            .normalize();

    let offsets = [
        (-0.8, 0.0),
        (-0.3, 0.5),
        (0.2, 0.0),
        (0.65, -0.35),
        (0.75, 0.45),
    ];

    for (i, (a, b)) in
        offsets.iter().enumerate()
    {
        let material =
            if i % 2 == 0 {
                material_a
            } else {
                material_b
            };

        let position =
            surface
                + normal * 0.025
                + tangent_a
                    * (*a * size)
                + tangent_b
                    * (*b * size);

        objects.push(
            Object::Sphere(
                Sphere::new(
                    position,
                    size * 0.24,
                    material,
                ),
            ),
        );
    }
}

fn add_pipe_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    height: f32,
    material: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (height * 0.5),
                normal,
                radius,
                height,
                material,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (height + 0.025),
                normal,
                radius * 1.25,
                0.10,
                material,
            ),
        ),
    );
}

fn add_tower_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    scale: f32,
    wall: Material,
    roof: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    let body_height =
        0.80 * scale;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (body_height * 0.5),
                normal,
                0.30 * scale,
                body_height,
                wall,
            ),
        ),
    );

    let roof_height =
        0.48 * scale;

    objects.push(
        Object::Cone(
            Cone::new_oriented(
                surface
                    + normal
                        * (
                            body_height
                                + roof_height * 0.35
                        ),
                normal,
                0.42 * scale,
                roof_height,
                roof,
            ),
        ),
    );
}

fn add_stump_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    radius: f32,
    height: f32,
    material: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (height * 0.5),
                normal,
                radius,
                height,
                material,
            ),
        ),
    );
}

fn add_mushroom_on_planet(
    objects: &mut Vec<Object>,
    direction: Vec3,
    scale: f32,
    cap_material: Material,
    stem_material: Material,
) {
    let (surface, normal) =
        surface_point(direction);

    let stem_height =
        scale * 0.80;

    let stem_radius =
        scale * 0.22;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                surface
                    + normal
                        * (stem_height * 0.5),
                normal,
                stem_radius,
                stem_height,
                stem_material,
            ),
        ),
    );

    let cap_center =
        surface
            + normal
                * (
                    stem_height
                        + scale * 0.18
                );

    objects.push(
        Object::Sphere(
            Sphere::new(
                cap_center,
                scale,
                cap_material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                cap_center
                    + normal
                        * (scale * 0.70),
                scale * 0.20,
                stem_material,
            ),
        ),
    );
}