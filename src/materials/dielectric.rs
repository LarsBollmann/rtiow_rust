use std::fmt::Debug;
use rand::Rng;
use crate::{hittable::HitRecord, ray::Ray, vec::Vec3};

use super::Material;


#[derive(Debug)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.is_front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.dir.unit_vector();

        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();
        let new_direction = if cannot_reflect
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            ray.dir.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refraction_ratio)
        };

        Some((attenuation, Ray::new(hit_record.p, new_direction)))
    }
}
