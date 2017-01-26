#![allow(unused_variables)]
#![allow(unused_assignments)]

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

extern crate rand;
extern crate bmp;
extern crate rayon;

use std::f64;
use bmp::Image;
use rand::Rng;
use prelude::*;

mod scatter;
mod vec;
mod ray;
mod color;
mod camera;
mod scene;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use ray::Ray;
    pub use vec::Vec3;
    pub use color::Color;
    pub use camera::Camera;
    pub use scene::{Scene, Sphere, Intersectable};
}

pub fn trace_scene(width: u32,
                   height: u32,
                   num_samples: u32,
                   camera: &Camera,
                   scene: &Scene)
                   -> Image {
    let mut rng = rand::thread_rng();
    let mut image = Image::new(width, height);
    for (x, y) in image.coordinates() {
        let (x_trans, y_trans) = (x as f64, (height - y - 1) as f64);
        let mut color = Color::black();
        for _ in 0..num_samples {
            let u = (x_trans + rng.next_f64()) / width as f64;
            let v = (y_trans + rng.next_f64()) / height as f64;

            let ray = camera.create_ray(u, v);
            color = color + trace_ray_in_scene(&ray, scene, 0);
            // color = panic!("Step 2b) Call the 'trace_ray_in_scene' function with the appropriate parameters");
        }
        color = color / num_samples as f64;
        image.set_pixel(x, y, color.gamma2().into());
    }
    image
}

fn trace_ray_in_scene(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    if depth == 50 {
        return Color::black(); // Return black to avoid being stuck with an unlimited recursion
    }
    // _ => panic!("Step 2b) Return a gradient by calling the 'gradient' function, passing the ray as parameter")
    match scene.intersects(ray, 0.0, f64::MAX) {
        Some(intersection) => {
            match intersection.shape.scatter(ray, &intersection) {
                Some((attenuation, scattered)) => {
                    attenuation * trace_ray_in_scene(&scattered, scene, depth + 1)
                }
                None => Color::black(),
            }
        }
        None => gradient(ray),
    }
}

fn gradient(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
}
