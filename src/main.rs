const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

mod structs;

use structs::color::Color;

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {} ", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0_f64;

            let pixel_color = Color::new(r, g, b);

            pixel_color.write_color();
        }
    }

    eprintln!("\rDone.")
}
