use std::ops::{ Add, Sub, Mul, Div, Index };

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
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
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
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
            _ => panic!("Index out of bounds: {}", index)
        }
    }
}

#[cfg(test)]
mod tests {
    use hamcrest::{ assert_that, is, equal_to, close_to };
    use vec::Vec3;

    #[test]
    fn vec3_can_be_addded() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(0.0, 1.0, 2.0);

        let c = a + b;

        assert_that(c.x, is(equal_to(0.0)));
        assert_that(c.y, is(equal_to(2.0)));
        assert_that(c.z, is(equal_to(4.0)));
    }

    #[test]
    fn vec3_can_be_subtracted() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(0.0, 1.0, 2.0);

        let c = a - b;

        assert_that(c.x, is(equal_to(0.0)));
        assert_that(c.y, is(equal_to(0.0)));
        assert_that(c.z, is(equal_to(0.0)));
    }

    #[test]
    fn vec3_can_be_multiplied() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(0.0, 1.0, 2.0);

        let c = a * b;

        assert_that(c.x, is(equal_to(0.0)));
        assert_that(c.y, is(equal_to(1.0)));
        assert_that(c.z, is(equal_to(4.0)));
   }

    #[test]
    fn vec3_can_be_multiplied_with_f64() {
        let a = Vec3::new(0.0, 1.0, 2.0);

        let c = a * 2.0;

        assert_that(c.x, is(equal_to(0.0)));
        assert_that(c.y, is(equal_to(2.0)));
        assert_that(c.z, is(equal_to(4.0)));
    }

    #[test]
    fn vec3_can_be_equal() {
        let a = Vec3::new(1.2,  2.2, 3.2);
        let b = Vec3::new(1.2,  2.2, 3.2);

        assert_that(a, is(equal_to(b)));
   }

    #[test]
    fn vec3_has_length() {
        let a = Vec3::new(1.2, 2.2, 3.2);

        let l = a.length();

        assert_that(l, is(close_to(4.06448, 0.00001)));
    }

    #[test]
    fn vec3_can_be_normalized() {
        let v = Vec3::new(3.0, 4.0, 5.0);

        let v = v.normalize();

        assert_that(v.x, is(close_to(0.424264, 0.00001)));
    }

    #[test]
    fn vec3_can_be_indexed() {
        let a = Vec3::new(1.0, 2.0, 3.0);

        assert_that(a.x, is(equal_to(1.0)));
        assert_that(a.y, is(equal_to(2.0)));
        assert_that(a.z, is(equal_to(3.0)));
    }
}
