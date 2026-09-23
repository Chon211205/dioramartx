use crate::core::vec3::Vec3;
use crate::textures::texture::TextureMap;

#[derive(Clone, Copy)]
pub enum MaterialPattern {
    Solid,
    Grass,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Vec3,
    pub albedo: f32,
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub pattern: MaterialPattern,

    pub albedo_texture: Option<&'static TextureMap>,
    pub normal_texture: Option<&'static TextureMap>,
    pub roughness_texture: Option<&'static TextureMap>,
    pub ao_texture: Option<&'static TextureMap>,
}

impl Material {
    pub fn new(
        color: Vec3,
        albedo: f32,
        specular: f32,
        transparency: f32,
        reflectivity: f32,
    ) -> Self {
        Self {
            color,
            albedo,
            specular,
            transparency,
            reflectivity,
            pattern: MaterialPattern::Solid,
            albedo_texture: None,
            normal_texture: None,
            roughness_texture: None,
            ao_texture: None,
        }
    }

    pub fn grass(
        color: Vec3,
        albedo: f32,
        specular: f32,
        transparency: f32,
        reflectivity: f32,
    ) -> Self {
        Self {
            color,
            albedo,
            specular,
            transparency,
            reflectivity,
            pattern: MaterialPattern::Grass,
            albedo_texture: None,
            normal_texture: None,
            roughness_texture: None,
            ao_texture: None,
        }
    }

    pub fn textured(
        color: Vec3,
        albedo: f32,
        specular: f32,
        transparency: f32,
        reflectivity: f32,
        albedo_texture: Option<&'static TextureMap>,
        normal_texture: Option<&'static TextureMap>,
        roughness_texture: Option<&'static TextureMap>,
        ao_texture: Option<&'static TextureMap>,
    ) -> Self {
        Self {
            color,
            albedo,
            specular,
            transparency,
            reflectivity,
            pattern: MaterialPattern::Solid,
            albedo_texture,
            normal_texture,
            roughness_texture,
            ao_texture,
        }
    }

    pub fn grass_textured(
        color: Vec3,
        albedo: f32,
        specular: f32,
        transparency: f32,
        reflectivity: f32,
        albedo_texture: Option<&'static TextureMap>,
        normal_texture: Option<&'static TextureMap>,
        roughness_texture: Option<&'static TextureMap>,
        ao_texture: Option<&'static TextureMap>,
    ) -> Self {
        Self {
            color,
            albedo,
            specular,
            transparency,
            reflectivity,
            pattern: MaterialPattern::Grass,
            albedo_texture,
            normal_texture,
            roughness_texture,
            ao_texture,
        }
    }
}