use minifb::*;

// Constants for the screen
const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
    let mut pixels = vec![color_from_rgb(255, 87, 51); WIDTH * HEIGHT];
    let mut window = Window::new("Fractals", 600, 600, WindowOptions {
        resize: true,
        ..WindowOptions::default()
    }).expect("Unexpected error opening new window");
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
