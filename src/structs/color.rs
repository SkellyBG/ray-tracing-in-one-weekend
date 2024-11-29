use std::ops::{Add, Mul};

pub struct Color((f64, f64, f64));

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self((r, g, b))
    }

    pub fn write_color(&self) {
        let r = self.0 .0;
        let g = self.0 .1;
        let b = self.0 .2;

        let ir = (255.999 * r).floor() as i32;
        let ig = (255.999 * g).floor() as i32;
        let ib = (255.999 * b).floor() as i32;

        println!("{} {} {}", ir, ig, ib);
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((
            self.0 .0 + rhs.0 .0,
            self.0 .1 + rhs.0 .1,
            self.0 .2 + rhs.0 .2,
        ))
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self((self.0 .0 * rhs, self.0 .1 * rhs, self.0 .2 * rhs))
    }
}
