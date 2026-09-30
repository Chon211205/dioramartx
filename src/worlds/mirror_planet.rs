use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cuboid::Cuboid;
use crate::objects::object::Object;
use crate::textures::texture::TextureMap;

const WIDTH: f32 = 2.0;
const HEIGHT: f32 = 3.6;
const DEPTH: f32 = 0.15;
const FRAME_WIDTH: f32 = 0.18;

static WOOD_COLOR: OnceLock<TextureMap> = OnceLock::new();
static WOOD_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static WOOD_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();

fn wood_color() -> &'static TextureMap {
    WOOD_COLOR.get_or_init(|| {
        TextureMap::from_file("assets/textures/Fase1Cube/WoodFloor051_1K-PNG_Color.png")
    })
}

fn wood_normal() -> &'static TextureMap {
    WOOD_NORMAL.get_or_init(|| {
        TextureMap::from_file("assets/textures/Fase1Cube/WoodFloor051_1K-PNG_NormalGL.png")
    })
}

fn wood_roughness() -> &'static TextureMap {
    WOOD_ROUGHNESS.get_or_init(|| {
        TextureMap::from_file("assets/textures/Fase1Cube/WoodFloor051_1K-PNG_Roughness.png")
    })
}

pub fn create_mirror_planet_preview() -> Vec<Object> {
    create_mirror(false)
}

pub fn create_mirror_planet_world() -> Vec<Object> {
    create_mirror(true)
}

fn create_mirror(webcam: bool) -> Vec<Object> {
    let mirror = if webcam {
        Material::webcam(Vec3::new(0.10, 0.12, 0.15), 0.90, 0.65, 0.12)
    } else {
        Material::new(Vec3::new(0.75, 0.80, 0.88), 0.28, 1.0, 0.0, 0.92)
    };
    let wood = Material::textured(
        Vec3::new(0.72, 0.52, 0.32),
        0.82,
        0.32,
        0.0,
        0.08,
        Some(wood_color()),
        Some(wood_normal()),
        Some(wood_roughness()),
        None,
    );
    let back = Material::new(Vec3::new(0.07, 0.08, 0.10), 0.65, 0.55, 0.0, 0.18);

    let yaw = -0.16_f32;
    let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin());
    let up = Vec3::new(0.0, 1.0, 0.0);
    let forward = Vec3::new(yaw.sin(), 0.0, yaw.cos());
    let front_center = forward * (DEPTH * 0.5 + 0.006);
    let side = wood;

    vec![
        Object::Cuboid(Cuboid::from_basis_faces(
            front_center,
            Vec3::new(WIDTH - FRAME_WIDTH * 2.0, HEIGHT - FRAME_WIDTH * 2.0, 0.025),
            right,
            up,
            forward,
            side,
            side,
            side,
            side,
            mirror,
            back,
        )),
        Object::Cuboid(Cuboid::from_basis_faces(
            -forward * 0.015,
            Vec3::new(WIDTH, HEIGHT, DEPTH),
            right,
            up,
            forward,
            back,
            back,
            back,
            back,
            back,
            back,
        )),
        Object::Cuboid(Cuboid::from_basis(
            up * (HEIGHT * 0.5 - FRAME_WIDTH * 0.5) + forward * 0.045,
            Vec3::new(WIDTH, FRAME_WIDTH, DEPTH + 0.06),
            right,
            up,
            forward,
            wood,
        )),
        Object::Cuboid(Cuboid::from_basis(
            -up * (HEIGHT * 0.5 - FRAME_WIDTH * 0.5) + forward * 0.045,
            Vec3::new(WIDTH, FRAME_WIDTH, DEPTH + 0.06),
            right,
            up,
            forward,
            wood,
        )),
        Object::Cuboid(Cuboid::from_basis(
            -right * (WIDTH * 0.5 - FRAME_WIDTH * 0.5) + forward * 0.045,
            Vec3::new(FRAME_WIDTH, HEIGHT - FRAME_WIDTH * 2.0, DEPTH + 0.06),
            right,
            up,
            forward,
            wood,
        )),
        Object::Cuboid(Cuboid::from_basis(
            right * (WIDTH * 0.5 - FRAME_WIDTH * 0.5) + forward * 0.045,
            Vec3::new(FRAME_WIDTH, HEIGHT - FRAME_WIDTH * 2.0, DEPTH + 0.06),
            right,
            up,
            forward,
            wood,
        )),
    ]
}
