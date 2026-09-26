use crate::acceleration::aabb::Aabb;
use crate::core::vec3::Vec3;
use crate::materials::material::Material;

use crate::objects::cone::Cone;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::plane::Plane;
use crate::objects::sphere::Sphere;
use crate::objects::torus::Torus;
use crate::objects::ellipsoid::Ellipsoid;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Cylinder(Cylinder),
    Cone(Cone),
    Cube(Cube),
    Hemisphere(Hemisphere),
    Torus(Torus),
    Ellipsoid(Ellipsoid),

}

impl Object {
    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
    ) -> Option<f32> {
        match self {
            Object::Sphere(
                sphere,
            ) => {
                sphere.intersect(
                    origin,
                    direction,
                )
            }

            Object::Plane(
                plane,
            ) => {
                plane.intersect(
                    origin,
                    direction,
                )
            }

            Object::Cylinder(
                cylinder,
            ) => {
                cylinder.intersect(
                    origin,
                    direction,
                )
            }

            Object::Cone(
                cone,
            ) => {
                cone.intersect(
                    origin,
                    direction,
                )
            }

            Object::Cube(
                cube,
            ) => {
                cube.intersect(
                    origin,
                    direction,
                )
            }

            Object::Hemisphere(
                hemisphere,
            ) => {
                hemisphere.intersect(
                    origin,
                    direction,
                )
            }

            Object::Torus(
                torus,
            ) => {
                torus.intersect(
                    origin,
                    direction,
                )
            }

            Object::Torus(torus) => {
                torus.intersect(
                    origin,
                    direction,
                )
            }

            Object::Ellipsoid(ellipsoid) => {
                ellipsoid.intersect(
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
            Object::Sphere(
                sphere,
            ) => {
                sphere.normal_at(
                    point,
                )
            }

            Object::Plane(
                plane,
            ) => {
                plane.normal_at()
            }

            Object::Cylinder(
                cylinder,
            ) => {
                cylinder.normal_at(
                    point,
                )
            }

            Object::Cone(
                cone,
            ) => {
                cone.normal_at(
                    point,
                )
            }

            Object::Cube(
                cube,
            ) => {
                cube.normal_at(
                    point,
                )
            }

            Object::Hemisphere(
                hemisphere,
            ) => {
                hemisphere.normal_at(
                    point,
                )
            }

            Object::Torus(
                torus,
            ) => {
                torus.normal_at(
                    point,
                )
            }

            Object::Ellipsoid(ellipsoid) => {
                ellipsoid.normal_at(
                    point,
                )
            }
        }
    }

    pub fn material(
        &self,
    ) -> Material {
        match self {
            Object::Sphere(
                sphere,
            ) => {
                sphere.material
            }

            Object::Plane(
                plane,
            ) => {
                plane.material
            }

            Object::Cylinder(
                cylinder,
            ) => {
                cylinder.material
            }

            Object::Cone(
                cone,
            ) => {
                cone.material
            }

            Object::Cube(
                cube,
            ) => {
                cube.material
            }

            Object::Hemisphere(
                hemisphere,
            ) => {
                hemisphere.material
            }

            Object::Torus(
                torus,
            ) => {
                torus.material
            }

            Object::Ellipsoid(ellipsoid) => {
                ellipsoid.material
            }
        }
    }

    pub fn material_at(
        &self,
        point: &Vec3,
    ) -> Material {
        match self {
            Object::Sphere(
                sphere,
            ) => {
                sphere.material
            }

            Object::Plane(
                plane,
            ) => {
                plane.material
            }

            Object::Cylinder(
                cylinder,
            ) => {
                cylinder.material
            }

            Object::Cone(
                cone,
            ) => {
                cone.material
            }

            Object::Cube(
                cube,
            ) => {
                cube.material
            }

            Object::Hemisphere(
                hemisphere,
            ) => {
                hemisphere.material_at(
                    point,
                )
            }

            Object::Torus(
                torus,
            ) => {
                torus.material
            }

            Object::Ellipsoid(ellipsoid) => {
                ellipsoid.material
            }
        }
    }

    pub fn bounding_box(
        &self,
    ) -> Option<Aabb> {
        match self {
            Object::Sphere(
                sphere,
            ) => {
                let radius =
                    Vec3::new(
                        sphere.radius,
                        sphere.radius,
                        sphere.radius,
                    );

                Some(
                    Aabb::new(
                        sphere.center
                            - radius,
                        sphere.center
                            + radius,
                    ),
                )
            }

            Object::Plane(
                _,
            ) => {
                None
            }

            Object::Cylinder(
                cylinder,
            ) => {
                let axis =
                    cylinder.axis
                        .normalize();

                let half_height =
                    cylinder.height
                        * 0.5;

                let axis_extent =
                    Vec3::new(
                        axis.x.abs(),
                        axis.y.abs(),
                        axis.z.abs(),
                    )
                    * half_height;

                let radial_extent =
                    Vec3::new(
                        cylinder.radius,
                        cylinder.radius,
                        cylinder.radius,
                    );

                let extent =
                    axis_extent
                        + radial_extent;

                Some(
                    Aabb::new(
                        cylinder.center
                            - extent,
                        cylinder.center
                            + extent,
                    ),
                )
            }

            Object::Cone(
                cone,
            ) => {
                let axis =
                    cone.axis
                        .normalize();

                let half_height =
                    cone.height
                        * 0.5;

                let axis_extent =
                    Vec3::new(
                        axis.x.abs(),
                        axis.y.abs(),
                        axis.z.abs(),
                    )
                    * half_height;

                let radial_extent =
                    Vec3::new(
                        cone.radius,
                        cone.radius,
                        cone.radius,
                    );

                let extent =
                    axis_extent
                        + radial_extent;

                Some(
                    Aabb::new(
                        cone.center
                            - extent,
                        cone.center
                            + extent,
                    ),
                )
            }

            Object::Cube(cube) => {
                let (
                    min,
                    max,
                ) = cube.aabb();

                Some(
                    Aabb::new(
                        min,
                        max,
                    ),
                )
            }

            Object::Hemisphere(
                hemisphere,
            ) => {
                let radius =
                    Vec3::new(
                        hemisphere.radius,
                        hemisphere.radius,
                        hemisphere.radius,
                    );

                Some(
                    Aabb::new(
                        hemisphere.center
                            - radius,
                        hemisphere.center
                            + radius,
                    ),
                )
            }

            Object::Torus(
                torus,
            ) => {
                Some(
                    Aabb::new(
                        torus.min(),
                        torus.max(),
                    ),
                )
            }

            Object::Ellipsoid(ellipsoid) => {
                Some(
                    Aabb::new(
                        ellipsoid.min(),
                        ellipsoid.max(),
                    ),
                )
            }
        }
    }

    pub fn centroid(
        &self,
    ) -> Vec3 {
        match self {
            Object::Plane(
                plane,
            ) => {
                plane.point
            }

            _ => {
                if let Some(
                    bounds,
                ) = self.bounding_box()
                {
                    (
                        bounds.min
                            + bounds.max
                    )
                        * 0.5
                } else {
                    Vec3::new(
                        0.0,
                        0.0,
                        0.0,
                    )
                }
            }
        }
    }
}