// (lsp-rust-analyzer-inlay-hints-mode -1)

mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::{Color, Point3, Vec3};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt[name = "rt"]]
struct Opt {
    #[structopt(short, long, default_value = "400")]
    width: usize,
    #[structopt(short, long, default_value = "100")]
    samples_per_pixel: usize,
}

// https://plasma-umass.org/coz/
// https://github.com/plasma-umass/coz/tree/master/rust

fn write_color<W: std::io::Write>(w: &mut W, c: &Color, samples_per_pixel: usize) {
    let mut r = c.x;
    let mut g = c.y;
    let mut b = c.z;

    // divide color by numver of samples and gamma-correct for gamma=2.0
    let scale = 1. / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let r = (256. * r.clamp(0., 0.999)) as u64;
    let g = (256. * g.clamp(0., 0.999)) as u64;
    let b = (256. * b.clamp(0., 0.999)) as u64;

    w.write_all(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("error printing color");
    // println!("{} {} {}", ir, ig, ib);
}

// for testing; pretty gradient
// linearly blends white and blue depending on height of y coord after scaling the ray to unit
fn ray_color(r: &Ray, world: &impl hittable::Hittable, depth: usize) -> Color {
    use nalgebra::Unit;

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) {
        // recurse for diffusion / ray bouncing
        if let Some((scattered, attenuation)) = rec.material().scatter(&r, &rec) {
            let new_color = ray_color(&scattered, world, depth - 1);
            // should be attenuation * ray_color(&scattered, world, depth - 1); where * is elementwise. is there a way to do this in nalg
            // https://docs.rs/nalgebra/0.25.3/nalgebra/base/struct.Matrix.html#method.mul
            // element-wise multiplication? transpose?
            let multpld = Vec3::new(
                new_color.x * attenuation.x,
                new_color.y * attenuation.y,
                new_color.z * attenuation.z,
            );
            return multpld;
        }
        return Color::new(0., 0., 0.);
    }
    let unit_direction = Unit::new_normalize(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // coz doesnt work now that i have rayon...
    coz::thread_init();

    let opt = Opt::from_args();

    // eye is at 0,0,0; y is up, x is right, z is into the screen
    // traverse the screen from upper left, use 2 offset vercors along the sides to move the ray endpoint across the screen

    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = opt.width; // 3840
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = opt.samples_per_pixel;
    let max_depth = 50; // max ray bounces

    // world
    use std::sync::Arc;

    let material_ground = Arc::new(material::Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(material::Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(material::Dielectric::new(1.5));
    let material_right = Arc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.));

    // https://stackoverflow.com/questions/63893847/error-when-passing-rcdyn-trait-as-a-function-argument

    let objs: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground.clone(),
        )),
        Arc::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            material_center.clone(),
        )),
        Arc::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        )),
        Arc::new(Sphere::new(
            Point3::new(-1., 0., -1.),
            -0.4, // negative radius -> surface points inward -> hollow sphere
            material_left.clone(),
        )),
        Arc::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right.clone(),
        )),
    ];
    let world = HittableList::new(objs);

    // camera
    let camera = Camera::new();

    // render
    use rand::prelude::*;

    print!("P3\n{} {}\n255\n", image_width, image_height);
    let lines = (0..=(image_height - 1)).rev().collect::<Vec<_>>();
    use rayon::prelude::*;

    let (tx, rx) = std::sync::mpsc::channel::<(i64, Vec<u8>)>();

    let reader_thread = std::thread::spawn(move || {
        let mut buf: Box<Vec<_>> = Box::new(Vec::with_capacity(
            image_width as usize * image_height as usize * 10,
        ));
        loop {
            coz::scope!("reading channel");
            if let Some(res) = rx.recv().ok() {
                buf.push(res);
            } else {
                break;
            }
        }
        buf.sort_unstable_by_key(|&(idx, _)| -idx);
        coz::begin!("flushing channel");
        for (_i, l) in buf.into_iter() {
            print!(
                "{}",
                std::str::from_utf8(&l).expect("failed to restringify")
            );
        }
        coz::end!("flushing channel");
    });

    let ctr = std::sync::atomic::AtomicUsize::new(0);

    lines.par_iter().for_each_with(tx, |tx, &j| {
        coz::scope!("scanline");
        let mut buf = Vec::with_capacity(16);
        let mut rng = rand::thread_rng();
        if j % 50 == 0 {
            let count = ctr.fetch_add(50, std::sync::atomic::Ordering::Relaxed);
            eprint!("\rscanlines remaining: {}", image_height as usize - count);
        }
        for i in 0..image_width {
            let mut color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world, max_depth);
            }
            write_color(&mut buf, &color, samples_per_pixel);
        }
        tx.send((j, buf)).expect("failed to send line");
    });

    reader_thread.join().expect("Failed to join reader thread");
    eprintln!("\ndone");
}
