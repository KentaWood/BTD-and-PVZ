use std::time::{Duration, Instant};

use engine_immediate as engine;
use engine_immediate::{collision::*, collisiontwo::*, geom::*, Camera, Engine, SheetRegion};
mod util;
use util::convert_mouse_pos;
use util::screen_to_grid;

const W: f32 = 1400.0;
const H: f32 = 600.0;

const SPRITE_MONKEY_PEASHOOTER: SheetRegion = SheetRegion::new(0, 700, 0, 0, 230, 230);
const MONKEY_SIZE_PEASHOOTER: Vec2 = Vec2 { x: 90.0, y: 90.0 };

const SPRITE_CIRCLE_PEASHOOTER: SheetRegion = SheetRegion::new(0, 0, 0, 0, 1200, 1200);
const CIRCLE_SIZE_PEASHOOTER: Vec2 = Vec2 { x: 140.0, y: 90.0 };

const SPRITE_BALLOON_RED: SheetRegion = SheetRegion::new(0, 0, 0, 0, 52, 73);
const SPRITE_BALLOON_PURPLE: SheetRegion = SheetRegion::new(0, 58, 0, 0, 52, 73);
const SPRITE_BALLOON_GREEN: SheetRegion = SheetRegion::new(0, 116, 0, 0, 52, 73);
const BALLOON_SIZE_NORMAL: Vec2 = Vec2 { x: 44.0, y: 44.0 };

const SPRITE_DART: SheetRegion = SheetRegion::new(0, 415, 40, 0, 90, 110);
const DART_SIZE: Vec2 = Vec2 { x: 20.0, y: 30.0 };

const MONKEY_COST: usize = 20;

struct Game {
    font: engine::BitFont,
    spritesheet: engine::Spritesheet,
    spritesheet2: engine::Spritesheet,
    spritesheet3: engine::Spritesheet,
    spritesheet4: engine::Spritesheet,
    spritesheet5: engine::Spritesheet,
    points: u32,
    monkeys: Vec<Monkey>,
    balloons: Vec<Balloon>,
    darts: Vec<Dart>,
    circles: Vec<Circle>,
    monkey_count: usize,
    balloon_count: usize,
    dart_count: usize,
    monkey_index: usize,
    mouse_clicked: bool,
    once: bool,
    start: bool,
    oneplacement: bool,
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

        let sprite_img2 = image::open("assets/balloon.png")
            .unwrap()
            .into_rgba8();
        let spritesheet2 = engine.add_spritesheet(sprite_img2, Some("character spritesheet"));

        let sprite_img3 = image::open("assets/monkey.png")
            .unwrap()
            .into_rgba8();
        let spritesheet3 = engine.add_spritesheet(sprite_img3, Some("character spritesheet"));

        let sprite_img4 = image::open("assets/demo.png")
            .unwrap()
            .into_rgba8();
        let spritesheet4 = engine.add_spritesheet(sprite_img4, Some("score spritesheet"));

        let sprite_img5 = image::open("assets/cirlce.png")
            .unwrap()
            .into_rgba8();
        let spritesheet5 = engine.add_spritesheet(sprite_img5, Some("cirlce spritesheet"));


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
            points: 20,
            font,
            monkeys: Vec::with_capacity(16),
            balloons: Vec::with_capacity(100),
            darts: Vec::with_capacity(16),
            circles: Vec::with_capacity(16),
            mouse_clicked: false,
            balloon_count: 0,
            monkey_count: 0,
            dart_count: 0,
            monkey_index: 0,
            once: false,
            start: false,
            oneplacement: true,
        }

    }
    fn update(&mut self, engine: &mut Engine) {
        if self.mouse_clicked && !self.start {
            self.once = true;
            self.start = true;
        }

        if self.once {
            for i in 0..50 {
                self.balloons.push(Balloon {
                    pos: Vec2 { x: -10.0 - (i as f32 *100.0), y: 320.0 },
                    vel: Vec2 { x: 2.0, y: 0.0 },
                    health: 3,
                    segment: 0,
                });
            }
            

            self.balloon_count = 50;
            
            self.once = false;

            self.circles.push(Circle {
                pos: Vec2 {x: 450.0, y: 400.0},
                filled: false
            });
            self.circles.push(Circle {
                pos: Vec2 {x: 1050.0, y: 150.0},
                filled: false
            });
            self.circles.push(Circle {
                pos: Vec2 {x: 1050.0, y: 450.0},
                filled: false
            });
            self.circles.push(Circle {
                pos: Vec2 {x: 275.0, y: 550.0},
                filled: false
            });
            self.circles.push(Circle {
                pos: Vec2 {x: 1050.0, y: 300.0},
                filled: false
            });
            self.circles.push(Circle {
                pos: Vec2 {x: 170.0, y: 240.0},
                filled: false
            });
        }

        let the_collisions = Collisiontwo::new(&self.balloons, &self.darts, &self.monkeys, &self.circles);
        let vec_coll_dart = the_collisions.check_collision_dart();
        if !vec_coll_dart.is_empty() {
            for (p, z) in vec_coll_dart.iter() {
                self.darts.remove(*p);
                self.dart_count -= 1;
                self.balloons[*z].health -= 1;
                self.points +=1;
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

        for monkey in self.monkeys.iter_mut(){
            let time = monkey.action_time.elapsed();
            //println!("{:?}", time);

            if monkey.action_time.elapsed() > Duration::from_millis(700) {
                self.darts.push(Dart {
                    pos: monkey.pos,
                    vel: Vec2 { x: 0.0, y: -4.0 },
                });
                monkey.action_time = Instant::now();
                self.dart_count += 1;
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
            self.darts.remove(*i);
            self.dart_count -= 1;
        }

            if self.mouse_clicked {
                if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
                    let mouse_pos = engine.input.mouse_pos();
                    let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());
    
                    // println!("{:?}", mouse_pos);
                   // println!("{}, {}", mouse_x, mouse_y);
                
                    let the_collisions = Collisiontwo::new(&self.balloons, &self.darts, &self.monkeys, &self.circles);
                    let circle_interact = the_collisions.circle_monkey(mouse_x, mouse_y);
                    println!("{}", circle_interact);
                    if circle_interact != 10 && !self.circles[circle_interact].filled && self.points >= 20 {
                    self.monkeys.push(Monkey {
                        pos: Vec2 {
                            x: self.circles[circle_interact].pos.x,
                            y: self.circles[circle_interact].pos.y,
                        },
                        action_time: Instant::now(),
                    });
                    self.monkey_count += 1;
                    self.circles[circle_interact].filled = true;
                    self.points = self.points - 20;
                }
                } 
            } else if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
                let mouse_pos = engine.input.mouse_pos();
                let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());
    
                self.mouse_clicked = true;
                /*self.monkeys.push(Monkey {
                    pos: Vec2 {
                        x: mouse_x,
                        y: mouse_y,
                    },
                    action_time: Instant::now(),
                });
                self.monkey_count += 1;*/
            }

        
        //Handles the placement of plants
       
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

        for circle in self.circles.iter() {
            engine.draw_sprite(
                self.spritesheet5,
                AABB {
                    center: circle.pos,
                    size: CIRCLE_SIZE_PEASHOOTER,
                },
                SPRITE_CIRCLE_PEASHOOTER,
            );
        }

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

        for balloon in self.balloons.iter() {
            if balloon.health == 1 {
                engine.draw_sprite(
                    self.spritesheet2,
                    AABB {
                        center: balloon.pos,
                        size: BALLOON_SIZE_NORMAL,
                    },
                    SPRITE_BALLOON_RED,
                ); 
            } else if balloon.health == 2 {
                engine.draw_sprite(
                    self.spritesheet2,
                    AABB {
                        center: balloon.pos,
                        size: BALLOON_SIZE_NORMAL,
                    },
                    SPRITE_BALLOON_PURPLE,
                );
            } else if balloon.health == 3 {
                engine.draw_sprite(
                    self.spritesheet2,
                    AABB {
                        center: balloon.pos,
                        size: BALLOON_SIZE_NORMAL,
                    },
                    SPRITE_BALLOON_GREEN,
                );
            }
        }

        engine.draw_string(
            self.spritesheet4,
            &self.font,
            &self.points.to_string(),
            Vec2 {
                x: 75.0,
                y: 550.0,
            },
            48.0,

        );

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
