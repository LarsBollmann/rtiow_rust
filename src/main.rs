use std::sync::Arc;

use raytracing::{
    camera::{Camera, CameraArgs},
    geometries::Sphere,
    hittable::HittableList,
    materials::{Dielectric, Lambertian, Material, Metal},
    vec::Vec3,
};

fn main() {
    let camera = Camera::new(CameraArgs {
        samples_per_pixel: 500,
        image_width: 1200,
        max_depth: 50,
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_distance: 10.0,
        fov: 20.0,
        ..Default::default()
    });

    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_material = rand::random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_material < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_material < 0.95 {
                    let albedo = Vec3::random_range(0.5..1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    camera.render(&world);
}
