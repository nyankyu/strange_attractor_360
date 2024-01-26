mod particle;
mod param;

#[allow(unused_imports)]
pub(crate) use param::lorenz_attractor::LorenzAttractor;
#[allow(unused_imports)]
pub(crate) use param::halvorsen_attractor::HalvorsenAttractor;
#[allow(unused_imports)]
pub(crate) use param::thomas_attractor::ThomasAttractor;

use nannou::prelude::*;
use particle::Particle;

const SCALE: f32 = crate::WINDOW_H as f32 / PI;

pub(crate) trait AttractorParam {
    const ORBIT_NUM: usize;
    const ORBIT_LEN: usize;

    const DELTA_T: f32;

    const CAMERA_X: f32;
    const CAMERA_Y: f32;
    const CAMERA_Z: f32;

    const CENTER_X: f32;
    const CENTER_Y: f32;
    const CENTER_Z: f32;

    const ORBIT_WEIGHT: f32;
    const ORBIT_WEIGHT2: f32;
    const DELTA_THETA: f32;

    const ROTAION_X: f32;
    const ROTAION_Y: f32;
    const ROTAION_Z: f32;

    fn new() -> Self;
    fn random_point() -> Vec3;
    fn make_next(p: &Vec3) -> Vec3;
}

pub(crate) struct Attractor<Param: AttractorParam> {
    _param: Param,
    orbits: Vec<Particle>,
    theta: f32,
    rotation: Mat3,
    /// direction right: +y, left: -y, top: +z, bottom: -z, front: +x, back: -x
    camera: Vec3,
    center: Vec3,
}

impl<Param: AttractorParam> Attractor<Param> {
    pub(crate) fn new() -> Self {
        Attractor {
            _param: Param::new(),
            orbits: (0..Param::ORBIT_NUM).map(|_| Particle::new::<Param>()).collect(),
            theta: 0.0,
            rotation: Mat3::ZERO,
            camera: vec3(Param::CAMERA_X, Param::CAMERA_Y, Param::CAMERA_Z),
            center: vec3(Param::CENTER_X, Param::CENTER_Y, Param::CENTER_Z),
        }
    }

    pub(crate) fn update(&mut self) {
        self.orbits.iter_mut().for_each(|p| p.update::<Param>());
        self.theta += Param::DELTA_THETA;
        self.rotation = Mat3::from_euler(
            nannou::glam::EulerRot::ZYX,
            self.theta * Param::ROTAION_X,
            self.theta * Param::ROTAION_Y,
            self.theta * Param::ROTAION_Z,
        );
    }
    pub(crate) fn draw(&self, draw: &Draw) {
        let draw = draw.scale(SCALE);
        self.orbits.iter().for_each(|particle| {
            particle.draw::<Param>(&draw, self.rotation, self.center, self.camera)
        });
        
    }
}
