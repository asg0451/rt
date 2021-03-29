use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Unit};
use rand::Rng;

pub struct Dielectric {
    ir: f64, // index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // schlick's approximation
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face() {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Unit::new_normalize(r_in.direction());
        let cos_theta = (-unit_direction.as_ref())
            .dot(rec.normal().as_ref())
            .min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let rand = rand::thread_rng().gen();
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand {
            crate::vec3::reflect(&unit_direction, &rec.normal())
        } else {
            crate::vec3::refract(&unit_direction, &rec.normal(), refraction_ratio)
        };

        let scattered = Ray::new(rec.p(), direction);
        Some((scattered, attenuation))
    }
}
