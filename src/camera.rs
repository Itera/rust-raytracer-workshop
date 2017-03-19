use std::f64::consts::PI;
use rand::{self, Rng};

use prelude::*;

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    view_point: Vec3,
    lens_radius: f64,
    orthogonal_up: Vec3,
    vertical_field_of_view: f64,
    aspect_ratio: f64,
    aperture: f64,
    distance_to_focus: f64,
}

impl Camera {
    pub fn new(origin: Vec3,
               view_point: Vec3,
               orthogonal_up: Vec3,
               vertical_field_of_view: f64,
               aspect_ratio: f64,
               aperture: f64,
               distance_to_focus: f64)
               -> Camera {
        let theta = vertical_field_of_view * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = (origin - view_point).normalize();
        let u = (orthogonal_up.cross(w)).normalize();
        let v = w.cross(u);
        let lower_left_corner = origin - half_width * distance_to_focus * u -
                                half_height * distance_to_focus * v -
                                distance_to_focus * w;
        let horizontal = 2.0 * half_width * distance_to_focus * u;
        let vertical = 2.0 * half_height * distance_to_focus * v;
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
            view_point: view_point,
            orthogonal_up: orthogonal_up,
            vertical_field_of_view: vertical_field_of_view,
            aspect_ratio: aspect_ratio,
            aperture: aperture,
            distance_to_focus: distance_to_focus,
        }
    }

    pub fn create_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_point_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;
        let direection = self.lower_left_corner + u * self.horizontal + v * self.vertical -
                         self.origin - offset;
        Ray::new(self.origin + offset, direection.normalize())
    }

    pub fn look_at(&self, at: Vec3) -> Camera {
        Camera::new(self.origin,
                    at,
                    self.orthogonal_up,
                    self.vertical_field_of_view,
                    self.aspect_ratio,
                    self.aperture,
                    self.distance_to_focus)
    }

    pub fn move_to(&self, origin: Vec3) -> Camera {
        Camera::new(origin,
                    self.view_point,
                    self.orthogonal_up,
                    self.vertical_field_of_view,
                    self.aspect_ratio,
                    self.aperture,
                    self.distance_to_focus)
    }
}

fn random_point_in_unit_disc() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.next_f64(), rng.next_f64(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}
