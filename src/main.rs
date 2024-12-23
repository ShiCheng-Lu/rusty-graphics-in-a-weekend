use sphere::Sphere;
use hittable::{Hittable, HittableArray};
use camera::Camera;
use ray::{Interval, Ray};
use vec3::Vec3f;

mod vec3;
mod ray;
mod sphere;
mod hittable;
mod camera;

fn main() {

    let mut world = HittableArray::new();
    world.add(Sphere::new(0.0, 0.0, -1.0, 0.5));
    world.add(Sphere::new(0.0,-100.5,-1.0, 100.0));

    let camera = Camera::new(400, 16.0 / 9.0);

    camera.render(&world);
}
