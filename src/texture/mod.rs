use crate::vec3::{Color, Point3, Vec3};

mod checker;
mod solid_color;

pub use crate::texture::checker::Checker;
pub use crate::texture::solid_color::SolidColor;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
