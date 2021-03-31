use crate::texture::{SolidColor, Texture};
use crate::vec3::{Color, Point3, Vec3};

use std::sync::Arc;

pub struct Checker {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
    pub fn from_colors(odd: Color, even: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
