use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Hemisphere {
    pub center: Vec3,
    pub radius: f32,
    pub normal: Vec3,

    pub material: Material,
    pub flat_material: Material,
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
            flat_material: material,
        }
    }

    pub fn new_with_materials(
        center: Vec3,
        radius: f32,
        normal: Vec3,
        curved_material: Material,
        flat_material: Material,
    ) -> Self {
        Self {
            center,
            radius,
            normal: normal.normalize(),

            material: curved_material,
            flat_material,
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        let oc =
            *origin - self.center;

        let a =
            direction.dot(direction);

        let b =
            2.0 * oc.dot(direction);

        let c =
            oc.dot(&oc)
                - self.radius * self.radius;

        let discriminant =
            b * b
                - 4.0 * a * c;

        let mut best_t =
            f32::INFINITY;

        if discriminant >= 0.0 {
            let sqrt_d =
                discriminant.sqrt();

            let t1 =
                (-b - sqrt_d)
                    / (2.0 * a);

            let t2 =
                (-b + sqrt_d)
                    / (2.0 * a);

            for t in [
                t1,
                t2,
            ] {
                if t > 0.001 {
                    let hit_point =
                        *origin
                            + *direction * t;

                    let local =
                        hit_point
                            - self.center;

                    let side =
                        local.dot(
                            &self.normal,
                        );

                    if side >= -0.0001
                        && t < best_t
                    {
                        best_t = t;
                    }
                }
            }
        }

        let denom =
            direction.dot(
                &self.normal,
            );

        if denom.abs() > 0.000001 {
            let t =
                (
                    self.center
                        - *origin
                )
                    .dot(
                        &self.normal,
                    )
                    / denom;

            if t > 0.001 {
                let hit_point =
                    *origin
                        + *direction * t;

                let local =
                    hit_point
                        - self.center;

                let projected =
                    local
                        - self.normal
                            * local.dot(
                                &self.normal,
                            );

                if projected.length()
                    <= self.radius
                    && t < best_t
                {
                    best_t = t;
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
        if self.is_flat_face(point) {
            -self.normal
        } else {
            (
                *point
                    - self.center
            )
                .normalize()
        }
    }

    pub fn material_at(
        &self,
        point: &Vec3,
    ) -> Material {
        if self.is_flat_face(point) {
            self.flat_material
        } else {
            self.material
        }
    }

    pub fn is_flat_face(
        &self,
        point: &Vec3,
    ) -> bool {
        let local =
            *point
                - self.center;

        let distance_to_plane =
            local
                .dot(
                    &self.normal,
                )
                .abs();

        let projected =
            local
                - self.normal
                    * local.dot(
                        &self.normal,
                    );

        distance_to_plane < 0.006
            && projected.length()
                <= self.radius + 0.002
    }
}