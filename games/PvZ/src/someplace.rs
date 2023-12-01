use engine_immediate as engine;
use engine_immediate::{geom::*, Camera, Engine, SheetRegion};
use rand::Rng;

const W: f32 = 1400.0;
const H: f32 = 600.0;

// struct Plant {
//     pos: Vec2,
// }

// struct Zombie {
//     pos: Vec2,
//     vel: Vec2,
// }

struct Game {

    score: u32,
    font: engine::BitFont,
    spritesheet: engine::Spritesheet,
    spritesheet2: engine::Spritesheet,
    // plants: Vec<Plant>,
    // zombies: Vec<Zombie>,
    mode: u32, // 0 = PvZ, 1 = Game Won, 2 = Game Over 
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
            
        }

        
    }
    fn update(&mut self, engine: &mut Engine) {

        
        if engine.input.is_mouse_down(winit::event::MouseButton::Left) {

            let mouse_pos = engine.input.mouse_pos();
            let mouse_x =  mouse_pos.x as f32 / 1.14285714;
            let mouse_y =  (1200.0 - mouse_pos.y as f32) / 2.0;

            // println!("{:?}", mouse_pos);
            println!("{}, {}", mouse_x, mouse_y);

            // self.plants.push(Plant {
            //     pos: Vec2 {
            //         x: mouse_x,
            //         y: mouse_y,
            //     },
            // });

            

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

        //draw zombie
        engine.draw_sprite(
            self.spritesheet2,
            AABB{
                center: Vec2 {
                    x: W / 2.0 ,
                    y: H / 2.0 ,
                },
                size: Vec2 { x: 64.0, y: 128.0 },
            },
            
            SheetRegion::new(0, 0, 0, 0, 64, 128),
        );


        // for plant in self.plants.iter() {
        //     engine.draw_sprite(
        //         self.spritesheet2,
        //         AABB {
        //             center: plant.pos,
        //             size: Vec2 { x: 64.0, y: 64.0 },
        //         },
        //         SheetRegion::new(0, 0, 64, 64, 64, 64),
        //     );
        // }
            
    }
}
fn main() {
    Engine::new(winit::window::WindowBuilder::new()).run::<Game>();
}
