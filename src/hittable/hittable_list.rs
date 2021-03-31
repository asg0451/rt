use crate::hittable::{aabb::Aabb, HitRecord, Hittable};
use crate::ray::Ray;

use std::convert::AsRef;
use std::sync::Arc;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn into_inner(self) -> Vec<Arc<dyn Hittable>> {
        self.objects
    }
}

impl AsRef<[Arc<dyn Hittable>]> for HittableList {
    fn as_ref(&self) -> &[Arc<dyn Hittable>] {
        &self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;

        for o in self.objects.iter() {
            let max = closest.as_ref().map_or(t_max, |hr| hr.t());
            if let shr @ Some(_) = o.hit(r, t_min, max) {
                closest = shr;
            }
        }
        closest
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        let mut temp: Option<Aabb> = None;
        for o in self.objects.iter() {
            if let Some(bb) = o.bounding_box() {
                temp = Some(if let Some(temp) = temp {
                    temp.surrounding_box(&bb)
                } else {
                    bb
                });
            } else {
                return None;
            }
        }
        temp
    }
}
