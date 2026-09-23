use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::sphere::Sphere;

pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        match self {
            Object::Sphere(sphere) => {
                sphere.intersect(origin, direction)
            }
        }
    }

    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        match self {
            Object::Sphere(sphere) => {
                sphere.normal_at(point)
            }
        }
    }

    pub fn material(&self) -> Material {
        match self {
            Object::Sphere(sphere) => {
                sphere.material
            }
        }
    }
}