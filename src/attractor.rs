mod param;
mod particle;

use nannou::glam::Vec3Swizzles;
use nannou::prelude::*;

#[allow(unused_imports)]
pub(crate) use param::halvorsen_attractor::HalvorsenAttractor;
#[allow(unused_imports)]
pub(crate) use param::lorenz_attractor::LorenzAttractor;
#[allow(unused_imports)]
pub(crate) use param::thomas_attractor::ThomasAttractor;

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
    fn random_point() -> Vec3A;
    fn make_next(p: &Vec3A) -> Vec3A;
}

pub(crate) struct Attractor<Param: AttractorParam> {
    _param: Param,
    orbits: Vec<Particle>,
    theta: f32,
    rotation: Mat3A,
    /// direction right: +y, left: -y, top: +z, bottom: -z, front: +x, back: -x
    camera: Vec3A,
    center: Vec3A,
}

impl<Param: AttractorParam> Attractor<Param> {
    pub(crate) fn new() -> Self {
        Attractor {
            _param: Param::new(),
            orbits: (0..Param::ORBIT_NUM)
                .map(|_| Particle::new::<Param>())
                .collect(),
            theta: 0.0,
            rotation: Mat3A::ZERO,
            camera: vec3a(Param::CAMERA_X, Param::CAMERA_Y, Param::CAMERA_Z),
            center: vec3a(Param::CENTER_X, Param::CENTER_Y, Param::CENTER_Z),
        }
    }

    pub(crate) fn update(&mut self) {
        self.orbits.iter_mut().for_each(|p| p.update::<Param>());
        self.theta += Param::DELTA_THETA;
        self.rotation = Mat3A::from_euler(
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
        if !crate::RECORDING {
            self.draw_axis(&draw);
        }
    }

    fn draw_axis(&self, draw: &Draw) {
        let origin = self.coordinate(&Vec3A::ZERO);
        let axis_x = self.coordinate(&vec3a(500.0, 0.0, 0.0));
        let axis_y = self.coordinate(&vec3a(0.0, 500.0, 0.0));
        let axis_z = self.coordinate(&vec3a(0.0, 0.0, 500.0));

        draw.line()
            .start(origin)
            .end(axis_x)
            .weight(0.002)
            .color(BLUE);

        draw.line()
            .start(origin)
            .end(axis_y)
            .weight(0.002)
            .color(GREEN);

        draw.line()
            .start(origin)
            .end(axis_z)
            .weight(0.002)
            .color(WHITE);
    }

    fn coordinate(&self, p: &Vec3A) -> Vec2 {
        let rotated = self.rotation * (*p - self.center);
        let dist_xy = self.camera.xy().distance(rotated.xy());
        let longitude =
            ((rotated.x - self.camera.x) / dist_xy).acos() * (rotated.y - self.camera.y).signum();
        let latitude = ((rotated.z - self.camera.z) / dist_xy).atan();

        vec2(longitude, latitude)
    }
}
