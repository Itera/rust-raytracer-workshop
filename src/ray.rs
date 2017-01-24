use vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_along_direction(&self, param: f64) -> Vec3 {
        self.origin + self.direction * param
    }
}

#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use vec::Vec3;
    use ray::Ray;

    #[test]
    fn can_init_ray() {
        let r = Ray::new(Vec3::new(0.0, 1.0, 2.0), Vec3::new(2.0, 1.0, 0.0));

        assert_that!(r.origin[2], is(equal_to(2.0)));
        assert_that!(r.direction[2], is(equal_to(0.0)));
    }

    #[test]
    fn can_find_point_along_direction() {
        let r = Ray::new(Vec3::new(0.0, 1.0, 2.0), Vec3::new(2.0, 1.0, 0.0));

        assert_that!(r.point_along_direction(2.0),
                     is(equal_to(Vec3::new(4.0, 3.0, 2.0))));
    }
}
