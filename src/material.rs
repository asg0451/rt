use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Unit};

pub trait Material: Send + Sync {
    // -> scattered ray, attenuation
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction =
            rec.normal().clone().into_inner() + crate::vec3::random_unit().into_inner();

        // catch degenerate scatter direction
        if crate::vec3::near_zero(&scatter_direction) {
            scatter_direction = rec.normal().clone().into_inner();
        }

        Some((Ray::new(rec.p(), scatter_direction), self.albedo.clone()))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = crate::vec3::reflect(&Unit::new_normalize(r_in.direction()), &rec.normal());
        let scattered = Ray::new(
            rec.p(),
            reflected + self.fuzz * crate::vec3::random_in_unit_sphere(),
        );
        if scattered.direction().dot(&rec.normal()) > 0. {
            Some((scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}
