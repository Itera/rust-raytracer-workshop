use scatter;
use prelude::*;

pub trait Intersectable: Sync {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;

    fn scatter(&self, _: &Ray, _: &Intersection) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Intersection {
    pub point: f64,
    pub intersection_point: Vec3,
    pub normal: Vec3,
    pub shape: Box<Intersectable>,
}

impl Intersection {
    pub fn new(point: f64,
               intersection_point: Vec3,
               normal: Vec3,
               shape: Box<Intersectable>)
               -> Intersection {
        Intersection {
            point: point,
            intersection_point: intersection_point,
            normal: normal,
            shape: shape,
        }
    }
}

pub struct Scene {
    pub shapes: Vec<Box<Intersectable>>,
}

impl Scene {
    pub fn new(shapes: Vec<Box<Intersectable>>) -> Scene {
        Scene { shapes: shapes }
    }
}

impl Intersectable for Scene {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;
        let mut closest_so_far: f64 = t_max;

        for shape in self.shapes.iter() {
            match shape.intersects(ray, t_min, closest_so_far) {
                Some(other_intersection) => {
                    closest_so_far = other_intersection.point;
                    intersection = Some(other_intersection);
                }
                None => (),
            }
        }
        intersection
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub color: Color,
    diffusiveness: Option<f64>,
    refraction_index: Option<f64>,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64, color: Color) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            color: color,
            diffusiveness: None,
            refraction_index: None,
        }
    }

    pub fn reflective(origin: Vec3, radius: f64, color: Color, diffusiveness: f64) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            color: color,
            diffusiveness: Some(diffusiveness),
            refraction_index: None,
        }
    }

    pub fn refractive(origin: Vec3, radius: f64, color: Color, refraction_index: f64) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            color: color,
            diffusiveness: None,
            refraction_index: Some(refraction_index),
        }
    }
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let translated_origin = ray.origin - self.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let b: f64 = translated_origin.dot(ray.direction);
        let c: f64 = translated_origin.dot(translated_origin) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
        if discriminant > 0.0 {
            let point = (-b - (b * b - a * c).sqrt()) / a;
            if point < t_max && point > t_min {
                return create_intersection(self, point, ray);
            }

            let point = (-b + (b * b - a * c).sqrt()) / a;
            if point < t_max && point > t_min {
                return create_intersection(self, point, ray);
            }
            None
        } else {
            None
        }
    }

    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        if let Some(diffusiveness) = self.diffusiveness {
            scatter::reflection(self.color, diffusiveness, ray, intersection)
        } else if let Some(refraction_index) = self.refraction_index {
            scatter::refraction(refraction_index, ray, intersection)
        } else {
            scatter::diffusive(self.color, intersection)
        }
    }
}

fn create_intersection(sphere: &Sphere, point: f64, ray: &Ray) -> Option<Intersection> {
    let intersection_point = ray.point_along_direction(point);
    // let surface_normal = panic!("Step 3b) Calculate the surface normal. The formula is available in the README");
    let surface_normal = (intersection_point - sphere.origin) / sphere.radius;
    Some(Intersection::new(point, intersection_point, surface_normal, Box::new(sphere.clone())))
}
