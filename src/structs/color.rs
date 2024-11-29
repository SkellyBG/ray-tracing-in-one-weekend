use super::vec3::Vec3;

pub struct Color(Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::from((r, g, b)))
    }

    pub fn write_color(&self) {
        let r = self.0.x;
        let g = self.0.y;
        let b = self.0.z;

        let ir = (255.999 * r).floor() as i32;
        let ig = (255.999 * g).floor() as i32;
        let ib = (255.999 * b).floor() as i32;

        println!("{} {} {}", ir, ig, ib);
    }
}
