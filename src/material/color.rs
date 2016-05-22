use std::ops::{ Add, Mul };
use std::convert::Into;

use bmp::Pixel;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
     pub r: f64,
     pub g: f64,
     pub b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        debug_assert!(r <= 1.0 && g <= 1.0 && b <= 1.0, "All color channels must be less than 1.0");
        Color { r: r, g: g, b: b }
    }

    pub fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn white() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }

    pub fn gamma2(&self) -> Color {
        Color::new(self.r.sqrt(), self.g.sqrt(), self.b.sqrt())
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color::new(self.r * color.r, self.g * color.g, self.b * color.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, val: f64) -> Color {
        Color::new(self.r * val, self.g * val, self.b * val)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        color * self
    }
}


impl Add<Color> for Color {
    type Output = Color;

    fn add(self, color: Color) -> Color {
        Color::new(self.r + color.r, self.g + color.g, self.b + color.b)
    }
}

impl Into<Pixel> for Color {
    fn into(self) -> Pixel {
        Pixel {
            r: (255.99 * self.r) as u8,
            g: (255.99 * self.g) as u8,
            b: (255.99 * self.b) as u8,
        }
    }
}
