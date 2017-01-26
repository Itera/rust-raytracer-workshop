extern crate raytracer;

use raytracer::prelude::*;

fn create_camera(width: u32, height: u32) -> Camera {
    let origin = Vec3::new(0.0, 1.0, 2.0);
    let view_point = Vec3::new(0.0, 0.0, -1.0);
    let orthogonal_up = Vec3::new(0.0, 1.0, 0.0);
    let vertical_field_of_view = 35.0;
    let aspect_ratio = width as f64 / height as f64;
    let aperture = 0.0;
    let distance_to_focus = (origin - view_point).length();
    // panic!("Step 2a) Initialize and return a new Camera by calling its 'new' function with the parameters defined above");
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


fn main() {
    let (width, height, number_of_samples) = (
        // panic!("Step 2a) Initialize a camera with width = 600"),
        600,
        // panic!("Step 2a) Initialize a camera with height = 300"),
        300,
        50
    );
    let camera = create_camera(width, height);
    let scene = create_scene();

    let image = raytracer::trace_scene(width, height, number_of_samples, &camera, &scene);

    let _ = image.save("scene.bmp");
}
