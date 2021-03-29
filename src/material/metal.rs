use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Unit};

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
