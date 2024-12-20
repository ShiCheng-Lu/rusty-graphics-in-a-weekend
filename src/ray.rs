use crate::vec3::{Point3f, Vec3f};

struct Ray {
    pub origin: Point3f,
    pub direction: Vec3f,
}

impl Ray {
    fn new(origin: &Point3f, direction: &Vec3f) -> Ray {
        let direction_len = direction.length();
        return Ray {
            origin: origin.clone(),
            direction: (direction.clone() / direction_len)
        };
    }

    fn at(&self, time: f32) -> Point3f {
        return self.origin.clone() + self.direction.clone() * time;
    }
}
