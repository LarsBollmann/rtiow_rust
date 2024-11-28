use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};

use crate::interval::Interval;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug, Clone)]
pub struct Plane {
    origin: Vec3,
    normal: Vec3,
    material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(origin: Vec3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        Self {
            origin,
            normal: normal.unit_vector(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let denom = ray.dir.dot(self.normal);
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = (self.origin - ray.orig).dot(self.normal) / denom;
        if !range.surrounds(t) {
            return None;
        }

        let p = ray.at(t);
        Some(HitRecord::new(
            p,
            self.normal,
            t,
            ray,
            self.material.clone(),
        ))
    }
}
