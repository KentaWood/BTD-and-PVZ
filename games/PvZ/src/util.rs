pub fn screen_to_grid(x: f32, y: f32) -> (f32, f32) {
    let mut grid_x = 700.0; // Default values
    let mut grid_y = 300.0;

    if x > 240.0 && x < 340.0 {
        grid_x = 280.0;
    }

    if x > 340.0 && x < 410.0 {
        grid_x = 380.0;
    }

    if x > 410.0 && x < 500.0 {
        grid_x = 450.0;
    }

    if x > 500.0 && x < 575.0 {
        grid_x = 540.0;
    }

    if x > 575.0 && x < 660.0 {
        grid_x = 620.0;
    }

    if x > 660.0 && x < 730.0 {
        grid_x = 700.0;
    }

    if x > 730.0 && x < 815.0 {
        grid_x = 780.0;
    }

    if x > 815.0 && x < 900.0 {
        grid_x = 850.0;
    }

    if x > 900.0 && x < 1000.0 {
        grid_x = 940.0;
    }
    //y- cord
    if y > 30.0 && y < 125.0 {
        grid_y = 70.0;
    }

    if y > 125.0 && y < 220.0 {
        grid_y = 175.0;
    }

    if y > 220.0 && y < 320.0 {
        grid_y = 265.0;
    }

    if y > 320.0 && y < 415.0 {
        grid_y = 370.0;
    }

    if y > 415.0 && y < 535.0 {
        grid_y = 470.0;
    }

    (grid_x, grid_y)
}
pub fn convert_mouse_pos(mouse_pos: (i32, i32)) -> (f32, f32) {
    let mouse_x = mouse_pos.0 as f32 / 1.142_857_2;
    let mouse_y = (1200.0 - mouse_pos.1 as f32) / 2.0;
    (mouse_x, mouse_y)
}