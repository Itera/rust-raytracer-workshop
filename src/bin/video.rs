extern crate raytracer;
extern crate gif;

use raytracer::prelude::*;
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::fs::File;

fn create_camera(width: u32, height: u32) -> Camera {
    let origin = Vec3::new(0.0, 1.0, 2.0);
    let view_point = Vec3::new(0.0, 0.0, -1.0);
    let orthogonal_up = Vec3::new(0.0, 1.0, 0.0);
    let vertical_field_of_view = 35.0;
    let aspect_ratio = width as f64 / height as f64;
    let aperture = 0.2;
    let distance_to_focus = (origin - view_point).length();
    Camera::new(origin,
                view_point,
                orthogonal_up,
                vertical_field_of_view,
                aspect_ratio,
                aperture,
                distance_to_focus)
}

fn create_scene() -> Scene {
    Scene::new(vec![
        Box::new(Sphere::refractive(
            Vec3::new(0.5, -0.2, -0.4),
            0.3,
            Color::new(0.6, 0.3, 0.0),
            1.5),
        ),
        Box::new(Sphere::reflective(
            Vec3::new(1.1, 0.0, -1.2),
            0.5,
            Color::new(0.6, 0.6, 0.6),
            0.0),
        ),
        Box::new(Sphere::refractive(
            Vec3::new(-1.1, 0.0, -0.8),
            0.5,
            Color::new(0.7, 0.3, 0.7),
            1.5),
        ),
        Box::new(Sphere::refractive(
            Vec3::new(-1.1, 0.0, -0.8),
            -0.47,
            Color::new(0.7, 0.3, 0.7),
            1.5),
        ),
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Color::new(0.1, 0.2, 0.5))),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Color::new(0.5, 0.8, 0.0))),
    ])
}

fn create_frames() -> Keyframes {
    Keyframes::new(vec![
        Keyframe::new(
            0,
            Vec3::new(0.0, 1.0, 2.0),
            Vec3::new(0.0, 0.0, -1.0),
            vec![Vec3::new(0.5, -0.2, -0.4), Vec3::new(1.1, 0.0, -1.2), Vec3::new(-1.1, 0.0, -0.8), Vec3::new(-1.1, 0.0, -0.8), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, -100.5, -1.0)]
        ),
        Keyframe::new(
            25,
            Vec3::new(1.0, 1.0, 2.0),
            Vec3::new(-0.5, 0.0, -1.0),
            vec![Vec3::new(1.0, -0.2, -0.4), Vec3::new(1.1, 0.0, -1.2), Vec3::new(-1.1, 0.0, -0.8), Vec3::new(-1.1, 0.0, -0.8), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, -100.5, -1.0)]
        ),
        Keyframe::new(
            50,
            Vec3::new(2.0, 1.0, 2.0),
            Vec3::new(-1.0, 0.0, -1.0),
            vec![Vec3::new(2.0, -0.2, -0.4), Vec3::new(1.1, 0.0, -1.2), Vec3::new(-1.1, 0.0, -0.8), Vec3::new(-1.1, 0.0, -0.8), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, -100.5, -1.0)]
        )
    ])
}

fn main() {
    let (width, height, number_of_samples) = (300, 150, 2);
    let orginal_camera = create_camera(width, height);
    let orginal_scene = create_scene();
    let frames = create_frames();

    let mut image = File::create("video.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width as u16, height as u16, &[]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    for i in 0..50 {
        let (scene, camera) = animate(&orginal_scene, &orginal_camera, &frames, i);

        let mut pixels = pixels_to_vec(raytracer::trace_scene(width, height, number_of_samples, &camera, &scene));

        let frame = Frame::from_rgb(width as u16, height as u16, &mut *pixels);

        // Write frame to file
        encoder.write_frame(&frame).unwrap();
    }
}

fn pixels_to_vec(pixels: Vec<Color>) -> Vec<u8> {
    let mut vec : Vec<u8> = Vec::with_capacity(pixels.len() * 3);

    for Color {r, g, b} in pixels {
        vec.push((r * 255.0) as u8);
        vec.push((g * 255.0) as u8);
        vec.push((b * 255.0) as u8);
    }

    return vec;
}
