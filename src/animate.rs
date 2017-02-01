use vec::Vec3;
use scene::{ Scene, Sphere };
use camera::Camera;
use color::Color;

#[derive(Clone, Debug)]
pub struct Keyframe {
    t: usize,
    camera_position: Vec3,
    camera_look_at: Vec3,
    positions: Vec<Vec3>,
}

impl Keyframe {
    pub fn new(
        t: usize,
        camera_position: Vec3,
        camera_look_at: Vec3,
        positions: Vec<Vec3>
    ) -> Keyframe {
        Keyframe {
            t: t,
            camera_position: camera_position,
            camera_look_at: camera_look_at,
            positions: positions,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Keyframes {
    frames: Vec<Keyframe>,
}

impl Keyframes {

    pub fn new(frames: Vec<Keyframe>) -> Keyframes {
        Keyframes { frames: frames }
    }

    fn get_or_last(&self, i: usize) -> Keyframe {
        let last = self.frames.len()-1;

        if i < last {
            self.frames[i].clone()
        } else {
            self.frames[last].clone()
        }
    }

    fn pos(&self, t: usize, s: usize) -> Vec3 {
        for i in 0..self.frames.len() {
            let next = self.get_or_last(i);

            if next.t > t {
                let prev = self.get_or_last(i-1);

                let p = (t - prev.t) as f64 / (next.t - prev.t) as f64;

                let prev_vec = prev.positions[s];
                let next_vec = next.positions[s];

                let delta = next_vec - prev_vec;

                return prev_vec + (delta * p);
            }
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    fn cam(&self, t: usize) -> Vec3 {
        for i in 0..self.frames.len() {
            let next = self.get_or_last(i);

            if next.t > t {
                let prev = self.get_or_last(i-1);

                let p = (t - prev.t) as f64 / (next.t - prev.t) as f64;

                let prev_vec = prev.camera_position;
                let next_vec = next.camera_position;

                let delta = next_vec - prev_vec;

                println!("cam: {:?}", prev_vec + (delta * p));
                return prev_vec + (delta * p);
            }
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    fn look(&self, t: usize) -> Vec3 {
        for i in 0..self.frames.len() {
            let next = self.get_or_last(i);

            if next.t > t {
                let prev = self.get_or_last(i-1);

                let p = (t - prev.t) as f64 / (next.t - prev.t) as f64;

                let prev_vec = prev.camera_look_at;
                let next_vec = next.camera_look_at;

                let delta = next_vec - prev_vec;

                println!("look: {:?}", prev_vec + (delta * p));
                return prev_vec + (delta * p);
            }
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
}

pub fn animate(scene: &Scene, camera: &Camera, frames: &Keyframes, t: usize) -> (Scene, Camera) {
    let mut shapes = Vec::new();
    for s in 0..scene.shapes.len() {
        shapes.push(scene.shapes[s].move_to(frames.pos(t, s)));
    }
    (Scene::new(shapes), camera.look_at(frames.look(t)).move_to(frames.cam(t)))
}
