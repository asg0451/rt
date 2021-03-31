use crate::hittable::{aabb::Aabb, hittable_list::HittableList, HitRecord, Hittable};
use crate::ray::Ray;

use rand::Rng;

use std::sync::Arc;

// bounding volume hierarchy
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>, bbox: Aabb) -> Self {
        Self { left, right, bbox }
    }

    // randomly choose an axis, sort the primitives, put half in each subtree
    pub fn from_hittable_list(list: HittableList) -> Self {
        let axis = rand::thread_rng().gen_range(0..2);
        let mut objects = list.into_inner();

        assert!(!objects.is_empty());

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = if objects.len() == 1 {
            (objects[0].clone(), objects[0].clone())
        } else {
            objects.sort_unstable_by(|a, b| {
                // idk what we do here if no bbs like eg an infinite plane
                // TODO: this constraint should be in the type system
                let a_key = a.bounding_box().unwrap().min()[axis];
                let b_key = b.bounding_box().unwrap().min()[axis];
                a_key.partial_cmp(&b_key).unwrap()
            });

            let (l, r) = objects.split_at(objects.len());
            (
                Arc::new(BvhNode::from_hittable_list(HittableList::new(l.to_owned()))),
                Arc::new(BvhNode::from_hittable_list(HittableList::new(r.to_owned()))),
            )
        };

        let bbox = left
            .bounding_box()
            .unwrap()
            .surrounding_box(&right.bounding_box().unwrap());
        Self { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        todo!()
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}
