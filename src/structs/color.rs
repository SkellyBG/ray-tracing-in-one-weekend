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
