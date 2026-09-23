use crate::acceleration::bvh::Bvh;
use crate::objects::object::Object;

pub struct Scene {
    pub objects: Vec<Object>,
    pub bvh: Bvh,
}

impl Scene {
    pub fn new(
        objects: Vec<Object>,
    ) -> Self {
        let bvh =
            Bvh::build(
                &objects,
            );

        Self {
            objects,
            bvh,
        }
    }

    pub fn objects(
        &self,
    ) -> &[Object] {
        &self.objects
    }
}