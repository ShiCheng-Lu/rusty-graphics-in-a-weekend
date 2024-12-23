use std::sync::Arc;

use crate::material::{self, Material};
use crate::vec3::{Point3f, Vec3f};
use crate::ray::{Ray, Interval};
use crate::hittable::{HitResult, Hittable};


pub struct Sphere {
    pub center: Point3f,
    pub radius: f32,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32, material: Arc<dyn Material + Send + Sync>) -> Sphere {
        return Sphere {
            center: Vec3f::new(x, y, z),
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let oc: Vec3f = self.center.clone() - ray.origin.clone();
        let a: f32 = ray.direction.length_squared();
        let h: f32 = Vec3f::dot(&ray.direction, &oc); 
        let c: f32 = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return Option::None
        }
        
        let sqrt_discriminant = discriminant.sqrt();
        // prefer the smaller/more negative root
        let mut root = (h - sqrt_discriminant) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            // both roots are outside of the interval
            if !interval.surrounds(root) {
                return Option::None;
            }
        }
        let location = ray.at(root);
        let normal = (location.clone() - self.center.clone()) / self.radius;
        let material = self.material.clone();

        return Option::Some(HitResult {
            at: root,
            location,
            normal,
            material,
        })
    }
}