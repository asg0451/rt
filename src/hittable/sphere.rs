use crate::hittable::{aabb::Aabb, HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;

use nalgebra::Unit;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    // p = point on sphere of radius 1, centered at origin
    // v,u in [0,1]
    fn uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;
        let u = phi / (2. * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().magnitude_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // find nearest root in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let rec_p = r.at(root);
        let outward_normal = Unit::new_normalize((rec_p - self.center) / self.radius);
        let (u, v) = Sphere::uv(&outward_normal);
        let rec = HitRecord::from_ray_outward_normal_material(
            rec_p,
            root,
            r,
            outward_normal,
            u,
            v,
            Arc::clone(&self.material),
        );
        Some(rec)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
