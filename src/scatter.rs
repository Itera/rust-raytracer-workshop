use rand::{self, Rng};
use bmp;

use ::vec::Vec3;
use ::ray::Ray;
use ::color::Color;
use ::scene::*;

const INTERSECTION_ORIGIN_OFFSET: f64 = 0.00000001;

pub fn diffusive(attenuation: Color, intersection: &Intersection) -> Option<(Color, Ray)> {
    Some((attenuation, scatter_ray(intersection)))
}

pub fn reflection(attenuation: Color,
                  diffusiveness: f64,
                  ray: &Ray,
                  intersection: &Intersection)
                  -> Option<(Color, Ray)> {
    let reflected = reflect(ray.direction, intersection.normal) +
                    diffusiveness * random_point_in_unit_sphere();
    let origin = reflection_origin(intersection);
    if reflected.dot(intersection.normal) > 0.0 {
        Some((attenuation, Ray::new(origin, reflected)))
    } else {
        None
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
    let should_refract = refracted.is_some() &&
                         shlick_approximation(cosine, refraction_index) <
                         rand::thread_rng().next_f64();

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

pub fn texture(texture: &bmp::Image, intersection: &Intersection) -> Option<(Color, Ray)> {
    panic!("Step 6b) Calculate the (u, v) coordinates of the surface normal in the intersection, \
            similarily to how you did it in Step 5. Then, convert the respective pixel from the \
            texture to a Color. You can use the scatter_ray() function below to calculate the Ray.")
}

fn scatter_ray(intersection: &Intersection) -> Ray {
    let target = intersection.intersection_point + intersection.normal +
                 random_point_in_unit_sphere();
    let origin = reflection_origin(intersection);
    let direction = (target - origin).normalize();
    Ray::new(origin, direction)
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
    (v - 2.0 * v.dot(n) * n).normalize()
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((ni_over_nt * (uv - n * dt) - n * discriminant.sqrt()).normalize())
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
