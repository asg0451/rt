pub use nalgebra::Unit;
use nalgebra::Vector3;

pub type Color = Vector3<f64>;
pub type Vec3 = Vector3<f64>;
pub type Point3 = Vector3<f64>;

pub fn random_in_unit_sphere() -> Vec3 {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );
        if p.magnitude_squared() >= 1. {
            continue;
        }
        return p;
    }
}

// use either of these as diffusion renderer in ray_color fn
pub fn random_unit() -> Unit<Vec3> {
    Unit::new_normalize(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: &Unit<Vec3>) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.);
        if p.magnitude_squared() >= 1. {
            continue;
        }
        return p;
    }
}

// TODO: newtype wrapper to implement these?
pub fn near_zero(v: &Vec3) -> bool {
    let s = 1e-8;
    v.x < s && v.y < s && v.z < s
}

pub fn reflect(v: &Vec3, normal: &Unit<Vec3>) -> Vec3 {
    v - 2. * v.dot(&normal) * normal.as_ref()
}

pub fn refract(uv: &Vec3, normal: &Unit<Vec3>, eta_ratio: f64) -> Vec3 {
    let cos_theta: f64 = (-uv.dot(normal)).min(1.);
    let r_out_perp = eta_ratio * (uv + normal.as_ref() * cos_theta);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * normal.as_ref();
    r_out_perp + r_out_parallel
}

pub fn mul_elemwise(me: &Vec3, other: &Vec3) -> Vec3 {
    Vec3::new(me.x * other.x, me.y * other.y, me.z * other.z)
}
