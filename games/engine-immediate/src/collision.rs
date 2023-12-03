pub use glam::*;

use crate::geom::{Zombie, Pea};

pub struct Collision {
    zombies: Vec<Zombie>,
    peas: Vec<Pea>,
}

impl Collision {
    pub fn new(zombies: &Vec<Zombie>, peas: &Vec<Pea>) -> Self {
        Collision { zombies: zombies.to_vec(), peas: peas.to_vec()}
    }

    pub fn check_collision(&self) -> Vec<(usize, usize)> {
        let mut collisions: Vec<(usize,usize)> = Vec::with_capacity(16);
        let mut pea_c = 0;
        let mut zomb = 0;
        for pea in self.peas.iter() {
            for zombie in self.zombies.iter() {
                if pea.pos.x + 24.0 > zombie.pos.x {
                    let tup = (pea_c, zomb);
                    collisions.push(tup);
                }
                zomb += 1;
            }
            pea_c += 1;
            zomb = 0;
        }
        return collisions;
    }
}