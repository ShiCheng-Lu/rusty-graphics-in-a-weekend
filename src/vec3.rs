use std::ops;


#[derive(Clone)]
pub struct Vec3f {
    e: [f32; 3]
}

impl Vec3f {
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

impl ops::MulAssign<f32> for Vec3f {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
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
    fn to_colour_string(&self) -> String {

        return format!("args");
    }
}
