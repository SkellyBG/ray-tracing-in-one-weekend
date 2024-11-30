const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = {
    let height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    if height < 1 {
        1
    } else {
        height
    }
};

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

const FOCAL_LENGTH: f64 = 1.0;

mod structs;

use structs::{color::Color, point3::Point3, ray::Ray, vec3::Vec3};

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();

    let a = 0.5 * (unit_direction.y + 1.0);

    Color::from((1.0, 1.0, 1.0)) * (1.0 - a) + Color::from((0.5, 0.7, 1.0)) * a
}

fn main() {
    let camera_center = Point3::new();

    let viewport_u = Vec3::from((VIEWPORT_WIDTH, 0.0, 0.0));
    let viewport_v = Vec3::from((0.0, -VIEWPORT_HEIGHT, 0.0));

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        camera_center - Vec3::from((0.0, 0.0, FOCAL_LENGTH)) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {} ", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);

            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(ray);

            pixel_color.write_color();
        }
    }

    eprintln!("\rDone.")
}
