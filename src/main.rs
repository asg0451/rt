// (lsp-rust-analyzer-inlay-hints-mode -1)

mod hittable;
mod ray;
mod vec3;

use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::*;

fn print_color(c: &Color) {
    let ir = (255.999 * c.x) as i64;
    let ig = (255.999 * c.y) as i64;
    let ib = (255.999 * c.z) as i64;
    println!("{} {} {}", ir, ig, ib);
}

// for testing; pretty gradient
// linearly blends white and blue depending on height of y coord after scaling the ray to unit
fn ray_color(r: &Ray, world: &impl hittable::Hittable) -> Color {
    use nalgebra::Unit;

    if let Some(rec) = world.hit(r, 0., std::f64::INFINITY) {
        return 0.5 * (rec.normal().clone().into_inner() + Color::new(1., 1., 1.));
    }
    let unit_direction = Unit::new_normalize(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // eye is at 0,0,0; y is up, x is right, z is into the screen
    // traverse the screen from upper left, use 2 offset vercors along the sides to move the ray endpoint across the screen

    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // world
    use std::rc::Rc;
    let world = HittableList::new(vec![
        Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)),
        Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)),
    ]);
    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..=(image_height - 1)).rev() {
        eprintln!("\rscanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(&r, &world);
            print_color(&color);
        }
    }
    eprintln!("done")
}
