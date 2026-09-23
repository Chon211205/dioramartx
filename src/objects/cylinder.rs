use crate::core::vec3::Vec3;
use crate::materials::material::Material;

pub struct Cylinder {
    pub center: Vec3,
    pub axis: Vec3,
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
        Self::new_oriented(
            center,
            Vec3::new(0.0, 1.0, 0.0),
            radius,
            height,
            material,
        )
    }

    pub fn new_oriented(
        center: Vec3,
        axis: Vec3,
        radius: f32,
        height: f32,
        material: Material,
    ) -> Self {
        Self {
            center,
            axis: axis.normalize(),
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
        let oc = *origin - self.center;

        let d_axis = direction.dot(&self.axis);
        let oc_axis = oc.dot(&self.axis);

        let d_perp =
            *direction - self.axis * d_axis;

        let oc_perp =
            oc - self.axis * oc_axis;

        let a = d_perp.dot(&d_perp);
        let b = 2.0 * oc_perp.dot(&d_perp);
        let c =
            oc_perp.dot(&oc_perp)
                - self.radius * self.radius;

        let half_height = self.height * 0.5;

        let mut closest = f32::INFINITY;

        if a.abs() > 0.0001 {
            let discriminant =
                b * b - 4.0 * a * c;

            if discriminant >= 0.0 {
                let sqrt_d =
                    discriminant.sqrt();

                let t1 =
                    (-b - sqrt_d)
                        / (2.0 * a);

                let t2 =
                    (-b + sqrt_d)
                        / (2.0 * a);

                for t in [t1, t2] {
                    if t > 0.001 {
                        let axial =
                            oc_axis
                                + d_axis * t;

                        if axial >= -half_height
                            && axial <= half_height
                            && t < closest
                        {
                            closest = t;
                        }
                    }
                }
            }
        }

        if d_axis.abs() > 0.0001 {
            for cap in [-half_height, half_height] {
                let t =
                    (cap - oc_axis)
                        / d_axis;

                if t > 0.001 {
                    let point =
                        oc
                            + *direction * t;

                    let axial =
                        point.dot(&self.axis);

                    let radial =
                        point
                            - self.axis * axial;

                    if radial.dot(&radial)
                        <= self.radius * self.radius
                        && t < closest
                    {
                        closest = t;
                    }
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
        let local =
            *point - self.center;

        let axial =
            local.dot(&self.axis);

        let half_height =
            self.height * 0.5;

        let epsilon = 0.002;

        if (axial - half_height).abs()
            < epsilon
        {
            return self.axis;
        }

        if (axial + half_height).abs()
            < epsilon
        {
            return -self.axis;
        }

        let radial =
            local
                - self.axis * axial;

        radial.normalize()
    }
}