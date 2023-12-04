pub use glam::*;

use crate::geom::{Dart, Monkey, Balloon};

pub struct Collisiontwo {
    balloons: Vec<Balloon>,
    darts: Vec<Dart>,
    monkeys: Vec<Monkey>,
}

// NOTE::: IDEALLY MOVE DEFAULT SIZES SO THEY AREN"T HARD CODED (LIKE 24.0, 128.0, etc)
impl Collisiontwo {
    pub fn new(balloons: &Vec<Balloon>, darts: &Vec<Dart>, monkeys: &Vec<Monkey>) -> Self {
        Collisiontwo {
            balloons: balloons.to_vec(),
            darts: darts.to_vec(),
            monkeys: monkeys.to_vec(),
        }
    }

    pub fn check_collision_dart(&self) -> Vec<(usize, usize)> {
        let mut collisions: Vec<(usize, usize)> = Vec::with_capacity(16);
        let mut dart_c = 0;
        let mut monk = 0;
        for dart in self.darts.iter() {
            for balloon in self.balloons.iter() {
                if dart.pos.x + 20.0 > balloon.pos.x && dart.pos.x < balloon.pos.x + 44.0 {
                    if dart.pos.y < balloon.pos.y + 44.0 && dart.pos.y + 30.0 > balloon.pos.y {
                        let tup = (dart_c, monk);
                        collisions.push(tup);
                    }
                }
                monk += 1;
            }
            dart_c += 1;
            monk = 0;
        }
        return collisions;
    }
}
