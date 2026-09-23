use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Cylinder {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub material: Material,
}

impl Cylinder {
    pub fn new(
        center: Vec3,
        radius: f32,
        height: f32,
        material: Material,
    ) -> Self {
        Self {
            center,
            radius,
            height,
            material,
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        let local_origin = *origin - self.center;

        let half_height = self.height * 0.5;

        let mut closest = f32::INFINITY;

        let a =
            direction.x * direction.x
            + direction.z * direction.z;

        if a.abs() > 0.0001 {
            let b =
                2.0
                    * (
                        local_origin.x * direction.x
                        + local_origin.z * direction.z
                    );

            let c =
                local_origin.x * local_origin.x
                + local_origin.z * local_origin.z
                - self.radius * self.radius;

            let discriminant =
                b * b - 4.0 * a * c;

            if discriminant >= 0.0 {
                let sqrt_discriminant =
                    discriminant.sqrt();

                let t1 =
                    (-b - sqrt_discriminant)
                        / (2.0 * a);

                let t2 =
                    (-b + sqrt_discriminant)
                        / (2.0 * a);

                for t in [t1, t2] {
                    if t > 0.001 {
                        let y =
                            local_origin.y
                                + direction.y * t;

                        if y >= -half_height
                            && y <= half_height
                            && t < closest
                        {
                            closest = t;
                        }
                    }
                }
            }
        }

        if direction.y.abs() > 0.0001 {
            let bottom_t =
                (-half_height - local_origin.y)
                    / direction.y;

            if bottom_t > 0.001 {
                let x =
                    local_origin.x
                        + direction.x * bottom_t;

                let z =
                    local_origin.z
                        + direction.z * bottom_t;

                if x * x + z * z
                    <= self.radius * self.radius
                    && bottom_t < closest
                {
                    closest = bottom_t;
                }
            }

            let top_t =
                (half_height - local_origin.y)
                    / direction.y;

            if top_t > 0.001 {
                let x =
                    local_origin.x
                        + direction.x * top_t;

                let z =
                    local_origin.z
                        + direction.z * top_t;

                if x * x + z * z
                    <= self.radius * self.radius
                    && top_t < closest
                {
                    closest = top_t;
                }
            }
        }

        if closest < f32::INFINITY {
            Some(closest)
        } else {
            None
        }
    }

    pub fn normal_at(
        &self,
        point: &Vec3,
    ) -> Vec3 {
        let local_point =
            *point - self.center;

        let half_height =
            self.height * 0.5;

        let epsilon = 0.001;

        if (local_point.y - half_height).abs()
            < epsilon
        {
            return Vec3::new(
                0.0,
                1.0,
                0.0,
            );
        }

        if (local_point.y + half_height).abs()
            < epsilon
        {
            return Vec3::new(
                0.0,
                -1.0,
                0.0,
            );
        }

        Vec3::new(
            local_point.x,
            0.0,
            local_point.z,
        )
        .normalize()
    }
}