use nannou::prelude::*;
use crate::WINDOW_H;
use crate::AttractorParam;

const A: f32 = 1.33;

pub(crate) struct HalvorsenAttractor {}

impl AttractorParam for HalvorsenAttractor {
    const ORBIT_NUM: usize = 22000;
    const ORBIT_LEN: usize = 5;
    const ORBIT_WEIGHT: f32 = 17.0 / WINDOW_H as f32;
    const ORBIT_WEIGHT2: f32 = Self::ORBIT_WEIGHT / 2.0;

    const DELTA_T: f32 = 0.003;

    const CAMERA_X: f32 = -5.0;
    const CAMERA_Y: f32 = 0.0;
    const CAMERA_Z: f32 = -7.0;

    const CENTER_X: f32 = 7.0;
    const CENTER_Y: f32 = 0.0;
    const CENTER_Z: f32 = 5.0;

    const DELTA_THETA: f32 = 0.001;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 7.9;
    const ROTAION_Z: f32 = 1.3;

    fn new() -> Self {
        HalvorsenAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-2.0, 2.0),
            random_range(-2.0, 2.0),
            random_range(-2.0, 2.0),
        )
    }

    fn make_next(p: &Vec3A) -> Vec3A {
        let dx = -vec3a(A, 4.0, 4.0).dot(*p) - p.y * p.y;
        let dy = -vec3a(4.0, A, 4.0).dot(*p) - p.z * p.z;
        let dz = -vec3a(4.0, 4.0, A).dot(*p) - p.x * p.x;
        *p + vec3a(dx, dy, dz) * Self::DELTA_T
    }
}
