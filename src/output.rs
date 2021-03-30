use crate::vec3::Color;
use anyhow::Result;
use image::{Rgb, RgbImage};
use std::path::{Path, PathBuf};

pub fn write_color<W: std::io::Write>(w: &mut W, c: &Color, samples_per_pixel: usize) {
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

pub struct ImageOutput {
    img: RgbImage,
    filename: PathBuf,
    samples_per_pixel: u32,
}

impl ImageOutput {
    pub fn new<P: AsRef<Path>>(
        width: u32,
        height: u32,
        samples_per_pixel: u32,
        filename: P,
    ) -> Self {
        Self {
            img: RgbImage::new(width, height),
            filename: filename.as_ref().to_owned(),
            samples_per_pixel,
        }
    }

    pub fn save(&mut self) -> Result<()> {
        self.img.save(&self.filename)?;
        Ok(())
    }

    pub fn put_pixel_color(&mut self, x: u32, y: u32, c: Color) {
        let mut r = c.x;
        let mut g = c.y;
        let mut b = c.z;

        // divide color by numver of samples and gamma-correct for gamma=2.0
        let scale = 1. / self.samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let r = (256. * r.clamp(0., 0.999)) as u8;
        let g = (256. * g.clamp(0., 0.999)) as u8;
        let b = (256. * b.clamp(0., 0.999)) as u8;

        let rgb = Rgb([r, g, b]);
        self.img.put_pixel(x, y, rgb);
    }
}
