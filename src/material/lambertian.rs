use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction =
            rec.normal().clone().into_inner() + crate::vec3::random_unit().into_inner();

        // catch degenerate scatter direction
        if crate::vec3::near_zero(&scatter_direction) {
            scatter_direction = rec.normal().clone().into_inner();
        }

        Some((Ray::new(rec.p(), scatter_direction), self.albedo))
    }
}
