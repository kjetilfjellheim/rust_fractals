use num::complex::Complex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

extern crate web_sys;
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

const COLOR_MAPPING: [Color;16] = [
    Color {
        red: 0, green: 0, blue: 0
    },
    Color {
        red: 0, green: 0, blue: 32
    },
    Color {
        red: 0, green: 0, blue: 64
    },
    Color {
        red: 0, green: 0, blue: 92
    },
    Color {
        red: 0, green: 0, blue: 128
    },
    Color {
        red: 0, green: 0, blue: 160
    },
    Color {
        red: 0, green: 0, blue: 192
    },
    Color {
        red: 0, green: 0, blue: 224
    },
    Color {
        red: 0, green: 0, blue: 255
    },
    Color {
        red: 32, green: 0, blue: 224
    },
    Color {
        red:64, green: 0, blue: 192
    },
    Color {
        red: 128, green: 0, blue: 160
    },
    Color {
        red: 160, green: 0, blue: 128
    },
    Color {
        red: 192, green: 0, blue: 96
    },
    Color {
        red: 224, green: 0, blue: 64
    },
    Color {
        red: 255, green: 0, blue: 0
    }
    ];

const ITERATIONS: i32 = 100000;

const START_IMG: f64 = -1.0;
const END_IMG: f64 = 5.0;
const START_REAL: f64 = -3.0;
const END_REAL: f64 = 3.0;

#[wasm_bindgen]
pub fn render(width: i32, height: i32, context: &CanvasRenderingContext2d) {

    let mut data = get_mandelbrotdata(width, height);

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut data),
        width as u32,
        height as u32,
    )
    .expect("Imagedata was not correctly created");

    context.put_image_data(&data, 0.0, 0.0)
    .expect("Could put image data");
}

pub fn get_mandelbrotdata(width: i32, height: i32) -> Vec<u8> {

    let mut data = Vec::new();

    for x in 0..width {
        for y in 0..height {
            let c = Complex::<f64>::new(
                START_REAL + (y as f64 / height as f64) * (END_REAL - START_REAL),
                START_IMG + (x as f64 / width as f64) * (END_IMG - START_IMG),                
            );
            let m = mandelbrot(c);
            let clr = get_colorindex(m);
            data.push(COLOR_MAPPING[clr as usize].red);
            data.push(COLOR_MAPPING[clr as usize].green);
            data.push(COLOR_MAPPING[clr as usize].blue);
            data.push(255);
        }
    }
    data
}

fn get_colorindex(iterations: i32) -> i32 {
    iterations % COLOR_MAPPING.len() as i32    
}

fn mandelbrot(c: Complex<f64>) -> i32 {
    let mut z: Complex<f64> = Complex::new(0.0, 0.0);
    let mut n: i32 = 0;
    while abs(z) <= 2.0 && n < ITERATIONS {
        z = (z * z) + c;
        n += 1;
    }
    n
}

fn abs(c: Complex<f64>) -> f64 {
    f64::sqrt((c.im as f64 * c.im as f64) + (c.re as f64 * c.re as f64))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_mandelbrot() {
        let data = get_mandelbrotdata(100, 100);
        println!("Generated");
    }
}
