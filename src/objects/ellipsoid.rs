use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Ellipsoid {
    pub center: Vec3,
    pub radii: Vec3,
    pub material: Material,
}

impl Ellipsoid {
    pub fn new(
        center: Vec3,
        radii: Vec3,
        material: Material,
    ) -> Self {
        Self {
            center,
            radii,
            material,
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        let oc =
            *origin - self.center;

        let ox =
            oc.x / self.radii.x;

        let oy =
            oc.y / self.radii.y;

        let oz =
            oc.z / self.radii.z;

        let dx =
            direction.x / self.radii.x;

        let dy =
            direction.y / self.radii.y;

        let dz =
            direction.z / self.radii.z;

        let a =
            dx * dx
                + dy * dy
                + dz * dz;

        let b =
            2.0
                * (
                    ox * dx
                        + oy * dy
                        + oz * dz
                );

        let c =
            ox * ox
                + oy * oy
                + oz * oz
                - 1.0;

        let discriminant =
            b * b
                - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d =
            discriminant.sqrt();

        let t1 =
            (-b - sqrt_d)
                / (2.0 * a);

        let t2 =
            (-b + sqrt_d)
                / (2.0 * a);

        if t1 > 0.001 {
            Some(t1)
        } else if t2 > 0.001 {
            Some(t2)
        } else {
            None
        }
    }

    pub fn normal_at(
        &self,
        point: &Vec3,
    ) -> Vec3 {
        let local =
            *point - self.center;

        Vec3::new(
            local.x
                / (
                    self.radii.x
                        * self.radii.x
                ),

            local.y
                / (
                    self.radii.y
                        * self.radii.y
                ),

            local.z
                / (
                    self.radii.z
                        * self.radii.z
                ),
        )
        .normalize()
    }

    pub fn min(
        &self,
    ) -> Vec3 {
        self.center - self.radii
    }

    pub fn max(
        &self,
    ) -> Vec3 {
        self.center + self.radii
    }
}