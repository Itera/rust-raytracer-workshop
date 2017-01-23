use rand::{self, Rng};

use vec::Vec3;
use ray::Ray;
use color::Color;
use scene::intersection::{Intersectable, Intersection};

const INTERSECTION_ORIGIN_OFFSET: f64 = 0.00000001;

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
        let origin = ray.origin - self.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let b: f64 = origin.dot(ray.direction);
        let c: f64 = origin.dot(origin) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return create_intersection(self, temp, ray);
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return create_intersection(self, temp, ray);
            }
            None
        } else {
            None
        }
    }

    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        if let Some(diffusiveness) = self.diffusiveness {
            scatter_reflection(self.color, diffusiveness, ray, intersection)
        } else if let Some(refraction_index) = self.refraction_index {
            scatter_refraction(refraction_index, ray, intersection)
        } else {
            scatter_diffusive(self.color, intersection)

        }
    }
}

fn create_intersection(sphere: &Sphere, t: f64, ray: &Ray) -> Option<Intersection> {
    Some(Intersection::new(t,
                           ray.point_along_direction(t),
                           (ray.point_along_direction(t) - sphere.origin) / sphere.radius,
                           Box::new(sphere.clone())))
}

fn scatter_diffusive(attenuation: Color, intersection: &Intersection) -> Option<(Color, Ray)> {
    let target = intersection.point + intersection.normal + random_point_in_unit_sphere();
    let origin = reflection_origin(intersection);
    let direction = target - origin;
    Some((attenuation, Ray::new(origin, direction)))
}

fn scatter_reflection(attenuation: Color,
                      diffusiveness: f64,
                      ray: &Ray,
                      intersection: &Intersection)
                      -> Option<(Color, Ray)> {
    let reflected = reflect(ray.direction.normalize(), intersection.normal) +
                    diffusiveness * random_point_in_unit_sphere();
    let origin = reflection_origin(intersection);
    match reflected.dot(intersection.normal) > 0.0 {
        true => Some((attenuation, Ray::new(origin, reflected))),
        false => None,
    }
}

fn scatter_refraction(refraction_index: f64,
                      ray: &Ray,
                      intersection: &Intersection)
                      -> Option<(Color, Ray)> {
    let attenuation = Color::white();

    let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(intersection.normal) > 0.0 {
        (intersection.normal.invert(),
         refraction_index,
         refraction_index * ray.direction.dot(intersection.normal) / ray.direction.length())
    } else {
        (intersection.normal,
         1.0 / refraction_index,
         -ray.direction.dot(intersection.normal) / ray.direction.length())
    };

    let refracted = refract(ray.direction, outward_normal, ni_over_nt);
    let should_refract = match refracted {
        Some(_) => shlick(cosine, refraction_index) < rand::thread_rng().next_f64(),
        None => false,
    };

    match refracted {
        Some(refracted) if should_refract => {
            let origin = refraction_origin(intersection.point, outward_normal);
            Some((attenuation, Ray::new(origin, refracted)))
        }
        _ => {
            let origin = reflection_origin(intersection);
            let reflected = reflect(ray.direction, intersection.normal);
            Some((attenuation, Ray::new(origin, reflected)))
        }
    }

}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) -
                Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn reflection_origin(intersection: &Intersection) -> Vec3 {
    intersection.point + intersection.normal * INTERSECTION_ORIGIN_OFFSET
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn refraction_origin(origin: Vec3, normal: Vec3) -> Vec3 {
    origin - normal * INTERSECTION_ORIGIN_OFFSET
}

fn shlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[cfg(test)]
mod tests {
    use hamcrest::{assert_that, is, equal_to};

    use vec::Vec3;
    use ray::Ray;
    use color::Color;
    use scene::{Sphere, Intersectable};

    #[test]
    fn should_intersect_sphere() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 1.0, Color::white());

        let i = s.intersects(&Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0)),
                        0.0,
                        1000.0)
            .unwrap();

        assert_that(i.t, is(equal_to(1.0)));
        assert_that(i.point, is(equal_to(Vec3::new(0.0, 0.0, 0.0))));
        assert_that(i.normal, is(equal_to(Vec3::new(0.0, 0.0, 1.0))));
    }
}
