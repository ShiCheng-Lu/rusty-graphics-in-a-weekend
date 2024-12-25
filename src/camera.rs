use std::sync::Arc;
use std::thread;

use crate::hittable::{Hittable, HittableArray};
use crate::ray::{Ray, Interval};
use crate::vec3::{Colour, Point3f, Vec3f};
use rand::{random, Rng};

pub struct Camera {
    orientation: Ray,
    image_size: (usize, usize),
    pixel_topleft: Vec3f,
    pixel_delta: (Vec3f, Vec3f),
    defocus_delta: (Vec3f, Vec3f),
    samples: i32,
    sample_scale: f32,
    sample_depth: i32,
    thread_count: usize,
    defocus_blur: f32,
}

impl Camera {
    pub fn new(image_size: (usize, usize), origin: Vec3f, look_at: Vec3f, fov: f32, defocus_blur: f32, focus_distance: f32) -> Camera {
        let viewport_width =  f32::tan(fov.clamp(0.0, 170.0).to_radians() / 2.0) * focus_distance;
        let viewport_height = viewport_width * (image_size.1 as f32) / (image_size.0 as f32);

        let look_direction = (look_at - origin.clone()).normalize();
        let look_up = Vec3f::new(0.0, 1.0, 0.0);

        let viewport_x = Vec3f::cross(&look_direction, &look_up);
        let viewport_y = Vec3f::cross(&look_direction, &viewport_x);

        let pixel_topleft = origin.clone() + look_direction.clone() * focus_distance;
        let pixel_topleft = pixel_topleft - viewport_y.clone() / 2.0 * viewport_height;
        let pixel_topleft = pixel_topleft - viewport_x.clone() / 2.0 * viewport_width;

        let pixel_delta_x = viewport_x.clone() * viewport_width / (image_size.0 as f32);
        let pixel_delta_y = viewport_y.clone() * viewport_height / (image_size.1 as f32);

        let defocus_radius = f32::tan(defocus_blur.to_radians() / 2.0) * focus_distance;
        let defocus_delta_x = viewport_x.clone() * defocus_radius;
        let defocus_delta_y = viewport_y.clone() * defocus_radius;

        let sample_depth = 20;
        let samples = 10;
        let thread_count = 10;
        let sample_scale = 1.0 / (samples as f32 * thread_count as f32);

        return Camera {
            orientation: Ray::new(&origin, &look_direction),
            image_size,
            pixel_topleft,
            pixel_delta: (pixel_delta_x, pixel_delta_y),
            defocus_delta: (defocus_delta_x, defocus_delta_y),
            samples,
            sample_scale,
            sample_depth,
            thread_count,
            defocus_blur,
        };
    }

    fn random_on_hemisphere(normal: Vec3f) -> Vec3f {
        let vector = Vec3f::random_orientation();
        if Vec3f::dot(&vector, &normal) >= 0.0 {
            return vector;
        } else {
            return -vector;
        }
    }

    fn ray_colour(&self, ray: &Ray, world: &HittableArray, depth: i32) -> Colour {
        if depth <= 0 {
            return Colour::ZERO;
        }
        let hit_result = world.hit(&ray, &Interval::RAY);
        
        match hit_result {
            None => {
                let a = 0.5 * (ray.direction.y() + 1.0);
                return Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a;
            },
            Some(result) => {
                // let direction: Vec3f = Camera::random_on_hemisphere(result.normal);
                let scatter_result = result.material.scatter(ray, &result);
                if scatter_result.scattered {
                    return scatter_result.attenuation * self.ray_colour(&scatter_result.ray, world, depth - 1);
                } else {
                    return Colour::ZERO;
                }
            },
        }
    }

    fn rand_defocus(&self) -> Vec3f {
        if self.defocus_blur <= 0.0 {
            return Vec3f::new(0.0, 0.0, 0.0);
        }
        let mut x: f32 = random();
        let mut y: f32 = random();
        while x * x + y * y > 1.0 {
            x = random();
            y = random();
        } 
        return self.defocus_delta.0.clone() * x + self.defocus_delta.1.clone() * y;
    }
    
    fn get_ray(&self, image_x: usize, image_y: usize) -> Ray {
        let dx: f32 = rand::thread_rng().gen::<f32>() - 0.5;
        let dy: f32 = rand::thread_rng().gen::<f32>() - 0.5;
        
        let x = self.pixel_delta.0.clone() * (image_x as f32 + dx);
        let y = self.pixel_delta.1.clone() * (image_y as f32 + dy);

        let origin = self.orientation.origin.clone() + self.rand_defocus();

        let direction = self.pixel_topleft.clone() + x + y - origin.clone();
        return Ray::new(&origin, &direction);
    }

    fn linear_to_gamma(value: f32) -> f32 {
        if value > 0.0 {
            return value.sqrt();
        }
        return 0.0;
    }

    fn to_colour(colour: Colour) -> String {
        let r = Camera::linear_to_gamma(colour.x());
        let g = Camera::linear_to_gamma(colour.y());
        let b = Camera::linear_to_gamma(colour.z());

        return format!("{} {} {}",
            (r * 255.0) as i32,
            (g * 255.0) as i32,
            (b * 255.0) as i32,
        );
    }

    pub fn render_iteration(&self, world: &HittableArray) -> Vec<Colour> {
        let mut image = vec![Colour::ZERO; self.image_size.0 * self.image_size.1];
        
        for image_y in 0..(self.image_size.1) {
            for image_x in 0..(self.image_size.0) {
                let ray = self.get_ray(image_x, image_y);
                let colour = self.ray_colour(&ray, world, self.sample_depth);
                image[image_y * self.image_size.0 + image_x] = colour;
            }
        }

        return image;
    }

    pub fn render(self, world: HittableArray) {
        let mut threads = vec![];

        let c_arc = Arc::new(self);
        let w_arc = Arc::new(world);
        
        for _ in 0..c_arc.thread_count {
            let camera_arc = c_arc.clone();
            let world_arc = w_arc.clone();
            threads.push(thread::spawn(move || {
                let mut image = vec![Colour::ZERO; camera_arc.image_size.0 * camera_arc.image_size.1];
                for _ in 0..camera_arc.samples {
                    let layer = camera_arc.render_iteration(&world_arc);
                    for i in 0..image.len() {
                        image[i] += layer[i].clone();
                    }
                }
                image
            }));
        }

        let mut image = vec![Colour::ZERO; c_arc.image_size.0 * c_arc.image_size.1];
        for thread in threads {
            match thread.join() {
                Ok(layer) => {
                    for i in 0..image.len() {
                        image[i] += layer[i].clone();
                    }
                },
                Err(error) => {
                    // terrible!!!
                }
            };
        }

        print!("P3\n{} {}\n255\n", c_arc.image_size.0, c_arc.image_size.1);

        for colour in image {
            println!("{}", Camera::to_colour(colour * c_arc.sample_scale));
        }

        // for image_y in 0..(self.image_height) {
        //     for image_x in 0..(self.image_width) {
        //         let mut colour = Vec3f::ZERO.clone();
        //         for _ in 0..(self.samples) {
        //             let ray = self.get_ray(image_x, image_y);
                
        //             colour += self.ray_colour(&ray, world, self.sample_depth);
        //         }
        //         colour *= self.sample_scale;
        //     }
        // }
    }
}