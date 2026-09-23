use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        material: Material,
    ) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            material,
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        let denominator =
            self.normal.dot(direction);

        if denominator.abs() < 0.0001 {
            return None;
        }

        let t =
            (self.point - *origin)
                .dot(&self.normal)
                / denominator;

        if t > 0.001 {
            Some(t)
        } else {
            None
        }
    }

    pub fn normal_at(&self) -> Vec3 {
        self.normal
    }
}