use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Cone {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub material: Material,
}

impl Cone {
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
        let apex_y = half_height;
        let base_y = -half_height;

        let k = self.radius / self.height;
        let k2 = k * k;

        let oy = local_origin.y - apex_y;

        let a =
            direction.x * direction.x
            + direction.z * direction.z
            - k2 * direction.y * direction.y;

        let b =
            2.0
                * (
                    local_origin.x * direction.x
                    + local_origin.z * direction.z
                    - k2 * oy * direction.y
                );

        let c =
            local_origin.x * local_origin.x
            + local_origin.z * local_origin.z
            - k2 * oy * oy;

        let mut closest = f32::INFINITY;

        let discriminant =
            b * b - 4.0 * a * c;

        if discriminant >= 0.0
            && a.abs() > 0.0001
        {
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

                    if y >= base_y
                        && y <= apex_y
                        && t < closest
                    {
                        closest = t;
                    }
                }
            }
        }

        if direction.y.abs() > 0.0001 {
            let t =
                (base_y - local_origin.y)
                    / direction.y;

            if t > 0.001 {
                let x =
                    local_origin.x
                        + direction.x * t;

                let z =
                    local_origin.z
                        + direction.z * t;

                if x * x + z * z
                    <= self.radius * self.radius
                    && t < closest
                {
                    closest = t;
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

        let base_y =
            -half_height;

        let epsilon =
            0.001;

        if (local_point.y - base_y).abs()
            < epsilon
        {
            return Vec3::new(
                0.0,
                -1.0,
                0.0,
            );
        }

        let radial =
            (
                local_point.x * local_point.x
                + local_point.z * local_point.z
            )
            .sqrt();

        let slope =
            self.radius / self.height;

        Vec3::new(
            local_point.x,
            radial * slope,
            local_point.z,
        )
        .normalize()
    }
}