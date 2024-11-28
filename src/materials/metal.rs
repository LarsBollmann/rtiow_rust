use crate::{hittable::HitRecord, ray::Ray, vec::Vec3};
use std::fmt::Debug;

use super::Material;

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut reflected_dir = ray.dir.reflect(hit_record.normal).unit_vector();
        reflected_dir += Vec3::random_in_unit_sphere() * self.fuzz;
        let ray_reflected = Ray::new(hit_record.p, reflected_dir);

        if ray_reflected.dir.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, ray_reflected))
        } else {
            None
        }
    }
}
