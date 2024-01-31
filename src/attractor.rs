mod param;
mod particle;

use std::marker::PhantomData;

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
    const DRAW_SKIP: usize;

    const DELTA_T: f32;

    /// direction right: +y, left: -y, top: +z, bottom: -z, front: +x, back: -x
    const CAMERA: Vec3A;
    const CENTER: Vec3A;

    const ORBIT_WEIGHT: f32;
    const ORBIT_WEIGHT2: f32;
    const DELTA_THETA: f32;

    const ROTAION_X: f32;
    const ROTAION_Y: f32;
    const ROTAION_Z: f32;

    const COLOR: Rgba8;

    fn new() -> Self;
    fn random_point() -> Vec3A;
    fn make_next(p: &Vec3A) -> Vec3A;
}

pub(crate) struct Attractor<Param: AttractorParam> {
    _param: PhantomData<fn() -> Param>,
    orbits: Vec<Particle<Param>>,
    theta: f32,
    rotation: Mat3A,
}

impl<Param: AttractorParam> Attractor<Param> {
    pub(crate) fn new() -> Self {
        Attractor {
            _param: PhantomData,
            orbits: (0..Param::ORBIT_NUM)
                .map(|_| {
                    let mut particle = Particle::new();
                    for _ in 0..Param::DRAW_SKIP {
                        particle.update();
                    }
                    particle
                })
                .collect(),
            theta: 0.0,
            rotation: Mat3A::ZERO,
        }
    }

    pub(crate) fn update(&mut self) {
        self.orbits.iter_mut().for_each(|p| p.update());
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
        self.orbits
            .iter()
            .for_each(|particle| particle.draw(&draw, self.rotation));
        if !crate::RECORDING {
            self.draw_axis::<Param>(&draw);
        }
    }

    fn draw_axis<AttractorParam>(&self, draw: &Draw) {
        let origin = self.coordinate::<Param>(&Vec3A::ZERO);
        let axis_x = self.coordinate::<Param>(&vec3a(500.0, 0.0, 0.0));
        let axis_y = self.coordinate::<Param>(&vec3a(0.0, 500.0, 0.0));
        let axis_z = self.coordinate::<Param>(&vec3a(0.0, 0.0, 500.0));

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

    fn coordinate<AttractorParam>(&self, p: &Vec3A) -> Vec2 {
        let rotated = self.rotation * (*p - Param::CENTER);
        let dist_xy = Param::CAMERA.xy().distance(rotated.xy());
        let longitude = ((rotated.x - Param::CAMERA.x) / dist_xy).acos()
            * (rotated.y - Param::CAMERA.y).signum();
        let latitude = ((rotated.z - Param::CAMERA.z) / dist_xy).atan();

        vec2(longitude, latitude)
    }
}
