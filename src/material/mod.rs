use std::fmt::Debug;

use rand::{ self, Rng };

use vec::Vec3;
use ray::Ray;
use scene::Intersection;
pub use self::color::Color;

const ORIGIN_OFFSET: f64 = 0.00000001;

mod color;

pub trait Material : Debug {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Color, Ray)>;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo: albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let target = intersection.point + intersection.normal + random_point_in_unit_sphere();
        let origin = reflection_origin(intersection);
        let direction = target - origin;
        let attenuation = self.albedo;
        Some((attenuation, Ray::new(origin, direction)))
    }
}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 { return p; }
    }
}

fn reflection_origin(intersection: &Intersection) -> Vec3 {
    intersection.point + intersection.normal * ORIGIN_OFFSET
}
