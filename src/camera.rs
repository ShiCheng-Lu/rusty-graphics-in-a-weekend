use crate::hittable::{Hittable, HittableArray};
use crate::ray::{Ray, Interval};
use crate::vec3::{Colour, Point3f, Vec3f};

pub struct Camera {
    orientation: Ray,
    focal_length: f32,
    image_width: i32,
    image_height: i32,
    viewport_width: f32,
    viewport_height: f32,
    width_scale: f32,
    height_scale: f32,
}

impl Camera {
    pub fn new(width: i32, aspect_ratio: f32) -> Camera {
        let image_width = width;
        let image_height = ((image_width as f32) / aspect_ratio) as i32;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32) / (image_height as f32);
        let width_scale = viewport_width / (image_width as f32);
        let height_scale = viewport_height / (image_height as f32);

        return Camera {
            orientation: Ray::new(&Vec3f::ZERO, &Vec3f::new(0.0, 0.0, 1.0)),
            focal_length: 1.0,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            width_scale,
            height_scale,
        };
    }

    fn ray_colour(&self, ray: &Ray, world: &HittableArray) -> Colour {
        let a = 0.5 * (ray.direction.y() + 1.0);
        let hit_result = world.hit(&ray, &Interval::FORWARD);
        
        match hit_result {
            None => {
                return Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a;
            },
            Some(result) => {
                return (result.normal + Vec3f::new(1.0, 1.0, 1.0)) * 0.5;
            },
        }
    }
    
    fn get_ray(&self, image_x: i32, image_y: i32) -> Ray {
        let x = (image_x as f32) * self.width_scale - self.viewport_width * 0.5;
        let y = (image_y as f32) * self.height_scale - self.viewport_height * 0.5;

        let direction = Point3f::new(x, -y, -self.focal_length);
        return Ray::new(&self.orientation.origin, &direction);
    }

    pub fn render(&self, world: &HittableArray) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for image_y in 0..(self.image_height) {
            for image_x in 0..(self.image_width) {
    
                let ray = self.get_ray(image_x, image_y);
                
                let colour = self.ray_colour(&ray, world);
    
                println!("{}", colour.to_colour_string());
            }
        }
    }
}