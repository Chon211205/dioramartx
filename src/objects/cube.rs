use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Cube {
    pub center: Vec3,
    pub half_size: f32,
    pub material: Material,
}

impl Cube {
    pub fn new(
        center: Vec3,
        size: f32,
        material: Material,
    ) -> Self {
        Self {
            center,
            half_size: size * 0.5,
            material,
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        let min = self.min();
        let max = self.max();

        let mut t_min = -f32::INFINITY;
        let mut t_max = f32::INFINITY;

        if !update_slab(
            origin.x,
            direction.x,
            min.x,
            max.x,
            &mut t_min,
            &mut t_max,
        ) {
            return None;
        }

        if !update_slab(
            origin.y,
            direction.y,
            min.y,
            max.y,
            &mut t_min,
            &mut t_max,
        ) {
            return None;
        }

        if !update_slab(
            origin.z,
            direction.z,
            min.z,
            max.z,
            &mut t_min,
            &mut t_max,
        ) {
            return None;
        }

        if t_max < 0.001 {
            return None;
        }

        if t_min > 0.001 {
            Some(t_min)
        } else if t_max > 0.001 {
            Some(t_max)
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

        let ax = local.x.abs();
        let ay = local.y.abs();
        let az = local.z.abs();

        if ax >= ay && ax >= az {
            Vec3::new(
                local.x.signum(),
                0.0,
                0.0,
            )
        } else if ay >= ax && ay >= az {
            Vec3::new(
                0.0,
                local.y.signum(),
                0.0,
            )
        } else {
            Vec3::new(
                0.0,
                0.0,
                local.z.signum(),
            )
        }
    }

    pub fn min(&self) -> Vec3 {
        Vec3::new(
            self.center.x - self.half_size,
            self.center.y - self.half_size,
            self.center.z - self.half_size,
        )
    }

    pub fn max(&self) -> Vec3 {
        Vec3::new(
            self.center.x + self.half_size,
            self.center.y + self.half_size,
            self.center.z + self.half_size,
        )
    }
}

fn update_slab(
    origin: f32,
    direction: f32,
    min: f32,
    max: f32,
    t_min: &mut f32,
    t_max: &mut f32,
) -> bool {
    const EPSILON: f32 = 1e-8;

    if direction.abs() < EPSILON {
        return origin >= min && origin <= max;
    }

    let inverse =
        1.0 / direction;

    let mut t0 =
        (min - origin) * inverse;

    let mut t1 =
        (max - origin) * inverse;

    if t0 > t1 {
        std::mem::swap(
            &mut t0,
            &mut t1,
        );
    }

    *t_min =
        t_min.max(t0);

    *t_max =
        t_max.min(t1);

    *t_max >= *t_min
}