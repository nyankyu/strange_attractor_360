use std::{collections::VecDeque, marker::PhantomData};

use nannou::{glam::Vec3Swizzles, prelude::*};

use super::AttractorParam;

const TRANSPARENT: Rgba8 = Rgba8 {
    color: BLACK,
    alpha: 0,
};

pub(super) struct Particle<Param: AttractorParam> {
    _param: PhantomData<fn() -> Param>,
    orbit: VecDeque<Vec3A>,
}

impl<Param: AttractorParam> Particle<Param> {
    pub(super) fn new() -> Self {
        let mut list = VecDeque::with_capacity(Param::ORBIT_LEN + 1);
        list.push_back(Param::random_point());

        Particle {
            _param: PhantomData,
            orbit: list,
        }
    }

    pub(super) fn update(&mut self) {
        let last = Param::make_next(&self.orbit.back().unwrap());
        self.orbit.push_back(last);

        if self.orbit.len() > Param::ORBIT_LEN {
            self.orbit.pop_front();
        }
    }

    pub(super) fn draw(&self, draw: &Draw, rotation: Mat3A) {
        let mut coordinate_depth = self.orbit.iter().map(|&p| {
            let rotated = rotation * (p - Param::CENTER);
            let coordinate = equirectangular::<Param>(&rotated);
            let depth = Param::CAMERA.distance(rotated);
            (coordinate, depth)
        });

        let mut pre = coordinate_depth.next().unwrap().0;
        for (coordinate, depth) in coordinate_depth {
            if pre.distance(coordinate) < PI / crate::WINDOW_H as f32 {
                continue;
            }

            let color = if depth < 0.2 {
                TRANSPARENT
            } else {
                Param::COLOR
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

fn equirectangular<Param: AttractorParam>(p: &Vec3A) -> Vec2 {
    let dist_xy = Param::CAMERA.xy().distance(p.xy());
    let longitude = ((p.x - Param::CAMERA.x) / dist_xy).acos() * (p.y - Param::CAMERA.y).signum();
    let latitude = ((p.z - Param::CAMERA.z) / dist_xy).atan();

    vec2(longitude, latitude)
}
