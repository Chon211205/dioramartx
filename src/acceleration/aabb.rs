use crate::core::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(
        min: Vec3,
        max: Vec3,
    ) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn empty() -> Self {
        Self {
            min: Vec3::new(
                f32::INFINITY,
                f32::INFINITY,
                f32::INFINITY,
            ),
            max: Vec3::new(
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
            ),
        }
    }

    pub fn union(
        &self,
        other: &Aabb,
    ) -> Aabb {
        Aabb {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),

            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    pub fn expand_point(
        &mut self,
        point: Vec3,
    ) {
        self.min.x =
            self.min.x.min(point.x);

        self.min.y =
            self.min.y.min(point.y);

        self.min.z =
            self.min.z.min(point.z);

        self.max.x =
            self.max.x.max(point.x);

        self.max.y =
            self.max.y.max(point.y);

        self.max.z =
            self.max.z.max(point.z);
    }

    pub fn centroid(
        &self,
    ) -> Vec3 {
        (
            self.min
                + self.max
        ) * 0.5
    }

    pub fn extent(
        &self,
    ) -> Vec3 {
        self.max
            - self.min
    }

    pub fn longest_axis(
        &self,
    ) -> usize {
        let extent =
            self.extent();

        if extent.x >= extent.y
            && extent.x >= extent.z
        {
            0
        } else if extent.y >= extent.z {
            1
        } else {
            2
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
        mut t_min: f32,
        mut t_max: f32,
    ) -> bool {
        for axis in 0..3 {
            let (
                origin_axis,
                direction_axis,
                min_axis,
                max_axis,
            ) = match axis {
                0 => (
                    origin.x,
                    direction.x,
                    self.min.x,
                    self.max.x,
                ),

                1 => (
                    origin.y,
                    direction.y,
                    self.min.y,
                    self.max.y,
                ),

                _ => (
                    origin.z,
                    direction.z,
                    self.min.z,
                    self.max.z,
                ),
            };

            if direction_axis.abs()
                < 0.000001
            {
                if origin_axis < min_axis
                    || origin_axis > max_axis
                {
                    return false;
                }

                continue;
            }

            let inv_direction =
                1.0
                    / direction_axis;

            let mut t0 =
                (
                    min_axis
                        - origin_axis
                ) * inv_direction;

            let mut t1 =
                (
                    max_axis
                        - origin_axis
                ) * inv_direction;

            if inv_direction < 0.0 {
                std::mem::swap(
                    &mut t0,
                    &mut t1,
                );
            }

            t_min =
                t_min.max(t0);

            t_max =
                t_max.min(t1);

            if t_max < t_min {
                return false;
            }
        }

        true
    }

    pub fn surface_area(
        &self,
    ) -> f32 {
        let e =
            self.extent();

        2.0
            * (
                e.x * e.y
                    + e.y * e.z
                    + e.z * e.x
            )
    }
}