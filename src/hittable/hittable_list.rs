use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

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
}
