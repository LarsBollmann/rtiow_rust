use std::fmt::Debug;
use crate::{hittable::HitRecord, ray::Ray, vec::Vec3};

mod metal;
mod lambertian;
mod dielectric;

pub use metal::Metal;
pub use lambertian::Lambertian;
pub use dielectric::Dielectric;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

