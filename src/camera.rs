use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    // vertical fov in degrees
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = Unit::new_normalize(lookfrom - lookat);
        let u = Unit::new_normalize(vup.cross(w.as_ref()));
        let v: Vec3 = w.cross(u.as_ref());

        let origin = lookfrom;
        let horizontal = viewport_width * u.as_ref();
        let vertical: Vec3 = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w.as_ref();

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
