use std::ops::{Add, Sub, Mul, Div, Index};

use matrix::Matrix4;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3::new(self.x * k, self.y * k, self.z * k)
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, vec: Vec3) -> Vec3 {
        let x = self.y * vec.z - self.z * vec.y;
        let y = self.z * vec.x - self.x * vec.z;
        let z = self.x * vec.y - self.y * vec.x;
        Vec3::new(x, y, z)
    }

    pub fn invert(&self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, vec: &Vec3) -> bool {
        self.x == vec.x && self.y == vec.y && self.z == vec.z
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
        // panic!("Step 1a) Implement vector with vector addition")
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x * vec.x, self.y * vec.y, self.z * vec.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, val: f64) -> Vec3 {
        Vec3::new(self.x * val, self.y * val, self.z * val)
        // panic!("Step 1b) Implement vector with scalar multiplication")
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
        // panic!("Step 1c) Implement scalar with vector multiplication")
    }
}

impl Mul<Vec3> for Matrix4 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
	     let m = self.m;

         let x = vec[0] * m[0][0] + vec[1] * m[0][1] + vec[2] * m[0][2] + m[0][3];
         let y = vec[0] * m[1][0] + vec[1] * m[1][1] + vec[2] * m[1][2] + m[1][3];
         let z = vec[0] * m[2][0] + vec[1] * m[2][1] + vec[2] * m[2][2] + m[2][3];
         let w = vec[0] * m[3][0] + vec[1] * m[3][1] + vec[2] * m[3][2] + m[3][3];

         Vec3::new(x / w, y / w, z / w)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x / vec.x, self.y / vec.y, self.z / vec.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, val: f64) -> Vec3 {
        let k = 1.0 / val;
        Vec3::new(self.x * k, self.y * k, self.z * k)
    }
}

impl Index<u32> for Vec3 {
    type Output = f64;

    fn index<'a>(&'a self, index: u32) -> &'a f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}
