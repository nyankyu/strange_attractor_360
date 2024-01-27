use std::collections::VecDeque;

use nannou::{glam::Vec3Swizzles, prelude::*};

use super::AttractorParam;

pub(super) struct Particle {
    orbit: VecDeque<Vec3A>,
}

impl Particle {
    pub(super) fn new<Param: AttractorParam>() -> Self {
        let mut list = VecDeque::with_capacity(Param::ORBIT_LEN + 1);
        list.push_back(Param::random_point());

        Particle {
            orbit: list
        }
    }

    pub(super) fn update<Param: AttractorParam>(&mut self) {
        let last = Param::make_next(&self.orbit.back().unwrap());
        self.orbit.push_back(last);

        if self.orbit.len() > Param::ORBIT_LEN {
            self.orbit.pop_front();
        }
    }

    pub(super) fn draw<Param: AttractorParam>(
        &self,
        draw: &Draw,
        rotation: Mat3A,
        center: Vec3A,
        camera: Vec3A,
    ) {
        let mut coordinate_depth = self.orbit.iter().map(|&p| {
            let rotated = rotation * (p - center);
            let coordinate = equirectangular(&camera, &rotated);
            let depth = camera.distance(rotated);
            (coordinate, depth)
        });

        let mut pre = coordinate_depth.next().unwrap().0;
        for (coordinate, depth) in coordinate_depth {
            if pre.distance(coordinate) < PI / crate::WINDOW_H as f32 {
                continue;
            }

            let color = if depth < 0.2 {
                rgba8(0, 0, 0, 0)
            } else {
                rgba8(180, 0, 0, 255)
            };
            let weight = 2.0 * (Param::ORBIT_WEIGHT2 / depth).atan();

            let len_x = (pre.x - coordinate.x).abs();
            if len_x > PI {
                let center_y = (pre.y + coordinate.y) / 2.0;
                draw.line()
                    .weight(weight)
                    .join_round()
                    .start(vec2(PI * pre.x.signum(), center_y))
                    .end(pre)
                    .color(color);
                draw.line()
                    .weight(weight)
                    .join_round()
                    .start(vec2(PI * coordinate.x.signum(), center_y))
                    .end(coordinate)
                    .color(color);
            } else {
                draw.line()
                    .weight(weight)
                    .join_round()
                    .start(pre)
                    .end(coordinate)
                    .color(color);
            }

            pre = coordinate;
        }
    }
}

fn equirectangular(camera: &Vec3A, p: &Vec3A) -> Vec2 {
    let dist_xy = camera.xy().distance(p.xy());
    let longitude = ((p.x - camera.x) / dist_xy).acos() * (p.y - camera.y).signum();
    let latitude = ((p.z - camera.z) / dist_xy).atan();

    vec2(longitude, latitude)
}
