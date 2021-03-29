use crate::ray::Ray;
use crate::vec3::*;
use nalgebra::Unit;

mod hittable_list;
mod sphere;
pub use crate::hittable::hittable_list::HittableList;
pub use crate::hittable::sphere::Sphere;

#[derive(Debug)]
pub struct HitRecord {
    p: Point3,
    normal: Unit<Vec3>,
    t: f64,
    front_face: bool,
}

// normals are unit, and they point outwards
impl HitRecord {
    pub fn from_ray_outward_normal(
        p: Point3,
        t: f64,
        r: &Ray,
        mut outward_normal: Unit<Vec3>,
    ) -> Self {
        let front_face = r.direction().dot(outward_normal.as_mut_unchecked()) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            // TODO: this is kinda unwieldly..
            Unit::new_normalize(-outward_normal.into_inner())
        };
        Self {
            t,
            p,
            front_face,
            normal,
        }
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn normal(&self) -> Unit<Vec3> {
        self.normal
    }
    pub fn p(&self) -> Point3 {
        self.p
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
