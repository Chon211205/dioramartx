use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Cube {
    pub center: Vec3,
    pub half_size: f32,
    pub material: Material,
    pub right: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Material) -> Self {
        Self {
            center,
            half_size: size * 0.5,
            material,
            right: Vec3::new(1.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            forward: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn new_oriented(
        center: Vec3,
        size: f32,
        up: Vec3,
        material: Material,
    ) -> Self {
        let up = up.normalize();

        let helper = if up.y.abs() < 0.99 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let right = helper.cross(&up).normalize();
        let forward = up.cross(&right).normalize();

        Self::from_basis(
            center,
            size,
            right,
            up,
            forward,
            material,
        )
    }

    pub fn from_basis(
        center: Vec3,
        size: f32,
        right: Vec3,
        up: Vec3,
        forward: Vec3,
        material: Material,
    ) -> Self {
        Self {
            center,
            half_size: size * 0.5,
            material,
            right: right.normalize(),
            up: up.normalize(),
            forward: forward.normalize(),
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        let relative_origin = *origin - self.center;

        let local_origin = Vec3::new(
            relative_origin.dot(&self.right),
            relative_origin.dot(&self.up),
            relative_origin.dot(&self.forward),
        );

        let local_direction = Vec3::new(
            direction.dot(&self.right),
            direction.dot(&self.up),
            direction.dot(&self.forward),
        );

        let min = -self.half_size;
        let max = self.half_size;

        let mut t_min = -f32::INFINITY;
        let mut t_max = f32::INFINITY;

        if !update_slab(
            local_origin.x,
            local_direction.x,
            min,
            max,
            &mut t_min,
            &mut t_max,
        ) {
            return None;
        }

        if !update_slab(
            local_origin.y,
            local_direction.y,
            min,
            max,
            &mut t_min,
            &mut t_max,
        ) {
            return None;
        }

        if !update_slab(
            local_origin.z,
            local_direction.z,
            min,
            max,
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

    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        let relative = *point - self.center;

        let local = Vec3::new(
            relative.dot(&self.right),
            relative.dot(&self.up),
            relative.dot(&self.forward),
        );

        let ax = local.x.abs();
        let ay = local.y.abs();
        let az = local.z.abs();

        if ax >= ay && ax >= az {
            (self.right * local.x.signum()).normalize()
        } else if ay >= ax && ay >= az {
            (self.up * local.y.signum()).normalize()
        } else {
            (self.forward * local.z.signum()).normalize()
        }
    }

    pub fn min(&self) -> Vec3 {
        self.aabb().0
    }

    pub fn max(&self) -> Vec3 {
        self.aabb().1
    }

    pub fn aabb(&self) -> (Vec3, Vec3) {
        let ex = self.right.x.abs() * self.half_size
            + self.up.x.abs() * self.half_size
            + self.forward.x.abs() * self.half_size;

        let ey = self.right.y.abs() * self.half_size
            + self.up.y.abs() * self.half_size
            + self.forward.y.abs() * self.half_size;

        let ez = self.right.z.abs() * self.half_size
            + self.up.z.abs() * self.half_size
            + self.forward.z.abs() * self.half_size;

        let extent = Vec3::new(ex, ey, ez);

        (
            self.center - extent,
            self.center + extent,
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

    let inverse = 1.0 / direction;

    let mut t0 = (min - origin) * inverse;
    let mut t1 = (max - origin) * inverse;

    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }

    *t_min = t_min.max(t0);
    *t_max = t_max.min(t1);

    *t_max >= *t_min
}
