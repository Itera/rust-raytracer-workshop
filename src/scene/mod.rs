use vec::Vec3;
use ray::Ray;

pub use self::shapes::Sphere;

mod shapes;

pub struct Scene {
    pub shapes: Vec<Sphere>,
}

impl Scene {
    pub fn new(shapes: Vec<Sphere>) -> Scene {
        Scene {
            shapes: shapes,
        }
    }
}
