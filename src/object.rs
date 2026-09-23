use crate::vec3::Vec3;

pub struct Object {
    pub center: Vec3,
    pub radius: f32,
    pub color: Vec3,
}

impl Object {
    pub fn new(center: Vec3, radius: f32, color: Vec3) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }

    pub fn intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<f32> {
        let oc = *origin - self.center;

        let a = direction.dot(direction);
        let b = 2.0 * oc.dot(direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        if t1 > 0.001 {
            Some(t1)
        } else if t2 > 0.001 {
            Some(t2)
        } else {
            None
        }
    }
}