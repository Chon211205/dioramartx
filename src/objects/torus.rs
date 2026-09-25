use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Torus {
    pub center: Vec3,
    pub major_radius: f32,
    pub minor_radius: f32,
    pub material: Material,
}

impl Torus {
    pub fn new(
        center: Vec3,
        major_radius: f32,
        minor_radius: f32,
        material: Material,
    ) -> Self {
        Self {
            center,
            major_radius,
            minor_radius,
            material,
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        const MAX_STEPS: usize = 160;
        const HIT_EPSILON: f32 = 0.001;
        const MAX_DISTANCE: f32 = 50.0;
        const MIN_STEP: f32 = 0.0005;

        let mut traveled = 0.001;

        for _ in 0..MAX_STEPS {
            let point =
                *origin
                    + *direction * traveled;

            let distance =
                self.sdf(point);

            if distance.abs()
                < HIT_EPSILON
            {
                return Some(traveled);
            }

            traveled +=
                distance
                    .abs()
                    .max(MIN_STEP);

            if traveled
                > MAX_DISTANCE
            {
                break;
            }
        }

        None
    }

    pub fn normal_at(
        &self,
        point: &Vec3,
    ) -> Vec3 {
        let local =
            *point
                - self.center;

        let radial =
            (
                local.x * local.x
                    + local.z * local.z
            )
                .sqrt();

        if radial < 0.000001 {
            return Vec3::new(
                0.0,
                1.0,
                0.0,
            );
        }

        let ring_x =
            self.major_radius
                * local.x
                / radial;

        let ring_z =
            self.major_radius
                * local.z
                / radial;

        Vec3::new(
            local.x - ring_x,
            local.y,
            local.z - ring_z,
        )
        .normalize()
    }

    pub fn sdf(
        &self,
        point: Vec3,
    ) -> f32 {
        let local =
            point
                - self.center;

        let radial =
            (
                local.x * local.x
                    + local.z * local.z
            )
                .sqrt();

        let qx =
            radial
                - self.major_radius;

        let qy =
            local.y;

        (
            qx * qx
                + qy * qy
        )
            .sqrt()
            - self.minor_radius
    }

    pub fn min(
        &self,
    ) -> Vec3 {
        let horizontal_extent =
            self.major_radius
                + self.minor_radius;

        Vec3::new(
            self.center.x
                - horizontal_extent,
            self.center.y
                - self.minor_radius,
            self.center.z
                - horizontal_extent,
        )
    }

    pub fn max(
        &self,
    ) -> Vec3 {
        let horizontal_extent =
            self.major_radius
                + self.minor_radius;

        Vec3::new(
            self.center.x
                + horizontal_extent,
            self.center.y
                + self.minor_radius,
            self.center.z
                + horizontal_extent,
        )
    }
}