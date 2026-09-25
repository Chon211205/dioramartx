use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Hemisphere {
    pub center: Vec3,
    pub radius: f32,
    pub normal: Vec3,
    pub material: Material,
}

impl Hemisphere {
    pub fn new(
        center: Vec3,
        radius: f32,
        normal: Vec3,
        material: Material,
    ) -> Self {
        Self {
            center,
            radius,
            normal: normal.normalize(),
            material,
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        let oc = *origin - self.center;

        let a = direction.dot(direction);
        let b = 2.0 * oc.dot(direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        let mut best_t = f32::INFINITY;

        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();

            let t1 = (-b - sqrt_d) / (2.0 * a);
            let t2 = (-b + sqrt_d) / (2.0 * a);

            for t in [t1, t2] {
                if t > 0.001 {
                    let hit_point = *origin + *direction * t;
                    let local = hit_point - self.center;

                    if local.dot(&self.normal) >= -0.0001 {
                        if t < best_t {
                            best_t = t;
                        }
                    }
                }
            }
        }

        let denom = direction.dot(&self.normal);

        if denom.abs() > 0.000001 {
            let t = (self.center - *origin).dot(&self.normal) / denom;

            if t > 0.001 {
                let hit_point = *origin + *direction * t;
                let local = hit_point - self.center;

                let projected =
                    local - self.normal * local.dot(&self.normal);

                if projected.length() <= self.radius && denom < 0.0 {
                    if t < best_t {
                        best_t = t;
                    }
                }
            }
        }

        if best_t.is_finite() {
            Some(best_t)
        } else {
            None
        }
    }

    pub fn normal_at(
        &self,
        point: &Vec3,
    ) -> Vec3 {
        let local = *point - self.center;

        let distance_to_plane =
            local.dot(&self.normal).abs();

        let projected =
            local - self.normal * local.dot(&self.normal);

        if distance_to_plane < 0.001
            && projected.length() <= self.radius + 0.001
        {
            self.normal * -1.0
        } else {
            local.normalize()
        }
    }
}