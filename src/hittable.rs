use std::sync::Arc;

use crate::material::Material;
use crate::ray::{Ray, Interval};
use crate::vec3::Vec3f;

pub struct HitResult {
    pub at: f32,
    pub location: Vec3f,
    pub normal: Vec3f,
    pub material: Arc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;
}
pub struct HittableArray {
    pub array: Vec<Arc<dyn Hittable + Send + Sync>>
}

impl HittableArray {
    pub fn new() -> HittableArray {
        return HittableArray {
            array: Vec::new()
        }
    }

    pub fn add(& mut self, value: Arc<dyn Hittable + Send + Sync>) {
        self.array.push(value);
    }
}

impl Hittable for HittableArray {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let mut closest_result: Option<HitResult> = Option::None;

        for hittable in &self.array {
            if let Some(result) =  hittable.hit(&ray, interval) {
                match closest_result {
                    None => closest_result = Some(result),
                    Some(ref close) => {
                        if result.at < close.at {
                            closest_result = Some(result);
                        }
                    }
                }
            }
        }

        return closest_result;
    }
}