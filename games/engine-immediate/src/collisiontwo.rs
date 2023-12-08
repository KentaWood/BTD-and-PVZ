pub use glam::*;

use crate::geom::{Balloon, Circle, Dart};

pub struct Collisiontwo {
    balloons: Vec<Balloon>,
    darts: Vec<Dart>,
    circles: Vec<Circle>,
}

// NOTE::: IDEALLY MOVE DEFAULT SIZES SO THEY AREN"T HARD CODED (LIKE 24.0, 128.0, etc)
impl Collisiontwo {
    pub fn new(balloons: &[Balloon], darts: &[Dart], circles: &[Circle]) -> Self {
        Collisiontwo {
            balloons: balloons.to_vec(),
            darts: darts.to_vec(),
            circles: circles.to_vec(),
        }
    }

    pub fn check_collision_dart(&self) -> Vec<(usize, usize)> {
        let mut collisions: Vec<(usize, usize)> = Vec::with_capacity(16);
        let mut monk = 0;
        for (dart_c, dart) in self.darts.iter().enumerate() {
            for balloon in self.balloons.iter() {
                if dart.pos.x + 20.0 > balloon.pos.x
                    && dart.pos.x < balloon.pos.x + 44.0
                    && dart.pos.y < balloon.pos.y + 44.0
                    && dart.pos.y + 30.0 > balloon.pos.y
                {
                    let tup = (dart_c, monk);
                    collisions.push(tup);
                }
                monk += 1;
            }
            monk = 0;
        }
        collisions
    }

    pub fn circle_monkey(&self, x: f32, y: f32) -> usize {
        for (circle_index, circle) in self.circles.iter().enumerate() {
            if x > circle.pos.x - 80.0
                && x < circle.pos.x + 80.0
                && y > circle.pos.y - 50.0
                && y < circle.pos.y + 50.0
            {
                return circle_index;
            }
        }
        10
    }
}
