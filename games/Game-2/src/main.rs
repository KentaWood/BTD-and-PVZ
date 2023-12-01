use engine_immediate as engine;
use engine_immediate::{geom::*, Camera, Engine, SheetRegion};
use rand::Rng;

const W: f32 = 1400.0;
const H: f32 = 600.0;


struct Game {

    score: u32,
    font: engine::BitFont,
    spritesheet: engine::Spritesheet,
    mode: u32, // 0 = PvZ, 1 = Game Won, 2 = Game Over 
}

impl engine::Game for Game {
    fn new(engine: &mut Engine) -> Self {
        engine.set_camera(Camera {
            screen_pos: [0.0, 0.0],
            screen_size: [W, H],
        });
        
        #[cfg(not(target_arch = "wasm32"))]
        let sprite_img = image::open("/Users/nobuko/Desktop/CS- 181G Projects/unit-3-Alex /Unit-3/assets/title.jpg").unwrap().into_rgba8();
        let spritesheet = engine.add_spritesheet(sprite_img, Some("demo spritesheet"));

        let font = engine::BitFont::with_sheet_region(
            '0'..='9',
            SheetRegion::new(0, 0, 512, 0, 80, 8),
            10,
        );
        Game {
            spritesheet,
            score: 0,
            font,
            mode: 0,
        }
    }
    fn update(&mut self, engine: &mut Engine) {

        
        if engine.input.is_mouse_down(winit::event::MouseButton::Left) {

            let mouse_pos = engine.input.mouse_pos();
            println!("{:?}", mouse_pos);
        }
    }
    fn render(&mut self, engine: &mut Engine) {
        // set bg image
        engine.draw_sprite(
            self.spritesheet,
            AABB {
                center: Vec2 {
                    x: W / 2.0 ,
                    y: H / 2.0 ,
                },
                size: Vec2 { x: W, y: H },
            },
            SheetRegion::new(0, 0, 0, 16, 6284, 3535),
        );
    }
}
fn main() {
    Engine::new(winit::window::WindowBuilder::new()).run::<Game>();
}
