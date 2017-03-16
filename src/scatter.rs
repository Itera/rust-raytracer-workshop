use rand::{self, Rng};

use ::vec::Vec3;
use ::ray::Ray;
use ::color::Color;
use ::scene::*;

const INTERSECTION_ORIGIN_OFFSET: f64 = 0.00000001;

pub fn scatter_ray(intersection: &Intersection) -> Ray {
    let target = intersection.intersection_point + intersection.normal +
                 random_point_in_unit_sphere();
    let origin = reflection_origin(intersection);
    let direction = (target - origin).normalize();
    Ray::new(origin, direction)
}

pub fn diffusive(attenuation: Color, intersection: &Intersection) -> Option<(Color, Ray)> {
    Some((attenuation, scatter_ray(intersection)))
}

pub fn reflection(attenuation: Color,
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

pub fn refraction(refraction_index: f64,
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
        Some(_) => shlick_approximation(cosine, refraction_index) < rand::thread_rng().next_f64(),
        None => false,
    };

    match refracted {
        Some(refracted) if should_refract => {
            let origin = refraction_origin(intersection.intersection_point, outward_normal);
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
    intersection.intersection_point + intersection.normal * INTERSECTION_ORIGIN_OFFSET
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

fn shlick_approximation(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
