use crate::vec3::{Point3f, Vec3f};
use crate::ray::{Ray, Interval};
use crate::hittable::{HitResult, Hittable};


pub struct Sphere {
    pub center: Point3f,
    pub radius: f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Sphere {
        return Sphere {
            center: Vec3f::new(x, y, z),
            radius,
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
            println!("here");
        }
        let location = ray.at(root);
        let normal = (location.clone() - self.center.clone()) / self.radius;

        return Option::Some(HitResult {
            at: root,
            location,
            normal
        })
    }
}