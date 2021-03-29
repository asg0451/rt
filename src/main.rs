// (lsp-rust-analyzer-inlay-hints-mode -1)

mod camera;
mod hittable;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::*;

fn write_color<W: std::io::Write>(w: &mut W, c: &Color, samples_per_pixel: usize) {
    let mut r = c.x;
    let mut g = c.y;
    let mut b = c.z;

    let scale = 1. / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;
    let r = (256. * r.clamp(0., 0.999)) as u64;
    let g = (256. * g.clamp(0., 0.999)) as u64;
    let b = (256. * b.clamp(0., 0.999)) as u64;

    w.write_all(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("error printing color");
    // println!("{} {} {}", ir, ig, ib);
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
    let samples_per_pixel = 100;

    // world
    use std::rc::Rc;
    let world = HittableList::new(vec![
        Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)),
        Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)),
    ]);

    // camera
    let camera = Camera::new();

    // render
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..=(image_height - 1)).rev() {
        eprintln!("\rscanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world);
            }
            write_color(&mut std::io::stdout(), &color, samples_per_pixel);
        }
    }
    eprintln!("done")
}
