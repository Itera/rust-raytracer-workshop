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

    pub fn point_along_direction(&self, delta: f64) -> Vec3 {
        self.origin + self.direction * delta
        // panic!("Step 3a) Calculate the point along the direction of the ray. Hint: Remember to \
        //         take the origin of the ray into the account of the final point")
    }
}
