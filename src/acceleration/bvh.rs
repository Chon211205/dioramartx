use crate::acceleration::aabb::Aabb;
use crate::core::vec3::Vec3;
use crate::objects::object::Object;

const MAX_OBJECTS_PER_LEAF: usize = 4;

pub struct Bvh {
    root: Option<BvhNode>,
    unbounded_objects: Vec<usize>,
}

enum BvhNode {
    Leaf {
        bounds: Aabb,
        object_indices: Vec<usize>,
    },

    Branch {
        bounds: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl Bvh {
    pub fn build(
        objects: &[Object],
    ) -> Self {
        let mut bounded_objects =
            Vec::new();

        let mut unbounded_objects =
            Vec::new();

        for (
            index,
            object,
        ) in objects
            .iter()
            .enumerate()
        {
            if object
                .bounding_box()
                .is_some()
            {
                bounded_objects.push(
                    index,
                );
            } else {
                unbounded_objects.push(
                    index,
                );
            }
        }

        let root =
            if bounded_objects
                .is_empty()
            {
                None
            } else {
                Some(
                    BvhNode::build(
                        objects,
                        bounded_objects,
                    ),
                )
            };

        Self {
            root,
            unbounded_objects,
        }
    }

    pub fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
        objects: &[Object],
    ) -> Option<(usize, f32)> {
        let mut closest_distance =
            f32::INFINITY;

        let mut closest_index =
            None;

        if let Some(
            root,
        ) = &self.root
        {
            root.intersect(
                origin,
                direction,
                objects,
                &mut closest_distance,
                &mut closest_index,
            );
        }

        for &index in
            &self.unbounded_objects
        {
            if let Some(
                distance,
            ) =
                objects[index]
                    .intersect(
                        origin,
                        direction,
                    )
            {
                if distance
                    < closest_distance
                {
                    closest_distance =
                        distance;

                    closest_index =
                        Some(
                            index,
                        );
                }
            }
        }

        closest_index.map(
            |index| {
                (
                    index,
                    closest_distance,
                )
            },
        )
    }

    pub fn any_hit(
        &self,
        origin: &Vec3,
        direction: &Vec3,
        max_distance: f32,
        objects: &[Object],
    ) -> bool {
        if let Some(
            root,
        ) = &self.root
        {
            if root.any_hit(
                origin,
                direction,
                max_distance,
                objects,
            ) {
                return true;
            }
        }

        for &index in
            &self.unbounded_objects
        {
            if let Some(
                distance,
            ) =
                objects[index]
                    .intersect(
                        origin,
                        direction,
                    )
            {
                if distance > 0.001
                    && distance
                        < max_distance
                {
                    return true;
                }
            }
        }

        false
    }
}

impl BvhNode {
    fn build(
        objects: &[Object],
        mut indices: Vec<usize>,
    ) -> Self {
        let bounds =
            calculate_bounds(
                objects,
                &indices,
            );

        if indices.len()
            <= MAX_OBJECTS_PER_LEAF
        {
            return BvhNode::Leaf {
                bounds,
                object_indices: indices,
            };
        }

        let centroid_bounds =
            calculate_centroid_bounds(
                objects,
                &indices,
            );

        let axis =
            centroid_bounds
                .longest_axis();

        indices.sort_unstable_by(
            |a, b| {
                let centroid_a =
                    objects[*a]
                        .centroid();

                let centroid_b =
                    objects[*b]
                        .centroid();

                let value_a =
                    component(
                        &centroid_a,
                        axis,
                    );

                let value_b =
                    component(
                        &centroid_b,
                        axis,
                    );

                value_a
                    .partial_cmp(
                        &value_b,
                    )
                    .unwrap_or(
                        std::cmp::Ordering::Equal,
                    )
            },
        );

        let middle =
            indices.len()
                / 2;

        let right_indices =
            indices
                .split_off(
                    middle,
                );

        let left_indices =
            indices;

        if left_indices
            .is_empty()
            || right_indices
                .is_empty()
        {
            let mut all =
                left_indices;

            all.extend(
                right_indices,
            );

            return BvhNode::Leaf {
                bounds,
                object_indices: all,
            };
        }

        let left =
            BvhNode::build(
                objects,
                left_indices,
            );

        let right =
            BvhNode::build(
                objects,
                right_indices,
            );

        BvhNode::Branch {
            bounds,
            left: Box::new(
                left,
            ),
            right: Box::new(
                right,
            ),
        }
    }

    fn intersect(
        &self,
        origin: &Vec3,
        direction: &Vec3,
        objects: &[Object],
        closest_distance: &mut f32,
        closest_index: &mut Option<usize>,
    ) {
        match self {
            BvhNode::Leaf {
                bounds,
                object_indices,
            } => {
                if !bounds.intersect(
                    origin,
                    direction,
                    0.001,
                    *closest_distance,
                ) {
                    return;
                }

                for &index in
                    object_indices
                {
                    if let Some(
                        distance,
                    ) =
                        objects[index]
                            .intersect(
                                origin,
                                direction,
                            )
                    {
                        if distance > 0.001
                            && distance
                                < *closest_distance
                        {
                            *closest_distance =
                                distance;

                            *closest_index =
                                Some(
                                    index,
                                );
                        }
                    }
                }
            }

            BvhNode::Branch {
                bounds,
                left,
                right,
            } => {
                if !bounds.intersect(
                    origin,
                    direction,
                    0.001,
                    *closest_distance,
                ) {
                    return;
                }

                left.intersect(
                    origin,
                    direction,
                    objects,
                    closest_distance,
                    closest_index,
                );

                right.intersect(
                    origin,
                    direction,
                    objects,
                    closest_distance,
                    closest_index,
                );
            }
        }
    }

    fn any_hit(
        &self,
        origin: &Vec3,
        direction: &Vec3,
        max_distance: f32,
        objects: &[Object],
    ) -> bool {
        match self {
            BvhNode::Leaf {
                bounds,
                object_indices,
            } => {
                if !bounds.intersect(
                    origin,
                    direction,
                    0.001,
                    max_distance,
                ) {
                    return false;
                }

                for &index in
                    object_indices
                {
                    if let Some(
                        distance,
                    ) =
                        objects[index]
                            .intersect(
                                origin,
                                direction,
                            )
                    {
                        if distance > 0.001
                            && distance
                                < max_distance
                        {
                            return true;
                        }
                    }
                }

                false
            }

            BvhNode::Branch {
                bounds,
                left,
                right,
            } => {
                if !bounds.intersect(
                    origin,
                    direction,
                    0.001,
                    max_distance,
                ) {
                    return false;
                }

                if left.any_hit(
                    origin,
                    direction,
                    max_distance,
                    objects,
                ) {
                    return true;
                }

                right.any_hit(
                    origin,
                    direction,
                    max_distance,
                    objects,
                )
            }
        }
    }
}

fn calculate_bounds(
    objects: &[Object],
    indices: &[usize],
) -> Aabb {
    let mut bounds =
        Aabb::empty();

    for &index in indices {
        if let Some(
            object_bounds,
        ) =
            objects[index]
                .bounding_box()
        {
            bounds =
                bounds.union(
                    &object_bounds,
                );
        }
    }

    bounds
}

fn calculate_centroid_bounds(
    objects: &[Object],
    indices: &[usize],
) -> Aabb {
    let mut bounds =
        Aabb::empty();

    for &index in indices {
        bounds.expand_point(
            objects[index]
                .centroid(),
        );
    }

    bounds
}

fn component(
    vector: &Vec3,
    axis: usize,
) -> f32 {
    match axis {
        0 => {
            vector.x
        }

        1 => {
            vector.y
        }

        _ => {
            vector.z
        }
    }
}