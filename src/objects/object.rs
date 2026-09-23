use crate::core::vec3::Vec3;
use crate::materials::material::Material;

use crate::objects::cone::Cone;
use crate::objects::cylinder::Cylinder;
use crate::objects::plane::Plane;
use crate::objects::sphere::Sphere;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Cylinder(Cylinder),
    Cone(Cone),
}

impl Object {
    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        match self {
            Object::Sphere(sphere) => {
                sphere.intersect(
                    origin,
                    direction,
                )
            }

            Object::Plane(plane) => {
                plane.intersect(
                    origin,
                    direction,
                )
            }

            Object::Cylinder(cylinder) => {
                cylinder.intersect(
                    origin,
                    direction,
                )
            }

            Object::Cone(cone) => {
                cone.intersect(
                    origin,
                    direction,
                )
            }
        }
    }

    pub fn normal_at(
        &self,
        point: &Vec3,
    ) -> Vec3 {
        match self {
            Object::Sphere(sphere) => {
                sphere.normal_at(point)
            }

            Object::Plane(plane) => {
                plane.normal_at()
            }

            Object::Cylinder(cylinder) => {
                cylinder.normal_at(point)
            }

            Object::Cone(cone) => {
                cone.normal_at(point)
            }
        }
    }

    pub fn material(&self) -> Material {
        match self {
            Object::Sphere(sphere) => {
                sphere.material
            }

            Object::Plane(plane) => {
                plane.material
            }

            Object::Cylinder(cylinder) => {
                cylinder.material
            }

            Object::Cone(cone) => {
                cone.material
            }
        }
    }
}