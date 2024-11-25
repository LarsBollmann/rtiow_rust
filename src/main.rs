
use std::sync::Arc;

use raytracing::{
    camera::{Camera, CameraArgs}, geometries::Sphere, hittable::HittableList, material::{Dielectric, Lambertian, Metal}, vec::Vec3
};

fn main() {
    let camera = Camera::new(CameraArgs{samples_per_pixel: 100, max_depth: 50, ..Default::default()});

    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.0/1.33));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    camera.render(&world);

    // let vec = Vec3::new(0.0, 2.0, 1.0);
    // let normal = Vec3::new(0.0, 0.0, 1.0);
    // let reflected = vec.reflect(normal);
    // println!("{:?}", reflected);
    // let refracted = vec.unit_vector().refract(normal, 1.5);
    // println!("{:?}", refracted);
    
}
