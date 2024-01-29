use nannou::prelude::*;
use crate::WINDOW_H;
use crate::AttractorParam;

const SIGMA: f32 = 10.0;
const BETA: f32 = 8.0 / 3.0;
const RHO: f32 = 28.0;

pub(crate) struct LorenzAttractor {}

impl AttractorParam for LorenzAttractor {
    const ORBIT_NUM: usize = 40;
    const ORBIT_LEN: usize = 600;
    const DRAW_SKIP: usize = Self::ORBIT_LEN * 2;
    const ORBIT_WEIGHT: f32 = 20.0 / WINDOW_H as f32;
    const ORBIT_WEIGHT2: f32 = Self::ORBIT_WEIGHT / 2.0;

    const DELTA_T: f32 = 0.001;

    const CAMERA_X: f32 = -10.0;
    const CAMERA_Y: f32 = 0.0;
    const CAMERA_Z: f32 = 15.0;

    const CENTER_X: f32 = 0.0;
    const CENTER_Y: f32 = 0.0;
    const CENTER_Z: f32 = 15.0;

    const DELTA_THETA: f32 = 0.0006;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 7.9;
    const ROTAION_Z: f32 = 1.3;

    fn new() -> Self {
        LorenzAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-30.0, 30.0),
            random_range(-30.0, 30.0),
            random_range(0.0, 60.0),
        )
    }

    fn make_next(p: &Vec3A) -> Vec3A {
        let dx = SIGMA * (p.y - p.x);
        let dy = p.x * (RHO - p.z) - p.y;
        let dz = p.x * p.y - BETA * p.z;
        *p + vec3a(dx, dy, dz) * Self::DELTA_T
    }
}