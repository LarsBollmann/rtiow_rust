use std::{fs::{File, create_dir_all}, io::Write};

use indicatif::ProgressIterator;
use raytracing::{ray::Ray, vec::{Color, Vec3}};


fn ray_color(ray: &Ray) -> Color {
    let unit_vector = ray.dir.unit_vector();
    let a = 0.5 * (unit_vector.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    create_dir_all("output").unwrap();
    let mut file = File::create("output/image.ppm").unwrap();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let focal_length = 1.0;
    let camera_origin = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_origin - Vec3::new(0.0, 0.0, focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;
    println!("viewport_upper_left: {:?}", viewport_upper_left);
    let pixel_00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    write!(&mut file, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for row in (0..image_height).progress() {
        for column in 0..image_width {
            let pixel_center = pixel_00_location + pixel_delta_u * column + pixel_delta_v * row;
            let ray_direction = pixel_center - camera_origin;
            let ray = Ray::new(camera_origin, ray_direction);

            let color = ray_color(&ray);
            writeln!(&mut file, "{}", color.to_bytes_string()).unwrap();
        }
    }
    
}
