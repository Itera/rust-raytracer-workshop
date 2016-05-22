#[cfg(test)]
extern crate hamcrest;

extern crate rand;
extern crate bmp;

use bmp::Image;
use rand::{ Rng };

use vec::Vec3;
use ray::Ray;
use camera::Camera;
use material::{ Material, Lambertian, Color };
use scene::{ Scene, Sphere, Intersectable };

#[allow(dead_code)]
mod vec;
#[allow(dead_code)]
mod ray;
mod camera;
mod material;
mod scene;

fn gradient(point: Vec3) -> Color {
    let t = 0.5 * (point.y + 1.0);
    (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
}

fn color(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    if depth == 50 { return Color::black(); }

    match scene.intersects(ray) {
        Some(ref intersection) => {
            if let Some((attenuation, scattered)) = intersection.material.scatter(ray, intersection) {
                attenuation * color(&scattered, scene, depth + 1)
            } else {
                Color::black()
            }
        },
        None => gradient(ray.direction.normalize())
    }
}

fn create_camera() -> Camera {
    Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0))
}

fn create_scene() -> Scene {
    Scene::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))))),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Color::new(0.5, 0.8, 0.0))))),
    ])
}

fn main() {
    let (width, height) = (300, 150);

    let num_samples = 100;
    let mut rng = rand::thread_rng();

    let camera = create_camera();
    let scene = create_scene();

    let mut image = Image::new(width, height);
    for (x, y) in image.coordinates() {
        let mut c = Color::black();
        for _ in 0 .. num_samples {
            let u = (x as f64 + rng.next_f64()) / width as f64;
            let v = ((height - y - 1) as f64 + rng.next_f64()) / height as f64;

            let ray = camera.create_ray(u, v);
            c = c + color(&ray, &scene, 0);
        }
        c = c / num_samples as f64;

        // use gamma 2 to achieve more natural ligthning
        // achieved by raising the color to the power 1/gamma
        image.set_pixel(x, y, c.gamma2().into());
    }
    let _ = image.save("scene.bmp");
}
