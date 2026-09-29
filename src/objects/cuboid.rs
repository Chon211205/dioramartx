use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Cuboid {
    pub center: Vec3,
    pub half_extents: Vec3,
    pub right_material: Material,
    pub left_material: Material,
    pub top_material: Material,
    pub bottom_material: Material,
    pub front_material: Material,
    pub back_material: Material,
    pub right: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
}

impl Cuboid {
    pub fn from_basis(
        center: Vec3,
        size: Vec3,
        right: Vec3,
        up: Vec3,
        forward: Vec3,
        material: Material,
    ) -> Self {
        Self::from_basis_faces(
            center, size, right, up, forward, material, material, material, material,
            material, material,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_basis_faces(
        center: Vec3,
        size: Vec3,
        right: Vec3,
        up: Vec3,
        forward: Vec3,
        right_material: Material,
        left_material: Material,
        top_material: Material,
        bottom_material: Material,
        front_material: Material,
        back_material: Material,
    ) -> Self {
        Self {
            center,
            half_extents: size * 0.5,
            right_material,
            left_material,
            top_material,
            bottom_material,
            front_material,
            back_material,
            right: right.normalize(),
            up: up.normalize(),
            forward: forward.normalize(),
        }
    }

    pub fn intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<f32> {
        let relative = *origin - self.center;
        let local_origin = Vec3::new(
            relative.dot(&self.right),
            relative.dot(&self.up),
            relative.dot(&self.forward),
        );
        let local_direction = Vec3::new(
            direction.dot(&self.right),
            direction.dot(&self.up),
            direction.dot(&self.forward),
        );
        let mut t_min = -f32::INFINITY;
        let mut t_max = f32::INFINITY;

        if !update_slab(local_origin.x, local_direction.x, -self.half_extents.x, self.half_extents.x, &mut t_min, &mut t_max)
            || !update_slab(local_origin.y, local_direction.y, -self.half_extents.y, self.half_extents.y, &mut t_min, &mut t_max)
            || !update_slab(local_origin.z, local_direction.z, -self.half_extents.z, self.half_extents.z, &mut t_min, &mut t_max)
        {
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

    fn local_point(&self, point: &Vec3) -> Vec3 {
        let relative = *point - self.center;
        Vec3::new(
            relative.dot(&self.right),
            relative.dot(&self.up),
            relative.dot(&self.forward),
        )
    }

    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        let local = self.local_point(point);
        let distances = Vec3::new(
            (local.x.abs() - self.half_extents.x).abs(),
            (local.y.abs() - self.half_extents.y).abs(),
            (local.z.abs() - self.half_extents.z).abs(),
        );
        if distances.x <= distances.y && distances.x <= distances.z {
            self.right * local.x.signum()
        } else if distances.y <= distances.z {
            self.up * local.y.signum()
        } else {
            self.forward * local.z.signum()
        }
    }

    pub fn material_at(&self, point: &Vec3) -> Material {
        let local = self.local_point(point);
        let distances = Vec3::new(
            (local.x.abs() - self.half_extents.x).abs(),
            (local.y.abs() - self.half_extents.y).abs(),
            (local.z.abs() - self.half_extents.z).abs(),
        );
        if distances.x <= distances.y && distances.x <= distances.z {
            if local.x >= 0.0 { self.right_material } else { self.left_material }
        } else if distances.y <= distances.z {
            if local.y >= 0.0 { self.top_material } else { self.bottom_material }
        } else if local.z >= 0.0 {
            self.front_material
        } else {
            self.back_material
        }
    }

    pub fn aabb(&self) -> (Vec3, Vec3) {
        let extent = Vec3::new(
            self.right.x.abs() * self.half_extents.x + self.up.x.abs() * self.half_extents.y + self.forward.x.abs() * self.half_extents.z,
            self.right.y.abs() * self.half_extents.x + self.up.y.abs() * self.half_extents.y + self.forward.y.abs() * self.half_extents.z,
            self.right.z.abs() * self.half_extents.x + self.up.z.abs() * self.half_extents.y + self.forward.z.abs() * self.half_extents.z,
        );
        (self.center - extent, self.center + extent)
    }
}

fn update_slab(origin: f32, direction: f32, min: f32, max: f32, t_min: &mut f32, t_max: &mut f32) -> bool {
    if direction.abs() < 1e-8 {
        return origin >= min && origin <= max;
    }
    let inverse = 1.0 / direction;
    let mut t0 = (min - origin) * inverse;
    let mut t1 = (max - origin) * inverse;
    if t0 > t1 { std::mem::swap(&mut t0, &mut t1); }
    *t_min = t_min.max(t0);
    *t_max = t_max.min(t1);
    *t_max >= *t_min
}
