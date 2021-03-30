use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

pub trait Material: Send + Sync {
    // -> scattered ray, attenuation
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
