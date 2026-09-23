use crate::core::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(
        position: Vec3,
        color: Vec3,
        intensity: f32,
    ) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }
}