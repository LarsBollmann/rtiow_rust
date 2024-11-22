use std::{
    default,
    fs::{create_dir_all, File},
    io::Write,
};

use indicatif::ProgressIterator;

use crate::{
    hittable::HittableList,
    interval::Interval,
    ray::Ray,
    vec::{Color, Vec3},
};

#[derive(Debug, Clone)]
pub struct CameraArgs {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub origin: Vec3,
    pub focal_length: f64,
}

impl default::Default for CameraArgs {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            origin: Vec3::default(),
            focal_length: 1.0,
        }
    }
}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    origin: Vec3,
    pixel_00_location: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(args: CameraArgs) -> Self {
        let image_height = ((args.image_width as f64 / args.aspect_ratio) as i32).max(1);

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (args.image_width as f64 / image_height as f64);
        let focal_length = 1.0;
        let camera_origin = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / args.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            camera_origin - Vec3::new(0.0, 0.0, focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;

        let pixel_00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width: args.image_width,
            image_height: image_height as u32,
            origin: camera_origin,
            pixel_00_location,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        if let Some(hit_record) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_dir = ray.dir.unit_vector();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &HittableList) {
        create_dir_all("output").unwrap();
        let mut file = File::create("output/image.ppm").unwrap();

        write!(
            &mut file,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )
        .unwrap();

        for row in (0..self.image_height).progress() {
            for column in 0..self.image_width {
                let pixel_center =
                    self.pixel_00_location + self.pixel_delta_u * column + self.pixel_delta_v * row;
                let ray_direction = pixel_center - self.origin;
                let ray = Ray::new(self.origin, ray_direction);

                let color = self.ray_color(&ray, world);
                writeln!(&mut file, "{}", color.to_bytes_string()).unwrap();
            }
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let args = CameraArgs::default();
        Self::new(args)
    }
}
