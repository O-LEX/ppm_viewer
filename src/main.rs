use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs;

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

fn parse_p3(contents: &str) -> Image {
    let mut tokens = contents.split_whitespace();
    let magic = tokens.next().unwrap();
    if magic != "P3" {
        panic!("Unsupported format: {}", magic);
    }

    let width: usize = tokens.next().unwrap().parse().unwrap();
    let height: usize = tokens.next().unwrap().parse().unwrap();
    let _max_value: u8 = tokens.next().unwrap().parse().unwrap();

    let mut pixels = Vec::with_capacity(width * height);

    for _ in 0..(width * height) {
        let r: u8 = tokens.next().unwrap().parse().unwrap();
        let g: u8 = tokens.next().unwrap().parse().unwrap();
        let b: u8 = tokens.next().unwrap().parse().unwrap();
        pixels.push(Pixel { r, g, b });
    }

    Image {
        width,
        height,
        pixels,
    }
}

fn render(image: &Image) {
    let mut buffer: Vec<u32> = Vec::with_capacity(image.width * image.height);

    for pixel in &image.pixels {
        let r = pixel.r as u32;
        let g = pixel.g as u32;
        let b = pixel.b as u32;

        let color = (r << 16) | (g << 8) | b;
        buffer.push(color);
    }

    let window_height = 600;
    let window_width = (image.width * window_height) / image.height;

    let mut window = Window::new(
        "PPM Viewer",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, image.width, image.height)
            .unwrap();
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(&filename).unwrap();

    let image = parse_p3(&contents);
    render(&image);
}
