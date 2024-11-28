use std::sync::Arc;

use crate::{interval::Interval, materials::Material, ray::Ray, vec::Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        outward_normal: Vec3,
        t: f64,
        ray: &Ray,
        material: Arc<dyn Material>,
    ) -> Self {
        let (is_front_face, normal) = Self::get_face_normal(ray, outward_normal);
        Self {
            p,
            normal,
            t,
            is_front_face,
            material,
        }
    }

    fn get_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let is_front_face = ray.dir.dot(outward_normal) < 0.0;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (is_front_face, normal)
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let mut closest_so_far = range.max;
        let mut hit_record = None;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, Interval::new(range.min, closest_so_far)) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord>;
}
