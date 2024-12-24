use core::f32;
use std::ops;

use rand::random;

use crate::ray::Interval;


#[derive(Clone)]
pub struct Vec3f {
    pub e: [f32; 3]
}

impl Vec3f {
    pub const ZERO: Vec3f = Vec3f { e: [ 0.0, 0.0, 0.0 ] };

    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        return Vec3f { e: [x, y, z] };
    }
    
    pub fn x(&self) -> f32 {
        return self.e[0];
    }
    
    pub fn y(&self) -> f32 {
        return self.e[1];
    }
    
    pub fn z(&self) -> f32 {
        return self.e[2];
    }

    pub fn length_squared(&self) -> f32 {
        return (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2]);
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn dot(a: &Vec3f, b: &Vec3f) -> f32 {
        return (a.e[0] * b.e[0]) + (a.e[1] * b.e[1]) + (a.e[2] * b.e[2]);
    }

    pub fn cross(a: &Vec3f, b: &Vec3f) -> Vec3f {
        return Vec3f { e: [
            a.e[1] * b.e[2] - a.e[2] * b.e[1],
            a.e[2] * b.e[0] - a.e[0] * b.e[2],
            a.e[0] * b.e[1] - a.e[1] * b.e[0]
        ] };
    }

    pub fn random(min: f32, max: f32) -> Vec3f {
        let range = max - min;
        return Vec3f::new(
            random::<f32>() * range + min,
            random::<f32>() * range + min,
            random::<f32>() * range + min,
        );
    }

    pub fn random_orientation() -> Vec3f {
        let valid_range = Interval::new(f32::EPSILON, 1.0);
        let mut vector = Vec3f::random(-1.0, 1.0);
        while !valid_range.contains(vector.length_squared()) {
            vector = Vec3f::random(-1.0, 1.0);
        }
        vector /= vector.length();
        return vector;
    }

    pub fn is_nearly_zero(&self) -> bool {
        return self.e[0].abs() <= f32::EPSILON && self.e[1].abs() <= f32::EPSILON && self.e[2].abs() <= f32::EPSILON
    }

    pub fn reflect(&self, normal: &Vec3f) -> Vec3f {
        return self.clone() - normal.clone() * 2.0 * Vec3f::dot(self, normal);
    }

    pub fn refract(&self, normal: &Vec3f, refractive_ratio: f32) -> Vec3f {
        let cos_theta = f32::min(-Vec3f::dot(self, normal), 1.0);
        let ray_perp = (self.clone() + normal.clone() * cos_theta) * refractive_ratio;
        let ray_parallel = - normal.clone() * (1.0 - ray_perp.length_squared()).abs().sqrt();
        return ray_perp + ray_parallel;
    }

}

impl ToString for Vec3f {
    fn to_string(&self) -> String {
        return format!("{} {} {}", self.e[0], self.e[1], self.e[2]);
    }
}

impl ops::Neg for Vec3f {
    type Output = Vec3f;
    fn neg(self) -> Vec3f {
        return Vec3f { e: [-self.x(), -self.y(), -self.z()] };
    }
}

impl ops::Index<usize> for Vec3f {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.e[index];
    }
}

impl ops::AddAssign<Vec3f> for Vec3f {
    fn add_assign(&mut self, rhs: Vec3f) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Add<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Vec3f) -> Self::Output {
        return Vec3f { e: [
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        ]}
    }
}

impl ops::SubAssign<Vec3f> for Vec3f {
    fn sub_assign(&mut self, rhs: Vec3f) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl ops::Sub<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Vec3f) -> Self::Output {
        return Vec3f { e: [
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        ]}
    }
}

impl ops::Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: f32) -> Self::Output {
        return Vec3f { e: [
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs,
        ] }
    }
}


impl ops::Mul<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        return Vec3f { e: [
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        ] }
    }
}

impl ops::MulAssign<f32> for Vec3f {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::Div<f32> for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: f32) -> Self::Output {
        return self * (1.0 / rhs);
    }
}

impl ops::DivAssign<f32> for Vec3f {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

pub type Point3f = Vec3f;

pub type Colour = Vec3f;

impl Colour {
    pub fn to_colour_string(&self) -> String {
        return format!("{} {} {}",
            (self.e[0] * 255.0) as i32,
            (self.e[1] * 255.0) as i32,
            (self.e[2] * 255.0) as i32,
        );
    }
}
