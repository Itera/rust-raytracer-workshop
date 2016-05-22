#[cfg(test)]
extern crate hamcrest;

extern crate bmp;

use bmp::Image;

use vec::Vec3;
use ray::Ray;
use camera::Camera;
use material::Color;
use scene::{ Scene, Sphere };

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
    gradient(ray.direction.normalize())
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
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
    ])
}

fn main() {
    let (width, height) = (300, 150);

    let camera = create_camera();
    let scene = create_scene();

    let mut image = Image::new(width, height);
    for (x, y) in image.coordinates() {
        let u = x as f64 / width as f64;
        let v = (height - y - 1) as f64 / height as f64;

        let ray = camera.create_ray(u, v);
        let c = color(&ray, &scene, 0);

        // use gamma 2 to achieve more natural ligthning
        // achieved by raising the color to the power 1/gamma
        image.set_pixel(x, y, c.gamma2().into());
    }
    let _ = image.save("scene.bmp");
}
