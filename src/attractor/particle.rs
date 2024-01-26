use nannou::{glam::Vec3Swizzles, prelude::*};

use super::AttractorParam;

pub(super) struct Particle {
    orbit: Vec<Vec3>,
    last: Vec3,
}

impl Particle {
    pub(super) fn new<Param: AttractorParam>() -> Self {
        let last = Param::random_point();

        Particle {
            orbit: vec![last; Param::ORBIT_LEN],
            last,
        }
    }

    pub(super) fn update<Param: AttractorParam>(&mut self) {
        let last = Param::make_next(&self.last);
        self.orbit.push(last);
        self.last = last;

        if self.orbit.len() > Param::ORBIT_LEN {
            self.orbit.remove(0);
        }
    }

    pub(super) fn draw<Param: AttractorParam>(
        &self,
        draw: &Draw,
        rotation: Mat3,
        center: Vec3,
        camera: Vec3,
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

fn equirectangular(camera: &Vec3, p: &Vec3) -> Vec2 {
    let dist_xy = camera.xy().distance(p.xy());
    let longitude = ((p.x - camera.x) / dist_xy).acos() * (p.y - camera.y).signum();
    let latitude = ((p.z - camera.z) / dist_xy).atan();

    vec2(longitude, latitude)
}