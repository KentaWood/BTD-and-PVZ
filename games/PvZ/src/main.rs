use engine_immediate as engine;
use engine_immediate::{geom::*, collision::*, Camera, Engine, SheetRegion};
use rand::Rng;
mod util;
use util::screen_to_grid;
use util::convert_mouse_pos;


const W: f32 = 1400.0;
const H: f32 = 600.0;

const SPRITE_PLANT_PEASHOOTER: SheetRegion = SheetRegion::new(0, 448 , 128, 0, 64, 64);
const PLANT_SIZE_PEASHOOTER: Vec2 = Vec2 { x: 64.0, y: 64.0 };

const SPRITE_ZOMBIE_NORMAL: SheetRegion = SheetRegion::new(0, 0, 0, 0, 64, 128);
const ZOMBIE_SIZE_NORMAL: Vec2 = Vec2 { x: 64.0, y: 128.0 };

const SPRITE_PEA: SheetRegion = SheetRegion::new(0, 66, 36, 0, 10, 16);
const PEA_SIZE: Vec2 = Vec2 { x: 24.0, y: 32.0 };


fn empty_Space(plants: &Vec<Plant>, x: f32, y: f32) -> bool {
    for plant in plants {
        if plant.pos.x == x && plant.pos.y == y {
            return false; // Found a match, return true
        }
    }
    true // No match found, return false
}


struct Game {

    score: u32,
    font: engine::BitFont,
    spritesheet: engine::Spritesheet,
    spritesheet2: engine::Spritesheet,
    spritesheet3: engine::Spritesheet,
    plants: Vec<Plant>,
    zombies: Vec<Zombie>,
    peas: Vec<Pea>,
    zombie_count :usize,
    plant_count: usize,
    pea_count: usize,
    mode: u32, // 0 = PvZ, 1 = Game Won, 2 = Game Over 
    mouse_clicked: bool,
    plant_index: usize,
    once: bool,
}

impl engine::Game for Game {
    fn new(engine: &mut Engine) -> Self {
        engine.set_camera(Camera {
            screen_pos: [0.0, 0.0],
            screen_size: [W, H],
        });
        
        #[cfg(not(target_arch = "wasm32"))]
        let sprite_img = image::open("assets/plantsZombies/167822.png").unwrap().into_rgba8();
        let spritesheet = engine.add_spritesheet(sprite_img, Some("background spritesheet"));

        let sprite_img2 = image::open("assets/plantsZombies/icon.png").unwrap().into_rgba8();
        let spritesheet2 = engine.add_spritesheet(sprite_img2, Some("character spritesheet"));

        let sprite_img3 = image::open("assets/plantsZombies/PeashooterAssetsDS.webp").unwrap().into_rgba8();
        let spritesheet3 = engine.add_spritesheet(sprite_img3, Some("character spritesheet"));

        let font = engine::BitFont::with_sheet_region(
            '0'..='9',
            SheetRegion::new(0, 0, 512, 0, 80, 8),
            10,
        );

    
        Game {
            spritesheet,
            spritesheet2,
            spritesheet3,
            score: 0,
            font,
            mode: 0,
            plants: Vec::with_capacity(16),
            zombies: Vec::with_capacity(16),
            peas: Vec::with_capacity(16),
            mouse_clicked: false,
            plant_index: 0,
            zombie_count: 0,
            plant_count: 0,
            pea_count: 0,
            once: true,
            
        }

        
    }
    fn update(&mut self, engine: &mut Engine) {

        if self.once {
            // pea shooting of the plants 
            self.plants.push(Plant {
                pos: Vec2 {
                    x: 280.0,
                    y: 70.0,
                },
            });
        
            self.peas.push(Pea {
                pos: Vec2 {
                    x: 280.0,
                    y: 70.0,
                },
                vel: Vec2 {
                    x: 4.0,
                    y: 0.0,
                },
            });
        
            self.plant_count = 1;
            self.pea_count = 1;
        
            for pea in self.peas.iter_mut() {
                pea.pos.x += pea.vel.x;
            }
        
            //self.once = false;
        }

        for pea in self.peas.iter_mut() {
            pea.pos.x += pea.vel.x;
        }
        
        //moving of the zombies

        if self.once{

            self.zombies.push(Zombie {
                pos: Vec2 {
                    x: 1100.0,
                    y: 90.0,
                },
                vel: Vec2 {
                    x: -0.25,
                    y: 0.0,
                },
                health: 3,
            });
            


            self.once = false;

        }

        let the_collisions = Collision::new(&self.zombies, &self.peas);
        let vec_coll = the_collisions.check_collision();
        if vec_coll.len() != 0 {
            for (p, z) in vec_coll.iter() {
                self.peas.remove(*p);
                self.zombies[*z].health = self.zombies[*z].health - 1;
                println!("{}", self.zombies[*z].health);
                if self.zombies[*z].health == 0 {
                    self.zombies.remove(*z);
                }
            }
        }

        
        for zombie in self.zombies.iter_mut() {
            zombie.pos.x += zombie.vel.x;
        }
        

        //Handles the placement of plants
        if self.mouse_clicked {
            if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
                let mouse_pos = engine.input.mouse_pos();
                let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());

                // println!("{:?}", mouse_pos);
                println!("{}, {}", mouse_x, mouse_y);
                self.plants[self.plant_index].pos.x = mouse_x;
                self.plants[self.plant_index].pos.y = mouse_y;

                

            }

            else{

                let mouse_pos = engine.input.mouse_pos();
                let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());
                let (grid_x, grid_y) = screen_to_grid(mouse_x, mouse_y);

                if empty_Space(&self.plants, grid_x, grid_y) {

                    self.plants[self.plant_index].pos.x = grid_x;
                    self.plants[self.plant_index].pos.y = grid_y;

                    self.mouse_clicked = false;

                    self.plant_index += 1;
                    
                }

                else{
                    
                    self.mouse_clicked = false;
                    self.plants.pop();
                }
            }
        }

        else{
            if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
                let mouse_pos = engine.input.mouse_pos();
                let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());

                self.mouse_clicked = true;
                self.plants.push(Plant {
                    pos: Vec2 {
                        x: mouse_x,
                        y: mouse_y,
                    },
                });
                }
        }



    }
    
    fn render(&mut self, engine: &mut Engine) {
        //draw bg
        engine.draw_sprite(
            self.spritesheet,
            AABB {
                center: Vec2 {
                    x: W / 2.0 ,
                    y: H / 2.0 ,
                },
                size: Vec2 { x: W, y: H },
            },
            SheetRegion::new(0, 0, 0, 16, 1400, 600),
        );

        //draw Zombie 
        engine.draw_sprite(
            self.spritesheet2,
            AABB{
                center: Vec2 {
                    x: 1100.0 ,
                    y: 90.0 ,
                },
                size: ZOMBIE_SIZE_NORMAL,
            },
            
            SPRITE_ZOMBIE_NORMAL,
        );

        engine.draw_sprite(
            self.spritesheet2,
            AABB{
                center: Vec2 {
                    x: 280.0 ,
                    y: 70.0 ,
                },
                size: PLANT_SIZE_PEASHOOTER,
            },
            
            SPRITE_PLANT_PEASHOOTER,
        );

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

        
            
    }
}
fn main() {
    Engine::new(winit::window::WindowBuilder::new()).run::<Game>();
}
