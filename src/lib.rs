// https://wasmbyexample.dev/examples/reading-and-writing-graphics/reading-and-writing-graphics.rust.en-us.html
#![allow(dead_code)]
use image;
use image::GenericImage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    println!("The Rust function say() received {}", s);
    let r = String::from("hello ");
    return r + s;
}

enum Square {
    Canvas(Vec<u8>, usize),
    Img(image::DynamicImage, u32),
}

impl Square {
    fn new_canvas(size: usize) -> Square {
        Square::Canvas(vec![0u8; size * size * 4], size)
    }

    fn new_img(size: u32) -> Square {
        Square::Img(image::DynamicImage::new_rgba8(size, size), size)
    }

    fn set_point(&mut self, x: f64, y: f64, r: u8, g: u8, b: u8, a: u8) {
        let size = match self {
            Square::Canvas(_, size) => *size as isize,
            Square::Img(_, size) => *size as isize,
        };
        let center = size / 2;
        let mut x = (x as isize) + center;
        let mut y = (y as isize) + center;
        if x < 0 {
            x = 0;
        };
        if y < 0 {
            y = 0;
        };
        if x >= size {
            x = size - 1
        };
        if y >= size {
            y = size - 1
        };
        match self {
            Square::Canvas(meta, _) => {
                let idx = (x * y * 4) as usize;
                meta[idx] = r;
                meta[idx] = g;
                meta[idx] = b;
                meta[idx] = a;
            }
            Square::Img(img, _) => img.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a])),
        }
    }

    fn into_data(self) -> Vec<u8> {
        match self {
            Square::Canvas(meta, _) => meta,
            Square::Img(img, _) => {
                let mut data = vec![];
                {
                    let mut buf = std::io::BufWriter::new(&mut data);
                    let _ = img.write_to(&mut buf, image::ImageFormat::Png);
                }
                data
            }
        }
    }
}

static mut BUFFER: Vec<u8> = vec![];
#[wasm_bindgen]
pub fn get_buffer_size() -> usize {
    unsafe { BUFFER.len() }
}

#[wasm_bindgen]
pub fn draw(
    outer_r: u32,
    inner_r: u32,
    dist: u32,
    square_size: u32,
    r: u8,
    g: u8,
    blue: u8,
) -> Vec<u8> {
    let b = blue;
    let outer_r = outer_r as f64;
    let inner_r = inner_r as f64;
    let dist = dist as f64;
    let mut p = Pannel {
        outer_r,
        inner_r,
        distance: dist,
        alpha: 0.0,
        beta: 0.0,
    };
    let mut square = Square::new_img(square_size);
    let step = std::f64::consts::PI / 720.0;
    for _ in 0..72000 {
        let (x, y) = p.step(step);
        square.set_point(x, y, r, g, b, 255);
    }
    square.into_data()
    // unsafe {
    //     BUFFER = square.into_data();
    //     BUFFER.as_ptr()
    // }
}

struct Pannel {
    outer_r: f64,
    inner_r: f64,
    distance: f64,
    alpha: f64,
    beta: f64,
}
impl Pannel {
    fn step(&mut self, alpha_delta: f64) -> (f64, f64) {
        self.alpha += alpha_delta;
        let a = alpha_delta * self.outer_r;
        let beta_delta = a / self.inner_r;
        self.beta += beta_delta;
        self.current_pos()
    }

    #[inline]
    fn current_pos(&self) -> (f64, f64) {
        let a = self.beta - self.alpha;
        let delta_y = a.sin() * self.distance;
        let delta_x = a.cos() * self.distance;

        let center_y = self.alpha.sin() * (self.outer_r - self.inner_r);
        let center_x = self.alpha.cos() * (self.outer_r - self.inner_r);
        let x = center_x - delta_x;
        let y = center_y - delta_y;
        (x, y)
    }
}
