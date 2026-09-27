use crate::core::vec3::Vec3;
use crate::objects::object::Object;

pub struct PlanetDefinition {
    pub name: &'static str,
    pub create: fn() -> Vec<Object>,
    pub position: Vec3,
    pub preview_scale: f32,
    pub hit_radius: f32,
    pub node_offset: Vec3,
}

impl PlanetDefinition {
    pub fn new(
        name: &'static str,
        create: fn() -> Vec<Object>,
        position: Vec3,
        preview_scale: f32,
        hit_radius: f32,
        node_offset: Vec3,
    ) -> Self {
        Self {
            name,
            create,
            position,
            preview_scale,
            hit_radius,
            node_offset,
        }
    }
}