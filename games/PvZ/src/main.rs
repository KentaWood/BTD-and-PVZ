use engine_immediate as engine;
use engine_immediate::{geom::*, Camera, Engine, SheetRegion};
use rand::Rng;

const W: f32 = 1400.0;
const H: f32 = 600.0;

const SPRITE_PLANT_PEASHOOTER: SheetRegion = SheetRegion::new(0, 448 , 128, 0, 64, 64);
const SPRITE_ZOMBIE_NORMAL: SheetRegion = SheetRegion::new(0, 0, 0, 0, 64, 128);

const PLANT_SIZE_PEASHOOTER: Vec2 = Vec2 { x: 64.0, y: 64.0 };
const ZOMBIE_SIZE_NORMAL: Vec2 = Vec2 { x: 64.0, y: 128.0 };

fn screen_to_grid(x: f32, y: f32) -> (f32, f32) {
    let mut grid_x = 700.0; // Default values
    let mut grid_y = 300.0;

    if x > 240.0 && x < 340.0 {
        grid_x = 280.0;
    }

    if x > 340.0 && x < 410.0{
        grid_x = 380.0;
    }

    if x > 410.0 && x < 500.0{
        grid_x = 450.0;
    }

    if x > 500.0 && x < 575.0{
        grid_x = 540.0;
    }

    if x > 575.0 && x < 660.0{
        grid_x = 620.0;
    }

    if x > 660.0 && x < 730.0{
        grid_x = 700.0;
    }

    if x > 730.0 && x < 815.0{
        grid_x = 780.0;
    }

    if x > 815.0 && x < 900.0{
        grid_x = 850.0;
    }

    if x > 900.0 && x < 1000.0{
        grid_x = 940.0;
    }
    //y- cord
    if y > 30.0 && y < 125.0 {
        grid_y = 70.0;
    }

    if y > 125.0 && y < 220.0{
        grid_y = 175.0;
    }

    if y > 220.0 && y < 320.0{
        grid_y = 265.0;
    }

    if y > 320.0 && y < 415.0{
        grid_y = 370.0;
    }

    if y > 415.0 && y < 535.0{
        grid_y = 470.0;
    }

    (grid_x, grid_y)
}

fn empty_Space(plants: &Vec<Plant>, x: f32, y: f32) -> bool {
    for plant in plants {
        if plant.pos.x == x && plant.pos.y == y {
            return false; // Found a match, return true
        }
    }
    true // No match found, return false
}



struct Plant {
    pos: Vec2,
}

struct Zombie {
    pos: Vec2,
    vel: Vec2,
}

struct Game {

    score: u32,
    font: engine::BitFont,
    spritesheet: engine::Spritesheet,
    spritesheet2: engine::Spritesheet,
    plants: Vec<Plant>,
    zombies: Vec<Zombie>,
    mode: u32, // 0 = PvZ, 1 = Game Won, 2 = Game Over 
    mouse_clicked: bool,
    plant_index: usize,
}

impl engine::Game for Game {
    fn new(engine: &mut Engine) -> Self {
        engine.set_camera(Camera {
            screen_pos: [0.0, 0.0],
            screen_size: [W, H],
        });
        
        #[cfg(not(target_arch = "wasm32"))]
        let sprite_img = image::open("/Users/nobuko/Desktop/CS- 181G Projects/unit-3-Alex /Unit-3/assets/plantsZombies/167822.png").unwrap().into_rgba8();
        let spritesheet = engine.add_spritesheet(sprite_img, Some("background spritesheet"));

        let sprite_img2 = image::open("/Users/nobuko/Desktop/CS- 181G Projects/unit-3-Alex /Unit-3/assets/plantsZombies/icon.png").unwrap().into_rgba8();
        let spritesheet2 = engine.add_spritesheet(sprite_img2, Some("character spritesheet"));

        let font = engine::BitFont::with_sheet_region(
            '0'..='9',
            SheetRegion::new(0, 0, 512, 0, 80, 8),
            10,
        );

    
        Game {
            spritesheet,
            spritesheet2,
            score: 0,
            font,
            mode: 0,
            plants: Vec::with_capacity(16),
            zombies:Vec::with_capacity(16),
            mouse_clicked: false,
            plant_index: 0,
            
        }

        
    }
    fn update(&mut self, engine: &mut Engine) {

        if self.mouse_clicked {


            if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
                let mouse_pos = engine.input.mouse_pos();
                let mouse_x =  mouse_pos.x as f32 / 1.14285714;
                let mouse_y =  (1200.0 - mouse_pos.y as f32) / 2.0;

                // println!("{:?}", mouse_pos);
                println!("{}, {}", mouse_x, mouse_y);
                self.plants[self.plant_index].pos.x = mouse_x;
                self.plants[self.plant_index].pos.y = mouse_y;

                

            }

            else{
                let mouse_pos = engine.input.mouse_pos();
                let mouse_x =  mouse_pos.x as f32 / 1.14285714;
                let mouse_y =  (1200.0 - mouse_pos.y as f32) / 2.0;

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
                let mouse_x =  mouse_pos.x as f32 / 1.14285714;
                let mouse_y =  (1200.0 - mouse_pos.y as f32) / 2.0;

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

        // engine.draw_sprite(
        //     self.spritesheet2,
        //     AABB{
        //         center: Vec2 {
        //             x: 280.0 ,
        //             y: 200.0 ,
        //         },
        //         size: PLANT_SIZE_PEASHOOTER,
        //     },
            
        //     SPRITE_PLANT_PEASHOOTER,
        // );
       

        for plant in self.plants.iter() {
            engine.draw_sprite(
                self.spritesheet2,
                AABB {
                    center: plant.pos,
                    size: PLANT_SIZE_PEASHOOTER,
                },
                SPRITE_PLANT,
            );
        }
            
    }
}
fn main() {
    Engine::new(winit::window::WindowBuilder::new()).run::<Game>();
}
