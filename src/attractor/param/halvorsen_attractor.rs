use crate::AttractorParam;
use crate::WINDOW_H;
use nannou::glam::const_mat3a;
use nannou::prelude::*;

const A: f32 = 1.4;
const MAT: Mat3A = const_mat3a!([-A, -4.0, -4.0], [-4.0, -A, -4.0], [-4.0, -4.0, -A]);

pub(crate) struct HalvorsenAttractor {}

impl AttractorParam for HalvorsenAttractor {
    const ORBIT_NUM: usize = 3000;
    const ORBIT_LEN: usize = 400;
    const DRAW_SKIP: usize = Self::ORBIT_LEN * 2;
    const ORBIT_WEIGHT: f32 = 6.0 / WINDOW_H as f32;
    const ORBIT_WEIGHT2: f32 = Self::ORBIT_WEIGHT / 2.0;

    const DELTA_T: f32 = 0.001;

    const CAMERA_X: f32 = -4.0;
    const CAMERA_Y: f32 = 0.0;
    const CAMERA_Z: f32 = -5.0;

    const CENTER_X: f32 = 0.0;
    const CENTER_Y: f32 = 0.0;
    const CENTER_Z: f32 = 0.0;

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 7.9;
    const ROTAION_Z: f32 = 1.3;

    const COLOR_R: u8 = 0;
    const COLOR_G: u8 = 100;
    const COLOR_B: u8 = 180;

    fn new() -> Self {
        HalvorsenAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-4.0, 4.0),
            random_range(-4.0, 4.0),
            random_range(-4.0, 4.0),
        )
    }

    fn make_next(p: &Vec3A) -> Vec3A {
        let d = MAT * *p - vec3a(p.y * p.y, p.z * p.z, p.x * p.x);
        *p + d * Self::DELTA_T
    }
}
