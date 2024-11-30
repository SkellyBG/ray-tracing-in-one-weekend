use super::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) {
        let r = self.x;
        let g = self.y;
        let b = self.z;

        let ir = (255.999 * r).floor() as i32;
        let ig = (255.999 * g).floor() as i32;
        let ib = (255.999 * b).floor() as i32;

        println!("{} {} {}", ir, ig, ib);
    }
}
