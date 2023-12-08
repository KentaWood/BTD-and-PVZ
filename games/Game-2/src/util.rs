pub fn convert_mouse_pos(mouse_pos: (i32, i32)) -> (f32, f32) {
    let mouse_x = mouse_pos.0 as f32 / 1.142_857_2;
    let mouse_y = (1200.0 - mouse_pos.1 as f32) / 2.0;
    (mouse_x, mouse_y)
}
