use nalgebra::{Unit, Vector3};

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
