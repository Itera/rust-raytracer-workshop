use ray::Ray;

pub use self::shapes::Sphere;
pub use self::intersection::{ Intersectable, Intersection };

mod shapes;
pub mod intersection;

pub struct Scene {
    pub shapes: Vec<Box<Intersectable>>,
}

impl Scene {
    pub fn new(shapes: Vec<Box<Intersectable>>) -> Scene {
        Scene {
            shapes: shapes,
        }
    }
}

impl Intersectable for Scene {
    fn intersects_with_limits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;
        let mut closest_so_far: f64 = t_max;

        for shape in self.shapes.iter() {
            match shape.intersects_with_limits(ray, t_min, closest_so_far) {
                Some(new_intersection) => {
                    closest_so_far = new_intersection.t;
                    intersection = Some(new_intersection);
                },
                None => ()
            }
        }
        intersection
    }
}
