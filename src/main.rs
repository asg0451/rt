// (lsp-rust-analyzer-inlay-hints-mode -1)

mod camera;
mod hittable;
mod material;
mod output;
mod random_scene;
mod ray;
mod vec3;

use std::sync::mpsc::{channel, Sender};

use camera::Camera;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt[name = "rt"]]
struct Opt {
    #[structopt(short, long, default_value = "1200")] // 1200
    width: u32,
    #[structopt(short, long, default_value = "500")] // 500
    samples_per_pixel: u32,
    #[structopt(short, long)]
    no_use_rayon: bool,
}

// https://plasma-umass.org/coz/
// https://github.com/plasma-umass/coz/tree/master/rust

// for testing; pretty gradient
// linearly blends white and blue depending on height of y coord after scaling the ray to unit
fn ray_color(r: &Ray, world: &impl hittable::Hittable, depth: i64) -> Color {
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
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = opt.samples_per_pixel;
    let max_depth = 50; // max ray bounces

    // world
    let world = random_scene::random_scene();

    // camera
    // depth of fieldx
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // render
    let (mut tx, rx) = channel::<(u32, u32, Color)>(); // is this usage of channel too expensive?
    let reader_thread = std::thread::spawn(move || {
        let mut buf =
            output::ImageOutput::new(image_width, image_height, samples_per_pixel, "out.png");
        while let Ok((x, y, color)) = rx.recv() {
            buf.put_pixel_color(x, y, color);
        }
        buf.save().expect("failed to save image");
    });

    let bar = ProgressBar::new(image_height.into()).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    let render_line = |tx: &mut Sender<(u32, u32, Color)>, &j| {
        coz::scope!("scanline");
        let mut rng = rand::thread_rng();
        for i in 0..image_width {
            let mut color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world, max_depth);
            }
            // image was upside down for some reason..
            tx.send((i, image_height - j - 1, color)).unwrap();
        }
        bar.inc(1);
    };

    let lines = (0..=(image_height - 1)).collect::<Vec<_>>();
    if opt.no_use_rayon {
        eprintln!("running sans rayon");
        lines.iter().for_each(|j| render_line(&mut tx, j));
    } else {
        lines.par_iter().for_each_with(tx.clone(), render_line);
    }

    bar.finish();
    drop(tx);

    reader_thread.join().expect("failed to join reader thread");
    eprintln!("\ndone");
}
