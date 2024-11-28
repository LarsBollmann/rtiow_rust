use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};

use crate::interval::Interval;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.orig;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd: f64 = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        if !range.surrounds(root) {
            root = (h + sqrtd) / a;
            if !range.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, normal, root, ray, self.material.clone()))
    }
}
