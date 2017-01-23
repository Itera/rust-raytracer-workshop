use vec::Vec3;
use ray::Ray;
use color::Color;

pub trait Intersectable {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;

    fn scatter(&self, _: &Ray, _: &Intersection) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Intersection {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub shape: Box<Intersectable>,
}

impl Intersection {
    pub fn new(t: f64, point: Vec3, normal: Vec3, shape: Box<Intersectable>) -> Intersection {
        Intersection {
            t: t,
            point: point,
            normal: normal,
            shape: shape,
        }
    }
}
