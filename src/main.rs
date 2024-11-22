
use raytracing::{
    camera::Camera, geometries::Sphere, hittable::HittableList, vec::Vec3
};

fn main() {
    let camera = Camera::default();

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    camera.render(&world);
    
}
