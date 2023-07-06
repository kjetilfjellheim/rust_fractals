use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use web_sys::{CanvasRenderingContext2d};

extern crate web_sys;
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Clone)]
struct Triangle {
    elements: Vec<Triangle>,
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {

    fn new(a: Point, b: Point, c: Point) -> Triangle {
        
        Triangle {
            a: a,
            b: b,
            c: c,
            elements: Vec::new()
        }
    }

    fn generate(self, current_depth: i32, max_depth: i32) -> Triangle {
        log!("{}", current_depth);
        let mut elements: Vec<Triangle> = Vec::new();
        if current_depth < max_depth {
            elements.push(Triangle::get_sub_point(self.a, self.b, self.c).generate(current_depth + 1, max_depth));
            elements.push(Triangle::get_sub_point(self.b, self.c, self.a).generate(current_depth + 1, max_depth));
            elements.push(Triangle::get_sub_point(self.c, self.a, self.b).generate(current_depth + 1, max_depth));
        }
        Triangle {
            a: self.a,
            b: self.b,
            c: self.c,
            elements: elements

        }
    }

    fn get_sub_point(a: Point, b: Point, c: Point) -> Triangle {
        Triangle {
            a: a,
            b: Point::new((a.x + b.x) / 2, (a.y + b.y) / 2),
            c: Point::new((a.x + c.x) / 2, (a.y + c.y) / 2),
            elements: Vec::new()
        }
    }


    fn draw(self, canvas: &CanvasRenderingContext2d) {
        canvas.set_fill_style(&JsValue::from_str("black"));
        canvas.begin_path();
        canvas.set_line_width(0.5);
        canvas.move_to(self.a.x as f64, self.a.y as f64);
        canvas.line_to(self.b.x as f64, self.b.y as f64);
        canvas.line_to(self.c.x as f64, self.c.y as f64);
        canvas.line_to(self.a.x as f64, self.a.y as f64);
        canvas.stroke();
        canvas.close_path();

        for element in self.elements {
            element.draw(canvas);
        }
    }

}

#[wasm_bindgen]
pub fn render(pixel_width: u32, depth: i32, context: &CanvasRenderingContext2d) {
    let a: Point = Point::new(pixel_width / 2, 0);
    let b: Point = Point::new(0, pixel_width);
    let c: Point = Point::new(pixel_width, pixel_width);

    let triangle: Triangle = Triangle::new(a, b, c);
    let triangle = triangle.generate(0, depth);
    triangle.draw(context);
}
