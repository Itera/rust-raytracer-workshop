use vec::Vec3;
use ray::Ray;

use material::Material;
use scene::intersection::{ Intersectable, Intersection };

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub material: Box<Material>,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64, material: Box<Material>) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            material: material,
        }
    }
}

impl Intersectable for Sphere {
    fn intersects_with_limits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let origin = ray.origin - self.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let b: f64 = origin.dot(ray.direction);
        let c: f64 = origin.dot(origin) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
        let intersection = |t| Some(Intersection::new(
            t,
            ray.point_along_direction(t),
            (ray.point_along_direction(t) - self.origin) / self.radius,
            &*self.material));
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
}

#[cfg(test)]
mod tests {
    use hamcrest::{ assert_that, is, equal_to };

    use scene::{ Sphere, Intersectable };
    use material::{ Lambertian, Color };
    use vec::Vec3;
    use ray::Ray;

    #[test]
    fn should_intersect_sphere() {
        let m = Box::new(Lambertian::new(Color::white()));
        let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 1.0, m);

        let i = s.intersects(
            &Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0))).unwrap();

        assert_that(i.t, is(equal_to(1.0)));
        assert_that(i.point, is(equal_to(Vec3::new(0.0, 0.0, 0.0))));
        assert_that(i.normal, is(equal_to(Vec3::new(0.0, 0.0, 1.0))));
    }
}
