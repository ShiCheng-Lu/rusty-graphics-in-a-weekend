use std::sync::Arc;
use std::thread;

use crate::hittable::{Hittable, HittableArray};
use crate::ray::{Ray, Interval};
use crate::vec3::{Colour, Point3f, Vec3f};
use rand::Rng;

pub struct Camera {
    orientation: Ray,
    focal_length: f32,
    image_width: usize,
    image_height: usize,
    viewport_width: f32,
    viewport_height: f32,
    width_scale: f32,
    height_scale: f32,
    samples: i32,
    sample_scale: f32,
    sample_depth: i32,
    thread_count: usize,
}

impl Camera {
    pub fn new(width: usize, aspect_ratio: f32, fov: f32) -> Camera {
        let image_width = width;
        let image_height = ((image_width as f32) / aspect_ratio) as usize;

        
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32) / (image_height as f32);
        let width_scale = viewport_width / (image_width as f32);
        let height_scale = viewport_height / (image_height as f32);
        let sample_depth = 20;
        let samples = 10;
        let thread_count = 10;
        let sample_scale = 1.0 / (samples as f32 * thread_count as f32);

        return Camera {
            orientation: Ray::new(&Vec3f::ZERO, &Vec3f::new(0.0, 0.0, 1.0)),
            focal_length: 1.0,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            width_scale,
            height_scale,
            samples,
            sample_scale,
            sample_depth,
            thread_count,
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
    
    fn get_ray(&self, image_x: usize, image_y: usize) -> Ray {
        let dx: f32 = rand::thread_rng().gen::<f32>() - 0.5;
        let dy: f32 = rand::thread_rng().gen::<f32>() - 0.5;
        
        let x = (image_x as f32 + dx) * self.width_scale - self.viewport_width * 0.5;
        let y = (image_y as f32 + dy) * self.height_scale - self.viewport_height * 0.5;

        let direction = Point3f::new(x, -y, -self.focal_length);
        return Ray::new(&self.orientation.origin, &direction);
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
        let mut image = vec![Colour::ZERO; self.image_height * self.image_width];
        
        for image_y in 0..(self.image_height) {
            for image_x in 0..(self.image_width) {
                let ray = self.get_ray(image_x, image_y);
                let colour = self.ray_colour(&ray, world, self.sample_depth);
                image[image_y * self.image_width + image_x] = colour;
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
                let mut image = vec![Colour::ZERO; camera_arc.image_height * camera_arc.image_width];
                for _ in 0..camera_arc.samples {
                    let layer = camera_arc.render_iteration(&world_arc);
                    for i in 0..image.len() {
                        image[i] += layer[i].clone();
                    }
                }
                image
            }));
        }

        let mut image = vec![Colour::ZERO; c_arc.image_height * c_arc.image_width];
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

        print!("P3\n{} {}\n255\n", c_arc.image_width, c_arc.image_height);

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