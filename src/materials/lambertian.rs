use super::Material;
use crate::{hittable::HitRecord, ray::Ray, vec::Vec3};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
