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

fn parse(content: &str) -> Option<Image> {
    let mut tokens = content
        .lines()
        .map(|line| {
            if let Some(idx) = line.find('#') {
                &line[..idx]
            } else {
                line
            }
        })
        .flat_map(|line| line.split_whitespace());

    let mut next_token = || tokens.next();

    if next_token()? != "P3" {
        return None;
    }

    let width: usize = next_token()?.parse().ok()?;
    let height: usize = next_token()?.parse().ok()?;
    let _max_value: u16 = next_token()?.parse().ok()?;

    let mut pixels = Vec::with_capacity(width * height);
    for _ in 0..(width * height) {
        let r: u8 = next_token()?.parse().ok()?;
        let g: u8 = next_token()?.parse().ok()?;
        let b: u8 = next_token()?.parse().ok()?;
        pixels.push(Pixel { r, g, b });
    }

    Some(Image {
        width,
        height,
        pixels,
    })
}

fn render(image: &Image) {
    let mut buffer: Vec<u32> = Vec::with_capacity(image.width * image.height);
    for pixel in &image.pixels {
        let color = ((pixel.r as u32) << 16) | ((pixel.g as u32) << 8) | (pixel.b as u32);
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
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            return;
        }
    };
    let content = fs::read_to_string(filename).unwrap();
    let image = parse(&content).unwrap();
    render(&image);
}
