use crate::core::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Vec3,
    pub albedo: f32,
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
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
        }
    }
}