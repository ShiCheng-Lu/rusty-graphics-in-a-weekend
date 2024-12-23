use core::f32;

use crate::vec3::{Point3f, Vec3f};

pub struct Ray {
    pub origin: Point3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: &Point3f, direction: &Vec3f) -> Ray {
        // let direction_len = direction.length();
        return Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        };
    }

    pub fn at(&self, time: f32) -> Point3f {
        return self.origin.clone() + self.direction.clone() * time;
    }
}

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const ALL: Interval = Interval { min: f32::INFINITY, max: f32::NEG_INFINITY };
    pub const EMPTY: Interval = Interval { min: f32::NEG_INFINITY, max: f32::INFINITY };
    pub const FORWARD: Interval = Interval { min: 0.0, max: f32::INFINITY };

    pub fn new(min: f32, max: f32) -> Interval {
        return Interval {
            min,
            max,
        };
    }

    pub fn size(&self) -> f32 {
        return self.max - self.min;
    }

    pub fn contains(&self, value: f32) -> bool {
        return self.min <= value && value <= self.max;
    }

    pub fn surrounds(&self, value: f32) -> bool {
        return self.min < value && value < self.max;
    }

    pub fn clamp(&self, value: f32) -> f32 {
        if value > self.max {
            return self.max;
        }
        if value < self.min {
            return self.min;
        }
        return value;
    }
}
