use std::time::{Duration, Instant};

use engine_immediate as engine;
use engine_immediate::{collision::*, collisiontwo::*, geom::*, Camera, Engine, SheetRegion};
mod util;
use util::convert_mouse_pos;

use kira::{
    manager::{
        backend::DefaultBackend, // changed to default backend
        AudioManager,
        AudioManagerSettings,
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

// NOTE: CLICK P TO PAUSE!!!

const W: f32 = 1400.0;
const H: f32 = 600.0;

const SPRITE_MONKEY_PEASHOOTER1: SheetRegion = SheetRegion::new(0, 700, 0, 0, 230, 240);
const SPRITE_MONKEY_PEASHOOTER2: SheetRegion = SheetRegion::new(0, 700, 230, 0, 210, 230);
const SPRITE_MONKEY_PEASHOOTER3: SheetRegion = SheetRegion::new(0, 700, 460, 0, 230, 230);
const SPRITE_MONKEY_PEASHOOTER4: SheetRegion = SheetRegion::new(0, 700, 690, 0, 210, 240);
const MONKEY_SIZE_PEASHOOTER: Vec2 = Vec2 { x: 90.0, y: 90.0 };

const SPRITE_CIRCLE_PEASHOOTER: SheetRegion = SheetRegion::new(0, 0, 0, 0, 1200, 1200);
const CIRCLE_SIZE_PEASHOOTER: Vec2 = Vec2 { x: 140.0, y: 90.0 };

const SPRITE_BALLOON_RED: SheetRegion = SheetRegion::new(0, 0, 0, 0, 52, 73);
const SPRITE_BALLOON_PURPLE: SheetRegion = SheetRegion::new(0, 58, 0, 0, 52, 73);
const SPRITE_BALLOON_GREEN: SheetRegion = SheetRegion::new(0, 116, 0, 0, 52, 73);
const BALLOON_SIZE_NORMAL: Vec2 = Vec2 { x: 44.0, y: 44.0 };

const SPRITE_DART: SheetRegion = SheetRegion::new(0, 415, 40, 0, 90, 110);
const SPRITE_DART1: SheetRegion = SheetRegion::new(0, 415, 290, 0, 110, 90);
const SPRITE_DART2: SheetRegion = SheetRegion::new(0, 415, 150, 0, 90, 110);
const SPRITE_DART3: SheetRegion = SheetRegion::new(0, 415, 400, 0, 110, 90);
const DART_SIZE: Vec2 = Vec2 { x: 20.0, y: 30.0 };
const DART_SIZE2: Vec2 = Vec2 { x: 30.0, y: 20.0 };

const MONKEY_COST: u32 = 20;

struct Game {
    font: engine::BitFont,
    spritesheet0: engine::Spritesheet,
    spritesheet: engine::Spritesheet,
    spritesheet2: engine::Spritesheet,
    spritesheet3: engine::Spritesheet,
    spritesheet4: engine::Spritesheet,
    spritesheet5: engine::Spritesheet,
    spritesheet6: engine::Spritesheet,
    spritesheet7: engine::Spritesheet,
    points: u32,
    monkeys: Vec<Monkey>,
    balloons: Vec<Balloon>,
    darts: Vec<Dart>,
    circles: Vec<Circle>,
    monkey_count: usize,
    balloon_count: usize,
    dart_count: usize,
    mouse_clicked: bool,
    once: bool,
    start: bool,
    pause: bool,
}

impl engine::Game for Game {
    fn new(engine: &mut Engine) -> Self {
        engine.set_camera(Camera {
            screen_pos: [0.0, 0.0],
            screen_size: [W, H],
        });

        #[cfg(not(target_arch = "wasm32"))]
        let sprite_img0 = image::open("assets/btdinstructions.png")
            .unwrap()
            .into_rgba8();
        let spritesheet0 = engine.add_spritesheet(sprite_img0, Some("background spritesheet"));

        let sprite_img = image::open("assets/btdbackground.jpg")
            .unwrap()
            .into_rgba8();
        let spritesheet = engine.add_spritesheet(sprite_img, Some("background spritesheet"));

        let sprite_img2 = image::open("assets/balloon.png").unwrap().into_rgba8();
        let spritesheet2 = engine.add_spritesheet(sprite_img2, Some("character spritesheet"));

        let sprite_img3 = image::open("assets/monkey3.png").unwrap().into_rgba8();
        let spritesheet3 = engine.add_spritesheet(sprite_img3, Some("character spritesheet"));

        let sprite_img4 = image::open("assets/demo.png").unwrap().into_rgba8();
        let spritesheet4 = engine.add_spritesheet(sprite_img4, Some("score spritesheet"));

        let sprite_img5 = image::open("assets/cirlce.png").unwrap().into_rgba8();
        let spritesheet5 = engine.add_spritesheet(sprite_img5, Some("circle spritesheet"));

        let sprite_img6 = image::open("assets/download.jpg").unwrap().into_rgba8();
        let spritesheet6 = engine.add_spritesheet(sprite_img6, Some("win spritesheet"));

        let sprite_img7 = image::open("assets/end.png").unwrap().into_rgba8();
        let spritesheet7 = engine.add_spritesheet(sprite_img7, Some("lose spritesheet"));

        let font = engine::BitFont::with_sheet_region(
            '0'..='9',
            SheetRegion::new(0, 0, 512, 0, 80, 8),
            10,
        );

        Game {
            spritesheet0,
            spritesheet,
            spritesheet2,
            spritesheet3,
            spritesheet4,
            spritesheet5,
            spritesheet6,
            spritesheet7,
            points: MONKEY_COST,
            font,
            monkeys: Vec::with_capacity(16),
            balloons: Vec::with_capacity(128),
            darts: Vec::with_capacity(256),
            circles: Vec::with_capacity(16),
            mouse_clicked: false,
            balloon_count: 0,
            monkey_count: 0,
            dart_count: 0,
            once: false,
            start: false,
            pause: false,
        }
    }
    fn update(&mut self, engine: &mut Engine) {
        if self.mouse_clicked && !self.start {
            self.once = true;
            self.start = true;
        }

        if self.once {
            for i in 0..10 {
                self.balloons.push(Balloon {
                    pos: Vec2 {
                        x: -10.0 - (i as f32 * 100.0),
                        y: 320.0,
                    },
                    vel: Vec2 { x: 2.0, y: 0.0 },
                    health: 3,
                    segment: 0,
                });
            }

            self.balloon_count = 50;

            self.once = false;

            let circlexy = [(450.0, 400.0), (1050.0, 150.0), (1050.0, 450.0), (275.0, 550.0), (1050.0, 300.0), (170.0, 240.0)];
            
            for (x1, y1) in circlexy.iter() {
                self.circles.push(Circle {
                    pos: Vec2 { x: *x1, y: *y1 },
                    filled: false,
                    monkey: 10,
                });
            }
        }

        let mut dart_delete: Vec<usize> = Vec::with_capacity(16);
        let the_collisions = Collisiontwo::new(&self.balloons, &self.darts, &self.circles);
        let vec_coll_dart = the_collisions.check_collision_dart();
        if !vec_coll_dart.is_empty() {
            for (p, z) in vec_coll_dart.iter() {
                dart_delete.push(*p);
                if self.balloons.len() > *z {
                    self.balloons[*z].health -= 1;
                    self.points += 1;
                    if self.balloons[*z].health == 0 {
                        self.balloons.remove(*z);
                        self.balloon_count -= 1;
                    }
                }
            }
        }
        dart_delete.sort();
        for i in dart_delete.iter() {
            if self.darts.len() > *i {
                self.darts.remove(*i);
                self.dart_count -= 1;
            }
        }

        for balloon in self.balloons.iter_mut() {
            balloon.balloon_change_velocity();
            if !self.pause {
                balloon.pos.x += balloon.vel.x;
                balloon.pos.y += balloon.vel.y;
            }
        }

        for monkey in self.monkeys.iter_mut() {
            if monkey.action_time.elapsed() > Duration::from_millis(700) {
                if monkey.dir == 0 {
                    self.darts.push(Dart {
                        pos: monkey.pos,
                        vel: Vec2 { x: 0.0, y: -4.0 },
                        dir: monkey.dir,
                    });
                    monkey.action_time = Instant::now();
                    self.dart_count += 1;
                } else if monkey.dir == 1 {
                    self.darts.push(Dart {
                        pos: monkey.pos,
                        vel: Vec2 { x: 4.0, y: 0.0 },
                        dir: monkey.dir,
                    });
                    monkey.action_time = Instant::now();
                    self.dart_count += 1;
                } else if monkey.dir == 2 {
                    self.darts.push(Dart {
                        pos: monkey.pos,
                        vel: Vec2 { x: 0.0, y: 4.0 },
                        dir: monkey.dir,
                    });
                    monkey.action_time = Instant::now();
                    self.dart_count += 1;
                } else if monkey.dir == 3 {
                    self.darts.push(Dart {
                        pos: monkey.pos,
                        vel: Vec2 { x: -4.0, y: 0.0 },
                        dir: monkey.dir,
                    });
                    monkey.action_time = Instant::now();
                    self.dart_count += 1;
                }
            }
        }

        let mut dart_delete: Vec<usize> = Vec::with_capacity(16);
        for (dart_index, dart) in self.darts.iter_mut().enumerate() {
            if !self.pause {
                dart.pos.y += dart.vel.y;
                if dart.pos.y < 0.0 || dart.pos.y > 4000.0 {
                    dart_delete.push(dart_index);
                }
                dart.pos.x += dart.vel.x;
                if dart.pos.x < 0.0 || dart.pos.x > 4000.0 {
                    dart_delete.push(dart_index);
                }
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
                let the_collisions = Collisiontwo::new(&self.balloons, &self.darts, &self.circles);
                let circle_interact = the_collisions.circle_monkey(mouse_x, mouse_y);
                if circle_interact != 10
                    && !self.circles[circle_interact].filled
                    && self.points >= MONKEY_COST
                {
                    self.monkeys.push(Monkey {
                        pos: Vec2 {
                            x: self.circles[circle_interact].pos.x,
                            y: self.circles[circle_interact].pos.y,
                        },
                        action_time: Instant::now(),
                        dir: 10,
                        circle: circle_interact,
                    });
                    self.monkey_count += 1;
                    self.circles[circle_interact].filled = true;
                    self.circles[circle_interact].monkey = self.monkey_count - 1;
                    self.points -= MONKEY_COST;
                }
            }
        } else if engine.input.is_mouse_down(winit::event::MouseButton::Left) {
            self.mouse_clicked = true;
        }

        if engine
            .input
            .is_key_released(winit::event::VirtualKeyCode::P)
        {
            self.pause = !self.pause;
        }
        if engine
            .input
            .is_mouse_released(winit::event::MouseButton::Left)
        {
            let mouse_pos = engine.input.mouse_pos();
            let (mouse_x, mouse_y) = convert_mouse_pos(mouse_pos.into());
            let the_collisions = Collisiontwo::new(&self.balloons, &self.darts, &self.circles);
            let circle_interact = the_collisions.circle_monkey(mouse_x, mouse_y);
            if circle_interact != 10 && self.circles[circle_interact].filled {
                self.monkeys[self.circles[circle_interact].monkey].dir += 1;
                if self.monkeys[self.circles[circle_interact].monkey].dir > 3 {
                    self.monkeys[self.circles[circle_interact].monkey].dir = 0;
                }
            }
        }

        //Handles the placement of plants
    }

    fn render(&mut self, engine: &mut Engine) {
        //draw bg
        if self.start {
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
        } else {
            engine.draw_sprite(
                self.spritesheet0,
                AABB {
                    center: Vec2 {
                        x: W / 2.0,
                        y: H / 2.0,
                    },
                    size: Vec2 { x: W, y: H },
                },
                SheetRegion::new(0, 0, 0, 16, 1034, 1048),
            );
        }

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
            if monkey.dir == 0 {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: monkey.pos,
                        size: MONKEY_SIZE_PEASHOOTER,
                    },
                    SPRITE_MONKEY_PEASHOOTER1,
                );
            } else if monkey.dir == 1 {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: monkey.pos,
                        size: MONKEY_SIZE_PEASHOOTER,
                    },
                    SPRITE_MONKEY_PEASHOOTER2,
                );
            }
            if monkey.dir == 2 {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: monkey.pos,
                        size: MONKEY_SIZE_PEASHOOTER,
                    },
                    SPRITE_MONKEY_PEASHOOTER3,
                );
            }
            if monkey.dir == 3 {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: monkey.pos,
                        size: MONKEY_SIZE_PEASHOOTER,
                    },
                    SPRITE_MONKEY_PEASHOOTER4,
                );
            }
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
            Vec2 { x: 75.0, y: 550.0 },
            48.0,
        );

        for dart in self.darts.iter() {
            if dart.dir == 0 {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: dart.pos,
                        size: DART_SIZE,
                    },
                    SPRITE_DART,
                );
            } else if dart.dir == 1 {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: dart.pos,
                        size: DART_SIZE2,
                    },
                    SPRITE_DART3,
                );
            } else if dart.dir == 2 {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: dart.pos,
                        size: DART_SIZE,
                    },
                    SPRITE_DART2,
                );
            } else if dart.dir == 3 {
                engine.draw_sprite(
                    self.spritesheet3,
                    AABB {
                        center: dart.pos,
                        size: DART_SIZE2,
                    },
                    SPRITE_DART1,
                );
            }
        }

        if self.balloons.is_empty() && self.start {
            engine.draw_sprite(
                self.spritesheet6,
                AABB {
                    center: Vec2 {
                        x: W / 2.0,
                        y: H / 2.0,
                    },
                    size: Vec2 { x: 900.0, y: 200.0 },
                },
                SheetRegion::new(0, 0, 0, 0, 447, 106),
            );
        }

        for balloon in self.balloons.iter() {
            if balloon.segment == 14 {
                self.circles = Vec::new();
                self.monkeys = Vec::new();
                self.darts = Vec::new();
                engine.draw_sprite(
                    self.spritesheet7,
                    AABB {
                        center: Vec2 {
                            x: W / 2.0,
                            y: H / 2.0,
                        },
                        size: Vec2 { x: 600.0, y: 570.0 },
                    },
                    SheetRegion::new(0, 0, 0, 0, 388, 314),
                );
            }
        }
    }
}
fn main() {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let sound_data =
        StaticSoundData::from_file("assets/btd.mp3", StaticSoundSettings::default()).unwrap();

    let _ = manager.play(sound_data.clone());

    Engine::new(winit::window::WindowBuilder::new()).run::<Game>();
}
