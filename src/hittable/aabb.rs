use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;

// axis-aligned bounding box

#[derive(Clone)]
pub struct Aabb {
    max: Point3,
    min: Point3,
}

impl Aabb {
    pub fn new(max: Point3, min: Point3) -> Self {
        Self { max, min }
    }

    /// Get a reference to the aabb's max.
    pub fn max(&self) -> &Point3 {
        &self.max
    }

    /// Get a reference to the aabb's min.
    pub fn min(&self) -> &Point3 {
        &self.min
    }

    // TODO: this is not the same as Hittable::hit. why not? its a bit ugly
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1. / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;
            if inv_d < 0. {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(&self, other: &Aabb) -> Aabb {
        let small = Point3::new(
            self.min().x.min(other.min().x),
            self.min().y.min(other.min().y),
            self.min().z.min(other.min().z),
        );

        let big = Point3::new(
            self.max().x.max(other.max().x),
            self.max().y.max(other.max().y),
            self.max().z.max(other.max().z),
        );

        Aabb::new(small, big)
    }
}
