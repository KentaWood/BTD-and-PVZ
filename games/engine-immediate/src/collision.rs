pub use glam::*;

use crate::geom::{Pea, Plant, Zombie};

pub struct Collision {
    zombies: Vec<Zombie>,
    peas: Vec<Pea>,
    plants: Vec<Plant>,
}

// NOTE::: IDEALLY MOVE DEFAULT SIZES SO THEY AREN"T HARD CODED (LIKE 24.0, 128.0, etc)
impl Collision {
    pub fn new(zombies: &Vec<Zombie>, peas: &Vec<Pea>, plants: &Vec<Plant>) -> Self {
        Collision {
            zombies: zombies.to_vec(),
            peas: peas.to_vec(),
            plants: plants.to_vec(),
        }
    }

    pub fn check_collision_pea(&self) -> Vec<(usize, usize)> {
        let mut collisions: Vec<(usize, usize)> = Vec::with_capacity(16);
        let mut pea_c = 0;
        let mut zomb = 0;
        for pea in self.peas.iter() {
            for zombie in self.zombies.iter() {
                if pea.pos.x + 24.0 > zombie.pos.x {
                    if pea.pos.y < zombie.pos.y + 64.0 && pea.pos.y + 32.0 > zombie.pos.y {
                        let tup = (pea_c, zomb);
                        collisions.push(tup);
                    }
                }
                zomb += 1;
            }
            pea_c += 1;
            zomb = 0;
        }
        return collisions;
    }

    pub fn check_collision_plant(&self) -> Vec<(usize, usize)> {
        let mut collisions: Vec<(usize, usize)> = Vec::with_capacity(16);
        let mut plan = 0;
        let mut zomb = 0;
        for plant in self.plants.iter() {
            for zombie in self.zombies.iter() {
                if plant.pos.x + 64.0 > zombie.pos.x {
                    if plant.pos.y < zombie.pos.y + 64.0
                        && plant.pos.y + 64.0 > zombie.pos.y
                        && plant.placed == true
                    {
                        let tup = (plan, zomb);
                        collisions.push(tup);
                    }
                }
                zomb += 1;
            }
            plan += 1;
            zomb = 0;
        }
        return collisions;
    }
}
