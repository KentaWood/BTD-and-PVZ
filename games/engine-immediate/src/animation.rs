pub struct Animation {
    x_move: f32,
    y_move: f32,
}

impl Animation {
    pub fn new(x_move: f32, y_move: f32) -> Self {
        Animation { x_move, y_move }
    }

    pub fn make_move(&self, screen_region: [f32; 4], pause: bool) -> [f32; 4] {
        if pause {return screen_region; }
        else {return [
            screen_region[0] + self.x_move,
            screen_region[1] + self.y_move,
            screen_region[2],
            screen_region[3],
        ];}
    }
}

       