#![allow(unused_variables)]
#![allow(dead_code)]

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

extern crate rand;
extern crate bmp;
extern crate rayon;

use std::f64;
use rand::Rng;
use prelude::*;

mod scatter;
mod vec;
mod ray;
mod color;
mod camera;
mod scene;
mod matrix;
mod animate;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use ray::Ray;
    pub use vec::Vec3;
    pub use matrix::Matrix4;
    pub use color::Color;
    pub use camera::Camera;
    pub use scene::{Scene, Sphere, Intersectable};
    pub use animate::{animate, Keyframes, Keyframe};
}

pub fn trace_scene(width: u32,
                   height: u32,
                   num_samples: u32,
                   camera: &Camera,
                   scene: &Scene)
                   -> Vec<Color> {
    let mut rng = rand::thread_rng();
    let mut pixels = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let (x_trans, y_trans) = (x as f64, y as f64);
            let mut color = Color::black();
            for _ in 0..num_samples {
                let u = (x_trans + rng.next_f64()) / width as f64;
                let v = ((height as f64 - y_trans - 1.0) + rng.next_f64()) / height as f64;

                let ray = camera.create_ray(u, v);
                color = panic!("Step 2b) Call the 'trace_ray_in_scene' function with the appropriate \
                                parameters");
            }
            color = color / num_samples as f64;
            pixels.push(color);
        }
    }
    pixels
}

fn trace_ray_in_scene(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    if depth == 50 {
        return Color::black(); // Return black to avoid being stuck with an unlimited recursion
    }
    panic!("Step 2b) Return a gradient by calling the 'gradient' function, passing the ray as \
            parameter")
}

fn gradient(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
}
