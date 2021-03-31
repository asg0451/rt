use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::Color;

use std::sync::Arc;

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
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

        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.p());
        Some((Ray::new(rec.p(), scatter_direction), attenuation))
    }
}
