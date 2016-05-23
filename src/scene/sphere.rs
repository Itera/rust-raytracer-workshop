use rand::{ self, Rng };

use vec::Vec3;
use ray::Ray;
use color::Color;
use scene::intersection::{ Intersectable, Intersection };

const INTERSECTION_ORIGIN_OFFSET: f64 = 0.00000001;

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64, color: Color) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            color: color,
        }
    }
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let origin = ray.origin - self.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let b: f64 = origin.dot(ray.direction);
        let c: f64 = origin.dot(origin) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
        let intersection = |t| Some(Intersection::new(
            t,
            ray.point_along_direction(t),
            (ray.point_along_direction(t) - self.origin) / self.radius,
            self));
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min { return intersection(temp); }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min { return intersection(temp); }
            None
        } else {
            None
        }
    }

    fn scatter(&self, _: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let target = intersection.point + intersection.normal + random_point_in_unit_sphere();
        let origin = reflection_origin(intersection);
        let direction = target - origin;
        let attenuation = self.color;
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
    intersection.point + intersection.normal * INTERSECTION_ORIGIN_OFFSET
}


#[cfg(test)]
mod tests {
    use hamcrest::{ assert_that, is, equal_to };

    use vec::Vec3;
    use ray::Ray;
    use color::Color;
    use scene::{ Sphere, Intersectable };

    #[test]
    fn should_intersect_sphere() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 1.0, Color::white());

        let i = s.intersects(
            &Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0))).unwrap();

        assert_that(i.t, is(equal_to(1.0)));
        assert_that(i.point, is(equal_to(Vec3::new(0.0, 0.0, 0.0))));
        assert_that(i.normal, is(equal_to(Vec3::new(0.0, 0.0, 1.0))));
    }
}
