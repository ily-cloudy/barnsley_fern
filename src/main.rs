use rand::Rng;
use minifb::{Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() { 
    // graphics set-up
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Barnsley Fractal",          
        WIDTH,                      
        HEIGHT,                     
        WindowOptions::default(),   
    )
    .unwrap_or_else(|e| panic!("{}", e));
    window.set_background_color(0, 0, 0);


    while window.is_open() && ! window.is_key_down(minifb::Key::Escape) {
        let mut rng = rand::thread_rng();
        
        // initial position
        let mut x: f64 = 0.00;
        let mut y: f64 = 0.00;
        
        // affine transformations
        let mut n: i32 = 1;
        while n < 10000 {
            let r: f64 = rng.gen();
            let x_next: f64; 
            let y_next: f64;

            if r < 0.01 {
                x_next = 0.00;
                y_next = 0.16 * y;

            } else if r < 0.86 {
                x_next = 0.85 * x + 0.04 * y;
                y_next = -0.04 * x + 0.85 * y + 1.60;

            } else if r < 0.93 {
                x_next = 0.20 * x - 0.26 * y;
                y_next = 0.23 * x + 0.22 * y + 1.60;

            } else {
                x_next = -0.15 * x + 0.25 * y;
                y_next = 0.26 * x + 0.24 * y + 0.44;

            }
            
            // assigning new position
            x = x_next;
            y = y_next;

            // scaling points
            let x_pos: usize = (x * WIDTH as f64 / 10.0 + WIDTH as f64 / 2.0) as usize;
            let y_pos: usize = (y * HEIGHT as f64 / 12.0 + HEIGHT as f64 / 2.0 - 300.0) as usize;

            // flipping image due to how minifb does coordinates 
            let y_pos_flipped: usize = HEIGHT - y_pos - 1; 
            buffer[y_pos_flipped * WIDTH + x_pos] = 0x006400;
            
            n += 1;
        }
        // update window
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}
