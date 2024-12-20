use vec3::{Point3f, Vec3f};

mod vec3;
mod ray;

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let mut colour: Vec3f = Vec3f::new(23.0, 23.0, 123.0);
    colour /= 100.0;

    print!("P3\n{image_width} {image_height}\n255\n");

    let origin: Point3f = Point3f::new(0.0, 0.0, 0.0);

    for y in 0..image_height {
        for x in 0..image_width {
            let r = colour.x();
            let g = colour.y();
            let b = colour.z();

            println!("{r} {g} {b}");
        }
    }

}
