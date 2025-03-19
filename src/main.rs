use std::time::SystemTime;

use minifb::*;

use fractals::mandelbrot::MandelbrotSet;

// Constants for the screen
const WIDTH: usize = 600;
const HEIGHT: usize = 600;
fn main() {
    // Initialize Window
    let mut pixels = vec![color_from_rgb(255, 87, 51); WIDTH * HEIGHT];
    let mut window = Window::new("Fractals", WIDTH, HEIGHT, WindowOptions {
        // resize: true,
        ..WindowOptions::default()
    }).expect("Unexpected error opening new window");
    window.update_with_buffer(&pixels, WIDTH, HEIGHT).expect("Failed to update frame buffer");
    window.set_target_fps(144);
    // Init Mandelbrot Set
    let fractal = MandelbrotSet::new();
    let pois = vec![
        (0.001643721971153, 0.822467633298876),
        (-0.7901, 0.15555),
        (-0.1624, 1.04095),
        (0.377, -0.1),
        (-1.485, 0.0),
        (-0.14705, -0.859),
        (0.28011, 0.00803),
    ];
    let mut time = SystemTime::now();
    let mut zoom: f64 = 1.0;
    let mut index = 0;
    let (mut off_x, mut off_y) = pois[index];
    while window.is_open() {
        // Display Fractals
        show_fractal(&mut window, &mut pixels, &fractal, zoom, off_x, off_y);
        zoom = 1.0005_f64.powf(time.elapsed().unwrap().as_millis() as f64);
        if zoom > 100000.0 {
            time = SystemTime::now();
            index = (index + 1) % pois.len();
            (off_x, off_y) = pois[index];
        }
    }
}

fn show_fractal(window: &mut Window, pixels: &mut Vec<u32>, fractal: &MandelbrotSet, zoom: f64, off_x: f64, off_y: f64) {
    pixels.iter_mut().enumerate().for_each(|(index, pixel)| {
        let pix_x = index % WIDTH;
        let pix_y = index / WIDTH;
        let x =  (pix_x as f64 / WIDTH as f64 * 6.0 - 4.0) / zoom + off_x;
        let y = (pix_y as f64 / HEIGHT as f64 * 6.0 - 3.0) / zoom + off_y;
        let (r, g, b) = fractal.process(x, y);
        *pixel = color_from_rgb(r, g, b);
    });
    window.update_with_buffer(&pixels, WIDTH, HEIGHT).expect("Failed to update frame buffer");
}

fn color_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    (r << 16) | (g << 8) | b
}

fn set_background(pixels: &mut Vec<u32>, r: u8, g: u8, b: u8) {
    pixels.fill(color_from_rgb(r, g, b));
}
