use crate::hittable::{Hittable, HittableList, Sphere};
use crate::material;
use crate::vec3::{Color, Point3};
use rand::prelude::*;
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let material_ground = Arc::new(material::Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut world = HittableList::new(vec![Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        material_ground,
    ))]);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).magnitude() > 0.9 {
                let (_sphere_material, obj): (Arc<dyn material::Material>, Arc<dyn Hittable>) =
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = crate::vec3::mul_elemwise(
                            &crate::vec3::random_in_unit_sphere(),
                            &crate::vec3::random_in_unit_sphere(),
                        );
                        let mat = Arc::new(material::Lambertian::new(albedo));
                        let sph = Arc::new(Sphere::new(center, 0.2, mat.clone()));
                        (mat, sph)
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = crate::vec3::random_in_unit_sphere();
                        let fuzz: f64 = rng.gen_range(0.5..1.);
                        let mat = Arc::new(material::Metal::new(albedo, fuzz));
                        let sph = Arc::new(Sphere::new(center, 0.2, mat.clone()));
                        (mat, sph)
                    } else {
                        // glass
                        let mat = Arc::new(material::Dielectric::new(1.5));
                        let sph = Arc::new(Sphere::new(center, 0.2, mat.clone()));
                        (mat, sph)
                    };
                world.add(obj);
            }
        }
    }

    let m1 = Arc::new(material::Dielectric::new(1.5));
    let m2 = Arc::new(material::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let m3 = Arc::new(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.));

    world.add(Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1., m1)));

    world.add(Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., m2)));

    world.add(Arc::new(Sphere::new(Point3::new(0.7, 0.6, 0.5), 1., m3)));

    world
}
