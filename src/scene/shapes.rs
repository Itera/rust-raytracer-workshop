use vec::Vec3;
use ray::Ray;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
        }
    }
}
