use std::sync::Arc;

use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use hittable::HittableArray;
use camera::Camera;
use vec3::{Colour, Vec3f};

mod vec3;
mod ray;
mod sphere;
mod hittable;
mod camera;
mod material;

fn main() {
    let material_ground = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    // let material_left   = Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3));
    let material_left   = Arc::new(Dielectric::new(1.5));
    let material_bubble   = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right  = Arc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0));

    let mut world = HittableArray::new();
    world.add(Arc::new(Sphere::new( 0.0, -100.5, -1.0, 100.0, material_ground)));
    world.add(Arc::new(Sphere::new( 0.0,    0.0, -1.2,   0.5, material_center)));
    world.add(Arc::new(Sphere::new(-1.0,    0.0, -1.0,   0.5, material_left)));
    world.add(Arc::new(Sphere::new(-1.0,    0.0, -1.0,   0.4, material_bubble)));
    world.add(Arc::new(Sphere::new( 1.0,    0.0, -1.0,   0.5, material_right)));

    let camera = Camera::new((400, 300), Vec3f::new(-2.0, 2.0, 1.0), Vec3f::new(0.0, 0.0, -1.0), 60.0, 10.0, 3.4);

    camera.render(world);
}
