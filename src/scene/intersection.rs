use std::f64;

use vec::Vec3;
use ray::Ray;
use material::Material;

pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.intersects_with_limits(ray, 0.0, f64::MAX)
    }

    fn intersects_with_limits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, point: Vec3, normal: Vec3, material: &'a Material) -> Intersection {
        Intersection {
            t: t,
            point: point,
            normal: normal,
            material: material,
        }
    }
}
