use frenderer::{Camera2D, Transform};
use std::{ borrow::Cow, mem, time::{Duration, Instant}, };
pub use glam::*;

#[derive(Clone, Copy)]
pub struct Plant {
    pub pos: Vec2,

    pub action_time: Instant,
    pub placed: bool,

}

#[derive(Clone, Copy)]
pub struct Pea {
    pub pos: Vec2,
    pub vel: Vec2,
}

#[derive(Clone, Copy)]
pub struct Zombie {
    pub pos: Vec2,
    pub vel: Vec2,
    pub health: usize,
}


#[derive(Clone, Copy)]
pub struct Monkey {
    pub pos: Vec2,
    pub dart: bool,
}

#[derive(Clone, Copy)]
pub struct Dart {
    pub pos: Vec2,
    pub vel: Vec2,
    pub monkey_num: usize,
}

#[derive(Clone, Copy)]
pub struct Balloon {
    pub pos: Vec2,
    pub vel: Vec2,
    pub health: usize,
    pub segment: usize,
}

impl Balloon {
    pub fn balloon_change_velocity(&mut self) {
        if self.segment == 0 && self.pos.x >= 280.0 {
            self.vel = Vec2 { x: 0.0, y: 1.5 };
            self.segment = 1;
        } else if self.segment == 1 && self.pos.y >= 475.0 {
            self.vel = Vec2 { x: 1.5, y: 0.0 };
            self.segment = 2;
        } else if self.segment == 2 && self.pos.x >= 600.0 {
            self.vel = Vec2 { x: 0.0, y: -1.0 };
            self.segment = 3;
        } else if self.segment == 3 && self.pos.y <= 175.0 {
            self.vel = Vec2 { x: -1.5, y: 0.0 };
            self.segment = 4;
        } else if self.segment == 4 && self.pos.x <= 150.0 {
            self.vel = Vec2 { x: 0.0, y: -1.0 };
            self.segment = 5;
        } else if self.segment == 5 && self.pos.y <= 80.0 {
            self.vel = Vec2 { x: 1.5, y: 0.0 };
            self.segment = 6;
        } else if self.segment == 6 && self.pos.x >= 1200.0 {
            self.vel = Vec2 { x: 0.0, y: 1.0 };
            self.segment = 7;
        } else if self.segment == 7 && self.pos.y >= 240.0 {
            self.vel = Vec2 { x: -1.5, y: 0.0 };
            self.segment = 8;
        } else if self.segment == 8 && self.pos.x <= 890.0 {
            self.vel = Vec2 { x: 0.0, y: 1.0 };
            self.segment = 9;
        } else if self.segment == 9 && self.pos.y >= 370.0 {
            self.vel = Vec2 { x: 1.5, y: 0.0 };
            self.segment = 10;
        } else if self.segment == 10 && self.pos.x >= 1200.0 {
            self.vel = Vec2 { x: 0.0, y: 1.0 };
            self.segment = 11;
        } else if self.segment == 11 && self.pos.y >= 540.0 {
            self.vel = Vec2 { x: -1.5, y: 0.0 };
            self.segment = 12;
        } else if self.segment == 12 && self.pos.x <= 770.0 {
            self.vel = Vec2 { x: 0.0, y: 1.0 };
            self.segment = 13;
        } else if self.segment == 13 && self.pos.y >= 610.0 {
            self.segment = 14;
        } 
    } 
}

    
        // 770, 540
        // 770, 610


#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod, Debug)]
pub struct Rect {
    pub corner: Vec2,
    pub size: Vec2,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod, Debug)]
pub struct AABB {
    pub center: Vec2,
    pub size: Vec2,
}

impl From<AABB> for Transform {
    fn from(val: AABB) -> Self {
        Transform {
            w: val.size.x as u16,
            h: val.size.y as u16,
            x: val.center.x,
            y: val.center.y,
            rot: 0.0,
        }
    }
}

impl From<Rect> for Transform {
    fn from(val: Rect) -> Self {
        Transform {
            w: val.size.x as u16,
            h: val.size.y as u16,
            x: val.corner.x + val.size.x / 2.0,
            y: val.corner.y + val.size.y / 2.0,
            rot: 0.0,
        }
    }
}

impl From<Rect> for Camera2D {
    fn from(val: Rect) -> Self {
        Camera2D {
            screen_pos: val.corner.into(),
            screen_size: val.size.into(),
        }
    }
}

impl From<AABB> for Camera2D {
    fn from(val: AABB) -> Self {
        Camera2D {
            screen_pos: (val.center - val.size / 2.0).into(),
            screen_size: val.size.into(),
        }
    }
}

impl From<AABB> for Rect {
    fn from(val: AABB) -> Self {
        Rect {
            corner: (val.center - val.size / 2.0),
            size: val.size,
        }
    }
}

impl From<Rect> for AABB {
    fn from(val: Rect) -> Self {
        AABB {
            center: (val.corner + val.size / 2.0),
            size: val.size,
        }
    }
}

impl Rect {
    pub fn displacement(&self, other: Rect) -> Option<Vec2> {
        let x_overlap = (self.corner.x + self.size.x).min(other.corner.x + other.size.x)
            - self.corner.x.max(other.corner.x);
        let y_overlap = (self.corner.y + self.size.y).min(other.corner.y + other.size.y)
            - self.corner.y.max(other.corner.y);
        if x_overlap >= 0.0 && y_overlap >= 0.0 {
            // This will return the magnitude of overlap in each axis.
            Some(Vec2 {
                x: x_overlap,
                y: y_overlap,
            })
        } else {
            None
        }
    }
}

impl AABB {
    pub fn displacement(&self, other: AABB) -> Option<Vec2> {
        Rect::from(*self).displacement(Rect::from(other))
    }
}