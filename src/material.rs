use rand::random;

use crate::hittable::HitResult;
use crate::ray::Ray;
use crate::vec3::{Colour, Vec3f};

pub struct ScatterResult {
    pub scattered: bool,
    pub ray: Ray,
    pub attenuation: Colour,
}

pub trait Material {
    fn scatter(&self, in_ray: &Ray, hit_result: &HitResult) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {    
    pub fn new(albedo: Colour) -> Lambertian {
        return Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, in_ray: &Ray, hit_result: &HitResult) -> ScatterResult {
        let mut scatter_direction = hit_result.normal.clone() + Vec3f::random_orientation();
        if scatter_direction.is_nearly_zero() {
            scatter_direction = hit_result.normal.clone();
        }
        let scattered_ray = Ray::new(&hit_result.location, &scatter_direction);
        let attenuation = self.albedo.clone();
        return ScatterResult {
            scattered: true,
            ray: scattered_ray,
            attenuation,
        }
    }
}



pub struct Metal {
    albedo: Colour,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f32) -> Metal {
        return Metal {
            albedo,
            fuzz
        }
    }
}

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, hit_result: &HitResult) -> ScatterResult {
        let mut reflected_direction = in_ray.direction.reflect(&hit_result.normal);
        reflected_direction /= reflected_direction.length();
        reflected_direction += Vec3f::random_orientation() * self.fuzz;
        
        let scattered_ray = Ray::new(&hit_result.location, &reflected_direction);
        let attenuation = self.albedo.clone();
        return ScatterResult {
            scattered: Vec3f::dot(&scattered_ray.direction, &hit_result.normal) > 0.0,
            ray: scattered_ray,
            attenuation,
        }
    }
}

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Dielectric {
        return Dielectric {
            refractive_index,
        }
    }

    fn reflected(&self, cos_theta: f32, refractive_ratio: f32) -> bool {
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        // total internal reflection
        if refractive_ratio * sin_theta > 1.0 {
            return true;
        }
        // Schlick's approximation for reflectance.
        let r0 = (1.0 - refractive_ratio) / (1.0 + refractive_ratio);
        let r0 = r0 * r0;
        let probability = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);

        if random::<f32>() < probability {
            return true;
        }
        return false;
    }
}

impl Material for Dielectric {
    fn scatter(&self, in_ray: &Ray, hit_result: &HitResult) -> ScatterResult {
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let internal = Vec3f::dot(&hit_result.normal, &in_ray.direction) > 0.0;
        
        let refractive_ratio = if internal { self.refractive_index } else { 1.0 / self.refractive_index };
        let normal = if internal { -hit_result.normal.clone() } else { hit_result.normal.clone() };

        let ray_direction = in_ray.direction.clone() / in_ray.direction.length();
        let cos_theta = f32::min(-Vec3f::dot(&ray_direction, &normal), 1.0);
        
        let result_ray = if self.reflected(cos_theta, refractive_ratio) {
            ray_direction.reflect(&normal)
        } else {
            ray_direction.refract(&normal, refractive_ratio)
        };
        let ray = Ray::new(&hit_result.location, &result_ray);
        
        return ScatterResult {
            scattered: true,
            ray,
            attenuation,
        }
    }
}
