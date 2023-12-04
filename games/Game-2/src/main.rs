use engine_immediate as engine;
use engine_immediate::{collision::*, collisiontwo::*, geom::*, Camera, Engine, SheetRegion};
mod util;
use util::convert_mouse_pos;
use util::screen_to_grid;

const W: f32 = 1400.0;
const H: f32 = 600.0;

const SPRITE_MONKEY_PEASHOOTER: SheetRegion = SheetRegion::new(0, 700, 0, 0, 230, 230);
const MONKEY_SIZE_PEASHOOTER: Vec2 = Vec2 { x: 90.0, y: 90.0 };

const SPRITE_BALLOON_NORMAL: SheetRegion = SheetRegion::new(0, 0, 0, 0, 23, 29);
const BALLOON_SIZE_NORMAL: Vec2 = Vec2 { x: 44.0, y: 44.0 };

const SPRITE_DART: SheetRegion = SheetRegion::new(0, 415, 40, 0, 90, 110);
const DART_SIZE: Vec2 = Vec2 { x: 20.0, y: 30.0 };

fn empty_space(monkeys: &Vec<Monkey>, x: f32, y: f32) -> bool {
    for monkey in monkeys {
        if monkey.pos.x == x && monkey.pos.y == y {
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
    monkeys: Vec<Monkey>,
    balloons: Vec<Balloon>,
    darts: Vec<Dart>,
    monkey_count: usize,
    balloon_count: usize,
    dart_count: usize,
    mode: u32, 
    monkey_index: usize,
    mouse_clicked: bool,
    once: bool,
    start: bool,
}

impl engine::Game for Game {
    fn new(engine: &mut Engine) -> Self {
        engine.set_camera(Camera {
            screen_pos: [0.0, 0.0],
            screen_size: [W, H],
        });

        #[cfg(not(target_arch = "wasm32"))]
        let sprite_img = image::open("assets/btdbackground.jpg")
            .unwrap()
            .into_rgba8();
        let spritesheet = engine.add_spritesheet(sprite_img, Some("background spritesheet"));

        let sprite_img2 = image::open("assets/bloonImg.png")
            .unwrap()
            .into_rgba8();
        let spritesheet2 = engine.add_spritesheet(sprite_img2, Some("character spritesheet"));

        let sprite_img3 = image::open("assets/monkey.png")
            .unwrap()
            .into_rgba8();
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
            monkeys: Vec::with_capacity(16),
            balloons: Vec::with_capacity(48),
            darts: Vec::with_capacity(16),
            mouse_clicked: false,
            balloon_count: 0,
            monkey_count: 0,
            dart_count: 0,
            monkey_index: 0,
            once: false,
            start: false,
        }
    }
    fn update(&mut self, engine: &mut Engine) {
        if self.mouse_clicked && !self.start {
            self.once = true;
            self.start = true;
        }
        
/* 
        let mut dart_delete: Vec<usize> = Vec::with_capacity(16);
        for (dart_index, dart) in self.darts.iter_mut().enumerate() {
            dart.pos.x += dart.vel.x;
            if dart.pos.x > 1800.0 {
                dart_delete.push(dart_index);
            }
        }
        for i in dart_delete.iter().rev() {
            self.darts.remove(*i);
            self.dart_count -= 1;
        }*/


        if self.once {
            for i in 0..50 {
                self.balloons.push(Balloon {
                    pos: Vec2 { x: -10.0 - (i as f32 *100.0), y: 320.0 },
                    vel: Vec2 { x: 2.0, y: 0.0 },
                    health: 1,
                    segment: 0,
                });
            }
            

            self.balloon_count = 50;
            
            self.once = false;
        }

        let the_collisions = Collisiontwo::new(&self.balloons, &self.darts, &self.monkeys);
        let vec_coll_dart = the_collisions.check_collision_dart();
        if !vec_coll_dart.is_empty() {
            for (p, z) in vec_coll_dart.iter() {
                let num_monkey = self.darts[*p].monkey_num;
                self.monkeys[num_monkey].dart = false;
                self.darts.remove(*p);
                self.dart_count -= 1;
                self.balloons[*z].health -= 1;
                if self.balloons[*z].health == 0 {
                    self.balloons.remove(*z);
                    self.balloon_count -= 1;
                }
            }
        }

        for balloon in self.balloons.iter_mut() {
            balloon.balloon_change_velocity();
            balloon.pos.x += balloon.vel.x;
            balloon.pos.y += balloon.vel.y;
            if balloon.segment > 13 {
              std::process::exit(0);
            }
        }

        for (monkey_index, monkey) in self.monkeys.iter_mut().enumerate() {
            if monkey.dart == false {
                self.darts.push(Dart {
                    pos: Vec2 { x: monkey.pos.x, y: monkey.pos.y },
                    vel: Vec2 { x: 0.0, y: -4.0 },
                    monkey_num: monkey_index,
                });
                self.dart_count += 1;
                monkey.dart = true;

            }
        }


        let mut dart_delete: Vec<usize> = Vec::with_capacity(16);
        for (dart_index, dart) in self.darts.iter_mut().enumerate() {
            dart.pos.y += dart.vel.y;
            if dart.pos.y < 0.0 {
                dart_delete.push(dart_index);
            }
        }
        for i in dart_delete.iter().rev() {
            let num_monkey = self.darts[*i].monkey_num;
            self.monkeys[num_monkey].dart = false;
            self.darts.remove(*i);
            self.dart_count -= 1;
        }


        //Handles the placement of plants
        if self.mouse_clicked {
            if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
                let mouse_pos = engine.input.mouse_pos();
                let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());

                // println!("{:?}", mouse_pos);
                println!("{}, {}", mouse_x, mouse_y);
                self.monkeys[self.monkey_index].pos.x = mouse_x;
                self.monkeys[self.monkey_index].pos.y = mouse_y;
            } else {
                let mouse_pos = engine.input.mouse_pos();
                let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());
                let (grid_x, grid_y) = screen_to_grid(mouse_x, mouse_y);

                if empty_space(&self.monkeys, grid_x, grid_y) {
                    self.monkeys[self.monkey_index].pos.x = grid_x;
                    self.monkeys[self.monkey_index].pos.y = grid_y;

                    self.mouse_clicked = false;

                    self.monkey_index += 1;
                } else {
                    self.mouse_clicked = false;
                    self.monkeys.pop();
                }
            }
        } else if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
            let mouse_pos = engine.input.mouse_pos();
            let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());

            self.mouse_clicked = true;
            self.monkeys.push(Monkey {
                pos: Vec2 {
                    x: mouse_x,
                    y: mouse_y,
                },
                dart: false
            });
            self.monkey_count += 1;
        }
    }

    fn render(&mut self, engine: &mut Engine) {
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
            SheetRegion::new(0, 0, 0, 16, 230, 230),
        );

        for monkey in self.monkeys.iter() {
            engine.draw_sprite(
                self.spritesheet3,
                AABB {
                    center: monkey.pos,
                    size: MONKEY_SIZE_PEASHOOTER,
                },
                SPRITE_MONKEY_PEASHOOTER,
            );
        }

        for zombie in self.balloons.iter() {
            engine.draw_sprite(
                self.spritesheet2,
                AABB {
                    center: zombie.pos,
                    size: BALLOON_SIZE_NORMAL,
                },
                SPRITE_BALLOON_NORMAL,
            );
        }

        for dart in self.darts.iter() {
            engine.draw_sprite(
                self.spritesheet3,
                AABB {
                    center: dart.pos,
                    size: DART_SIZE,
                },
                SPRITE_DART,
            );
        }
    }
}
fn main() {
    Engine::new(winit::window::WindowBuilder::new()).run::<Game>();
}
