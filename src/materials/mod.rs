use crate::{hittable::HitRecord, ray::Ray, vec::Vec3};
use std::fmt::Debug;

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}
