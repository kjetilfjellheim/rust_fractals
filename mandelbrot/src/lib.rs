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

struct MandelbrotResponse {
    z: Complex<f64>,
    iterations: i32
}

struct CenterPoint {
    center_x: f64,
    center_y: f64,
}

struct Viewport {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

#[derive(Debug, Copy, Clone)]
struct Image {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
pub fn render(pixel_width: u32, max_iterations: i32, context: &CanvasRenderingContext2d) {
    render_with_optional_canvas(pixel_width, max_iterations, Some(context));
}

pub fn render_with_optional_canvas(pixel_width: u32, max_iterations: i32, context: Option<&CanvasRenderingContext2d>,) {

    let scale: f64 = 0.0000001;

    let centerpoint: CenterPoint = CenterPoint {
        center_x: -0.55,
        center_y: -0.55,
    };

    let default_viewport: Viewport = Viewport {
        min_x: -1.0,
        min_y: -3.0,
        max_x: 5.0,
        max_y: 3.0
    };

    let image: Image = Image {
        width: pixel_width,
        height: pixel_width,
    };

    let scaled_viewport = get_viewport(default_viewport, centerpoint, scale);

    let mut data = get_mandelbrotdata(image, scaled_viewport, max_iterations);

    let img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut data),
        image.width,
        image.height
    )
    .expect("Imagedata was not correctly created");

    if context.is_some() { 
        context.unwrap().put_image_data(&img_data, 0.0, 0.0)
        .expect("Could not add image data to context"); 
    }
}

fn get_mandelbrotdata(image: Image, viewport: Viewport, max_iterations: i32) -> Vec<u8> {

    let mut data: Vec<u8> = Vec::new();

    for x in 0..image.width {
        for y in 0..image.height {
            let c = Complex::<f64>::new(
                viewport.min_y + (y as f64 / image.height as f64) * (viewport.max_y - viewport.min_y),
                viewport.min_x + (x as f64 / image.width as f64) * (viewport.max_x - viewport.min_x),                
            );
            let mandelbrot_response = mandelbrot(c, max_iterations);
            let clr = get_color(mandelbrot_response, max_iterations);
            data.push(clr.red);
            data.push(clr.green);
            data.push(clr.blue);
            data.push(255);
        }
    }
    data
}

fn get_viewport(default_viewport: Viewport, centerpoint : CenterPoint, scale: f64) -> Viewport {
    let new_width =  (default_viewport.max_x - default_viewport.min_x) * scale;
    let new_height =  (default_viewport.max_y - default_viewport.min_y) * scale;
    Viewport {
        min_x: centerpoint.center_x - (new_width / 2.0),
        min_y: centerpoint.center_y - (new_height / 2.0),
        max_x: centerpoint.center_x + (new_width / 2.0),
        max_y: centerpoint.center_y + (new_height / 2.0),

    }
}

fn get_color(mandelbrot_response: MandelbrotResponse, max_iterations: i32) -> Color {
    let factor: f64 = (mandelbrot_response.iterations as f64 / max_iterations as f64) * 255 as f64;    
    let logs = 2.0 + f64::log2(f64::log2(abs(mandelbrot_response.z) + 1.0 ) / f64::log2(2.0));
    let red = (factor + 5.0 - logs).floor() as u8;
    let green = red;
    let blue = (0.0 + (255.0 - 0.0) * (red as f64 / 255.0)) as u8;
    Color {
        red: red,
        green: green,
        blue: blue
    }
}

fn mandelbrot(c: Complex<f64>, max_iterations: i32) -> MandelbrotResponse {
    let mut z: Complex<f64> = Complex::new(0.0, 0.0);
    let mut n: i32 = 0;
    while abs(z) <= 2.0 && n < max_iterations {
        z = (z * z) + c;
        n += 1;
    }
    MandelbrotResponse { z: z, iterations: n }
}

fn abs(c: Complex<f64>) -> f64 {
    f64::sqrt((c.im as f64 * c.im as f64) + (c.re as f64 * c.re as f64))
}
