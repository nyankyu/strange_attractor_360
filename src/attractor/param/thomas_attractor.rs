use nannou::prelude::*;
use crate::WINDOW_H;
use crate::AttractorParam;

const B: f32 = 0.205;

pub(crate) struct ThomasAttractor {}

impl AttractorParam for ThomasAttractor {
    const ORBIT_NUM: usize = 8000;
    const ORBIT_LEN: usize = 7;
    const ORBIT_WEIGHT: f32 = 6.0 / WINDOW_H as f32;
    const ORBIT_WEIGHT2: f32 = Self::ORBIT_WEIGHT / 2.0;

    const DELTA_T: f32 = 0.03;

    const CAMERA_X: f32 = -1.0;
    const CAMERA_Y: f32 = 0.0;
    const CAMERA_Z: f32 = 0.0;

    const CENTER_X: f32 = 4.0;
    const CENTER_Y: f32 = 0.0;
    const CENTER_Z: f32 = 0.0;

    const DELTA_THETA: f32 = 0.0004;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 7.9;
    const ROTAION_Z: f32 = 1.3;

    fn new() -> Self {
        ThomasAttractor {}
    }

    fn random_point() -> Vec3 {
        vec3(
            random_range(-10.0, 10.0),
            random_range(-10.0, 10.0),
            random_range(-10.0, 10.0),
        )
    }

    fn make_next(p: &Vec3) -> Vec3 {
        let dx = p.y.sin() - B * p.x;
        let dy = p.z.sin() - B * p.y;
        let dz = p.x.sin() - B * p.z;
        *p + vec3(dx, dy, dz) * Self::DELTA_T
    }
}
