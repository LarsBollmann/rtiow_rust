use std::{
    default,
    fs::{create_dir_all, File},
    io::Write,
};

use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::prelude::*;

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
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub fov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub up: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
}

impl default::Default for CameraArgs {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            origin: Vec3::default(),
            focal_length: 1.0,
            samples_per_pixel: 10,
            max_depth: 10,
            fov: 120.0,
            lookfrom: Vec3::default(),
            lookat: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 1.0,
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
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    defocus_angle: f64,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(args: CameraArgs) -> Self {
        let image_height = ((args.image_width as f64 / args.aspect_ratio) as i32).max(1);

        let theta = args.fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * args.focus_distance;
        let viewport_width = viewport_height * (args.image_width as f64 / image_height as f64);

        let camera_origin = args.lookfrom;

        let w = (args.lookfrom - args.lookat).unit_vector();
        let u = args.up.unit_vector().cross(w);
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / args.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let defocus_disk_radius =
            (args.defocus_angle / 2.0).to_radians().tan() * args.focus_distance;
        let defocus_disk_u = defocus_disk_radius * u;
        let defocus_disk_v = defocus_disk_radius * v;

        let viewport_upper_left =
            camera_origin - args.focus_distance * w - 0.5 * viewport_u - 0.5 * viewport_v;

        let pixel_00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width: args.image_width,
            image_height: image_height as u32,
            origin: camera_origin,
            samples_per_pixel: args.samples_per_pixel,
            max_depth: args.max_depth,
            pixel_00_location,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle: args.defocus_angle,
        }
    }

    pub fn ray_color(depth: u32, ray: &Ray, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::default();
        }
        if let Some(hit_record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered_ray)) =
                hit_record.material.scatter(ray, &hit_record)
            {
                return attenuation * Self::ray_color(depth - 1, &scattered_ray, world);
            } else {
                return Color::default();
            }
        }

        let unit_dir = ray.dir.unit_vector();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn get_ray(&self, row: u32, column: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let (rand_u, rand_v) = (rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5));
        let pixel_sample = self.pixel_00_location
            + self.pixel_delta_v * (row as f64 + rand_v)
            + self.pixel_delta_u * (column as f64 + rand_u);

        let origin = if self.defocus_angle > 0.0 {
            let random = Vec3::random_in_unit_circle();
            self.origin + random.x * self.defocus_disk_u + random.y * self.defocus_disk_v
        } else {
            self.origin
        };
        let direction = pixel_sample - origin;

        Ray {
            orig: origin,
            dir: direction,
        }
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

        // for row in (0..self.image_height).progress() {
        //     for column in 0..self.image_width {
        //         let mut color = Color::default();

        //         for _ in 0..self.samples_per_pixel {
        //             let ray = self.get_ray(row, column);
        //             color += self.ray_color(self.max_depth, &ray, world);
        //         }
        //         color = color / self.samples_per_pixel as f64;
        //         writeln!(&mut file, "{}", color.to_bytes_string()).unwrap();
        //     }
        // }

        // Parallel implementation using rayon

        let colors: Vec<Color> = (0..self.image_height)
            .into_par_iter()
            .progress()
            .flat_map(|row| {
                (0..self.image_width).into_par_iter().map(move |column| {
                    let color: Color = (0..self.samples_per_pixel)
                        .into_par_iter()
                        .map(|_| {
                            let ray = self.get_ray(row, column);
                            Self::ray_color(self.max_depth, &ray, world)
                        })
                        .reduce(Color::default, |a, b| a + b);

                    color / self.samples_per_pixel as f64
                })
            })
            .collect();

        for color in colors {
            writeln!(&mut file, "{}", color.to_bytes_string()).unwrap();
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let args = CameraArgs::default();
        Self::new(args)
    }
}
