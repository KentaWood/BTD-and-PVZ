use engine_immediate as engine;
use engine_immediate::{collision::*, geom::*, Camera, Engine, SheetRegion};

use rand::Rng;
use std::{
    borrow::Cow,
    mem,
    time::{Duration, Instant},
};

mod util;
use util::convert_mouse_pos;
use util::screen_to_grid;
use util::select_tempo;

const W: f32 = 1400.0;
const H: f32 = 600.0;

const SPRITE_PLANT_PEASHOOTER: SheetRegion = SheetRegion::new(0, 448, 128, 0, 64, 64);
const PLANT_SIZE_PEASHOOTER: Vec2 = Vec2 { x: 64.0, y: 64.0 };

const SPRITE_ZOMBIE_NORMAL: SheetRegion = SheetRegion::new(0, 0, 28, 0, 64, 100);
const ZOMBIE_SIZE_NORMAL: Vec2 = Vec2 { x: 64.0, y: 128.0 };

const SPRITE_PEA: SheetRegion = SheetRegion::new(0, 66, 36, 0, 10, 16);
const PEA_SIZE: Vec2 = Vec2 { x: 24.0, y: 32.0 };

const ZOMBIR_Y_SPAWNS: &[f64] = &[90.0, 195.0, 285.0, 390.0, 490.0];

const SPAWN_TEMPO: [(u32, u32); 5] = [(3, 7), (2, 6), (1, 4), (1, 3), (1, 2)];
const ZOMBIE_SPEED: &[f64] = &[-0.5, -0.75, -1.0, -1.5, -2.25];

fn empty_space(plants: &Vec<Plant>, x: f32, y: f32) -> bool {
    for plant in plants {
        if plant.pos.x == x && plant.pos.y == y {
            return false; // Found a match, return true
        }
    }
    true // No match found, return false
}

struct Game {
    sunflower: u32,

    font: engine::BitFont,
    spritesheet: engine::Spritesheet,
    spritesheet2: engine::Spritesheet,
    spritesheet3: engine::Spritesheet,
    spritesheet4: engine::Spritesheet,
    spritesheet5: engine::Spritesheet,
    plants: Vec<Plant>,
    zombies: Vec<Zombie>,
    peas: Vec<Pea>,
    zombie_count: usize,
    plant_count: usize,
    pea_count: usize,

    mode: u32, // 0 = Starting Screen, 1 = PvZ, 2 = Game Over

    mouse_clicked: bool,
    plant_index: usize,
    once: bool,
    start: bool,

    game_time: Instant,
    sunflower_time: Instant,
    spawn_timer: Instant,
    spawn_in: Duration,
}

impl engine::Game for Game {
    fn new(engine: &mut Engine) -> Self {
        engine.set_camera(Camera {
            screen_pos: [0.0, 0.0],
            screen_size: [W, H],
        });

        #[cfg(not(target_arch = "wasm32"))]
        let sprite_img = image::open("assets/plantsZombies/167822.png")
            .unwrap()
            .into_rgba8();
        let spritesheet = engine.add_spritesheet(sprite_img, Some("background spritesheet"));

        let sprite_img2 = image::open("assets/plantsZombies/icon.png")
            .unwrap()
            .into_rgba8();
        let spritesheet2 = engine.add_spritesheet(sprite_img2, Some("character spritesheet"));

        let sprite_img3 = image::open("assets/plantsZombies/PeashooterAssetsDS.webp")
            .unwrap()
            .into_rgba8();
        let spritesheet3 = engine.add_spritesheet(sprite_img3, Some("pea spritesheet"));

        let sprite_img4 = image::open("assets/demo.png").unwrap().into_rgba8();
        let spritesheet4 = engine.add_spritesheet(sprite_img4, Some("score spritesheet"));

        let sprite_img5 = image::open("assets/plantsZombies/PvZ1ZombiesWon.webp")
            .unwrap()
            .into_rgba8();
        let spritesheet5 = engine.add_spritesheet(sprite_img5, Some("score spritesheet"));

        let font = engine::BitFont::with_sheet_region(
            '0'..='9',
            SheetRegion::new(0, 0, 512, 0, 80, 8),
            10,
        );

        Game {
            spritesheet,
            spritesheet2,
            spritesheet3,
            spritesheet4,
            spritesheet5,
            sunflower: 100,
            font,
            mode: 1,
            plants: Vec::with_capacity(16),
            zombies: Vec::with_capacity(16),
            peas: Vec::with_capacity(16),
            mouse_clicked: false,
            plant_index: 0,
            zombie_count: 0,
            plant_count: 0,
            pea_count: 0,
            once: false,
            start: false,

            game_time: Instant::now(),
            sunflower_time: Instant::now(),
            spawn_timer: Instant::now(),
            spawn_in: Duration::from_secs(3),
        }
    }
    fn update(&mut self, engine: &mut Engine) {
        if self.mouse_clicked && !self.start {
            self.once = true;
            self.start = true;
        }

        if self.mouse_clicked && !self.start {
            self.once = true;
            self.start = true;
        }

        //random spawning of the regualr zombies
        if self.spawn_timer.elapsed() > self.spawn_in {
            let tempo = select_tempo(self.game_time.elapsed()) as usize;

            for _ in 0..tempo + 1 {
                let random_index = rand::thread_rng().gen_range(0..ZOMBIR_Y_SPAWNS.len());
                let speed = ZOMBIE_SPEED[select_tempo(self.game_time.elapsed()) as usize];

                self.zombies.push(Zombie {
                    pos: Vec2 {
                        x: 1100.0,
                        y: ZOMBIR_Y_SPAWNS[random_index] as f32,
                    },
                    vel: Vec2 {
                        x: speed as f32,
                        y: 0.0,
                    },
                    health: 3,
                });

                self.zombie_count += 1;
            }

            let mut rng = rand::thread_rng();
            let fast = SPAWN_TEMPO[select_tempo(self.game_time.elapsed()) as usize].0;
            let slow = SPAWN_TEMPO[select_tempo(self.game_time.elapsed()) as usize].1;

            let random_number: u32 = rng.gen_range(fast..=slow);
            self.spawn_in = Duration::from_secs(random_number.into());
            self.spawn_timer = Instant::now();
        }

        //handles the amount of sunflower points
        if self.sunflower_time.elapsed() > Duration::from_secs(5) {
            self.sunflower += 50;
            self.sunflower_time = Instant::now();
        }

        //periodically shooting peas for every plant that it placed
        for plant in self.plants.iter_mut() {
            let time = plant.action_time.elapsed();
            println!("{:?}", time);

            if plant.action_time.elapsed() > Duration::from_secs(2) && plant.placed {
                self.peas.push(Pea {
                    pos: plant.pos,
                    vel: Vec2 { x: 4.0, y: 0.0 },
                });
                plant.action_time = Instant::now();
                self.pea_count += 1;
            }
        }

        //handles the deleting of the pea
        let mut pea_delete: Vec<usize> = Vec::with_capacity(16);
        for (pea_index, pea) in self.peas.iter_mut().enumerate() {
            pea.pos.x += pea.vel.x;
            if pea.pos.x > 1800.0 {
                pea_delete.push(pea_index);
            }
        }
        for i in pea_delete.iter().rev() {
            self.peas.remove(*i);
            self.pea_count -= 1;
        }

        //handles when the pea hits a zombie
        let the_collisions = Collision::new(&self.zombies, &self.peas, &self.plants);
        let vec_coll_pea = the_collisions.check_collision_pea();
        let vec_coll_plant = the_collisions.check_collision_plant();
        if !vec_coll_pea.is_empty() {
            for (p, z) in vec_coll_pea.iter().rev() {
                self.peas.remove(*p);
                self.pea_count -= 1;
                self.zombies[*z].health -= 1;
                if self.zombies[*z].health == 0 {
                    self.zombies.remove(*z);
                    self.zombie_count -= 1;
                }
            }
        }

        if !vec_coll_plant.is_empty() {
            for (p, _z) in vec_coll_plant.iter() {
                self.plants.remove(*p);
                self.plant_count -= 1;
                self.plant_index -= 1;
            }
        }

        //game over when a zombie touches the end
        for zombie in self.zombies.iter_mut() {
            zombie.pos.x += zombie.vel.x;
            if zombie.pos.x < 220.0 {
                self.mode = 2;
                println!("game over");
            }
        }

        //Handles the placement of plants
        if self.mouse_clicked {
            if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
                let mouse_pos = engine.input.mouse_pos();
                let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());

                self.plants[self.plant_index].pos.x = mouse_x;
                self.plants[self.plant_index].pos.y = mouse_y;
                println!("{},{}", mouse_x, mouse_y);
            } else {
                if self.sunflower >= 100 {
                    let mouse_pos = engine.input.mouse_pos();
                    let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());
                    let (grid_x, grid_y) = screen_to_grid(mouse_x, mouse_y);

                    if empty_space(&self.plants, grid_x, grid_y) {
                        self.plants[self.plant_index].pos.x = grid_x;
                        self.plants[self.plant_index].pos.y = grid_y;

                        self.plants[self.plant_index].placed = true;

                        self.mouse_clicked = false;

                        self.plant_index += 1;
                        self.sunflower -= 100;
                    } else {
                        self.mouse_clicked = false;
                        self.plant_index -= 1;
                        self.plants.pop();
                    }
                } else {
                    self.mouse_clicked = false;
                    self.plants.pop();
                }
            }
        } else if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
            let mouse_pos = engine.input.mouse_pos();
            let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());

            self.mouse_clicked = true;
            self.plants.push(Plant {
                pos: Vec2 {
                    x: mouse_x,
                    y: mouse_y,
                },

                action_time: Instant::now(),
                placed: false,
            });
            self.plant_count += 1;
        }
    }

    fn render(&mut self, engine: &mut Engine) {
        //start (Maybe)

        if self.mode == 0 {}

        //PvZ game rendering
        if self.mode == 1 || self.mode == 2 {
            //draw bg
            engine.draw_sprite(
                self.spritesheet,
                AABB {
                    center: Vec2 {
                        x: W / 2.0,
                        y: H / 2.0,
                    },
                    size: Vec2 { x: W, y: H },
                },
                SheetRegion::new(0, 0, 0, 16, 1400, 600),
            );

            //draw all the plants
            for plant in self.plants.iter() {
                engine.draw_sprite(
                    self.spritesheet2,
                    AABB {
                        center: plant.pos,
                        size: PLANT_SIZE_PEASHOOTER,
                    },
                    SPRITE_PLANT_PEASHOOTER,
                );
            }

            //draw all the zombies
            for zombie in self.zombies.iter() {
                engine.draw_sprite(
                    self.spritesheet2,
                    AABB {
                        center: zombie.pos,
                        size: ZOMBIE_SIZE_NORMAL,
                    },
                    SPRITE_ZOMBIE_NORMAL,
                );
            }

            //draw all the peas
            for pea in self.peas.iter() {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: pea.pos,
                        size: PEA_SIZE,
                    },
                    SPRITE_PEA,
                );
            }

            //draw the amount of sunflower the player has
            engine.draw_string(
                self.spritesheet4,
                &self.font,
                &self.sunflower.to_string(),
                Vec2 { x: 75.0, y: 550.0 },
                48.0,
            );

            //game over screen
            if self.mode == 2 {
                engine.draw_sprite(
                    self.spritesheet5,
                    AABB {
                        center: Vec2 {
                            x: W / 2.0,
                            y: H / 2.0,
                        },
                        size: Vec2 { x: 500.0, y: 414.0 },
                    },
                    SheetRegion::new(0, 0, 0, 0, 250, 207),
                );
            }
        }
    }
}
fn main() {
    Engine::new(winit::window::WindowBuilder::new()).run::<Game>();
}
