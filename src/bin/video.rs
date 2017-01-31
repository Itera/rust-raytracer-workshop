extern crate raytracer;
extern crate bmp;

use raytracer::prelude::*;
use bmp::{Image, Pixel};

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

    for i in 0..50 {
        let (scene, camera) = animate(&orginal_scene, &orginal_camera, &frames, i);
        let pixels = raytracer::trace_scene(width, height, number_of_samples, &camera, &scene);
        pixel_array_to_image(i, width, height, pixels)
    }
}

fn pixel_array_to_image(t: usize, width: u32, height: u32, pixels: Vec<Color>) {
    let mut image = Image::new(width, height);
    for y in 0..height {
        for x in 0..width {
            image.set_pixel(x,
                            height - y - 1,
                            to_pixel(pixels[(y * width + x) as usize].gamma2()));
        }
    }
    let _ = image.save(format!("scenes/scene-{:03}.bmp", t).as_str());
}

fn to_pixel(color: Color) -> Pixel {
    Pixel::new((255.99 * color.r) as u8,
               (255.99 * color.g) as u8,
               (255.99 * color.b) as u8)
}
