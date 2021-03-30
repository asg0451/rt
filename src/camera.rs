use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Unit<Vec3>,
    v: Unit<Vec3>,
    w: Unit<Vec3>,
    lens_radius: f64,
}

impl Camera {
    // vertical fov in degrees
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = Unit::new_normalize(lookfrom - lookat);
        let u = Unit::new_normalize(vup.cross(w.as_ref()));
        let v = Unit::new_normalize(w.cross(u.as_ref()));

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u.as_ref();
        let vertical: Vec3 = focus_dist * viewport_height * v.as_ref();
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_dist * w.as_ref();

        let lens_radius = aperture / 2.;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            w,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * crate::vec3::random_in_unit_disk();
        let offset = rd.x * self.u.as_ref() + rd.y * self.v.as_ref();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
